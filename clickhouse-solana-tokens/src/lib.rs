mod native_token;
mod spl_token;
use common::{clickhouse::set_clock, solana::update_genesis_clock};
use proto::pb::solana as pb;
use substreams::{errors::Error, pb::substreams::Clock};
use substreams_database_change::pb::database::DatabaseChanges;

#[substreams::handlers::map]
pub fn db_out(mut clock: Clock, spl_token: pb::spl::token::v1::Events, native_token: pb::native::token::v1::Events) -> Result<DatabaseChanges, Error> {
    clock = update_genesis_clock(clock);
    let mut tables = substreams_database_change::tables::Tables::new();

    spl_token::process_events(&mut tables, &clock, &spl_token);
    native_token::process_events(&mut tables, &clock, &native_token);

    // ONLY include blocks if events are present
    if tables.tables.len() > 0 {
        set_clock(&clock, tables.create_row("blocks", [("block_num", clock.number.to_string())]));
    }
    substreams::log::info!("Total rows {}", tables.all_row_count());
    Ok(tables.to_database_changes())
}
