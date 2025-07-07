/* ──────────────────────────────────────────────────────────────────────────
   0.  Common-fields target table
   ────────────────────────────────────────────────────────────────────────── */
CREATE TABLE IF NOT EXISTS swaps (
    -- block --
    block_num               UInt32,
    block_hash              FixedString(44),
    timestamp               DateTime(0, 'UTC'),

    -- ordering --
    transaction_index       UInt32,
    instruction_index       UInt32,

    -- transaction --
    signature               FixedString(88),
    program_id              LowCardinality(FixedString(44)),

    -- common fields --
    amm                         LowCardinality(FixedString(44)) COMMENT 'AMM protocol (Raydium Liquidity Pool V4)',
    amm_pool                    LowCardinality(FixedString(44)) COMMENT 'AMM market (Raydium "WSOL-USDT" Market)',
    input_mint                  LowCardinality(FixedString(44)) COMMENT 'Input token mint address',
    input_amount                UInt64                          COMMENT 'Amount of input tokens swapped',
    output_mint                 LowCardinality(FixedString(44)) COMMENT 'Output token mint address',
    output_amount               UInt64                          COMMENT 'Amount of output tokens received',

    -- indexes --
    INDEX idx_block_num         (block_num)         TYPE minmax         GRANULARITY 4,
    INDEX idx_timestamp         (timestamp)         TYPE minmax         GRANULARITY 4,
    INDEX idx_signature         (signature)         TYPE bloom_filter   GRANULARITY 4,
    INDEX idx_program_id        (program_id)        TYPE set(128)       GRANULARITY 4,
    INDEX idx_amm               (amm)               TYPE set(128)       GRANULARITY 4,
    INDEX idx_amm_pool          (amm_pool)          TYPE set(128)       GRANULARITY 4,
    INDEX idx_input_mint        (input_mint)        TYPE set(128)       GRANULARITY 4,
    INDEX idx_output_mint       (output_mint)       TYPE set(128)       GRANULARITY 4,
    INDEX idx_input_amount      (input_amount)      TYPE minmax         GRANULARITY 4,
    INDEX idx_output_amount     (output_amount)     TYPE minmax         GRANULARITY 4
)
ENGINE = ReplacingMergeTree
ORDER BY (program_id, amm, amm_pool, block_hash, transaction_index, instruction_index);

/* ──────────────────────────────────────────────────────────────────────────
   1.  Raydium-AMM → swaps
   ────────────────────────────────────────────────────────────────────────── */
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_raydium_amm_v4_swap_base_in
TO swaps AS
SELECT
    -- block --
    block_num,
    block_hash,
    timestamp,
    -- ordering --
    transaction_index,
    instruction_index,

    -- transaction --
    signature,
    program_id,

    -- common fields --
    program_id              AS amm,
    amm                     AS amm_pool,
    amm_coin_vault          AS input_mint,
    amount_in               AS input_amount,
    amm_pc_vault            AS output_mint,
    amount_out              AS output_amount

FROM raydium_amm_v4_swap_base_in AS s
-- ignore dust swaps (typically trying to disort the price)
WHERE s.amount_in > 1 AND s.amount_out > 1;
