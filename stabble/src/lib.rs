use common::solana::{get_fee_payer, get_signers, is_failed, is_invoke, is_success, parse_invoke_depth, parse_program_data, parse_program_id};
use proto::pb::stabble::v1 as pb;
use substreams::errors::Error;
use substreams_solana::{
    block_view::InstructionView,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction, TransactionStatusMeta},
};
use substreams_solana_idls::stabble;

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, Error> {
    Ok(pb::Events {
        transactions: block.transactions_owned().filter_map(process_transaction).collect(),
    })
}

fn process_transaction(tx: ConfirmedTransaction) -> Option<pb::Transaction> {
    let tx_meta = tx.meta.as_ref()?;

    let instructions: Vec<pb::Instruction> = tx.walk_instructions().filter_map(|iview| process_instruction(&iview)).collect();
    let logs = process_logs(tx_meta, &stabble::PROGRAM_ID.to_vec());

    if instructions.is_empty() && logs.is_empty() {
        return None;
    }

    Some(pb::Transaction {
        fee: tx_meta.fee,
        compute_units_consumed: tx_meta.compute_units_consumed(),
        signature: tx.hash().to_vec(),
        fee_payer: get_fee_payer(&tx).unwrap_or_default(),
        signers: get_signers(&tx).unwrap_or_default(),
        instructions,
        logs,
    })
}

fn process_instruction(ix: &InstructionView) -> Option<pb::Instruction> {
    let program_id = ix.program_id().0;

    if program_id != &stabble::PROGRAM_ID {
        return None;
    }

    match stabble::instructions::unpack(ix.data()) {
        Ok(stabble::instructions::StabbleInstruction::Swap(event)) => {
            let accounts = stabble::accounts::get_swap_accounts(ix).ok()?;
            Some(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::Swap(pb::SwapInstruction {
                    accounts: Some(pb::SwapAccounts {
                        user: accounts.user.to_bytes().to_vec(),
                        user_token_in: accounts.user_token_in.to_bytes().to_vec(),
                        user_token_out: accounts.user_token_out.to_bytes().to_vec(),
                        vault_token_in: accounts.vault_token_in.to_bytes().to_vec(),
                        vault_token_out: accounts.vault_token_out.to_bytes().to_vec(),
                        beneficiary_token_out: accounts.beneficiary_token_out.to_bytes().to_vec(),
                        pool: accounts.pool.to_bytes().to_vec(),
                        withdraw_authority: accounts.withdraw_authority.to_bytes().to_vec(),
                        vault: accounts.vault.to_bytes().to_vec(),
                        vault_authority: accounts.vault_authority.to_bytes().to_vec(),
                        vault_program: accounts.vault_program.to_bytes().to_vec(),
                        token_program: accounts.token_program.to_bytes().to_vec(),
                    }),
                    amount_in: event.amount_in,
                    minimum_amount_out: event.minimum_amount_out,
                })),
            })
        }
        _ => None,
    }
}

fn process_logs(tx_meta: &TransactionStatusMeta, program_id_bytes: &[u8]) -> Vec<pb::Log> {
    let mut logs = Vec::new();
    let mut is_invoked = false;

    for log_message in tx_meta.log_messages.iter() {
        let match_program_id = parse_program_id(log_message).map_or(false, |id| id == program_id_bytes.to_vec());

        if is_invoke(log_message) && match_program_id {
            if let Some(invoke_depth) = parse_invoke_depth(log_message) {
                is_invoked = true;
                if let Some(log_data) = parse_log_data(log_message, program_id_bytes, invoke_depth) {
                    logs.push(log_data);
                }
            }
        } else if match_program_id && (is_success(log_message) || is_failed(log_message)) {
            is_invoked = false;
        } else if is_invoked {
            if let Some(log_data) = parse_log_data(log_message, program_id_bytes, 0) {
                logs.push(log_data);
            }
        }
    }

    logs
}

fn parse_log_data(log_message: &str, program_id_bytes: &[u8], invoke_depth: u32) -> Option<pb::Log> {
    let data = parse_program_data(log_message)?;
    match stabble::events::unpack(data.as_slice()) {
        Ok(stabble::events::StabbleEvent::PoolBalanceUpdatedEvent(event)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::PoolBalanceUpdated(pb::PoolBalanceUpdatedEvent {
                pubkey: event.pubkey.to_bytes().to_vec(),
                balances: event.data.balances,
            })),
        }),
        Ok(stabble::events::StabbleEvent::PoolUpdatedEvent(event)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::PoolUpdated(pb::PoolUpdatedEvent {
                pubkey: event.pubkey.to_bytes().to_vec(),
                is_active: event.data.is_active,
                swap_fee: event.data.swap_fee,
                max_supply: event.data.max_supply,
            })),
        }),
        _ => None,
    }
}
