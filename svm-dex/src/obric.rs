use common::db::{common_key_v2, set_clock};
use proto::pb::obric;
use substreams::pb::substreams::Clock;
use substreams_database_change::tables::Tables;
use substreams_solana::base58;

fn write_swap_row(
    tables: &mut Tables,
    clock: &Clock,
    key: [(&'static str, String); 3],
    signature: &[u8],
    fee_payer: &[u8],
    fee: u64,
    compute_units_consumed: u64,
    input_amount: u64,
    min_output_amount: u64,
) {
    let row = tables
        .create_row("obric_swap", key)
        .set("input_amount", input_amount)
        .set("min_output_amount", min_output_amount);
    row.set("signature", base58::encode(signature))
        .set("fee_payer", base58::encode(fee_payer))
        .set("fee", fee)
        .set("compute_units_consumed", compute_units_consumed);
    set_clock(clock, row);
}

pub fn process_v2_events(tables: &mut Tables, clock: &Clock, events: &obric::v2::v1::Events) {
    for (tx_idx, tx) in events.transactions.iter().enumerate() {
        for (ix_idx, ix) in tx.instructions.iter().enumerate() {
            let key = common_key_v2(clock, tx_idx, ix_idx);
            match &ix.instruction {
                Some(obric::v2::v1::instruction::Instruction::SwapXToY(data)) => write_swap_row(
                    tables,
                    clock,
                    key,
                    &tx.signature,
                    &tx.fee_payer,
                    tx.fee,
                    tx.compute_units_consumed,
                    data.input_amount,
                    data.min_output_amount,
                ),
                Some(obric::v2::v1::instruction::Instruction::SwapYToX(data)) => write_swap_row(
                    tables,
                    clock,
                    key,
                    &tx.signature,
                    &tx.fee_payer,
                    tx.fee,
                    tx.compute_units_consumed,
                    data.input_amount,
                    data.min_output_amount,
                ),
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
                Some(obric::v3::v1::instruction::Instruction::SwapXToY(data)) => write_swap_row(
                    tables,
                    clock,
                    key,
                    &tx.signature,
                    &tx.fee_payer,
                    tx.fee,
                    tx.compute_units_consumed,
                    data.input_amount,
                    data.min_output_amount,
                ),
                Some(obric::v3::v1::instruction::Instruction::SwapYToX(data)) => write_swap_row(
                    tables,
                    clock,
                    key,
                    &tx.signature,
                    &tx.fee_payer,
                    tx.fee,
                    tx.compute_units_consumed,
                    data.input_amount,
                    data.min_output_amount,
                ),
                _ => {}
            }
        }
    }
}
