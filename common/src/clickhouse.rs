use proto::pb::jupiter::v1 as jupiter;
use proto::pb::raydium::amm::v1 as raydium;
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

/// inserts rows for transaction_index and instruction_index
pub fn common_key_v2(clock: &Clock, transaction_index: usize, instruction_index: usize) -> [(&'static str, String); 3] {
    [
        ("block_hash", clock.id.to_string()),
        ("transaction_index", transaction_index.to_string()),
        ("instruction_index", instruction_index.to_string()),
    ]
}

pub fn set_raydium_transaction_v2(transaction: &raydium::Transaction, row: &mut Row) {
    row.set("signature", base58::encode(transaction.signature.to_vec()))
        .set("fee_payer", base58::encode(transaction.fee_payer.to_vec()))
        .set("signers_raw", transaction.signers.iter().map(base58::encode).collect::<Vec<_>>().join(","))
        .set("fee", transaction.fee)
        .set("compute_units_consumed", transaction.compute_units_consumed);
}

pub fn set_raydium_instruction_v2(instruction: &raydium::Instruction, row: &mut Row) {
    row.set("program_id", base58::encode(instruction.program_id.to_vec()))
        .set("stack_height", instruction.stack_height);
}

pub fn set_jupiter_transaction_v2(transaction: &jupiter::Transaction, row: &mut Row) {
    row.set("signature", base58::encode(transaction.signature.to_vec()))
        .set("fee_payer", base58::encode(transaction.fee_payer.to_vec()))
        .set("signers_raw", transaction.signers.iter().map(base58::encode).collect::<Vec<_>>().join(","))
        .set("fee", transaction.fee)
        .set("compute_units_consumed", transaction.compute_units_consumed);
}

pub fn set_jupiter_instruction_v2(instruction: &jupiter::Instruction, row: &mut Row) {
    row.set("program_id", base58::encode(instruction.program_id.to_vec()))
        .set("stack_height", instruction.stack_height);
}

// message Transaction {
//     bytes  signature                = 1;
//     bytes  fee_payer                = 2;          // Fee-payer account address
//     repeated bytes signers          = 3;          // Signers of the tx
//     uint64 fee                      = 4;          // Lamports paid
//     uint64 compute_units_consumed   = 5;          // CU used
//     repeated Instruction instructions = 6;        // Executed instructions
//   }

// Helper function to set clock data in a row
pub fn set_clock(clock: &Clock, row: &mut Row) {
    row.set("block_num", clock.number.to_string())
        .set("block_hash", &clock.id)
        .set("timestamp", clock.timestamp.as_ref().expect("missing timestamp").seconds.to_string());
}

// TO-DO: handle multisig authority
pub fn set_instruction(tx_hash: Hash, program_id: Address, instruction: &str, row: &mut Row) {
    row.set("tx_hash", base58::encode(tx_hash))
        .set("program_id", base58::encode(program_id))
        .set("instruction", instruction);
}

pub fn set_authority(authority: Address, multisig_authority: Vec<Address>, row: &mut Row) {
    row.set("authority", base58::encode(authority)).set(
        "multisig_authority_raw",
        multisig_authority.iter().map(base58::encode).collect::<Vec<_>>().join(","),
    );
}

pub fn set_ordering(execution_index: u32, instruction_index: u32, inner_instruction_index: u32, stack_height: u32, clock: &Clock, row: &mut Row) {
    row.set("execution_index", execution_index)
        .set("instruction_index", instruction_index)
        .set("inner_instruction_index", inner_instruction_index)
        .set("stack_height", stack_height)
        .set("global_sequence", to_global_sequence(clock, execution_index as u64));
}
