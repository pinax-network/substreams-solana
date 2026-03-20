mod marinade;
mod native_stake;

use common::db::set_clock;
use proto::pb;
use substreams::{errors::Error, pb::substreams::Clock};
use substreams_database_change::pb::sf::substreams::sink::database::v1::DatabaseChanges;

#[substreams::handlers::map]
pub fn db_out(
    clock: Clock,
    marinade_events: pb::marinade::v1::Events,
    native_stake_events: pb::solana::native::stake::v1::Events,
) -> Result<DatabaseChanges, Error> {
    let mut tables = substreams_database_change::tables::Tables::new();

    marinade::process_events(&mut tables, &clock, &marinade_events);
    native_stake::process_events(&mut tables, &clock, &native_stake_events);

    if tables.all_row_count() > 0 {
        set_clock(&clock, tables.create_row("blocks", [("block_num", clock.number.to_string())]));
    }

    Ok(tables.to_database_changes())
}
