use std::collections::HashMap;

use common::clickhouse::{common_key_v2, set_clock, set_native_token_instruction_v2, set_native_token_transaction_v2};
use proto::pb::solana::native::token::v1 as pb;
use substreams::pb::substreams::Clock;
use substreams_solana::base58;

pub fn process_events(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, events: &pb::Events) {
    let mut system_post_balances_per_block = HashMap::new();
    for (transaction_index, transaction) in events.transactions.iter().enumerate() {
        // Native Token Balances
        // Only keep last post balance change per block
        for (i, post_balance) in transaction.post_balances.iter().enumerate() {
            let key = &post_balance.account;
            system_post_balances_per_block.insert(key, (post_balance, transaction, transaction_index, i));
        }
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
                Some(pb::instruction::Instruction::CreateAccount(data)) => {
                    handle_create_account(tables, clock, transaction, instruction, data, transaction_index, i);
                }
                Some(pb::instruction::Instruction::CreateAccountWithSeed(data)) => {
                    create_account_with_seed(tables, clock, transaction, instruction, data, transaction_index, i);
                }
                _ => {}
            }
        }
    }
    for (post_balance, transaction, transaction_index, i) in system_post_balances_per_block.values() {
        handle_balances("system_post_balances", tables, clock, transaction, post_balance, *transaction_index, *i);
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
    let key = common_key_v2(&clock, transaction_index, instruction_index);
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
    let key = common_key_v2(&clock, transaction_index, instruction_index);
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

fn handle_create_account(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    instruction: &pb::Instruction,
    data: &pb::CreateAccount,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(&clock, transaction_index, instruction_index);
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

fn create_account_with_seed(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    instruction: &pb::Instruction,
    data: &pb::CreateAccountWithSeed,
    transaction_index: usize,
    instruction_index: usize,
) {
    let base_account_raw = match &data.base_account {
        Some(base_account) => base58::encode(base_account),
        None => "".to_string(),
    };
    let key = common_key_v2(&clock, transaction_index, instruction_index);
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

fn handle_withdraw_nonce_account(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    instruction: &pb::Instruction,
    data: &pb::WithdrawNonceAccount,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(&clock, transaction_index, instruction_index);
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

fn handle_balances(
    table_name: &str,
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    data: &pb::Balance,
    transaction_index: usize,
    token_balance_index: usize,
) {
    let key = common_key_v2(&clock, transaction_index, token_balance_index);
    let row = tables
        .create_row(table_name, key)
        .set("account", base58::encode(&data.account))
        .set("amount", data.amount);

    set_native_token_transaction_v2(transaction, row);
    set_clock(clock, row);
}
