use common::db::{common_key_v2, set_authority, set_clock, set_spl_token_instruction_v2, set_spl_token_transaction_v2};
use proto::pb::solana::spl::token::v1 as pb;
use substreams::pb::substreams::Clock;
use substreams_solana::base58;

pub fn process_events(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, events: &pb::Events) {
    for (transaction_index, transaction) in events.transactions.iter().enumerate() {
        for (i, instruction) in transaction.instructions.iter().enumerate() {
            match &instruction.instruction {
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
                _ => {}
            }
        }
    }
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
    let key = common_key_v2(clock, transaction_index, instruction_index);
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
    let key = common_key_v2(clock, transaction_index, instruction_index);
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
    let key = common_key_v2(clock, transaction_index, instruction_index);
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
    let key = common_key_v2(clock, transaction_index, instruction_index);
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
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row("close_account", key)
        .set("account", base58::encode(&data.account))
        .set("destination", base58::encode(&data.destination));

    set_authority(&data.authority, &data.multisig_authority, row);
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
    let key = common_key_v2(clock, transaction_index, instruction_index);
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
    let key = common_key_v2(clock, transaction_index, instruction_index);
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
    let key = common_key_v2(clock, transaction_index, instruction_index);
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
    let key = common_key_v2(clock, transaction_index, instruction_index);
    let row = tables
        .create_row("thaw_account", key)
        .set("account", base58::encode(&data.account))
        .set("mint", base58::encode(&data.mint));

    set_authority(&data.authority, &data.multisig_authority, row);
    set_spl_token_instruction_v2(instruction, row);
    set_spl_token_transaction_v2(transaction, row);
    set_clock(clock, row);
}
