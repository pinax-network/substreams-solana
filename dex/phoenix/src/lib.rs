use common::solana::{get_fee_payer, get_signers, is_failed, is_invoke, is_success, parse_invoke_depth, parse_program_data, parse_program_id};
use proto::pb::phoenix::v1 as pb;
use substreams::errors::Error;
use substreams_solana::{
    block_view::InstructionView,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction, TransactionStatusMeta},
};
use substreams_solana_idls::phoenix;

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, Error> {
    Ok(pb::Events {
        transactions: block.transactions_owned().filter_map(process_transaction).collect(),
    })
}

fn process_transaction(tx: ConfirmedTransaction) -> Option<pb::Transaction> {
    let tx_meta = tx.meta.as_ref()?;

    let instructions: Vec<pb::Instruction> = tx.walk_instructions().filter_map(|iview| process_instruction(&iview)).collect();
    let logs = process_logs(tx_meta, &phoenix::PROGRAM_ID.to_vec());

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

    if program_id != &phoenix::PROGRAM_ID {
        return None;
    }

    match phoenix::instructions::unpack(ix.data()) {
        Ok(phoenix::instructions::PhonenixInstruction::Swap(event)) => {
            let accounts = phoenix::accounts::get_swap_accounts(ix).ok()?;
            Some(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::Swap(pb::SwapInstruction {
                    accounts: Some(pb::SwapAccounts {
                        phoenix_program: accounts.phoenix_program.to_bytes().to_vec(),
                        log_authority: accounts.log_authority.to_bytes().to_vec(),
                        market: accounts.market.to_bytes().to_vec(),
                        trader: accounts.trader.to_bytes().to_vec(),
                        base_account: accounts.base_account.to_bytes().to_vec(),
                        quote_account: accounts.quote_account.to_bytes().to_vec(),
                        base_vault: accounts.base_vault.to_bytes().to_vec(),
                        quote_vault: accounts.quote_vault.to_bytes().to_vec(),
                        token_program: accounts.token_program.to_bytes().to_vec(),
                    }),
                    order_packet: event.order_packet,
                })),
            })
        }
        Ok(phoenix::instructions::PhonenixInstruction::SwapWithFreeFunds(event)) => {
            let accounts = phoenix::accounts::get_swap_accounts(ix).ok()?;
            Some(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::SwapWithFreeFunds(pb::SwapWithFreeFundsInstruction {
                    accounts: Some(pb::SwapAccounts {
                        phoenix_program: accounts.phoenix_program.to_bytes().to_vec(),
                        log_authority: accounts.log_authority.to_bytes().to_vec(),
                        market: accounts.market.to_bytes().to_vec(),
                        trader: accounts.trader.to_bytes().to_vec(),
                        base_account: accounts.base_account.to_bytes().to_vec(),
                        quote_account: accounts.quote_account.to_bytes().to_vec(),
                        base_vault: accounts.base_vault.to_bytes().to_vec(),
                        quote_vault: accounts.quote_vault.to_bytes().to_vec(),
                        token_program: accounts.token_program.to_bytes().to_vec(),
                    }),
                    order_packet: event.order_packet,
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
    match phoenix::events::unpack(data.as_slice()) {
        Ok(phoenix::events::PhonenixEvent::Fill(event)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::Fill(pb::FillEvent {
                index: event.index as u32,
                maker_id: event.maker_id.to_bytes().to_vec(),
                order_sequence_number: event.order_sequence_number,
                price_in_ticks: event.price_in_ticks,
                base_lots_filled: event.base_lots_filled,
                base_lots_remaining: event.base_lots_remaining,
            })),
        }),
        _ => None,
    }
}
