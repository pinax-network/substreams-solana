CREATE TABLE IF NOT EXISTS blocks (
    block_num                   UInt32,
    block_hash                  FixedString(44),
    timestamp                   DateTime(0, 'UTC'),
    timestamp_since_genesis     DateTime(0, 'UTC')
        MATERIALIZED if (
            timestamp = 0,
            toDateTime(1584332940 + intDiv(block_num * 2, 5), 'UTC'),
            timestamp
        ),

    -- indexes --
    INDEX idx_block_hash                 (block_hash)               TYPE bloom_filter GRANULARITY 4,
    INDEX idx_timestamp                  (timestamp)                TYPE minmax GRANULARITY 4,
    INDEX idx_timestamp_since_genesis    (timestamp_since_genesis)  TYPE minmax GRANULARITY 4

) ENGINE = ReplacingMergeTree(timestamp)
ORDER BY block_num;