use common::db::{common_key_v2, set_clock, set_native_token_instruction_v2, set_native_token_transaction_v2};
use proto::pb::solana::native::token::v1 as pb;
use substreams::pb::substreams::Clock;
use substreams_solana::base58;

pub fn process_events(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, events: &pb::Events) {
    for (transaction_index, transaction) in events.transactions.iter().enumerate() {
        // Native Token Instructions
        for (i, instruction) in transaction.instructions.iter().enumerate() {
            match &instruction.instruction {
                Some(pb::instruction::Instruction::CreateAccount(data)) => {
                    handle_create_account(tables, clock, transaction, instruction, data, transaction_index, i);
                }
                Some(pb::instruction::Instruction::CreateAccountWithSeed(data)) => {
                    handle_create_account_with_seed(tables, clock, transaction, instruction, data, transaction_index, i);
                }
                _ => {}
            }
        }
    }
}

fn handle_create_account(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    instruction: &pb::Instruction,
    data: &pb::CreateAccount,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row("system_create_account", key)
        .set("source", base58::encode(&data.source))
        .set("new_account", base58::encode(&data.new_account))
        .set("owner", base58::encode(&data.owner))
        .set("lamports", data.lamports)
        .set("space", data.space);

    set_native_token_instruction_v2(instruction, row);
    set_native_token_transaction_v2(transaction, row);
    set_clock(clock, row);
}

fn handle_create_account_with_seed(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    instruction: &pb::Instruction,
    data: &pb::CreateAccountWithSeed,
    transaction_index: usize,
    instruction_index: usize,
) {
    let base_account_raw = data.base_account.as_ref().map(base58::encode).unwrap_or_default();
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row("system_create_account_with_seed", key)
        .set("source", base58::encode(&data.source))
        .set("new_account", base58::encode(&data.new_account))
        .set("base", base58::encode(&data.base))
        .set("base_account_raw", base_account_raw)
        .set("owner", base58::encode(&data.owner))
        .set("lamports", data.lamports)
        .set("space", data.space)
        .set("seed", &data.seed);

    set_native_token_instruction_v2(instruction, row);
    set_native_token_transaction_v2(transaction, row);
    set_clock(clock, row);
}
