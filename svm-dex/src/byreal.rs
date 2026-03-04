use common::db::{common_key_v2, set_clock};
use proto::pb::byreal::v1 as pb;
use substreams::pb::substreams::Clock;
use substreams_database_change::tables::Tables;
use substreams_solana::base58;

pub fn process_events(tables: &mut Tables, clock: &Clock, events: &pb::Events) {
    for (tx_idx, tx) in events.transactions.iter().enumerate() {
        for (ix_idx, ix) in tx.instructions.iter().enumerate() {
            if let Some(pb::instruction::Instruction::Swap(data)) = &ix.instruction {
                let key = common_key_v2(clock, tx_idx, ix_idx);
                let row = tables.create_row("byreal_swap", key)
                    .set("amount_in", data.amount_in)
                    .set("minimum_amount_out", data.minimum_amount_out);
                row.set("signature", base58::encode(&tx.signature))
                    .set("fee_payer", base58::encode(&tx.fee_payer))
                    .set("fee", tx.fee)
                    .set("compute_units_consumed", tx.compute_units_consumed);
                set_clock(clock, row);
            }
        }
    }
}
