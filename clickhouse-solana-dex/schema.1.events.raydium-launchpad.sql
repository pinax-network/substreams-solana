-- Raydium Launchpad Trade Events --
CREATE TABLE IF NOT EXISTS raydium_launchpad_buy AS base_events
COMMENT 'Raydium Launchpad Buy';
ALTER TABLE raydium_launchpad_buy
    ADD COLUMN IF NOT EXISTS payer             FixedString(44) COMMENT 'User account',
    ADD COLUMN IF NOT EXISTS pool_state        FixedString(44) COMMENT 'Pool state account',
    ADD COLUMN IF NOT EXISTS base_token_mint   FixedString(44) COMMENT 'Base token mint',
    ADD COLUMN IF NOT EXISTS quote_token_mint  FixedString(44) COMMENT 'Quote token mint',
    ADD COLUMN IF NOT EXISTS amount_in         UInt64 COMMENT 'Amount of tokens in',
    ADD COLUMN IF NOT EXISTS amount_out        UInt64 COMMENT 'Amount of tokens out',
    ADD COLUMN IF NOT EXISTS exact_in          Bool COMMENT 'Whether trade is exact in';

CREATE TABLE IF NOT EXISTS raydium_launchpad_sell AS raydium_launchpad_buy;
