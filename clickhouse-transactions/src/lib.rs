use common::bytes_to_hex;
use substreams::scalar::BigInt;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables;

use substreams_ethereum::pb::eth::v2::Block;

#[substreams::handlers::map]
pub fn db_out(block: Block) -> Result<DatabaseChanges, substreams::errors::Error> {
    let mut tables = Tables::new();

    let block_hash = bytes_to_hex(&block.hash);
    let block_number = block.number.to_string();
    let block_timestamp = block
        .header
        .as_ref()
        .and_then(|h| h.timestamp.as_ref())
        .map(|t| t.seconds as u64)
        .unwrap_or(0)
        .to_string();

    for trace in block.transactions() {
        tables
            .create_row("transactions", [("tx_hash", bytes_to_hex(&trace.hash))])
            .set("block_num", &block_number)
            .set("timestamp", &block_timestamp)
            .set("block_hash", &block_hash)
            .set("nonce", trace.nonce.to_string())
            .set("position", trace.index.to_string())
            .set("from_address", bytes_to_hex(&trace.from))
            .set("to_address", bytes_to_hex(&trace.to))
            .set(
                "value",
                trace
                    .value
                    .as_ref()
                    .map(|v| BigInt::from_unsigned_bytes_be(&v.bytes))
                    .unwrap_or_else(|| BigInt::zero())
                    .to_string(),
            )
            .set("calls_count", trace.calls.len().to_string())
            .set("logs_count", trace.receipt.as_ref().map(|r| r.logs.len()).unwrap_or(0).to_string())
            .set(
                "tx_fee",
                trace
                    .gas_price
                    .as_ref()
                    .map(|gp| (BigInt::from_unsigned_bytes_be(&gp.bytes) * BigInt::from(trace.gas_used)))
                    .unwrap_or_else(|| BigInt::zero())
                    .to_string(),
            )
            .set(
                "gas_price",
                trace
                    .gas_price
                    .as_ref()
                    .map(|v| BigInt::from_unsigned_bytes_be(&v.bytes))
                    .unwrap_or_else(|| BigInt::zero())
                    .to_string(),
            )
            .set("gas_limit", trace.gas_limit.to_string())
            .set("gas_used", trace.gas_used.to_string())
            .set(
                "cumulative_gas_used",
                trace.receipt.as_ref().map(|r| r.cumulative_gas_used).unwrap_or(0).to_string(),
            )
            .set(
                "max_fee_per_gas",
                trace
                    .max_fee_per_gas
                    .as_ref()
                    .map(|v| BigInt::from_unsigned_bytes_be(&v.bytes))
                    .unwrap_or_else(|| BigInt::zero())
                    .to_string(),
            )
            .set(
                "max_priority_fee_per_gas",
                trace
                    .max_priority_fee_per_gas
                    .as_ref()
                    .map(|v| BigInt::from_unsigned_bytes_be(&v.bytes))
                    .unwrap_or_else(|| BigInt::zero())
                    .to_string(),
            )
            .set("input", bytes_to_hex(&trace.input))
            .set("type", trace.r#type.to_string())
            .set("v", bytes_to_hex(&trace.v))
            .set("r", bytes_to_hex(&trace.r))
            .set("s", bytes_to_hex(&trace.s));
    }

    Ok(tables.to_database_changes())
}
