use common::clickhouse::{common_key, set_authority, set_clock, set_instruction, set_ordering};
use proto::pb::solana::spl;
use substreams::pb::substreams::Clock;
use substreams_solana::base58;

use crate::enums::TokenStandard;

pub fn process_spl_token_transfers(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, events: spl::token::transfers::v1::Events) {
    // -- Transfers --
    for event in events.transfers {
        handle_transfer(tables, clock, event);
    }
    for event in events.mints {
        handle_transfer(tables, clock, event);
    }
    for event in events.burns {
        handle_transfer(tables, clock, event);
    }
    for event in events.initialize_accounts {
        handle_initialize_account(tables, clock, event);
    }
    for event in events.initialize_mints {
        handle_initialize_mint(tables, clock, event);
    }
    for event in events.approves {
        handle_approve(tables, clock, event);
    }
    for event in events.revokes {
        handle_revoke(tables, clock, event);
    }
}

fn handle_transfer(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, event: spl::token::transfers::v1::Transfer) {
    let key = common_key(&clock, event.execution_index as u64);
    let instruction = event.instruction().as_str_name();

    let token_standard = match event.mint.clone() {
        Some(_) => TokenStandard::SplToken2022,
        None => TokenStandard::ClassicSplToken,
    };
    let mint_raw = match event.mint {
        Some(mint) => base58::encode(mint),
        None => "".to_string(),
    };
    let decimals_raw = match event.decimals {
        Some(decimals) => decimals.to_string(),
        None => "".to_string(),
    };
    let row = tables
        .create_row("transfers", key)
        .set("source", base58::encode(event.source))
        .set("destination", base58::encode(event.destination))
        .set("amount", event.amount.to_string())
        // -- SPL Token-2022 --
        .set("mint_raw", mint_raw)
        .set("decimals_raw", decimals_raw.to_string())
        .set("token_standard", token_standard.to_string()); // Enum8('Native' = 1, 'SPL Token' = 2, 'SPL Token-2022' = 3)

    set_instruction(event.tx_hash, event.program_id, instruction, row);
    set_authority(event.authority, event.multisig_authority, row);
    set_ordering(
        event.execution_index,
        event.instruction_index,
        event.inner_instruction_index,
        event.stack_height,
        clock,
        row,
    );
    set_clock(clock, row);
}

fn handle_initialize_account(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, event: spl::token::transfers::v1::InitializeAccount) {
    let key = common_key(&clock, event.execution_index as u64);
    let instruction = event.instruction().as_str_name();

    let row = tables
        .create_row("initialize_account", key)
        .set("account", base58::encode(event.account))
        .set("mint", base58::encode(event.mint))
        .set("owner", base58::encode(event.owner));

    set_instruction(event.tx_hash, event.program_id, instruction, row);
    set_ordering(
        event.execution_index,
        event.instruction_index,
        event.inner_instruction_index,
        event.stack_height,
        clock,
        row,
    );
    set_clock(clock, row);
}

fn handle_initialize_mint(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, event: spl::token::transfers::v1::InitializeMint) {
    let key = common_key(&clock, event.execution_index as u64);
    let instruction = event.instruction().as_str_name();

    let row = tables
        .create_row("initialize_mint", key)
        .set("mint", base58::encode(event.mint))
        .set("mint_authority", base58::encode(event.mint_authority))
        .set(
            "freeze_authority",
            event.freeze_authority.as_ref().map_or("".to_string(), |fa| base58::encode(fa)),
        )
        .set("decimals", event.decimals);

    set_instruction(event.tx_hash, event.program_id, instruction, row);
    set_ordering(
        event.execution_index,
        event.instruction_index,
        event.inner_instruction_index,
        event.stack_height,
        clock,
        row,
    );
    set_clock(clock, row);
}

fn handle_approve(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, event: spl::token::transfers::v1::Approve) {
    let key = common_key(&clock, event.execution_index as u64);
    let instruction = event.instruction().as_str_name();

    let token_standard = match event.mint.clone() {
        Some(_) => TokenStandard::SplToken2022,
        None => TokenStandard::ClassicSplToken,
    };
    let mint_raw = match event.mint {
        Some(mint) => base58::encode(mint),
        None => "".to_string(),
    };
    let decimals_raw = match event.decimals {
        Some(decimals) => decimals.to_string(),
        None => "".to_string(),
    };
    let row = tables
        .create_row("approve", key)
        .set("source", base58::encode(event.source))
        .set("delegate", base58::encode(event.delegate))
        .set("owner", base58::encode(event.owner))
        .set("amount", event.amount.to_string())
        // -- SPL Token-2022 --
        .set("mint_raw", mint_raw)
        .set("decimals_raw", decimals_raw.to_string())
        .set("token_standard", token_standard.to_string()); // Enum8('Native' = 1, 'SPL Token' = 2, 'SPL Token-2022 = 3)

    set_instruction(event.tx_hash, event.program_id, instruction, row);
    set_authority(event.authority, event.multisig_authority, row);
    set_ordering(
        event.execution_index,
        event.instruction_index,
        event.inner_instruction_index,
        event.stack_height,
        clock,
        row,
    );
    set_clock(clock, row);
}

fn handle_revoke(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, event: spl::token::transfers::v1::Revoke) {
    let key = common_key(&clock, event.execution_index as u64);
    let instruction = event.instruction().as_str_name();

    let row = tables
        .create_row("revoke", key)
        .set("source", base58::encode(event.source))
        .set("owner", base58::encode(event.owner));

    set_instruction(event.tx_hash, event.program_id, instruction, row);
    set_authority(event.authority, event.multisig_authority, row);
    set_ordering(
        event.execution_index,
        event.instruction_index,
        event.inner_instruction_index,
        event.stack_height,
        clock,
        row,
    );
    set_clock(clock, row);
}
