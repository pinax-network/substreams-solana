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

/* ──────────────────────────────────────────────────────────────────────────
   1.  Raydium-AMM → swaps  (canonical ordering)
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

    /* mapping – canonicalised */
    amm                                               AS pool,
    user                                              AS sender,

    /* canonical token addresses */
    if(mint_in < mint_out, mint_in,  mint_out)        AS token0,
    if(mint_in < mint_out, mint_out, mint_in)         AS token1,

    /* amounts follow the same ordering */
    if(mint_in < mint_out, s.amount_in, -toInt128(s.amount_out))    AS amount0,
    if(mint_in < mint_out, -toInt128(s.amount_out), s.amount_in)     AS amount1,

    /* price must be inverted when the tokens were swapped */
    if (mint_in < mint_out,
       toFloat64(s.amount_in) / s.amount_out,             -- original direction
       toFloat64(s.amount_out) / s.amount_in              -- inverted direction
    )                                                 AS price,

    /* constant */
    'raydium_amm_v4'                                  AS protocol
FROM raydium_amm_v4_swap AS s
-- ignore dust swaps (typically trying to disort the price)
WHERE s.amount_in > 1 AND s.amount_out > 1;
