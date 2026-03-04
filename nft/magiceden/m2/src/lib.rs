use common::solana::{get_fee_payer, get_signers};
use proto::pb::magiceden::m2::v1 as pb;
use substreams::errors::Error;
use substreams_solana::{
    block_view::InstructionView,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction},
};
use substreams_solana_idls::magiceden::m2;

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

    if program_id != &m2::PROGRAM_ID {
        return None;
    }

    let instruction = match m2::instructions::unpack(ix.data()) {
        Ok(m2::instructions::MagicEdenInstruction::Sell(event)) => {
            pb::instruction::Instruction::Sell(pb::SellInstruction {
                buyer_price: event.buyer_price,
                token_size: event.token_size,
                seller_state_expiry: event.seller_state_expiry,
            })
        }
        Ok(m2::instructions::MagicEdenInstruction::CancelSell(event)) => {
            pb::instruction::Instruction::CancelSell(pb::CancelSellInstruction {
                buyer_price: event.buyer_price,
                token_size: event.token_size,
                seller_state_expiry: event.seller_state_expiry,
            })
        }
        Ok(m2::instructions::MagicEdenInstruction::Buy(event)) => {
            pb::instruction::Instruction::Buy(pb::BuyInstruction {
                buyer_price: event.buyer_price,
                token_size: event.token_size,
                buyer_state_expiry: event.buyer_state_expiry,
            })
        }
        Ok(m2::instructions::MagicEdenInstruction::BuyV2(event)) => {
            pb::instruction::Instruction::BuyV2(pb::BuyV2Instruction {
                buyer_price: event.buyer_price,
                token_size: event.token_size,
                buyer_state_expiry: event.buyer_state_expiry,
                buyer_creator_royalty_bp: event.buyer_creator_royalty_bp as u32,
            })
        }
        Ok(m2::instructions::MagicEdenInstruction::CancelBuy(event)) => {
            pb::instruction::Instruction::CancelBuy(pb::CancelBuyInstruction {
                buyer_price: event.buyer_price,
                token_size: event.token_size,
                buyer_state_expiry: event.buyer_state_expiry,
            })
        }
        Ok(m2::instructions::MagicEdenInstruction::ExecuteSaleV2(event)) => {
            pb::instruction::Instruction::ExecuteSaleV2(pb::ExecuteSaleV2Instruction {
                buyer_price: event.buyer_price,
                token_size: event.token_size,
                buyer_state_expiry: event.buyer_state_expiry,
                seller_state_expiry: event.seller_state_expiry,
                maker_fee_bp: event.maker_fee_bp as i32,
                taker_fee_bp: event.taker_fee_bp as u32,
            })
        }
        Ok(m2::instructions::MagicEdenInstruction::OcpSell(event)) => {
            pb::instruction::Instruction::OcpSell(pb::OcpSellInstruction {
                price: event.args.price,
                expiry: event.args.expiry,
            })
        }
        Ok(m2::instructions::MagicEdenInstruction::OcpExecuteSaleV2(event)) => {
            pb::instruction::Instruction::OcpExecuteSaleV2(pb::OcpExecuteSaleV2Instruction {
                price: event.args.price,
                maker_fee_bp: event.args.maker_fee_bp as i32,
                taker_fee_bp: event.args.taker_fee_bp as u32,
            })
        }
        Ok(m2::instructions::MagicEdenInstruction::Mip1Sell(event)) => {
            pb::instruction::Instruction::Mip1Sell(pb::Mip1SellInstruction {
                price: event.args.price,
                expiry: event.args.expiry,
            })
        }
        Ok(m2::instructions::MagicEdenInstruction::Mip1ExecuteSaleV2(event)) => {
            pb::instruction::Instruction::Mip1ExecuteSaleV2(pb::Mip1ExecuteSaleV2Instruction {
                price: event.args.price,
                maker_fee_bp: event.args.maker_fee_bp as i32,
                taker_fee_bp: event.args.taker_fee_bp as u32,
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
