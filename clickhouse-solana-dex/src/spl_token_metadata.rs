use common::clickhouse::set_clock;
use proto::pb::solana::spl;
use substreams::pb::substreams::Clock;
use substreams_solana::base58;

pub fn process_events(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, events: &spl::token::transfers::v1::Events) {
    for event in events.initialize_mints.iter() {
        handle_initialize_mint(tables, clock, event);
    }
}

fn handle_initialize_mint(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, event: &spl::token::transfers::v1::InitializeMint) {
    let mint = base58::encode(event.mint.to_vec());
    let program_id: String = base58::encode(event.program_id.clone());
    let key = [("mint", mint.to_string()), ("program_id", program_id.to_string())];

    let row = tables
        .create_row("mints", key)
        .set("mint", mint)
        .set("program_id", program_id)
        .set("decimals", event.decimals);

    set_clock(clock, row);
}
