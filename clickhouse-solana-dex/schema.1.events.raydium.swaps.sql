-- ──────────────────────────────────────────────────────────────────────────
-- Raydium AMM Swaps  (updated to latest protobuf)
-- ──────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS raydium_amm_v4_swap_base_in (
    -- block --
    block_num                   UInt32,
    block_hash                  FixedString(44),
    timestamp                   DateTime(0, 'UTC'),

    -- ordering --
    transaction_index           UInt32,
    instruction_index           UInt32,

    -- transaction --
    signature                   FixedString(88),
    fee_payer                   FixedString(44),
    signers_raw                 String,
    signers                     MATERIALIZED arrayMap(x -> substring(x, 0, 44), splitByChar(',', signers_raw)),
    fee                         UInt64 DEFAULT 0,
    compute_units_consumed      UInt64 DEFAULT 0,

    -- instruction --
    program_id                  LowCardinality(FixedString(44)),
    stack_height                UInt32,

    -- event --
    amount_in                   UInt64,
    minimum_out                 UInt64,
    direction                   UInt64,
    user_source                 FixedString(44),
    pool_coin                   FixedString(44),
    pool_pc                     FixedString(44),
    out_amount                  UInt64,

    -- indexes --
    INDEX idx_block_num         (block_num)          TYPE minmax           GRANULARITY 4,
    INDEX idx_timestamp         (timestamp)          TYPE minmax           GRANULARITY 4,
    INDEX idx_signature         (signature)          TYPE bloom_filter     GRANULARITY 4,

    -- INDEX idx_amm               (amm)                TYPE set(128)         GRANULARITY 4,
    -- INDEX idx_user              (user)               TYPE bloom_filter         GRANULARITY 4,
    -- INDEX idx_mint_in           (mint_in)            TYPE set(128)         GRANULARITY 4,
    -- INDEX idx_mint_out          (mint_out)           TYPE set(128)         GRANULARITY 4,
    -- INDEX idx_pc_mint           (pc_mint)            TYPE set(128)         GRANULARITY 4,
    -- INDEX idx_coin_mint         (coin_mint)          TYPE set(128)         GRANULARITY 4,
    -- INDEX idx_amount_in         (amount_in)          TYPE minmax           GRANULARITY 4,
    -- INDEX idx_amount_out        (amount_out)         TYPE minmax           GRANULARITY 4,
    -- INDEX idx_amounts           (amount_in, amount_out)     TYPE minmax    GRANULARITY 4,
    -- INDEX idx_direction         (direction)          TYPE set(1)           GRANULARITY 1,
)
ENGINE = ReplacingMergeTree
ORDER BY (block_hash, transaction_index, instruction_index);
