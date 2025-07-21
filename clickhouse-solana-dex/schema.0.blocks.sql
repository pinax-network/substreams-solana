CREATE TABLE IF NOT EXISTS blocks (
    block_num                   UInt32,
    block_hash                  FixedString(44),
    timestamp                   DateTime(0, 'UTC'),

    -- indexes --
    INDEX idx_block_hash                 (block_hash)               TYPE bloom_filter GRANULARITY 1,
    INDEX idx_timestamp                  (timestamp)                TYPE minmax GRANULARITY 4

) ENGINE = ReplacingMergeTree(timestamp) -- in case of reorgs, keep the latest block by timestamp
ORDER BY block_num;