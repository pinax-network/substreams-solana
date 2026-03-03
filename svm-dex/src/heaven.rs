use common::db::{common_key_v2, set_clock};
use proto::pb::heaven::v1 as pb;
use substreams::pb::substreams::Clock;
use substreams_database_change::tables::{Row, Tables};
use substreams_solana::base58;

pub fn process_events(tables: &mut Tables, clock: &Clock, events: &pb::Events) {
    for (transaction_index, tx) in events.transactions.iter().enumerate() {
        for (log_index, log) in tx.logs.iter().enumerate() {
            if let Some(pb::log::Log::Sell(event)) = &log.log {
                handle_sell(tables, clock, tx, event, transaction_index, log_index);
            }
        }
    }
}

fn handle_sell(
    tables: &mut Tables,
    clock: &Clock,
    tx: &pb::Transaction,
    event: &pb::SellLog,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row("heaven_sell", key)
        .set("user", base58::encode(&event.user))
        .set("mint", base58::encode(&event.mint))
        .set("amount", event.amount);
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
