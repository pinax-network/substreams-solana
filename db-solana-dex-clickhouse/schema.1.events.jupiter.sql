-- Jupiter V4 & V6 Swaps --
CREATE TABLE IF NOT EXISTS jupiter_swap AS base_events
COMMENT 'Jupiter V4 & V6 Swaps';
ALTER TABLE jupiter_swap
    -- log --
    ADD COLUMN IF NOT EXISTS amm                         FixedString(44) COMMENT 'AMM pool account (Raydium V4)',
    ADD COLUMN IF NOT EXISTS input_mint                  FixedString(44) COMMENT 'Input token mint address',
    ADD COLUMN IF NOT EXISTS input_amount                UInt64 COMMENT 'Amount of input tokens swapped',
    ADD COLUMN IF NOT EXISTS output_mint                 FixedString(44) COMMENT 'Output token mint address',
    ADD COLUMN IF NOT EXISTS output_amount               UInt64 COMMENT 'Amount of output tokens received';
