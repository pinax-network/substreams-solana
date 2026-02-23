use common::solana::{get_fee_payer, get_signers, is_failed, is_invoke, is_success, parse_invoke_depth, parse_program_data, parse_program_id};
use proto::pb::moonshot::v1 as pb;
use substreams::errors::Error;
use substreams_solana::{
    block_view::InstructionView,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction, TransactionStatusMeta},
};
use substreams_solana_idls::moonshot;

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, Error> {
    Ok(pb::Events {
        transactions: block.transactions_owned().filter_map(process_transaction).collect(),
    })
}

fn process_transaction(tx: ConfirmedTransaction) -> Option<pb::Transaction> {
    let tx_meta = tx.meta.as_ref()?;

    let instructions: Vec<pb::Instruction> = tx.walk_instructions().filter_map(|iview| process_instruction(&iview)).collect();
    let logs = process_logs(tx_meta, &moonshot::PROGRAM_ID.to_vec());

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

    if program_id != &moonshot::PROGRAM_ID {
        return None;
    }

    match moonshot::instructions::unpack(ix.data()) {
        Ok(moonshot::instructions::MoonshotInstruction::Buy(event)) => {
            Some(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::Buy(pb::BuyInstruction {
                    amount: event.amount,
                    collateral_amount: event.collateral_amount,
                    slippage_bps: event.slippage_bps,
                })),
            })
        }
        Ok(moonshot::instructions::MoonshotInstruction::Sell(event)) => {
            Some(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: ix.stack_height(),
                instruction: Some(pb::instruction::Instruction::Sell(pb::SellInstruction {
                    amount: event.amount,
                    collateral_amount: event.collateral_amount,
                    slippage_bps: event.slippage_bps,
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
    match moonshot::events::unpack_event(data.as_slice()) {
        Ok(moonshot::events::MoonshotEvent::TradeEvent(event)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::Trade(pb::TradeEvent {
                amount: event.amount,
                collateral_amount: event.collateral_amount,
                dex_fee: event.dex_fee,
                helio_fee: event.helio_fee,
                allocation: event.allocation,
                curve: event.curve.to_bytes().to_vec(),
                cost_token: event.cost_token.to_bytes().to_vec(),
                sender: event.sender.to_bytes().to_vec(),
                trade_type: event.trade_type as u32,
                label: event.label,
            })),
        }),
        Ok(moonshot::events::MoonshotEvent::MigrationEvent(event)) => Some(pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::Migration(pb::MigrationEvent {
                tokens_migrated: event.tokens_migrated,
                tokens_burned: event.tokens_burned,
                collateral_migrated: event.collateral_migrated,
                fee: event.fee,
                label: event.label,
            })),
        }),
        _ => None,
    }
}
