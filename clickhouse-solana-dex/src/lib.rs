mod enums;
mod jupiter;
mod meteora_amm;
mod meteora_daam;
mod meteora_dllm;
mod pumpfun;
mod pumpfun_amm;
mod raydium_amm_v4;
mod raydium_clmm;
mod raydium_cpmm;
mod raydium_launchpad;

use common::clickhouse::set_clock;
use proto::pb;
use substreams::{errors::Error, pb::substreams::Clock};
use substreams_database_change::pb::sf::substreams::sink::database::v1::DatabaseChanges;

#[substreams::handlers::map]
pub fn db_out(
    mut clock: Clock,
    pumpfun_events: pb::pumpfun::v1::Events,
    pumpfun_amm_events: pb::pumpfun::amm::v1::Events,
    raydium_amm_v4_events: pb::raydium::amm::v1::Events,
    raydium_cpmm_events: pb::raydium::cpmm::v1::Events,
    raydium_clmm_events: pb::raydium::clmm::v1::Events,
    raydium_launchpad_events: pb::raydium::launchpad::v1::Events,
    meteora_dllm_events: pb::meteora::dllm::v1::Events,
    meteora_daam_events: pb::meteora::daam::v1::Events,
    meteora_amm_events: pb::meteora::amm::v1::Events,
    jupiter_v4_events: pb::jupiter::v1::Events,
    jupiter_v6_events: pb::jupiter::v1::Events,
) -> Result<DatabaseChanges, Error> {
    let mut tables = substreams_database_change::tables::Tables::new();

    // Process Events
    pumpfun::process_events(&mut tables, &clock, &pumpfun_events);
    pumpfun_amm::process_events(&mut tables, &clock, &pumpfun_amm_events);
    raydium_amm_v4::process_events(&mut tables, &clock, &raydium_amm_v4_events);
    raydium_cpmm::process_events(&mut tables, &clock, &raydium_cpmm_events);
    raydium_clmm::process_events(&mut tables, &clock, &raydium_clmm_events);
    raydium_launchpad::process_events(&mut tables, &clock, &raydium_launchpad_events);
    meteora_dllm::process_events(&mut tables, &clock, &meteora_dllm_events);
    meteora_daam::process_events(&mut tables, &clock, &meteora_daam_events);
    meteora_amm::process_events(&mut tables, &clock, &meteora_amm_events);
    jupiter::process_events(&mut tables, &clock, &jupiter_v4_events);
    jupiter::process_events(&mut tables, &clock, &jupiter_v6_events);

    // ONLY include blocks if events are present
    if tables.all_row_count() > 0 {
        set_clock(&clock, tables.create_row("blocks", [("block_num", clock.number.to_string())]));
    }

    Ok(tables.to_database_changes())
}
