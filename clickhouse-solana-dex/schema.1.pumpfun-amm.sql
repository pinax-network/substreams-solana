-- ──────────────────────────────────────────────────────────────────────────
-- Pump.fun AMM Swap
-- ──────────────────────────────────────────────────────────────────────────
-- Buy --
CREATE TABLE IF NOT EXISTS pumpfun_amm_buy (
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
    signers                     Array(FixedString(44)) MATERIALIZED arrayMap(x -> toFixedString(x, 44), splitByChar(',', signers_raw)),
    fee                         UInt64 DEFAULT 0,
    compute_units_consumed      UInt64 DEFAULT 0,

    -- instruction --
    program_id                  LowCardinality(FixedString(44)),
    stack_height                UInt32,

    -- data --
    base_amount_out             UInt64,
    max_quote_amount_in         UInt64,

    -- accounts --
    pool                                    FixedString(44),
    user                                    FixedString(44),
    global_config                           FixedString(44),
    base_mint                               FixedString(44),
    quote_mint                              FixedString(44),
    user_base_token_account                 FixedString(44),
    user_quote_token_account                FixedString(44),
    pool_base_token_account                 FixedString(44),
    pool_quote_token_account                FixedString(44),
    protocol_fee_recipient                  FixedString(44),
    protocol_fee_recipient_token_account    FixedString(44),
    coin_creator_vault_ata                  FixedString(44) DEFAULT '',
    coin_creator_vault_authority            FixedString(44) DEFAULT '',

    -- event --
    quote_amount_in             UInt64,
    quote_amount_in_with_lp_fee UInt64,
    user_quote_amount_in        UInt64,

    -- indexes --
    INDEX idx_block_num         (block_num)          TYPE minmax           GRANULARITY 4,
    INDEX idx_timestamp         (timestamp)          TYPE minmax           GRANULARITY 4,
    INDEX idx_signature         (signature)          TYPE bloom_filter     GRANULARITY 4
)
ENGINE = ReplacingMergeTree
ORDER BY (block_hash, transaction_index, instruction_index);

-- Sell --
CREATE TABLE IF NOT EXISTS pumpfun_amm_sell AS pumpfun_amm_buy;
ALTER TABLE pumpfun_amm_sell RENAME COLUMN IF EXISTS base_amount_out TO base_amount_in;
ALTER TABLE pumpfun_amm_sell RENAME COLUMN IF EXISTS max_quote_amount_in TO min_quote_amount_out;
ALTER TABLE pumpfun_amm_sell RENAME COLUMN IF EXISTS quote_amount_in TO quote_amount_out;
ALTER TABLE pumpfun_amm_sell RENAME COLUMN IF EXISTS quote_amount_in_with_lp_fee TO quote_amount_out_without_lp_fee;
ALTER TABLE pumpfun_amm_sell RENAME COLUMN IF EXISTS user_quote_amount_in TO user_quote_amount_out;
