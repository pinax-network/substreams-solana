use substreams::pb::substreams::Clock;
use substreams_database_change::tables::Row;
use substreams_solana::base58;

use crate::{to_global_sequence, Address, Hash};

pub fn common_key(clock: &Clock, execution_index: u64) -> [(&'static str, String); 4] {
    let seconds = clock.timestamp.as_ref().expect("clock.timestamp is required").seconds;
    [
        ("timestamp", seconds.to_string()),
        ("block_num", clock.number.to_string()),
        ("execution_index", execution_index.to_string()),
        ("block_hash", clock.id.to_string()),
    ]
}

// Helper function to set clock data in a row
pub fn set_clock(clock: &Clock, row: &mut Row) {
    row.set("block_num", clock.number.to_string())
        .set("block_hash", &clock.id)
        .set("timestamp", clock.timestamp.as_ref().expect("missing timestamp").seconds.to_string());
}

// TO-DO: handle multisig authority
pub fn set_instruction(tx_hash: Hash, program_id: Address, instruction: &str, authority: Address, _multisig_authority: Vec<Address>, row: &mut Row) {
    row.set("tx_hash", base58::encode(tx_hash))
        .set("program_id", base58::encode(program_id))
        .set("instruction", instruction)
        .set("authority", base58::encode(authority));
}

pub fn set_ordering(execution_index: u32, instruction_index: u32, inner_instruction_index: u32, stack_height: u32, clock: &Clock, row: &mut Row) {
    row.set("execution_index", execution_index)
        .set("instruction_index", instruction_index)
        .set("inner_instruction_index", inner_instruction_index)
        .set("stack_height", stack_height)
        .set("global_sequence", to_global_sequence(clock, execution_index as u64));
}
