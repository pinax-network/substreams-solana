-- Saros Swap --
CREATE TABLE IF NOT EXISTS saros_swap AS base_events
COMMENT 'Saros Swap';
ALTER TABLE saros_swap
    ADD COLUMN IF NOT EXISTS amount_in          UInt64 COMMENT 'Amount in',
    ADD COLUMN IF NOT EXISTS minimum_amount_out UInt64 COMMENT 'Minimum amount out';
