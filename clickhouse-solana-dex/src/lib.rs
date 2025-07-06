// mod pumpfun;
mod enums;
mod raydium_amm_v4;
mod spl_token_metadata;
use common::clickhouse::set_clock;
use proto::pb;
use substreams::{errors::Error, pb::substreams::Clock};
use substreams_database_change::pb::database::DatabaseChanges;

#[substreams::handlers::map]
pub fn db_out(
    mut clock: Clock,
    // DEXs
    raydium_events: pb::raydium::amm::v1::Events,
    // spl_token_transfer_events: pb::solana::spl::token::transfers::v1::Events,
    // pumpfun_events: pb::pumpfun::PumpfunBlockEvents,
) -> Result<DatabaseChanges, Error> {
    let mut tables = substreams_database_change::tables::Tables::new();

    // Process Events
    raydium_amm_v4::process_events(&mut tables, &clock, &raydium_events);
    // spl_token_metadata::process_events(&mut tables, &clock, &spl_token_transfer_events);
    // pumpfun::process_events(&mut tables, &clock, &pumpfun_events);

    // ONLY include blocks if events are present
    if tables.tables.len() > 0 {
        set_clock(&clock, tables.create_row("blocks", [("block_num", clock.number.to_string())]));
    }

    Ok(tables.to_database_changes())
}
