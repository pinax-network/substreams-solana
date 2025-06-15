-- latest balances by owner/contract --
CREATE TABLE IF NOT EXISTS balances  (
    -- block --
    block_num            UInt32,
    block_hash           FixedString(66),
    timestamp            DateTime(0, 'UTC'),

    -- ordering --
    execution_index     UInt32, -- relative index
    global_sequence     UInt64, -- latest global sequence (block_num << 32 + execution_index)

    -- account --
    program_id          FixedString(32),

    -- event --
    owner               FixedString(44),
    mint                FixedString(44),
    amount              UInt64,
    decimals            UInt8,

    -- indexes --
    INDEX idx_block_num          (block_num)           TYPE minmax GRANULARITY 4,
    INDEX idx_timestamp          (timestamp)           TYPE minmax GRANULARITY 4,

    -- indexes (event) --
    INDEX idx_program_id         (program_id)          TYPE set(8) GRANULARITY 4,
    INDEX idx_mint               (mint)                TYPE set(128) GRANULARITY 4,
    INDEX idx_owner              (owner)               TYPE bloom_filter GRANULARITY 4,
    INDEX idx_amount             (amount)              TYPE minmax GRANULARITY 4,
    INDEX idx_decimals           (decimals)            TYPE minmax GRANULARITY 4
)
ENGINE = ReplacingMergeTree(block_num)
ORDER BY (address, contract);

-- insert SPL Token balance changes --
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_spl_token_balance_changes
TO balances AS
SELECT
    -- block --
    b.block_num AS block_num,
    b.block_hash AS block_hash,
    b.timestamp AS timestamp,

    -- ordering --
    b.execution_index AS execution_index,
    b.global_sequence AS global_sequence,

    -- event --
    b.program_id AS program_id,
    b.owner AS owner,
    b.mint AS mint,
    b.amount AS amount,
    b.decimals AS decimals

FROM spl_token_balance_changes AS b;
