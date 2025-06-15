use common::clickhouse::set_clock;
use proto::pb::solana::spl;
use substreams::pb::substreams::Clock;
use substreams_solana::base58;

use crate::enums::TokenStandard;

pub fn process_spl_token_balances(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, events: spl::token::balances::v1::Events) {
    // -- Balances --
    for event in events.balances {
        handle_balance(tables, clock, event);
    }
}

fn handle_balance(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, event: spl::token::balances::v1::Balance) {
    let mint = base58::encode(event.mint);
    let owner = base58::encode(event.owner);
    let key = [("mint", mint.to_string()), ("owner", owner.to_string()), ("block_hash", clock.id.to_string())];
    let row = tables
        .create_row("balance_changes", key)
        // -- Ordering --
        .set("execution_index", event.execution_index)
        // -- Transaction --
        .set("tx_hash", base58::encode(event.tx_hash))
        .set("program_id", base58::encode(event.program_id))
        // -- Data --
        .set("owner", owner)
        .set("mint", mint)
        .set("amount", event.amount.to_string())
        .set("decimals", event.decimals.to_string())
        .set("token_standard", TokenStandard::SplToken.to_string()); // Enum8('Native' = 1, 'SPL Token' = 2, 'SPL Token-2022' = 3)

    set_clock(clock, row);
}
