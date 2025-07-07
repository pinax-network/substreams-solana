mod enums;
mod jupiter;
mod pumpfun;
mod raydium_amm_v4;

use common::clickhouse::set_clock;
use proto::pb;
use substreams::{errors::Error, pb::substreams::Clock};
use substreams_database_change::pb::database::DatabaseChanges;

#[substreams::handlers::map]
pub fn db_out(
    mut clock: Clock,
    pumpfun_events: pb::pumpfun::v1::Events,
    raydium_amm_v4_events: pb::raydium::amm::v1::Events,
    jupiter_v4_events: pb::jupiter::v1::Events,
    jupiter_v6_events: pb::jupiter::v1::Events,
) -> Result<DatabaseChanges, Error> {
    let mut tables = substreams_database_change::tables::Tables::new();

    // Process Events
    raydium_amm_v4::process_events(&mut tables, &clock, &raydium_amm_v4_events);
    jupiter::process_events(&mut tables, &clock, &jupiter_v4_events);
    jupiter::process_events(&mut tables, &clock, &jupiter_v6_events);
    pumpfun::process_events(&mut tables, &clock, &pumpfun_events);

    // ONLY include blocks if events are present
    if tables.tables.len() > 0 {
        set_clock(&clock, tables.create_row("blocks", [("block_num", clock.number.to_string())]));
    }

    Ok(tables.to_database_changes())
}
