use common::solana::{get_fee_payer, get_signers};
use proto::pb::magiceden::m3::v1 as pb;
use substreams::errors::Error;
use substreams_solana::{
    block_view::InstructionView,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction},
};
use substreams_solana_idls::magiceden::m3;

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, Error> {
    Ok(pb::Events {
        transactions: block.transactions_owned().filter_map(process_transaction).collect(),
    })
}

fn process_transaction(tx: ConfirmedTransaction) -> Option<pb::Transaction> {
    let tx_meta = tx.meta.as_ref()?;

    let instructions: Vec<pb::Instruction> = tx.walk_instructions().filter_map(|iview| process_instruction(&iview)).collect();

    if instructions.is_empty() {
        return None;
    }

    Some(pb::Transaction {
        fee: tx_meta.fee,
        compute_units_consumed: tx_meta.compute_units_consumed(),
        signature: tx.hash().to_vec(),
        fee_payer: get_fee_payer(&tx).unwrap_or_default(),
        signers: get_signers(&tx).unwrap_or_default(),
        instructions,
    })
}

fn process_instruction(ix: &InstructionView) -> Option<pb::Instruction> {
    let program_id = ix.program_id().0;

    if program_id != &m3::PROGRAM_ID {
        return None;
    }

    let instruction = match m3::instructions::unpack(ix.data()) {
        Ok(m3::instructions::MagicEdenInstruction::SolDepositBuy(event)) => {
            pb::instruction::Instruction::SolDepositBuy(pb::SolDepositBuyInstruction {
                payment_amount: event.args.payment_amount,
            })
        }
        Ok(m3::instructions::MagicEdenInstruction::SolWithdrawBuy(event)) => {
            pb::instruction::Instruction::SolWithdrawBuy(pb::SolWithdrawBuyInstruction {
                payment_amount: event.args.payment_amount,
            })
        }
        Ok(m3::instructions::MagicEdenInstruction::SolFulfillBuy(event)) => {
            pb::instruction::Instruction::SolFulfillBuy(pb::SolFulfillBuyInstruction {
                asset_amount: event.args.asset_amount,
                min_payment_amount: event.args.min_payment_amount,
                maker_fee_bp: event.args.maker_fee_bp as i32,
                taker_fee_bp: event.args.taker_fee_bp as i32,
            })
        }
        Ok(m3::instructions::MagicEdenInstruction::SolFulfillSell(event)) => {
            pb::instruction::Instruction::SolFulfillSell(pb::SolFulfillSellInstruction {
                asset_amount: event.args.asset_amount,
                max_payment_amount: event.args.max_payment_amount,
                buyside_creator_royalty_bp: event.args.buyside_creator_royalty_bp as u32,
                maker_fee_bp: event.args.maker_fee_bp as i32,
                taker_fee_bp: event.args.taker_fee_bp as i32,
            })
        }
        Ok(m3::instructions::MagicEdenInstruction::DepositSell(event)) => {
            pb::instruction::Instruction::DepositSell(pb::DepositSellInstruction {
                asset_amount: event.args.asset_amount,
            })
        }
        Ok(m3::instructions::MagicEdenInstruction::WithdrawSell(event)) => {
            pb::instruction::Instruction::WithdrawSell(pb::WithdrawSellInstruction {
                asset_amount: event.args.asset_amount,
            })
        }
        Ok(m3::instructions::MagicEdenInstruction::SolMip1FulfillBuy(event)) => {
            pb::instruction::Instruction::SolMip1FulfillBuy(pb::SolMip1FulfillBuyInstruction {
                asset_amount: event.args.asset_amount,
                min_payment_amount: event.args.min_payment_amount,
                maker_fee_bp: event.args.maker_fee_bp as i32,
                taker_fee_bp: event.args.taker_fee_bp as i32,
            })
        }
        Ok(m3::instructions::MagicEdenInstruction::SolMip1FulfillSell(event)) => {
            pb::instruction::Instruction::SolMip1FulfillSell(pb::SolMip1FulfillSellInstruction {
                asset_amount: event.args.asset_amount,
                max_payment_amount: event.args.max_payment_amount,
                maker_fee_bp: event.args.maker_fee_bp as i32,
                taker_fee_bp: event.args.taker_fee_bp as i32,
            })
        }
        Ok(m3::instructions::MagicEdenInstruction::SolOcpFulfillBuy(event)) => {
            pb::instruction::Instruction::SolOcpFulfillBuy(pb::SolOcpFulfillBuyInstruction {
                asset_amount: event.args.asset_amount,
                min_payment_amount: event.args.min_payment_amount,
                maker_fee_bp: event.args.maker_fee_bp as i32,
                taker_fee_bp: event.args.taker_fee_bp as i32,
            })
        }
        Ok(m3::instructions::MagicEdenInstruction::SolOcpFulfillSell(event)) => {
            pb::instruction::Instruction::SolOcpFulfillSell(pb::SolOcpFulfillSellInstruction {
                asset_amount: event.args.asset_amount,
                max_payment_amount: event.args.max_payment_amount,
                maker_fee_bp: event.args.maker_fee_bp as i32,
                taker_fee_bp: event.args.taker_fee_bp as i32,
            })
        }
        _ => return None,
    };

    Some(pb::Instruction {
        program_id: program_id.to_vec(),
        stack_height: ix.stack_height(),
        instruction: Some(instruction),
    })
}
