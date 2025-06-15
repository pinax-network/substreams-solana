-- latest balances by owner/contract --
CREATE TABLE IF NOT EXISTS balances  (
    -- block --
    block_num               UInt32,
    block_hash              FixedString(66),
    timestamp               DateTime(0, 'UTC'),

    -- account --
    program_id              FixedString(32),

    -- event --
    owner                   FixedString(44),
    mint                    FixedString(44),
    amount                  UInt64,
    decimals                UInt8,

    -- classification --
    token_standard              Enum8('Classic SPL Token' = 1, 'SPL Token-2022' = 2, 'Native' = 3),

    -- indexes --
    INDEX idx_block_num          (block_num)           TYPE minmax GRANULARITY 4,
    INDEX idx_timestamp          (timestamp)           TYPE minmax GRANULARITY 4,

    -- indexes (event) --
    INDEX idx_program_id         (program_id)          TYPE set(8) GRANULARITY 4,
    INDEX idx_mint               (mint)                TYPE set(128) GRANULARITY 4,
    INDEX idx_owner              (owner)               TYPE bloom_filter GRANULARITY 4,
    INDEX idx_amount             (amount)              TYPE minmax GRANULARITY 4,
    INDEX idx_decimals           (decimals)            TYPE minmax GRANULARITY 4,
    INDEX idx_token_standard     (token_standard)      TYPE set(2) GRANULARITY 1
)
ENGINE = ReplacingMergeTree(block_num)
ORDER BY (owner, mint);

-- insert SPL Token balance changes --
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_balances
TO balances AS
SELECT
    -- block --
    block_num,
    block_hash,
    timestamp,

    -- event --
    program_id,
    owner,
    mint,
    amount,
    decimals

FROM balance_changes;
