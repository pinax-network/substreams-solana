/* ──────────────────────────────────────────────────────────────────────────
   0.  Common-fields target table
   ────────────────────────────────────────────────────────────────────────── */
CREATE TABLE IF NOT EXISTS swaps (
    -- block --
    block_num               UInt32,
    block_hash              FixedString(44),
    timestamp               DateTime(0, 'UTC'),

    -- ordering --
    execution_index         UInt32,
    transaction_index       UInt32,
    instruction_index       UInt32,
    global_sequence         UInt64, -- (block_num << 32) | execution_index

    -- transaction --
    signature               FixedString(88),                    -- EVM aka `tx_hash`
    program_id              LowCardinality(FixedString(44)),    -- EVM aka `contract`

    -- common fields --
    pool                    LowCardinality(FixedString(44)),    -- Solana aka `amm`
    sender                  FixedString(44),                    -- Solana aka `user`
    token0                  FixedString(44),                    -- Solana aka `input_mint`
    amount0                 Int128,                             -- Solana aka `input_amount`
    token1                  FixedString(44),                    -- Solana aka `output_mint`
    amount1                 Int128,                             -- Solana aka `output_amount`
    price                   Float64,
    protocol                LowCardinality(String), -- 'raydium_amm_v4' | 'pumpfun'

    -- indexes --
    INDEX idx_signature         (signature)         TYPE bloom_filter   GRANULARITY 4,
    INDEX idx_pool              (pool)              TYPE set(128)       GRANULARITY 4,
    INDEX idx_sender            (sender)            TYPE bloom_filter   GRANULARITY 4,
    INDEX idx_token0            (token0)            TYPE set(128)       GRANULARITY 4,
    INDEX idx_amount0           (amount0)           TYPE minmax         GRANULARITY 4,
    INDEX idx_token1            (token1)            TYPE set(128)       GRANULARITY 4,
    INDEX idx_amount1           (amount1)           TYPE minmax         GRANULARITY 4,
    INDEX idx_price             (price)             TYPE minmax         GRANULARITY 4,
    INDEX idx_protocol          (protocol)          TYPE set(4)         GRANULARITY 1
)
ENGINE = ReplacingMergeTree
ORDER BY (timestamp, block_num, execution_index, block_hash, protocol);
