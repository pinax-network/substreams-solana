-- Solana Swaps --
CREATE TABLE IF NOT EXISTS swaps AS base_events
COMMENT 'Solana Swaps';
ALTER TABLE swaps
    -- log --
    ADD COLUMN IF NOT EXISTS amm                         FixedString(44) COMMENT 'AMM protocol (Raydium Liquidity Pool V4)',
    ADD COLUMN IF NOT EXISTS amm_name                    String MATERIALIZED program_names(amm),
    ADD COLUMN IF NOT EXISTS amm_pool                    FixedString(44) COMMENT 'AMM market (Raydium "WSOL-USDT" Market)',
    ADD COLUMN IF NOT EXISTS user                        FixedString(44) COMMENT 'User wallet address',
    ADD COLUMN IF NOT EXISTS input_mint                  FixedString(44) COMMENT 'Input token mint address',
    ADD COLUMN IF NOT EXISTS input_amount                UInt64 COMMENT 'Amount of input tokens swapped',
    ADD COLUMN IF NOT EXISTS input_type                  String MATERIALIZED token_types(input_mint),
    ADD COLUMN IF NOT EXISTS input_name                  String MATERIALIZED token_names(input_mint),
    ADD COLUMN IF NOT EXISTS output_mint                 FixedString(44) COMMENT 'Output token mint address',
    ADD COLUMN IF NOT EXISTS output_amount               UInt64 COMMENT 'Amount of output tokens received',
    ADD COLUMN IF NOT EXISTS output_type                 String MATERIALIZED token_types(output_mint),
    ADD COLUMN IF NOT EXISTS output_name                 String MATERIALIZED token_names(output_mint),

    -- INDEX for common fields --
    ADD INDEX IF NOT EXISTS idx_amm               (amm)               TYPE set(256)               GRANULARITY 1, -- 50 unique AMMs per 2x granules when using Jupiter V6
    ADD INDEX IF NOT EXISTS idx_amm_pool          (amm_pool)          TYPE bloom_filter(0.005)    GRANULARITY 1, -- 300 unique pools per granule
    ADD INDEX IF NOT EXISTS idx_user              (user)              TYPE bloom_filter(0.005)    GRANULARITY 1, -- 2500 unique users per granule
    ADD INDEX IF NOT EXISTS idx_input_mint        (input_mint)        TYPE bloom_filter(0.005)    GRANULARITY 1, -- 500 unique mints per granule
    ADD INDEX IF NOT EXISTS idx_output_mint       (output_mint)       TYPE bloom_filter(0.005)    GRANULARITY 1, -- 500 unique mints per granule
    ADD INDEX IF NOT EXISTS idx_input_amount      (input_amount)      TYPE minmax                 GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_output_amount     (output_amount)     TYPE minmax                 GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_mint_pair         (input_mint, output_mint)    TYPE bloom_filter(0.005)    GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_mint_pair_inverse (output_mint, input_mint)    TYPE bloom_filter(0.005)    GRANULARITY 1,

    -- PROJECTION (Part) --
    -- https://clickhouse.com/docs/sql-reference/statements/alter/projection#normal-projection-with-part-offset-field
    ADD PROJECTION IF NOT EXISTS prj_part_amm (SELECT amm, timestamp, _part_offset ORDER BY (amm, timestamp)),
    ADD PROJECTION IF NOT EXISTS prj_part_amm_pool (SELECT amm_pool, timestamp, _part_offset ORDER BY (amm_pool, timestamp)),
    ADD PROJECTION IF NOT EXISTS prj_part_user (SELECT user, timestamp, _part_offset ORDER BY (user, timestamp)),
    ADD PROJECTION IF NOT EXISTS prj_part_input_mint (SELECT input_mint, timestamp, _part_offset ORDER BY (input_mint, timestamp)),
    ADD PROJECTION IF NOT EXISTS prj_part_output_mint (SELECT output_mint, timestamp, _part_offset ORDER BY (output_mint, timestamp)),
    ADD PROJECTION IF NOT EXISTS prj_part_input_amount (SELECT input_amount, timestamp, _part_offset ORDER BY (input_amount, timestamp)),
    ADD PROJECTION IF NOT EXISTS prj_part_output_amount (SELECT output_amount, timestamp, _part_offset ORDER BY (output_amount, timestamp));

/* ──────────────────────────────────────────────────────────────────────────
   1.  Raydium-AMM → swaps
   ────────────────────────────────────────────────────────────────────────── */
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_raydium_amm_v4_swap_base_in
TO swaps AS
WITH
    direction = 'PC2Coin' AS PC2Coin
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
    fee_payer,
    signers_raw,
    fee,
    compute_units_consumed,

    -- instruction --
    program_id,
    stack_height,

    -- common fields --
    user_source_owner       AS user,
    s.program_id            AS amm,
    s.amm                   AS amm_pool,

    -- must JOIN with SPL Token to get the real mint address --
    -- vaults & amounts mapped by direction --
    if (PC2Coin, amm_pc_vault,  amm_coin_vault)  AS input_mint,
    amount_in               AS input_amount,

    if (PC2Coin, amm_coin_vault, amm_pc_vault)   AS output_mint,
    amount_out              AS output_amount

