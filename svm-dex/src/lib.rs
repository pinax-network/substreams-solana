mod darklake;
mod enums;
mod jupiter;
mod lifinity;
mod meteora_amm;
mod meteora_daam;
mod meteora_dllm;
mod moonshot;
mod openbook;
mod orca;
mod pancakeswap;
mod phoenix;
mod pumpfun;
mod pumpfun_amm;
mod pumpswap;
mod raydium_amm_v4;
mod raydium_clmm;
mod raydium_cpmm;
mod raydium_launchpad;
mod stabble;

use common::db::set_clock;
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
    orca_events: pb::orca::v1::Events,
    phoenix_events: pb::phoenix::v1::Events,
    openbook_events: pb::openbook::v1::Events,
    pumpswap_events: pb::pumpswap::v1::Events,
    darklake_events: pb::darklake::v1::Events,
    lifinity_events: pb::lifinity::v1::Events,
    moonshot_events: pb::moonshot::v1::Events,
    pancakeswap_events: pb::pancakeswap::v1::Events,
    stabble_events: pb::stabble::v1::Events,
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
    orca::process_events(&mut tables, &clock, &orca_events);
    phoenix::process_events(&mut tables, &clock, &phoenix_events);
    openbook::process_events(&mut tables, &clock, &openbook_events);
    pumpswap::process_events(&mut tables, &clock, &pumpswap_events);
    darklake::process_events(&mut tables, &clock, &darklake_events);
    lifinity::process_events(&mut tables, &clock, &lifinity_events);
    moonshot::process_events(&mut tables, &clock, &moonshot_events);
    pancakeswap::process_events(&mut tables, &clock, &pancakeswap_events);
    stabble::process_events(&mut tables, &clock, &stabble_events);

    // ONLY include blocks if events are present
    if tables.all_row_count() > 0 {
        set_clock(&clock, tables.create_row("blocks", [("block_num", clock.number.to_string())]));
    }

    Ok(tables.to_database_changes())
}
