use common::db::{common_key_v2, set_clock};
use proto::pb::obric;
use substreams::pb::substreams::Clock;
use substreams_database_change::tables::Tables;
use substreams_solana::base58;

pub fn process_v2_events(tables: &mut Tables, clock: &Clock, events: &obric::v2::v1::Events) {
    for (tx_idx, tx) in events.transactions.iter().enumerate() {
        for (ix_idx, ix) in tx.instructions.iter().enumerate() {
            let key = common_key_v2(clock, tx_idx, ix_idx);
            match &ix.instruction {
                Some(obric::v2::v1::instruction::Instruction::SwapXToY(data)) |
                Some(obric::v2::v1::instruction::Instruction::SwapYToX(data)) => {
                    let row = tables.create_row("obric_swap", key)
                        .set("input_amount", data.input_amount)
                        .set("min_output_amount", data.min_output_amount);
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

pub fn process_v3_events(tables: &mut Tables, clock: &Clock, events: &obric::v3::v1::Events) {
    for (tx_idx, tx) in events.transactions.iter().enumerate() {
        for (ix_idx, ix) in tx.instructions.iter().enumerate() {
            let key = common_key_v2(clock, tx_idx, ix_idx);
            match &ix.instruction {
                Some(obric::v3::v1::instruction::Instruction::SwapXToY(data)) |
                Some(obric::v3::v1::instruction::Instruction::SwapYToX(data)) => {
                    let row = tables.create_row("obric_swap", key)
                        .set("input_amount", data.input_amount)
                        .set("min_output_amount", data.min_output_amount);
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
