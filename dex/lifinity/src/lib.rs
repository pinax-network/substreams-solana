use common::solana::{get_fee_payer, get_signers};
use proto::pb::lifinity::v1 as pb;
use substreams::errors::Error;
use substreams_solana::{
    block_view::InstructionView,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction},
};
use substreams_solana_idls::lifinity;

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

    if program_id != &lifinity::PROGRAM_ID {
        return None;
    }

    match lifinity::instructions::unpack(ix.data()) {
        Ok(lifinity::instructions::LifinityInstruction::Swap(event)) => {
            let accounts = lifinity::accounts::get_swap_accounts(ix).ok()?;
            Some(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::Swap(pb::SwapInstruction {
                    accounts: Some(pb::SwapAccounts {
                        authority: accounts.authority.to_bytes().to_vec(),
                        amm: accounts.amm.to_bytes().to_vec(),
                        user_transfer_authority: accounts.user_transfer_authority.to_bytes().to_vec(),
                        source_info: accounts.source_info.to_bytes().to_vec(),
                        destination_info: accounts.destination_info.to_bytes().to_vec(),
                        swap_source: accounts.swap_source.to_bytes().to_vec(),
                        swap_destination: accounts.swap_destination.to_bytes().to_vec(),
                        pool_mint: accounts.pool_mint.to_bytes().to_vec(),
                        fee_account: accounts.fee_account.to_bytes().to_vec(),
                        token_program: accounts.token_program.to_bytes().to_vec(),
                        oracle_main_account: accounts.oracle_main_account.to_bytes().to_vec(),
                        oracle_sub_account: accounts.oracle_sub_account.to_bytes().to_vec(),
                        oracle_pc_account: accounts.oracle_pc_account.to_bytes().to_vec(),
                    }),
                    amount_in: event.amount_in,
                    minimum_amount_out: event.minimum_amount_out,
                })),
            })
        }
        _ => None,
    }
}
