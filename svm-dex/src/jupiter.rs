use common::db::{common_key_v2, set_clock, set_jupiter_instruction_v2, set_jupiter_transaction_v2};
use proto::pb::jupiter::v1 as pb;
use substreams::pb::substreams::Clock;
use substreams_solana::base58;

pub fn process_events(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, events: &pb::Events) {
    for (transaction_index, transaction) in events.transactions.iter().enumerate() {
        for (instruction_index, instruction) in transaction.instructions.iter().enumerate() {
            match &instruction.instruction {
                Some(pb::instruction::Instruction::SwapEvent(event)) => {
                    handle_swap(tables, clock, transaction, instruction, event, transaction_index, instruction_index);
                }
                _ => {}
            }
        }
    }
}

fn handle_swap(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    instruction: &pb::Instruction,
    data: &pb::SwapEvent,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(&clock, transaction_index, instruction_index);
    let row = tables
        .create_row("jupiter_swap", key)
        .set("amm", base58::encode(&data.amm))
        .set("input_mint", base58::encode(&data.input_mint))
        .set("input_amount", data.input_amount)
        .set("output_mint", base58::encode(&data.output_mint))
        .set("output_amount", data.output_amount);

    set_jupiter_instruction_v2(instruction, row);
    set_jupiter_transaction_v2(transaction, row);
    set_clock(clock, row);
}
