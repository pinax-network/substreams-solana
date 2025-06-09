mod pb;
use crate::pb::contract_creation::v1::Events;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables;

#[substreams::handlers::map]
pub fn db_out(events: Events) -> Result<DatabaseChanges, substreams::errors::Error> {
    let tables = events.contract_creations.into_iter().fold(Tables::new(), |mut tables, contract| {
        tables
            .create_row("contracts", [("address", contract.address)])
            .set("timestamp", contract.block_time.unwrap_or_default().seconds)
            .set("block_num", contract.block_number.to_string())
            .set("block_hash", contract.block_hash)
            .set("tx_hash", contract.transaction_hash)
            .set("tx_index", contract.transaction_index)
            .set("creator", contract.from)
            .set("factory", contract.factory.unwrap_or_default())
            .set("code", contract.code.unwrap_or_default())
            .set("code_hash", contract.code_hash.unwrap_or_default())
            .set("input", contract.input.unwrap_or_default());
        tables
    });

    Ok(tables.to_database_changes())
}
