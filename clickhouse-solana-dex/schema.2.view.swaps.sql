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
    protocol                LowCardinality(String), -- 'raydium_amm_v4' | 'pumpfun'

    -- indexes --
    INDEX idx_signature     (signature)                         TYPE bloom_filter   GRANULARITY 4,
    INDEX idx_user          (user)                              TYPE bloom_filter   GRANULARITY 4,
    INDEX idx_amm           (amm)                               TYPE set(128)       GRANULARITY 4,
    INDEX idx_input_mint    (input_mint)                        TYPE set(128)       GRANULARITY 4,
    INDEX idx_input_amount  (input_amount)                      TYPE minmax         GRANULARITY 4,
    INDEX idx_output_mint   (output_mint)                       TYPE set(128)       GRANULARITY 4,
    INDEX idx_output_amount (output_amount)                     TYPE minmax         GRANULARITY 4,
    INDEX idx_mints         (input_mint, output_mint)           TYPE set(128)       GRANULARITY 4,
    INDEX idx_amounts       (input_amount, output_amount)       TYPE minmax         GRANULARITY 4
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
    signature,
    program_id,

    /* mapping */
    amm,
    user,
    mint_in as input_mint,
    mint_out as output_mint,
    amount_in as input_amount,
    amount_out as output_amount,
    toFloat64(amount_in) / amount_out AS price,
    'raydium_amm_v4' AS protocol
FROM raydium_amm_v4_swap;
