-- Meteora DLLM Swap --
CREATE TABLE IF NOT EXISTS meteora_dllm_swap AS base_events
COMMENT 'Meteora DLLM Swap';
ALTER TABLE meteora_dllm_swap
    ADD COLUMN IF NOT EXISTS user        FixedString(44) COMMENT 'User account',
    ADD COLUMN IF NOT EXISTS lb_pair     FixedString(44) COMMENT 'Liquidity pair',
    ADD COLUMN IF NOT EXISTS input_mint  FixedString(44) COMMENT 'Input token mint',
    ADD COLUMN IF NOT EXISTS output_mint FixedString(44) COMMENT 'Output token mint',
    ADD COLUMN IF NOT EXISTS amount_in   UInt64 COMMENT 'Amount of tokens in',
    ADD COLUMN IF NOT EXISTS amount_out  UInt64 COMMENT 'Amount of tokens out';
