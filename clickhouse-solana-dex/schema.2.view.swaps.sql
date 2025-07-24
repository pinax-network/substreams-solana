/* ──────────────────────────────────────────────────────────────────────────
   0.  Common-fields target table
   ────────────────────────────────────────────────────────────────────────── */
CREATE TABLE IF NOT EXISTS swaps (
    -- block --
    block_num                   UInt32,
    block_hash                  FixedString(44),
    timestamp                   UInt32,
    datetime                    DateTime('UTC', 0) MATERIALIZED toDateTime(timestamp, 'UTC'),

    -- ordering --
    transaction_index           UInt32,
    instruction_index           UInt32,

    -- transaction --
    signature                   FixedString(88),
    fee_payer                   FixedString(44),
    signers_raw                 String,
    signers                     Array(FixedString(44)) MATERIALIZED arrayMap(x -> toFixedString(x, 44), splitByChar(',', signers_raw)),
    signer                      FixedString(44) MATERIALIZED if(length(signers) > 0, signers[1], ''),
    fee                         UInt64 DEFAULT 0,
    compute_units_consumed      UInt64 DEFAULT 0,

    -- instruction --
    program_id                  LowCardinality(FixedString(44)),
    program_name                LowCardinality(String) MATERIALIZED program_names(program_id),
    stack_height                UInt32,

    -- common fields --
    user                        FixedString(44)                 COMMENT 'User wallet address',
    amm                         LowCardinality(FixedString(44)) COMMENT 'AMM protocol (Raydium Liquidity Pool V4)',
    amm_name                    LowCardinality(String) MATERIALIZED program_names(amm),
    amm_pool                    LowCardinality(FixedString(44)) COMMENT 'AMM market (Raydium "WSOL-USDT" Market)',
    input_mint                  LowCardinality(FixedString(44)) COMMENT 'Input token mint address',
    input_amount                UInt64                          COMMENT 'Amount of input tokens swapped',
    output_mint                 LowCardinality(FixedString(44)) COMMENT 'Output token mint address',
    output_amount               UInt64                          COMMENT 'Amount of output tokens received',

    -- indexes -
    INDEX idx_signature         (signature)         TYPE bloom_filter   GRANULARITY 4,  -- always unique
    INDEX idx_fee_payer         (fee_payer)         TYPE set(4096)      GRANULARITY 1,
    INDEX idx_signer            (signer)            TYPE set(4096)      GRANULARITY 1,
    INDEX idx_block_num         (block_num)         TYPE minmax         GRANULARITY 1,
    INDEX idx_timestamp         (timestamp)         TYPE minmax         GRANULARITY 1,
    INDEX idx_program_id        (program_id)        TYPE set(8)         GRANULARITY 1, -- 5 unique programs per granule
    INDEX idx_program_name      (program_name)      TYPE set(8)         GRANULARITY 1,

    -- indexes for common fields --
    INDEX idx_user              (user)              TYPE set(4096)      GRANULARITY 1, -- 2500 unique users per granule
    INDEX idx_amm               (amm)               TYPE set(64)        GRANULARITY 1, -- 50 unique AMMs per 2x granules when using Jupiter V6
    INDEX idx_amm_name          (amm_name)          TYPE set(64)        GRANULARITY 1,
    INDEX idx_amm_pool          (amm_pool)          TYPE set(256)       GRANULARITY 1, -- 300 unique pools per granule
    INDEX idx_input_mint        (input_mint)        TYPE set(512)       GRANULARITY 1, -- 500 unique mints per granule
    INDEX idx_output_mint       (output_mint)       TYPE set(512)       GRANULARITY 1, -- 500 unique mints per granule
    INDEX idx_input_amount      (input_amount)      TYPE minmax         GRANULARITY 1,
    INDEX idx_output_amount     (output_amount)     TYPE minmax         GRANULARITY 1
)
ENGINE = MergeTree
-- Optimized for swaps by AMM DEXs ordered by latest/oldest timestamp
ORDER BY (
    program_id, amm, amm_pool, timestamp, block_num,
    block_hash, transaction_index, instruction_index
)
COMMENT 'Swaps, used by all AMMs and DEXs';

-- PROJECTIONS (Full) --
-- all the data from the original table will be duplicated
ALTER TABLE swaps ADD PROJECTION prj_timestamp (SELECT * ORDER BY timestamp);

-- PROJECTIONS (Part) --
-- https://clickhouse.com/docs/sql-reference/statements/alter/projection#normal-projection-with-part-offset-field
ALTER TABLE swaps ADD PROJECTION prj_part_program_id  (SELECT program_id,  _part_offset ORDER BY program_id);
ALTER TABLE swaps ADD PROJECTION prj_part_amm         (SELECT amm,         _part_offset ORDER BY amm);
ALTER TABLE swaps ADD PROJECTION prj_part_amm_pool    (SELECT amm_pool,    _part_offset ORDER BY amm_pool);
ALTER TABLE swaps ADD PROJECTION prj_part_signature   (SELECT signature,   _part_offset ORDER BY signature);
ALTER TABLE swaps ADD PROJECTION prj_part_fee_payer   (SELECT fee_payer,   _part_offset ORDER BY fee_payer);
ALTER TABLE swaps ADD PROJECTION prj_part_signer      (SELECT signer,      _part_offset ORDER BY signer);
ALTER TABLE swaps ADD PROJECTION prj_part_user        (SELECT user,        _part_offset ORDER BY user);
ALTER TABLE swaps ADD PROJECTION prj_part_input_mint  (SELECT input_mint,  _part_offset ORDER BY input_mint);
ALTER TABLE swaps ADD PROJECTION prj_part_output_mint (SELECT output_mint, _part_offset ORDER BY output_mint);

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
