use std::collections::HashMap;

use common::clickhouse::{common_key_v2, set_authority, set_clock, set_spl_token_instruction_v2, set_spl_token_transaction_v2};
use proto::pb::solana::spl::token::v1 as pb;
use substreams::pb::substreams::Clock;
use substreams_solana::base58;

pub fn process_events(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, events: &pb::Events) {
    // Only keep last balance change per block
    let mut post_token_balances_per_block = HashMap::new();
    let mut pre_token_balances_per_block = HashMap::new();
    for (transaction_index, transaction) in events.transactions.iter().enumerate() {
        // Keep first pre balance and last post balance per account
        for (i, balance) in transaction.pre_token_balances.iter().enumerate() {
            let key = balance.account.as_slice();
            if !pre_token_balances_per_block.contains_key(&key) {
                pre_token_balances_per_block.insert(key, (balance, transaction, transaction_index, i));
            }
        }
        for (i, balance) in transaction.post_token_balances.iter().enumerate() {
            let key = balance.account.as_slice();
            post_token_balances_per_block.insert(key, (balance, transaction, transaction_index, i));
        }
        for (i, instruction) in transaction.instructions.iter().enumerate() {
            match &instruction.instruction {
                // Transfers
                Some(pb::instruction::Instruction::Transfer(data)) => {
                    handle_transfer(tables, clock, transaction, instruction, data, transaction_index, i);
                }
                Some(pb::instruction::Instruction::Mint(data)) => {
                    handle_transfer(tables, clock, transaction, instruction, data, transaction_index, i);
                }
                Some(pb::instruction::Instruction::Burn(data)) => {
                    handle_transfer(tables, clock, transaction, instruction, data, transaction_index, i);
                }
                // Memo
                Some(pb::instruction::Instruction::Memo(data)) => {
                    handle_memo(tables, clock, transaction, instruction, data, transaction_index, i);
                }

                // Permissions
                Some(pb::instruction::Instruction::Approve(data)) => {
                    handle_approve(tables, clock, transaction, instruction, data, transaction_index, i);
                }
                Some(pb::instruction::Instruction::Revoke(data)) => {
                    handle_revoke(tables, clock, transaction, instruction, data, transaction_index, i);
                }
                Some(pb::instruction::Instruction::FreezeAccount(data)) => {
                    handle_freeze_account(tables, clock, transaction, instruction, data, transaction_index, i);
                }
                Some(pb::instruction::Instruction::ThawAccount(data)) => {
                    handle_thaw_account(tables, clock, transaction, instruction, data, transaction_index, i);
                }

                // Mints
                Some(pb::instruction::Instruction::InitializeMint(data)) => {
                    handle_initialize_mint(tables, clock, transaction, instruction, data, transaction_index, i);
                }

                // Accounts
                Some(pb::instruction::Instruction::InitializeAccount(data)) => {
                    handle_initialize_account(tables, clock, transaction, instruction, data, transaction_index, i);
                }
                Some(pb::instruction::Instruction::InitializeImmutableOwner(data)) => {
                    handle_initialize_immutable_owner(tables, clock, transaction, instruction, data, transaction_index, i);
                }
                Some(pb::instruction::Instruction::SetAuthority(data)) => {
                    handle_set_authority(tables, clock, transaction, instruction, data, transaction_index, i);
                }
                Some(pb::instruction::Instruction::CloseAccount(data)) => {
                    handle_close_account(tables, clock, transaction, instruction, data, transaction_index, i);
                }

                // Metadata
                Some(pb::instruction::Instruction::InitializeTokenMetadata(data)) => {
                    handle_initialize_token_metadata(tables, clock, transaction, instruction, data, transaction_index, i);
                }
                Some(pb::instruction::Instruction::UpdateTokenMetadataField(data)) => {
                    handle_update_token_metadata_field(tables, clock, transaction, instruction, data, transaction_index, i);
                }
                Some(pb::instruction::Instruction::UpdateTokenMetadataAuthority(data)) => {
                    handle_update_token_metadata_authority(tables, clock, transaction, instruction, data, transaction_index, i);
                }
                Some(pb::instruction::Instruction::RemoveTokenMetadataField(data)) => {
                    handle_remove_token_metadata_field(tables, clock, transaction, instruction, data, transaction_index, i);
                }
                _ => {}
            }
        }
    }
    let mut skipped = 0;
    for (post_balance, transaction, transaction_index, i) in post_token_balances_per_block.values() {
        // if balance not changed in the block - no need to include it - skip it
        if let Some((pre_balance, _, _, _)) = pre_token_balances_per_block.get(&post_balance.account.as_slice()) {
            if pre_balance.amount == post_balance.amount {
                skipped += 1;
                continue;
            }
        }
        handle_token_balances("post_token_balances", tables, clock, transaction, post_balance, *transaction_index, *i);
    }
    substreams::log::info!("Skipped {} out of {} spl token balances", skipped, post_token_balances_per_block.len());
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
    let decimals_raw = data.decimals.map(|d| d.to_string()).unwrap_or_default();
    let key = common_key_v2(&clock, transaction_index, instruction_index);
    let row = tables
        .create_row("spl_transfer", key)
        .set("source", base58::encode(&data.source))
        .set("destination", base58::encode(&data.destination))
        .set("amount", data.amount)
        .set("mint", base58::encode(&data.mint))
        // -- SPL Token-2022 --
        .set("decimals_raw", decimals_raw);

    set_authority(&data.authority, &data.multisig_authority, row);
    set_spl_token_instruction_v2(instruction, row);
    set_spl_token_transaction_v2(transaction, row);
    set_clock(clock, row);
}

