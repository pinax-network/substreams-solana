-- Meteora AMM Swap --
CREATE TABLE IF NOT EXISTS meteora_amm_swap AS base_events
COMMENT 'Meteora AMM Swap';
ALTER TABLE meteora_amm_swap
    ADD COLUMN IF NOT EXISTS user        FixedString(44) COMMENT 'User account',
    ADD COLUMN IF NOT EXISTS pool        FixedString(44) COMMENT 'Pool account',
    ADD COLUMN IF NOT EXISTS input_mint  FixedString(44) COMMENT 'Input token account',
    ADD COLUMN IF NOT EXISTS output_mint FixedString(44) COMMENT 'Output token account',
    ADD COLUMN IF NOT EXISTS amount_in   UInt64 COMMENT 'Amount of tokens in',
    ADD COLUMN IF NOT EXISTS amount_out  UInt64 COMMENT 'Amount of tokens out';
