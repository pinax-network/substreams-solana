-- Phoenix Swap --
CREATE TABLE IF NOT EXISTS phoenix_swap AS base_events
COMMENT 'Phoenix Swap';
ALTER TABLE phoenix_swap
    ADD COLUMN IF NOT EXISTS trader         FixedString(44) COMMENT 'Trader account',
    ADD COLUMN IF NOT EXISTS market         FixedString(44) COMMENT 'Market account',
    ADD COLUMN IF NOT EXISTS base_account   FixedString(44) COMMENT 'Base token account',
    ADD COLUMN IF NOT EXISTS quote_account  FixedString(44) COMMENT 'Quote token account';