FROM raydium_amm_v4_swap_base_in AS s
-- ignore dust swaps (typically trying to disort the price)
WHERE input_amount > 1 AND output_amount > 1;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_raydium_amm_v4_swap_base_out
TO swaps AS
WITH
    direction = 'PC2Coin' AS PC2Coin
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
    fee_payer,
    signers_raw,
    fee,
    compute_units_consumed,

    -- instruction --
    program_id,
    stack_height,

    -- common fields --
    user_source_owner       AS user,
    s.program_id            AS amm,
    s.amm                   AS amm_pool,

    -- must JOIN with SPL Token to get the real mint address --
    -- vaults & amounts mapped by direction --
    if (PC2Coin, amm_pc_vault,  amm_coin_vault)  AS input_mint,
    amount_in                                   AS input_amount,

    if (PC2Coin, amm_coin_vault, amm_pc_vault)   AS output_mint,
    amount_out                                  AS output_amount

FROM raydium_amm_v4_swap_base_out AS s
-- ignore dust swaps (typically trying to disort the price)
WHERE input_amount > 1 AND output_amount > 1;

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
    fee_payer,
    signers_raw,
    fee,
    compute_units_consumed,

    -- instruction --
    program_id,
    stack_height,

    -- common fields --
    s.fee_payer             AS user, -- Jupiter does not use user wallets, so we use fee_payer as a placeholder
    amm,
    ''                      AS amm_pool, -- Jupiter does not use AMM pools, so we leave it empty
    input_mint,
    input_amount,
    output_mint,
    output_amount

FROM jupiter_swap AS s
-- ignore dust swaps (typically trying to disort the price)
WHERE input_amount > 1 AND output_amount > 1;

/* ──────────────────────────────────────────────────────────────────────────
   1.  Pump.fun → swaps
   ────────────────────────────────────────────────────────────────────────── */
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_pumpfun_buy
TO swaps AS
WITH
    sol_amount + protocol_fee + creator_fee AS input_amount,
    'So11111111111111111111111111111111111111111' AS input_mint
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
    fee_payer,
    signers_raw,
    fee,
    compute_units_consumed,

    -- instruction --
    program_id,
    stack_height,

    -- common fields --
    user,
    program_id          AS amm,
    bonding_curve       AS amm_pool,
    input_mint,
    input_amount,
    mint                AS output_mint,
    token_amount        AS output_amount

FROM pumpfun_buy AS s
-- ignore dust swaps (typically trying to disort the price)
WHERE input_amount > 1 AND output_amount > 1;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_pumpfun_sell
TO swaps
AS SELECT
    -- block --
    block_num,
    block_hash,
    timestamp,

    -- ordering --
    transaction_index,
    instruction_index,

    -- transaction --
    signature,
    fee_payer,
    signers_raw,
    fee,
    compute_units_consumed,

    -- instruction --
    program_id,
    stack_height,

    -- common fields --
    user,
    program_id AS amm,
    bonding_curve AS amm_pool,
    mint AS input_mint,
    token_amount AS input_amount,
    'So11111111111111111111111111111111111111111' AS output_mint,
    (sol_amount + protocol_fee + creator_fee) AS output_amount

FROM pumpfun_sell AS s
-- ignore dust swaps (typically trying to disort the price)
WHERE input_amount > 1 AND output_amount > 1;

/* ──────────────────────────────────────────────────────────────────────────
   1.  Pump.fun AMM → swaps
   ────────────────────────────────────────────────────────────────────────── */
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_pumpfun_amm_buy
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
    fee_payer,
    signers_raw,
    fee,
    compute_units_consumed,

    -- instruction --
    program_id,
    stack_height,

    -- common fields --
    user,
    program_id          AS amm,
    pool                AS amm_pool,
    quote_mint          AS input_mint,
    quote_amount_in     AS input_amount,
    base_mint           AS output_mint,
    base_amount_out     AS output_amount

FROM pumpfun_amm_buy AS s
-- ignore dust swaps (typically trying to disort the price)
WHERE input_amount > 1 AND output_amount > 1;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_pumpfun_amm_sell
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
    fee_payer,
    signers_raw,
    fee,
    compute_units_consumed,

    -- instruction --
    program_id,
    stack_height,

    -- common fields --
    user,
    s.program_id        AS amm,
    pool                AS amm_pool,
    base_mint           AS input_mint,
    base_amount_in      AS input_amount,
    quote_mint          AS output_mint,
    quote_amount_out    AS output_amount

FROM pumpfun_amm_sell AS s
-- ignore dust swaps (typically trying to disort the price)
WHERE input_amount > 1 AND output_amount > 1;
