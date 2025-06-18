mod raydium_amm;
use common::clickhouse::set_clock;
use proto::pb;
use substreams::{errors::Error, pb::substreams::Clock};
use substreams_database_change::pb::database::DatabaseChanges;

#[substreams::handlers::map]
pub fn db_out(
    mut clock: Clock,
    // DEXs
    raydium_events: pb::raydium_amm::RaydiumAmmBlockEvents,
    pumpfun_events: pb::pumpfun::PumpfunBlockEvents,
) -> Result<DatabaseChanges, Error> {
    let mut tables = substreams_database_change::tables::Tables::new();

    raydium_amm::process_events(&mut tables, &clock, &raydium_events);

    // ONLY include blocks if events are present
    if tables.tables.len() > 0 {
        set_clock(&clock, tables.create_row("blocks", [("block_num", clock.number.to_string())]));
    }

    Ok(tables.to_database_changes())
}
