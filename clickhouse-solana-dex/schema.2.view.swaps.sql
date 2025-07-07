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
    program_name            LowCardinality(String) MATERIALIZED
        CASE program_id
            WHEN CAST ('675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8' AS FixedString(44)) THEN 'Raydium Liquidity Pool V4'
            WHEN CAST ('6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P' AS FixedString(44)) THEN 'Pump.fun'
            WHEN CAST ('JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB' AS FixedString(44)) THEN 'Jupiter Aggregator v4'
            WHEN CAST ('JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4' AS FixedString(44)) THEN 'Jupiter Aggregator v6'
            ELSE 'Unknown'
        END,

    -- common fields --
    user                        FixedString(44)                 COMMENT 'User wallet address',
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
    INDEX idx_user              (user)              TYPE bloom_filter   GRANULARITY 4,
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
    user_source_owner       AS user,
    program_id              AS amm,
    amm                     AS amm_pool,
    amm_coin_vault          AS input_mint,
    amount_in               AS input_amount,
    amm_pc_vault            AS output_mint,
    amount_out              AS output_amount

FROM raydium_amm_v4_swap_base_in AS s
-- ignore dust swaps (typically trying to disort the price)
WHERE s.amount_in > 1 AND s.amount_out > 1;

/* ──────────────────────────────────────────────────────────────────────────
   1.  Jupiter → swaps
   ────────────────────────────────────────────────────────────────────────── */
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_jupiter_swap
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
    fee_payer               AS user, -- Jupiter does not use user wallets, so we use fee_payer as a placeholder
    amm                     AS amm,
    ''                      AS amm_pool, -- Jupiter does not use AMM pools, so we leave it empty
    input_mint,
    input_amount,
    output_mint,
    output_amount

FROM jupiter_swap AS s
-- ignore dust swaps (typically trying to disort the price)
WHERE s.input_amount > 1 AND s.output_amount > 1;

/* ──────────────────────────────────────────────────────────────────────────
   1.  Pump.fun → swaps
   ────────────────────────────────────────────────────────────────────────── */
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_pumpfun_buy
TO swaps AS
WITH (
    sol_amount + protocol_fee + creator_fee AS input_amount,
    'So11111111111111111111111111111111111111111' AS input_mint,
)
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
    user                AS user,
    program_id          AS amm,
    bonding_curve       AS amm_pool,
    input_mint,
    input_amount,
    mint                AS output_mint,
    token_amount        AS output_amount

FROM pumpfun_buy AS s
-- ignore dust swaps (typically trying to disort the price)
WHERE s.sol_amount > 1 AND s.token_amount > 1;