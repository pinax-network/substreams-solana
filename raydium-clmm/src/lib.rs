use common::solana::{get_fee_payer, get_signers, is_invoke, parse_invoke_depth, parse_program_id, parse_raydium_log};
use proto::pb::raydium::clmm::v1 as pb;
use substreams::errors::Error;
use substreams_solana::pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction, TransactionStatusMeta};

// CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK
const RAYDIUM_CLMM_PROGRAM_ID: [u8; 32] = [
    165, 213, 202, 158, 4, 207, 93, 181, 144, 183, 20, 186, 47, 227, 44, 177, 89, 19, 63, 193, 193, 146, 183, 34, 87, 253, 7, 211, 156, 176, 64, 30,
];

const SWAP_EVENT_DISCRIMINATOR: [u8; 8] = [64, 198, 205, 232, 38, 8, 113, 226];

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, Error> {
    Ok(pb::Events {
        transactions: block.transactions_owned().filter_map(process_transaction).collect(),
    })
}

fn process_transaction(tx: ConfirmedTransaction) -> Option<pb::Transaction> {
    let tx_meta = tx.meta.as_ref()?;

    let logs = process_logs(tx_meta, &RAYDIUM_CLMM_PROGRAM_ID);

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
        let match_program_id = parse_program_id(log_message).map_or(false, |id| id == program_id_bytes);

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

    if data[..8] != SWAP_EVENT_DISCRIMINATOR {
        return None;
    }

    let event = decode_swap_event(&data[8..])?;

    Some(pb::Log {
        program_id: program_id_bytes.to_vec(),
        invoke_depth,
        log: Some(pb::log::Log::Swap(event)),
    })
}

fn decode_swap_event(data: &[u8]) -> Option<pb::SwapLog> {
    let mut idx = 0;
    fn take<'a>(data: &'a [u8], idx: &mut usize, len: usize) -> Option<&'a [u8]> {
        if *idx + len > data.len() {
            None
        } else {
            let slice = &data[*idx..*idx + len];
            *idx += len;
            Some(slice)
        }
    }

    let pool_state = take(data, &mut idx, 32)?.to_vec();
    let sender = take(data, &mut idx, 32)?.to_vec();
    let token_account_0 = take(data, &mut idx, 32)?.to_vec();
    let token_account_1 = take(data, &mut idx, 32)?.to_vec();
    let amount_0 = u64::from_le_bytes(take(data, &mut idx, 8)?.try_into().ok()?);
    let transfer_fee_0 = u64::from_le_bytes(take(data, &mut idx, 8)?.try_into().ok()?);
    let amount_1 = u64::from_le_bytes(take(data, &mut idx, 8)?.try_into().ok()?);
    let transfer_fee_1 = u64::from_le_bytes(take(data, &mut idx, 8)?.try_into().ok()?);
    let zero_for_one = take(data, &mut idx, 1)?[0] != 0;
    let sqrt_price_x64 = u128::from_le_bytes(take(data, &mut idx, 16)?.try_into().ok()?);
    let liquidity = u128::from_le_bytes(take(data, &mut idx, 16)?.try_into().ok()?);
    let tick = i32::from_le_bytes(take(data, &mut idx, 4)?.try_into().ok()?);

    Some(pb::SwapLog {
        pool_state,
        sender,
        token_account_0,
        token_account_1,
        amount_0,
        transfer_fee_0,
        amount_1,
        transfer_fee_1,
        zero_for_one,
        sqrt_price_x64: sqrt_price_x64.to_string(),
        liquidity: liquidity.to_string(),
        tick,
    })
}
