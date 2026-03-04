use common::db::{common_key_v2, set_clock};
use proto::pb::boop::v1 as pb;
use substreams::pb::substreams::Clock;
use substreams_database_change::tables::Tables;
use substreams_solana::base58;

pub fn process_events(tables: &mut Tables, clock: &Clock, events: &pb::Events) {
    for (tx_idx, tx) in events.transactions.iter().enumerate() {
        for (log_idx, log) in tx.logs.iter().enumerate() {
            let key = common_key_v2(clock, tx_idx, log_idx);
            match &log.log {
                Some(pb::log::Log::Bought(event)) => {
                    let row = tables.create_row("boop_buy", key)
                        .set("mint", base58::encode(&event.mint))
                        .set("amount_in", event.amount_in)
                        .set("amount_out", event.amount_out)
                        .set("swap_fee", event.swap_fee)
                        .set("buyer", base58::encode(&event.buyer));
                    row.set("signature", base58::encode(&tx.signature))
                        .set("fee_payer", base58::encode(&tx.fee_payer))
                        .set("fee", tx.fee)
                        .set("compute_units_consumed", tx.compute_units_consumed);
                    set_clock(clock, row);
                }
                Some(pb::log::Log::Sold(event)) => {
                    let row = tables.create_row("boop_sell", key)
                        .set("mint", base58::encode(&event.mint))
                        .set("amount_in", event.amount_in)
                        .set("amount_out", event.amount_out)
                        .set("swap_fee", event.swap_fee)
                        .set("seller", base58::encode(&event.seller));
                    row.set("signature", base58::encode(&tx.signature))
                        .set("fee_payer", base58::encode(&tx.fee_payer))
                        .set("fee", tx.fee)
                        .set("compute_units_consumed", tx.compute_units_consumed);
                    set_clock(clock, row);
                }
                _ => {}
            }
        }
    }
}
