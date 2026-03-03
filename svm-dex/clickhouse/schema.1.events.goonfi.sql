-- GoonFi Buy --
CREATE TABLE IF NOT EXISTS goonfi_buy AS base_events
COMMENT 'GoonFi Buy';
ALTER TABLE goonfi_buy
    ADD COLUMN IF NOT EXISTS is_bid             Bool COMMENT 'Is bid';

-- GoonFi Sell --
CREATE TABLE IF NOT EXISTS goonfi_sell AS base_events
COMMENT 'GoonFi Sell';
ALTER TABLE goonfi_sell
    ADD COLUMN IF NOT EXISTS is_bid             Bool COMMENT 'Is bid';
