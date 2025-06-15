use common::clickhouse::{common_key, set_clock, set_instruction, set_ordering};
use proto::pb::solana::spl;
use substreams::pb::substreams::Clock;
use substreams_solana::base58;

pub fn process_spl_token_balances(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, events: spl::token::balances::v1::Events) {
    // -- Balances --
    for event in events.balances {
        handle_balance(tables, clock, event);
    }
}

fn handle_balance(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, event: spl::token::balances::v1::Balance) {
    let key = common_key(&clock, event.execution_index as u64);
    let row = tables
        .create_row("spl_token_balance_changes", key)
        .set("owner", base58::encode(event.owner))
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
