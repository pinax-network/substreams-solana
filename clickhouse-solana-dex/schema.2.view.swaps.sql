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
            WHEN CAST ('pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA' AS FixedString(44)) THEN 'Pump.fun AMM'
            WHEN CAST ('JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB' AS FixedString(44)) THEN 'Jupiter Aggregator v4'
            WHEN CAST ('JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4' AS FixedString(44)) THEN 'Jupiter Aggregator v6'
            ELSE 'Unknown'
        END,

    -- common fields --
    user                        FixedString(44)                 COMMENT 'User wallet address',
    amm                         LowCardinality(FixedString(44)) COMMENT 'AMM protocol (Raydium Liquidity Pool V4)',
    amm_name                    LowCardinality(String) MATERIALIZED
        CASE amm
            WHEN CAST ('675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8' AS FixedString(44)) THEN 'Raydium Liquidity Pool V4'
            WHEN CAST ('6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P' AS FixedString(44)) THEN 'Pump.fun'
            WHEN CAST ('pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA' AS FixedString(44)) THEN 'Pump.fun AMM'
            WHEN CAST ('JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB' AS FixedString(44)) THEN 'Jupiter Aggregator v4'
            WHEN CAST ('JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4' AS FixedString(44)) THEN 'Jupiter Aggregator v6'

            -- Jupiter V4 & V6 --
            WHEN CAST ('dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN' AS FixedString(44)) THEN 'Meteora Dynamic Bonding Curve Program'
            WHEN CAST ('whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc' AS FixedString(44)) THEN 'Whirlpools Program'
            WHEN CAST ('LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo' AS FixedString(44)) THEN 'Meteora DLMM Program'
            WHEN CAST ('SoLFiHG9TfgtdUXUjWAxi3LtvYuFyDLVhBWxdMZxyCe' AS FixedString(44)) THEN 'SolFi'
            WHEN CAST ('CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK' AS FixedString(44)) THEN 'Raydium Concentrated Liquidity'
            WHEN CAST ('2wT8Yq49kHgDzXuPxZSaeLaH1qbmGXtEyPy64bL7aD3c' AS FixedString(44)) THEN 'Lifinity Swap V2'
            WHEN CAST ('cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG' AS FixedString(44)) THEN 'Meteora DAMM v2'
            WHEN CAST ('obriQD1zbpyLz95G5n7nJe6a4DPjpFwa5XYPoNm113y' AS FixedString(44)) THEN 'Obric V2'
            WHEN CAST ('ZERor4xhbUycZ6gb9ntrhqscUcZmAbQDjEAtCf4hbZY' AS FixedString(44)) THEN 'ZeroFi'
            WHEN CAST ('swapNyd8XiQwJ6ianp9snpu4brUqFxadzvHebnAXjJZ' AS FixedString(44)) THEN 'stabble Stable Swap'
            WHEN CAST ('opnb2LAfJYbRMAHHvqjCwQxanZn7ReEHp1k81EohpZb' AS FixedString(44)) THEN 'Openbook V2'
            WHEN CAST ('CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C' AS FixedString(44)) THEN 'Raydium CPMM'
            WHEN CAST ('goonERTdGsjnkZqWuVjs73BZ3Pb9qoCUdBUL17BnS5j' AS FixedString(44)) THEN 'GoonFi'
            WHEN CAST ('Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB' AS FixedString(44)) THEN 'Meteora Pools Program'
            WHEN CAST ('DEXYosS6oEGvk8uCDayvwEZz4qEyDJRf9nFgYCaqPMTm' AS FixedString(44)) THEN '1Dex Program'
            WHEN CAST ('H8W3ctz92svYg6mkn1UtGfu2aQr2fnUFHM1RhScEtQDt' AS FixedString(44)) THEN 'Cropper Whirlpool'
            WHEN CAST ('GAMMA7meSFWaBXF25oSUgmGRwaW6sCMFLmBNiMSdbHVT' AS FixedString(44)) THEN 'GooseFX: GAMMA'
            WHEN CAST ('NUMERUNsFCP3kuNmWZuXtm1AaQCPj9uw6Guv2Ekoi5P' AS FixedString(44)) THEN 'Numeraire'
            WHEN CAST ('SSwpkEEcbUqx4vtoEByFjSkhKdCT862DNVb52nZg1UZ' AS FixedString(44)) THEN 'Saber Stable Swap'
            WHEN CAST ('swapFpHZwjELNnjvThjajtiVmkz3yPQEHjLtka2fwHW' AS FixedString(44)) THEN 'stabble Weighted Swap'
            WHEN CAST ('HyaB3W9q6XdA5xwpU4XnSZV94htfmbmqJXZcEbRaJutt' AS FixedString(44)) THEN 'Invariant Swap'
            WHEN CAST ('PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY' AS FixedString(44)) THEN 'Phoenix'
            WHEN CAST ('LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj' AS FixedString(44)) THEN 'Raydium Launchpad'
            WHEN CAST ('SSwapUtytfBdBn1b9NUGG6foMVPtcWgpRU32HToDUZr' AS FixedString(44)) THEN 'Saros AMM'
            WHEN CAST ('PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu' AS FixedString(44)) THEN 'Jupiter Perpetuals'
            WHEN CAST ('5ocnV1qiCgaQR8Jb8xWnVbApfaygJ8tNoZfgPwsgx9kx' AS FixedString(44)) THEN 'Sanctum Program'
            WHEN CAST ('9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP' AS FixedString(44)) THEN 'Orca Token Swap V2'
            WHEN CAST ('Gswppe6ERWKpUTXvRPfXdzHhiCyJvLadVvXGfdpBqcE1' AS FixedString(44)) THEN 'Guac Swap'
            WHEN CAST ('BSwp6bEBihVLdqJRKGgzjcGLHkcTuzmSo1TQkHepzH8p' AS FixedString(44)) THEN 'BonkSwap'
            WHEN CAST ('MoonCVVNZFSYkqNXP6bxHLPL6QQJiMagDL3qcqUQTrG' AS FixedString(44)) THEN 'Moonit'
            WHEN CAST ('DecZY86MU5Gj7kppfUCEmd4LbXXuyZH1yHaP2NTqdiZB' AS FixedString(44)) THEN 'Saber Decimal Wrapper'
            WHEN CAST ('SwaPpA9LAaLfeLi3a68M4DjnLqgtticKg6CnyNwgAC8' AS FixedString(44)) THEN 'Swap Program'
            WHEN CAST ('stkitrT1Uoy18Dk1fTrgPw8W6MVzoCfYoAFT4MLsmhq' AS FixedString(44)) THEN 'Sanctum Router Program'
            WHEN CAST ('FLUXubRmkEi2q6K3Y9kBPg9248ggaZVsoSFhtJHSrm1X' AS FixedString(44)) THEN 'Fluxbeam Program'
            WHEN CAST ('MERLuDFBMmsHnsBPZw2sDQZHvXFMwp8EdjudcU2HKky' AS FixedString(44)) THEN 'Mercurial Stable Swap'
            WHEN CAST ('srAMMzfVHVAtgSJc8iH6CfKzuWuUTzLHVCE81QU1rgi' AS FixedString(44)) THEN 'Gavel'
            WHEN CAST ('SSwpMgqNDsyV7mAgN9ady4bDVu5ySjmmXejXvy2vLt1' AS FixedString(44)) THEN 'Step Finance Swap Program'
            WHEN CAST ('DjVE6JNiYqPL2QXyCUUh8rNjHrbz9hXHNYt99MQ59qw1' AS FixedString(44)) THEN 'Orca Token Swap'
            WHEN CAST ('Dooar9JkhdZ7J3LHN3A7YCuoGRUggXhQaG4kijfLGU2j' AS FixedString(44)) THEN 'StepN DOOAR Swap'
            WHEN CAST ('CURVGoZn8zycx6FXwwevgBTB2gVvdbGTEpvMJDbgs2t4' AS FixedString(44)) THEN 'Aldrin AMM V2'
            WHEN CAST ('CTMAxxk34HjKWxQ3QLZK1HpaLXmBveao3ESePXbiyfzh' AS FixedString(44)) THEN 'Cropper Finance'
            WHEN CAST ('SCHAtsf8mbjyjiv4LkhLKutTf6JnZAbdJKFkXQNMFHZ' AS FixedString(44)) THEN 'Sencha Cpamm'
            WHEN CAST ('treaf4wWBBty3fHdyBpo35Mz84M8k3heKXmjmi9vFt5' AS FixedString(44)) THEN 'Helium Treasury Management'
            WHEN CAST ('9tKE7Mbmj4mxDjWatikzGAtkoWosiiZX9y6J4Hfm2R8H' AS FixedString(44)) THEN 'Oasis'
            WHEN CAST ('DSwpgjMvXhtGn6BsbqmacdBZyfLj6jSWf3HJpdJtmg6N' AS FixedString(44)) THEN 'Dexlab Swap'
            WHEN CAST ('PSwapMdSai8tjrEXcxFeQth87xC4rRsa4VA5mhGhXkP' AS FixedString(44)) THEN 'Penguin Finance'
            WHEN CAST ('AMM55ShdkoGRB5jVYPjWziwk8m5MpwyDgsMWHaMSQWH6' AS FixedString(44)) THEN 'Aldrin AMM'
            WHEN CAST ('WooFif76YGRNjk1pA8wCsN67aQsD9f9iLsz4NcJ1AVb' AS FixedString(44)) THEN 'WOOFi'
            WHEN CAST ('CLMM9tUoggJu2wagPkkqs9eFG4BWhVBZWkP1qv3Sp7tR' AS FixedString(44)) THEN 'Crema Finance Program'
            ELSE 'Unknown'
        END,
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
ORDER BY (program_id, timestamp, block_num, block_hash, transaction_index, instruction_index);

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
    s.program_id            AS amm,
    s.amm                   AS amm_pool,

    -- must JOIN with SPL Token to get the real mint address --
    -- vaults & amounts mapped by direction --
    if (direction = 'PC2Coin', amm_pc_vault,  amm_coin_vault)  AS input_mint,
    amount_in               AS input_amount,

    if (direction = 'PC2Coin', amm_coin_vault, amm_pc_vault)   AS output_mint,
    amount_out              AS output_amount

