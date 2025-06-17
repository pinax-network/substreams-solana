-- SPL Token-2022 & Classic balance changes by mint/owner --
-- Only keep last balance change per account per block, use `execution_index` to resolve conflicts
CREATE TABLE IF NOT EXISTS balance_changes  (
    -- block --
    block_num           UInt32,
    block_hash          FixedString(44),
    timestamp           DateTime(0, 'UTC'),
    timestamp_since_genesis     DateTime(0, 'UTC')
        MATERIALIZED if (
            timestamp = 0,
            toDateTime(1584332940 + intDiv(block_num * 2, 5), 'UTC'),
            timestamp
        ),

    -- transaction --
    tx_hash             FixedString(88),

    -- ordering --
    execution_index     UInt32, -- relative index

    -- account --
    program_id          LowCardinality(FixedString(44)),

    -- event --
    owner               FixedString(44),
    mint                FixedString(44),
    amount              UInt64,
    decimals            UInt8,

    -- indexes --
    INDEX idx_block_num          (block_num)           TYPE minmax GRANULARITY 4,
    INDEX idx_timestamp          (timestamp)           TYPE minmax GRANULARITY 4,
    INDEX idx_timestamp_since_genesis    (timestamp_since_genesis)  TYPE minmax GRANULARITY 4

    -- indexes (event) --
    INDEX idx_program_id         (program_id)          TYPE set(8) GRANULARITY 4,
    INDEX idx_mint               (mint)                TYPE set(128) GRANULARITY 4,
    INDEX idx_owner              (owner)               TYPE bloom_filter GRANULARITY 4,
    INDEX idx_amount             (amount)              TYPE minmax GRANULARITY 4,
    INDEX idx_decimals           (decimals)            TYPE minmax GRANULARITY 4
)
ENGINE = ReplacingMergeTree(execution_index)
ORDER BY (mint, owner, block_hash);
