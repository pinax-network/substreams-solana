mod spl_token;
// mod native_token;
use common::clickhouse::set_clock;
use proto::pb::solana as pb;
use substreams::{errors::Error, pb::substreams::Clock};
use substreams_database_change::pb::database::DatabaseChanges;

#[substreams::handlers::map]
pub fn db_out(mut clock: Clock, spl_token: pb::spl::token::v1::Events, native_token: pb::native::token::v1::Events) -> Result<DatabaseChanges, Error> {
    let mut tables = substreams_database_change::tables::Tables::new();

    // native_token::process_spl_token_transfers(&mut tables, &clock, spl_transfers);
    spl_token::process_events(&mut tables, &clock, &spl_token);

    // ONLY include blocks if events are present
    if tables.tables.len() > 0 {
        set_clock(&clock, tables.create_row("blocks", [("block_num", clock.number.to_string())]));
    }

    Ok(tables.to_database_changes())
}