FROM raydium_amm_v4_swap_base_in AS s
-- ignore dust swaps (typically trying to disort the price)
WHERE input_amount > 1 AND output_amount > 1;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_raydium_amm_v4_swap_base_out
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
    s.program_id            AS amm,
    s.amm                   AS amm_pool,

    -- must JOIN with SPL Token to get the real mint address --
    -- vaults & amounts mapped by direction --
    if (direction = 'PC2Coin', amm_pc_vault,  amm_coin_vault)  AS input_mint,
    amount_in                                   AS input_amount,

    if (direction = 'PC2Coin', amm_coin_vault, amm_pc_vault)   AS output_mint,
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
    program_id,

    -- common fields --
    fee_payer               AS user, -- Jupiter does not use user wallets, so we use fee_payer as a placeholder
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
WITH (
    sol_amount + protocol_fee + creator_fee AS input_amount,
    'So11111111111111111111111111111111111111111' AS input_mint
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
TO swaps AS
WITH (
    sol_amount + protocol_fee + creator_fee AS output_amount,
    'So11111111111111111111111111111111111111111' AS output_mint
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
    user,
    program_id          AS amm,
    bonding_curve       AS amm_pool,
    mint                AS input_mint,
    token_amount        AS input_amount,
    output_mint,
    output_amount

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
    program_id,

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
    program_id,

    -- common fields --
    user,
    program_id          AS amm,
    pool                AS amm_pool,
    base_mint           AS input_mint,
    base_amount_in      AS input_amount,
    quote_mint          AS output_mint,
    quote_amount_out    AS output_amount

FROM pumpfun_amm_sell AS s
-- ignore dust swaps (typically trying to disort the price)
WHERE input_amount > 1 AND output_amount > 1;