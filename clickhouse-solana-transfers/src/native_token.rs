use common::clickhouse::{common_key_v2, set_clock, set_native_token_instruction_v2, set_native_token_transaction_v2};
use proto::pb::solana::native::token::v1 as pb;
use substreams::pb::substreams::Clock;
use substreams_solana::base58;

pub fn process_events(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, events: &pb::Events) {
    for (transaction_index, transaction) in events.transactions.iter().enumerate() {
        // Native Token Instructions
        for (i, instruction) in transaction.instructions.iter().enumerate() {
            match &instruction.instruction {
                Some(pb::instruction::Instruction::Transfer(data)) => {
                    handle_transfer(tables, clock, transaction, instruction, data, transaction_index, i);
                }
                Some(pb::instruction::Instruction::TransferWithSeed(data)) => {
                    handle_transfer_with_seed(tables, clock, transaction, instruction, data, transaction_index, i);
                }
                Some(pb::instruction::Instruction::WithdrawNonceAccount(data)) => {
                    handle_withdraw_nonce_account(tables, clock, transaction, instruction, data, transaction_index, i);
                }
                _ => {}
            }
        }
    }
}

fn handle_transfer(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    instruction: &pb::Instruction,
    data: &pb::Transfer,
    transaction_index: usize,
    instruction_index: usize,
) {
    // Skip transfers to self
    if data.source == data.destination {
        return;
    }
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row("system_transfer", key)
        .set("source", base58::encode(&data.source))
        .set("destination", base58::encode(&data.destination))
        .set("lamports", data.lamports);

    set_native_token_instruction_v2(instruction, row);
    set_native_token_transaction_v2(transaction, row);
    set_clock(clock, row);
}

fn handle_transfer_with_seed(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    instruction: &pb::Instruction,
    data: &pb::TransferWithSeed,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row("system_transfer_with_seed", key)
        .set("source", base58::encode(&data.source))
        .set("destination", base58::encode(&data.destination))
        .set("lamports", data.lamports)
        .set("source_base", base58::encode(&data.source_base))
        .set("source_owner", base58::encode(&data.source_owner))
        .set("source_seed", &data.source_seed);

    set_native_token_instruction_v2(instruction, row);
    set_native_token_transaction_v2(transaction, row);
    set_clock(clock, row);
}

fn handle_withdraw_nonce_account(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    instruction: &pb::Instruction,
    data: &pb::WithdrawNonceAccount,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row("system_withdraw_nonce_account", key)
        .set("destination", base58::encode(&data.destination))
        .set("lamports", data.lamports)
        .set("nonce_account", base58::encode(&data.nonce_account))
        .set("nonce_authority", base58::encode(&data.nonce_authority));

    set_native_token_instruction_v2(instruction, row);
    set_native_token_transaction_v2(transaction, row);
    set_clock(clock, row);
}
