-- Orca Swap --
CREATE TABLE IF NOT EXISTS orca_swap AS base_events
COMMENT 'Orca Whirlpool Swap';
ALTER TABLE orca_swap
    ADD COLUMN IF NOT EXISTS user         FixedString(44) COMMENT 'User (token authority)',
    ADD COLUMN IF NOT EXISTS whirlpool    FixedString(44) COMMENT 'Whirlpool account',
    ADD COLUMN IF NOT EXISTS input_mint   FixedString(44) COMMENT 'Input token mint',
    ADD COLUMN IF NOT EXISTS output_mint  FixedString(44) COMMENT 'Output token mint',
    ADD COLUMN IF NOT EXISTS amount_in    UInt64 COMMENT 'Amount of tokens in',
    ADD COLUMN IF NOT EXISTS amount_out   UInt64 COMMENT 'Amount of tokens out';