fn handle_initialize_mint(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    instruction: &pb::Instruction,
    data: &pb::InitializeMint,
    transaction_index: usize,
    instruction_index: usize,
) {
    let freeze_authority_raw = data.freeze_authority.as_ref().map(base58::encode).unwrap_or_default();
    let key = common_key_v2(&clock, transaction_index, instruction_index);
    let row = tables
        .create_row("initialize_mint", key)
        .set("mint", base58::encode(&data.mint))
        .set("mint_authority", base58::encode(&data.mint_authority))
        .set("decimals", data.decimals)
        // -- SPL Token-2022 --
        .set("freeze_authority_raw", freeze_authority_raw);

    set_spl_token_instruction_v2(instruction, row);
    set_spl_token_transaction_v2(transaction, row);
    set_clock(clock, row);
}

fn handle_initialize_account(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    instruction: &pb::Instruction,
    data: &pb::InitializeAccount,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(&clock, transaction_index, instruction_index);
    let row = tables
        .create_row("initialize_account", key)
        .set("account", base58::encode(&data.account))
        .set("mint", base58::encode(&data.mint))
        .set("owner", base58::encode(&data.owner));

    set_spl_token_instruction_v2(instruction, row);
    set_spl_token_transaction_v2(transaction, row);
    set_clock(clock, row);
}

fn handle_initialize_immutable_owner(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    instruction: &pb::Instruction,
    data: &pb::InitializeImmutableOwner,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(&clock, transaction_index, instruction_index);
    let row = tables
        .create_row("initialize_immutable_owner", key)
        .set("account", base58::encode(&data.account));

    set_spl_token_instruction_v2(instruction, row);
    set_spl_token_transaction_v2(transaction, row);
    set_clock(clock, row);
}

fn handle_set_authority(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    instruction: &pb::Instruction,
    data: &pb::SetAuthority,
    transaction_index: usize,
    instruction_index: usize,
) {
    let new_authority_raw = data.new_authority.as_ref().map(base58::encode).unwrap_or_default();
    let key = common_key_v2(&clock, transaction_index, instruction_index);
    let row = tables
        .create_row("set_authority", key)
        .set("account", base58::encode(&data.account))
        .set("authority_type", data.authority_type().as_str_name())
        .set("new_authority_raw", new_authority_raw);

    set_authority(&data.authority, &data.multisig_authority, row);
    set_spl_token_instruction_v2(instruction, row);
    set_spl_token_transaction_v2(transaction, row);
    set_clock(clock, row);
}

fn handle_close_account(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    instruction: &pb::Instruction,
    data: &pb::CloseAccount,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(&clock, transaction_index, instruction_index);
    let row = tables
        .create_row("close_account", key)
        .set("account", base58::encode(&data.account))
        .set("destination", base58::encode(&data.destination));

    set_authority(&data.authority, &data.multisig_authority, row);
    set_spl_token_instruction_v2(instruction, row);
    set_spl_token_transaction_v2(transaction, row);
    set_clock(clock, row);

    // Create a zero balance record for the closed account
    // Since the account is closed, set balance to 0 and mint to empty string
    let balance_key = common_key_v2(&clock, transaction_index, instruction_index);
    let balance_row = tables
        .create_row("post_token_balances", balance_key)
        .set("program_id", base58::encode(&instruction.program_id))
        .set("account", base58::encode(&data.account))
        .set("mint", "") // Set mint to empty string for closed accounts
        .set("amount", 0u64)
        .set("decimals", 0u8);

    set_spl_token_transaction_v2(transaction, balance_row);
    set_clock(clock, balance_row);
}

fn handle_initialize_token_metadata(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    instruction: &pb::Instruction,
    data: &pb::InitializeTokenMetadata,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(&clock, transaction_index, instruction_index);
    let row = tables
        .create_row("initialize_token_metadata", key)
        .set("metadata", base58::encode(&data.metadata))
        .set("update_authority", base58::encode(&data.update_authority))
        .set("mint", base58::encode(&data.mint))
        .set("mint_authority", base58::encode(&data.mint_authority))
        .set("name", &data.name)
        .set("symbol", &data.symbol)
        .set("uri", &data.uri);

    set_spl_token_instruction_v2(instruction, row);
    set_spl_token_transaction_v2(transaction, row);
    set_clock(clock, row);
}
fn handle_update_token_metadata_field(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    instruction: &pb::Instruction,
    data: &pb::UpdateTokenMetadataField,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(&clock, transaction_index, instruction_index);
    let row = tables
        .create_row("update_token_metadata_field", key)
        .set("metadata", base58::encode(&data.metadata))
        .set("update_authority", base58::encode(&data.update_authority))
        .set("field", &data.field)
        .set("value", &data.value);

    set_spl_token_instruction_v2(instruction, row);
    set_spl_token_transaction_v2(transaction, row);
    set_clock(clock, row);
}

