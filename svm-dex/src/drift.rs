use common::db::{common_key_v2, set_clock};
use proto::pb::drift::v1 as pb;
use substreams::pb::substreams::Clock;
use substreams_database_change::tables::Tables;
use substreams_solana::base58;

pub fn process_events(tables: &mut Tables, clock: &Clock, events: &pb::Events) {
    for (tx_idx, tx) in events.transactions.iter().enumerate() {
        for (log_idx, log) in tx.logs.iter().enumerate() {
            if let Some(pb::log::Log::Swap(event)) = &log.log {
                let key = common_key_v2(clock, tx_idx, log_idx);
                let row = tables.create_row("drift_swap", key)
                    .set("user", base58::encode(&event.user))
                    .set("amount_in", event.amount_in)
                    .set("amount_out", event.amount_out);
                row.set("signature", base58::encode(&tx.signature))
                    .set("fee_payer", base58::encode(&tx.fee_payer))
                    .set("fee", tx.fee)
                    .set("compute_units_consumed", tx.compute_units_consumed);
                set_clock(clock, row);
            }
        }
    }
}
