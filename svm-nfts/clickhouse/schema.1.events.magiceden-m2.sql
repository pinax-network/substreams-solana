-- Magic Eden M2 Sell --
CREATE TABLE IF NOT EXISTS magiceden_m2_sell AS base_events
COMMENT 'Magic Eden M2 Sell (list NFT)';
ALTER TABLE magiceden_m2_sell
    ADD COLUMN IF NOT EXISTS buyer_price            UInt64 COMMENT 'Listed price in lamports',
    ADD COLUMN IF NOT EXISTS token_size             UInt64 COMMENT 'Token size',
    ADD COLUMN IF NOT EXISTS seller_state_expiry    Int64  COMMENT 'Seller state expiry';

-- Magic Eden M2 Execute Sale (buy NFT) --
CREATE TABLE IF NOT EXISTS magiceden_m2_execute_sale AS base_events
COMMENT 'Magic Eden M2 Execute Sale (buy NFT)';
ALTER TABLE magiceden_m2_execute_sale
    ADD COLUMN IF NOT EXISTS buyer_price            UInt64 COMMENT 'Sale price in lamports',
    ADD COLUMN IF NOT EXISTS token_size             UInt64 COMMENT 'Token size',
    ADD COLUMN IF NOT EXISTS buyer_state_expiry     Int64  COMMENT 'Buyer state expiry',
    ADD COLUMN IF NOT EXISTS seller_state_expiry    Int64  COMMENT 'Seller state expiry',
    ADD COLUMN IF NOT EXISTS maker_fee_bp           Int32  COMMENT 'Maker fee in basis points',
    ADD COLUMN IF NOT EXISTS taker_fee_bp           UInt32 COMMENT 'Taker fee in basis points';
