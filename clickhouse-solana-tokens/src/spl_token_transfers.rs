use common::clickhouse::{common_key, set_clock, set_instruction, set_ordering};
use proto::pb::solana::spl;
use substreams::pb::substreams::Clock;
use substreams_solana::base58;

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
}

fn handle_transfer(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, event: spl::token::transfers::v1::Transfer) {
    let key = common_key(&clock, event.execution_index as u64);
    let instruction = event.instruction().as_str_name();

    // TO-DO: handle empty `mint` values
    let mint = match event.mint {
        Some(mint) => base58::encode(mint),
        None => return, // skip for now
    };
    let decimals = match event.decimals {
        Some(decimals) => decimals,
        None => return, // skip for now
    };
    let row = tables
        .create_row("spl_token_transfers", key)
        .set("source", base58::encode(event.source))
        .set("destination", base58::encode(event.destination))
        .set("mint", mint)
        .set("amount", event.amount.to_string())
        .set("decimals", decimals.to_string());

    set_instruction(event.tx_hash, event.program_id, instruction, event.authority, event.multisig_authority, row);
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
