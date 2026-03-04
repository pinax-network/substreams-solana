-- Magic Eden M3 Fulfill Buy --
CREATE TABLE IF NOT EXISTS magiceden_m3_fulfill_buy AS base_events
COMMENT 'Magic Eden M3 Fulfill Buy (NFT sold to pool bid)';
ALTER TABLE magiceden_m3_fulfill_buy
    ADD COLUMN IF NOT EXISTS asset_amount           UInt64 COMMENT 'Number of NFTs',
    ADD COLUMN IF NOT EXISTS min_payment_amount     UInt64 COMMENT 'Minimum payment amount in lamports',
    ADD COLUMN IF NOT EXISTS maker_fee_bp           Int32  COMMENT 'Maker fee in basis points',
    ADD COLUMN IF NOT EXISTS taker_fee_bp           Int32  COMMENT 'Taker fee in basis points';

-- Magic Eden M3 Fulfill Sell --
CREATE TABLE IF NOT EXISTS magiceden_m3_fulfill_sell AS base_events
COMMENT 'Magic Eden M3 Fulfill Sell (NFT bought from pool listing)';
ALTER TABLE magiceden_m3_fulfill_sell
    ADD COLUMN IF NOT EXISTS asset_amount               UInt64 COMMENT 'Number of NFTs',
    ADD COLUMN IF NOT EXISTS max_payment_amount         UInt64 COMMENT 'Maximum payment amount in lamports',
    ADD COLUMN IF NOT EXISTS buyside_creator_royalty_bp UInt32 COMMENT 'Buyside creator royalty in basis points',
    ADD COLUMN IF NOT EXISTS maker_fee_bp               Int32  COMMENT 'Maker fee in basis points',
    ADD COLUMN IF NOT EXISTS taker_fee_bp               Int32  COMMENT 'Taker fee in basis points';
