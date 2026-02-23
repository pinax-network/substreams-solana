use common::db::{common_key_v2, set_clock};
use proto::pb::moonshot::v1 as pb;
use substreams::pb::substreams::Clock;
use substreams_database_change::tables::{Row, Tables};
use substreams_solana::base58;

pub fn process_events(tables: &mut Tables, clock: &Clock, events: &pb::Events) {
    for (transaction_index, tx) in events.transactions.iter().enumerate() {
        for (log_index, log) in tx.logs.iter().enumerate() {
            if let Some(pb::log::Log::Trade(event)) = &log.log {
                let ix = if log_index < tx.instructions.len() { &tx.instructions[log_index] } else { continue };
                let table = if event.trade_type == 0 { "moonshot_buy" } else { "moonshot_sell" };
                handle_trade(tables, clock, tx, ix, event, table, transaction_index, log_index);
            }
        }
    }
}

fn handle_trade(
    tables: &mut Tables,
    clock: &Clock,
    tx: &pb::Transaction,
    ix: &pb::Instruction,
    event: &pb::TradeEvent,
    table: &str,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row(table, key)
        .set("amount", event.amount)
        .set("collateral_amount", event.collateral_amount)
        .set("dex_fee", event.dex_fee)
        .set("helio_fee", event.helio_fee)
        .set("sender", base58::encode(&event.sender))
        .set("trade_type", event.trade_type)
        .set("cost_token", base58::encode(&event.cost_token))
        .set("curve", base58::encode(&event.curve));
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
