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
    amm                     LowCardinality(FixedString(44)),    -- EVM aka `pool`
    user                    FixedString(44),                    -- EVM aka `sender`
    input_mint              FixedString(44),                    -- EVM aka `token0`
    input_amount            UInt64,                             -- EVM aka `amount0`
    output_mint             FixedString(44),                    -- EVM aka `token1`
    output_amount           UInt64,                             -- EVM aka `amount1`
    price                   Float64,
    protocol                LowCardinality(String), -- 'raydium-amm' | 'pump.fun'

    -- indexes --
    INDEX idx_signature     (signature)                 TYPE bloom_filter   GRANULARITY 4,
    INDEX idx_user          (user)                      TYPE bloom_filter   GRANULARITY 4,
    INDEX idx_amm           (amm)                       TYPE set(128)       GRANULARITY 4,
    INDEX idx_mints         (mint_in, mint_out)         TYPE set(128)       GRANULARITY 4,
    INDEX idx_amounts       (amount_in, amount_out)     TYPE minmax         GRANULARITY 4
)
ENGINE = ReplacingMergeTree
ORDER BY (timestamp, block_num, execution_index, block_hash, protocol);

/* ──────────────────────────────────────────────────────────────────────────
   1.  Raydium-AMM → swaps
   ────────────────────────────────────────────────────────────────────────── */
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_raydium_amm_v4_swap
TO swaps AS
SELECT
    /* passthrough */
    block_num,
    block_hash,
    timestamp,
    execution_index,
    transaction_index,
    instruction_index,
    global_sequence,
    tx_hash,
    program_id,

    /* mapping */
    amm,
    user,
    mint_in,
    mint_out,
    amount_in,
    amount_out,
    toFloat64(amount_in) / amount_out AS price
FROM raydium_amm_v4_swap;

-- /* ──────────────────────────────────────────────────────────────────────────
--    2.  Pump.fun → swaps
--    ────────────────────────────────────────────────────────────────────────── */
-- CREATE MATERIALIZED VIEW IF NOT EXISTS mv_pumpfun_swap
-- TO swaps AS
-- SELECT
--     /* passthrough */
--     block_num,
--     block_hash,
--     timestamp,
--     execution_index,
--     transaction_index,
--     instruction_index,
--     global_sequence,
--     tx_hash,
--     program_id,

--     /* mapping */
--     bonding_curve AS pool,
--     user as sender,
--     mint    AS mint_in,          -- single-mint pumpfun swap
--     ''      AS mint_out,
--     sol_amount   AS amount_in,
--     token_amount AS amount_out,
--     toFloat64(sol_amount) / token_amount AS price
-- FROM pumpfun_swap;