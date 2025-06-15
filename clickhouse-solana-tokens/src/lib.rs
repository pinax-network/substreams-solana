mod enums;
mod spl_token_balances;
mod spl_token_transfers;
use proto::pb::solana::spl;
use substreams::{errors::Error, pb::substreams::Clock};
use substreams_database_change::pb::database::DatabaseChanges;

#[substreams::handlers::map]
pub fn db_out(
    mut clock: Clock,

    // SPL Tokens
    spl_transfers: spl::token::transfers::v1::Events,
    spl_balances: spl::token::balances::v1::Events,
) -> Result<DatabaseChanges, Error> {
    let mut tables = substreams_database_change::tables::Tables::new();

    spl_token_transfers::process_spl_token_transfers(&mut tables, &clock, spl_transfers);
    spl_token_balances::process_spl_token_balances(&mut tables, &clock, spl_balances);

    Ok(tables.to_database_changes())
}
