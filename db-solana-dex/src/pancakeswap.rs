use common::db::{common_key_v2, set_clock};
use proto::pb::pancakeswap::v1 as pb;
use substreams::pb::substreams::Clock;
use substreams_database_change::tables::{Row, Tables};
use substreams_solana::base58;

pub fn process_events(tables: &mut Tables, clock: &Clock, events: &pb::Events) {
    for (transaction_index, tx) in events.transactions.iter().enumerate() {
        for (log_index, log) in tx.logs.iter().enumerate() {
            if let Some(pb::log::Log::Swap(event)) = &log.log {
                let ix = if log_index < tx.instructions.len() { &tx.instructions[log_index] } else { continue };
                handle_swap(tables, clock, tx, ix, event, transaction_index, log_index);
            }
        }
    }
}

fn handle_swap(
    tables: &mut Tables,
    clock: &Clock,
    tx: &pb::Transaction,
    ix: &pb::Instruction,
    event: &pb::SwapEvent,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row("pancakeswap_swap", key)
        .set("pool_state", base58::encode(&event.pool_state))
        .set("sender", base58::encode(&event.sender))
        .set("amount_0", event.amount_0)
        .set("amount_1", event.amount_1)
        .set("zero_for_one", event.zero_for_one)
        .set("tick", event.tick)
        .set("sqrt_price_x64", &event.sqrt_price_x64)
        .set("liquidity", &event.liquidity);
    set_instruction(ix, row);
    set_transaction(tx, row);
    set_clock(clock, row);
}

fn set_transaction(tx: &pb::Transaction, row: &mut Row) {
    row.set("signature", base58::encode(&tx.signature))
        .set("fee_payer", base58::encode(&tx.fee_payer))
        .set("signers_raw", tx.signers.iter().map(base58::encode).collect::<Vec<_>>().join(","))
        .set("fee", tx.fee)
        .set("compute_units_consumed", tx.compute_units_consumed);
}

fn set_instruction(ix: &pb::Instruction, row: &mut Row) {
    row.set("program_id", base58::encode(&ix.program_id)).set("stack_height", ix.stack_height);
}