fn handle_update_token_metadata_authority(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    instruction: &pb::Instruction,
    data: &pb::UpdateTokenMetadataAuthority,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(&clock, transaction_index, instruction_index);
    let row = tables
        .create_row("update_token_metadata_authority", key)
        .set("metadata", base58::encode(&data.metadata))
        .set("update_authority", base58::encode(&data.update_authority))
        .set("new_authority", base58::encode(&data.new_authority));

    set_spl_token_instruction_v2(instruction, row);
    set_spl_token_transaction_v2(transaction, row);
    set_clock(clock, row);
}

fn handle_remove_token_metadata_field(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    instruction: &pb::Instruction,
    data: &pb::RemoveTokenMetadataField,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(&clock, transaction_index, instruction_index);
    let row = tables
        .create_row("remove_token_metadata_field", key)
        .set("metadata", base58::encode(&data.metadata))
        .set("update_authority", base58::encode(&data.update_authority))
        .set("key", &data.key)
        .set("idempotent", data.idempotent);

    set_spl_token_instruction_v2(instruction, row);
    set_spl_token_transaction_v2(transaction, row);
    set_clock(clock, row);
}

fn handle_approve(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    instruction: &pb::Instruction,
    data: &pb::Approve,
    transaction_index: usize,
    instruction_index: usize,
) {
    let mint_raw = data.mint.as_ref().map(base58::encode).unwrap_or_default();
    let decimals_raw = data.decimals.map(|d| d.to_string()).unwrap_or_default();
    let key = common_key_v2(&clock, transaction_index, instruction_index);
    let row = tables
        .create_row("approve", key)
        .set("source", base58::encode(&data.source))
        .set("mint_raw", mint_raw)
        .set("delegate", base58::encode(&data.delegate))
        .set("owner", base58::encode(&data.owner))
        .set("amount", data.amount)
        .set("decimals_raw", decimals_raw);

    set_authority(&data.authority, &data.multisig_authority, row);
    set_spl_token_instruction_v2(instruction, row);
    set_spl_token_transaction_v2(transaction, row);
    set_clock(clock, row);
}

fn handle_revoke(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    instruction: &pb::Instruction,
    data: &pb::Revoke,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(&clock, transaction_index, instruction_index);
    let row = tables
        .create_row("revoke", key)
        .set("source", base58::encode(&data.source))
        .set("owner", base58::encode(&data.owner));

    set_authority(&data.authority, &data.multisig_authority, row);
    set_spl_token_instruction_v2(instruction, row);
    set_spl_token_transaction_v2(transaction, row);
    set_clock(clock, row);
}

fn handle_freeze_account(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    instruction: &pb::Instruction,
    data: &pb::FreezeAccount,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(&clock, transaction_index, instruction_index);
    let row = tables
        .create_row("freeze_account", key)
        .set("account", base58::encode(&data.account))
        .set("mint", base58::encode(&data.mint));

    set_authority(&data.authority, &data.multisig_authority, row);
    set_spl_token_instruction_v2(instruction, row);
    set_spl_token_transaction_v2(transaction, row);
    set_clock(clock, row);
}

fn handle_thaw_account(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    instruction: &pb::Instruction,
    data: &pb::ThawAccount,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(&clock, transaction_index, instruction_index);
    let row = tables
        .create_row("thaw_account", key)
        .set("account", base58::encode(&data.account))
        .set("mint", base58::encode(&data.mint));

    set_authority(&data.authority, &data.multisig_authority, row);
    set_spl_token_instruction_v2(instruction, row);
    set_spl_token_transaction_v2(transaction, row);
    set_clock(clock, row);
}

fn handle_token_balances(
    table_name: &str,
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    data: &pb::TokenBalance,
    transaction_index: usize,
    token_balance_index: usize,
) {
    let key = common_key_v2(&clock, transaction_index, token_balance_index);
    let row = tables
        .create_row(table_name, key)
        .set("program_id", base58::encode(&data.program_id))
        .set("account", base58::encode(&data.account))
        .set("mint", base58::encode(&data.mint))
        .set("amount", data.amount)
        .set("decimals", data.decimals);

    set_spl_token_transaction_v2(transaction, row);
    set_clock(clock, row);
}

fn handle_memo(
    tables: &mut substreams_database_change::tables::Tables,
    clock: &Clock,
    transaction: &pb::Transaction,
    instruction: &pb::Instruction,
    data: &pb::Memo,
    transaction_index: usize,
    instruction_index: usize,
) {
    let key = common_key_v2(&clock, transaction_index, instruction_index);
    let row = tables.create_row("spl_memo", key).set("memo", &data.memo);

    set_spl_token_instruction_v2(instruction, row);
    set_spl_token_transaction_v2(transaction, row);
    set_clock(clock, row);
}
