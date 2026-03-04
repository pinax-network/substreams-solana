mod magiceden_m2;
mod magiceden_m3;
mod tensor;

use common::db::set_clock;
use proto::pb;
use substreams::{errors::Error, pb::substreams::Clock};
use substreams_database_change::pb::sf::substreams::sink::database::v1::DatabaseChanges;

#[substreams::handlers::map]
pub fn db_out(
    clock: Clock,
    magiceden_m2_events: pb::magiceden::m2::v1::Events,
    magiceden_m3_events: pb::magiceden::m3::v1::Events,
    tensor_events: pb::tensor::v1::Events,
) -> Result<DatabaseChanges, Error> {
    let mut tables = substreams_database_change::tables::Tables::new();

    magiceden_m2::process_events(&mut tables, &clock, &magiceden_m2_events);
    magiceden_m3::process_events(&mut tables, &clock, &magiceden_m3_events);
    tensor::process_events(&mut tables, &clock, &tensor_events);

    if tables.all_row_count() > 0 {
        set_clock(&clock, tables.create_row("blocks", [("block_num", clock.number.to_string())]));
    }

    Ok(tables.to_database_changes())
}
