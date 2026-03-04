mod aldrin;
mod boop;
mod byreal;
mod darklake;
mod dflow;
mod drift;
mod dumpfun;
mod enums;
mod goonfi;
mod heaven;
mod jupiter;
mod lifinity;
mod meteora_amm;
mod meteora_daam;
mod meteora_dllm;
mod moonshot;
mod obric;
mod okx_dex;
mod openbook;
mod orca;
mod pancakeswap;
mod phoenix;
mod plasma;
mod pumpfun;
mod pumpfun_amm;
mod pumpswap;
mod raydium_amm_v4;
mod raydium_clmm;
mod raydium_cpmm;
mod raydium_launchpad;
mod sanctum;
mod saros;
mod serum;
mod solfi;
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
    dumpfun_events: pb::dumpfun::v1::Events,
    goonfi_events: pb::goonfi::v1::Events,
    heaven_events: pb::heaven::v1::Events,
    plasma_events: pb::plasma::v1::Events,
    saros_events: pb::saros::v1::Events,
    aldrin_events: pb::aldrin::v1::Events,
    boop_events: pb::boop::v1::Events,
    byreal_events: pb::byreal::v1::Events,
    dflow_events: pb::dflow::v1::Events,
    drift_events: pb::drift::v1::Events,
    obric_v2_events: pb::obric::v2::v1::Events,
    obric_v3_events: pb::obric::v3::v1::Events,
    okx_events: pb::okx::dex::v1::Events,
    sanctum_events: pb::sanctum::v1::Events,
    serum_events: pb::serum::v1::Events,
    solfi_v1_events: pb::solfi::v1::v1::Events,
    solfi_v2_events: pb::solfi::v2::v1::Events,
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
    dumpfun::process_events(&mut tables, &clock, &dumpfun_events);
    goonfi::process_events(&mut tables, &clock, &goonfi_events);
    heaven::process_events(&mut tables, &clock, &heaven_events);
    plasma::process_events(&mut tables, &clock, &plasma_events);
    saros::process_events(&mut tables, &clock, &saros_events);
    aldrin::process_events(&mut tables, &clock, &aldrin_events);
    boop::process_events(&mut tables, &clock, &boop_events);
    byreal::process_events(&mut tables, &clock, &byreal_events);
    dflow::process_events(&mut tables, &clock, &dflow_events);
    drift::process_events(&mut tables, &clock, &drift_events);
    obric::process_v2_events(&mut tables, &clock, &obric_v2_events);
    obric::process_v3_events(&mut tables, &clock, &obric_v3_events);
    okx_dex::process_events(&mut tables, &clock, &okx_events);
    sanctum::process_events(&mut tables, &clock, &sanctum_events);
    serum::process_events(&mut tables, &clock, &serum_events);
    solfi::process_v1_events(&mut tables, &clock, &solfi_v1_events);
    solfi::process_v2_events(&mut tables, &clock, &solfi_v2_events);

    // ONLY include blocks if events are present
    if tables.all_row_count() > 0 {
        set_clock(&clock, tables.create_row("blocks", [("block_num", clock.number.to_string())]));
    }

    Ok(tables.to_database_changes())
}
