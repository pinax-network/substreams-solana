use anchor_lang::prelude::*;
use common::solana::{get_fee_payer, get_signers, is_invoke, parse_invoke_depth, parse_program_id, parse_raydium_log};
use proto::pb::raydium::clmm::v1 as pb;
use raydium_amm_v3::states::pool::SwapEvent;
use substreams::errors::Error;
use substreams_solana::pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction, TransactionStatusMeta};

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, Error> {
    Ok(pb::Events {
        transactions: block.transactions_owned().filter_map(process_transaction).collect(),
    })
}

fn process_transaction(tx: ConfirmedTransaction) -> Option<pb::Transaction> {
    let tx_meta = tx.meta.as_ref()?;

    // CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK
    let logs = process_logs(tx_meta, &raydium_amm_v3::ID.to_bytes());

    if logs.is_empty() {
        return None;
    }

    Some(pb::Transaction {
        fee: tx_meta.fee,
        compute_units_consumed: tx_meta.compute_units_consumed(),
        signature: tx.hash().to_vec(),
        fee_payer: get_fee_payer(&tx).unwrap_or_default(),
        signers: get_signers(&tx).unwrap_or_default(),
        logs,
    })
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
        } else if is_invoked {
            if let Some(log_data) = parse_log_data(log_message, program_id_bytes, 0) {
                logs.push(log_data);
            }
        }
    }

    logs
}

fn parse_log_data(log_message: &str, program_id_bytes: &[u8], invoke_depth: u32) -> Option<pb::Log> {
    let data = parse_raydium_log(log_message)?;
    if data.len() < 8 {
        return None;
    }

    let disc: [u8; 8] = data[0..8].try_into().ok()?;
    let payload = &data[8..];

    if disc == SwapEvent::discriminator() {
        let event = SwapEvent::try_from_slice(payload).ok()?;
        let log = pb::Log {
            program_id: program_id_bytes.to_vec(),
            invoke_depth,
            log: Some(pb::log::Log::Swap(pb::SwapLog {
                pool_state: event.pool_state.to_bytes().to_vec(),
                sender: event.sender.to_bytes().to_vec(),
                token_account_0: event.token_account_0.to_bytes().to_vec(),
                token_account_1: event.token_account_1.to_bytes().to_vec(),
                amount_0: event.amount_0,
                transfer_fee_0: event.transfer_fee_0,
                amount_1: event.amount_1,
                transfer_fee_1: event.transfer_fee_1,
                zero_for_one: event.zero_for_one,
                sqrt_price_x64: event.sqrt_price_x64.to_string(),
                liquidity: event.liquidity.to_string(),
                tick: event.tick,
            })),
        };
        return Some(log);
    }

    None
}
