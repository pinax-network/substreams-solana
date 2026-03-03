-- Lifinity Swap --
CREATE TABLE IF NOT EXISTS lifinity_swap AS base_events
COMMENT 'Lifinity Swap';
ALTER TABLE lifinity_swap
    ADD COLUMN IF NOT EXISTS user               FixedString(44) COMMENT 'User transfer authority',
    ADD COLUMN IF NOT EXISTS amm                FixedString(44) COMMENT 'AMM account',
    ADD COLUMN IF NOT EXISTS swap_source        FixedString(44) COMMENT 'Swap source account',
    ADD COLUMN IF NOT EXISTS swap_destination   FixedString(44) COMMENT 'Swap destination account',
    ADD COLUMN IF NOT EXISTS amount_in          UInt64 COMMENT 'Amount in',
    ADD COLUMN IF NOT EXISTS minimum_amount_out UInt64 COMMENT 'Minimum amount out';
