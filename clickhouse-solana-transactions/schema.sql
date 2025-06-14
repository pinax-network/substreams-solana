-- EVM Transactions Table
CREATE TABLE IF NOT EXISTS transactions (
    block_num                   UInt64,
    block_hash                  FixedString(66),
    timestamp                   DateTime(0, 'UTC'),
    tx_hash                     FixedString(66),
    nonce                       UInt64,
    position                    UInt32,
    from_address                FixedString(42),
    to_address                  FixedString(42),
    value                       String,
    calls_count                 UInt32,
    logs_count                  UInt32,
    tx_fee                      String,
    gas_price                   String,
    gas_limit                   UInt64,
    gas_used                    UInt64,
    cumulative_gas_used         UInt64,
    max_fee_per_gas             String,
    max_priority_fee_per_gas    String,
    input                       String,
    type                        Int32,
    v                           String,
    r                           String,
    s                           String,

    INDEX idx_block_num         (block_num)              TYPE minmax GRANULARITY 4,
    INDEX idx_timestamp         (timestamp)              TYPE minmax GRANULARITY 4,
    INDEX idx_from              (from_address)           TYPE bloom_filter GRANULARITY 4,
    INDEX idx_to                (to_address)             TYPE bloom_filter GRANULARITY 4,

) ENGINE = ReplacingMergeTree
ORDER BY tx_hash;
