use common::db::{common_key_v2, set_clock};
use proto::pb::dflow::v1 as pb;
use substreams::pb::substreams::Clock;
use substreams_database_change::tables::Tables;
use substreams_solana::base58;

pub fn process_events(tables: &mut Tables, clock: &Clock, events: &pb::Events) {
    for (transaction_index, tx) in events.transactions.iter().enumerate() {
        for (instruction_index, ix) in tx.instructions.iter().enumerate() {
            if let Some(pb::instruction::Instruction::Swap(data)) = &ix.instruction {
                let key = common_key_v2(clock, transaction_index, instruction_index);
                let row = tables
                    .create_row("dflow_swap", key)
                    .set("amount_in", data.amount_in)
                    .set("minimum_amount_out", data.minimum_amount_out);
                row.set("program_id", base58::encode(&ix.program_id)).set("stack_height", ix.stack_height);
                row.set("signature", base58::encode(&tx.signature))
                    .set("fee_payer", base58::encode(&tx.fee_payer))
                    .set("fee", tx.fee)
                    .set("compute_units_consumed", tx.compute_units_consumed);
                set_clock(clock, row);
            }
        }
    }
}
