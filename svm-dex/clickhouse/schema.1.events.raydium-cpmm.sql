-- Raydium CPMM Swap --
CREATE TABLE IF NOT EXISTS raydium_cpmm_swap_base_in AS base_events
COMMENT 'Raydium CPMM Swap';
ALTER TABLE raydium_cpmm_swap_base_in
    ADD COLUMN IF NOT EXISTS payer               FixedString(44) COMMENT 'User account',
    ADD COLUMN IF NOT EXISTS pool_state          FixedString(44) COMMENT 'Pool state account',
    ADD COLUMN IF NOT EXISTS input_token_mint    FixedString(44) COMMENT 'Input token mint',
    ADD COLUMN IF NOT EXISTS output_token_mint   FixedString(44) COMMENT 'Output token mint',
    ADD COLUMN IF NOT EXISTS amount_in           UInt64 COMMENT 'Amount of tokens in',
    ADD COLUMN IF NOT EXISTS amount_out          UInt64 COMMENT 'Amount of tokens out';

CREATE TABLE IF NOT EXISTS raydium_cpmm_swap_base_out AS raydium_cpmm_swap_base_in;
