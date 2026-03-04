mod accounts;
mod balances;
mod metadata;
mod transfers;

use common::{db::set_clock, solana::update_genesis_clock};
use proto::pb::solana as pb;
use substreams::{errors::Error, pb::substreams::Clock};
use substreams_database_change::pb::sf::substreams::sink::database::v1::DatabaseChanges;

#[substreams::handlers::map]
pub fn db_out(mut clock: Clock, spl_token: pb::spl::token::v1::Events) -> Result<DatabaseChanges, Error> {
    clock = update_genesis_clock(clock);
    let mut tables = substreams_database_change::tables::Tables::new();

    transfers::process_events(&mut tables, &clock, &spl_token);
    balances::process_events(&mut tables, &clock, &spl_token);
    accounts::process_events(&mut tables, &clock, &spl_token);
    metadata::process_events(&mut tables, &clock, &spl_token);

    // ONLY include blocks if events are present
    if tables.all_row_count() > 0 {
        set_clock(&clock, tables.create_row("blocks", [("block_num", clock.number.to_string())]));
    }
    substreams::log::info!("Total rows {}", tables.all_row_count());
    Ok(tables.to_database_changes())
}
