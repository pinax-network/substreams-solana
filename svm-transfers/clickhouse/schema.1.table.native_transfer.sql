-- System Token Transfers --
CREATE TABLE IF NOT EXISTS system_transfer AS base_events
COMMENT 'System token transfer';
ALTER TABLE system_transfer
    ADD COLUMN IF NOT EXISTS source                  String,
    ADD COLUMN IF NOT EXISTS destination             String,
    ADD COLUMN IF NOT EXISTS lamports                UInt64,
    -- Indexes --
    ADD INDEX IF NOT EXISTS idx_source (source) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_destination (destination) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_lamports (lamports) TYPE minmax GRANULARITY 1;

-- TransferWithSeed --
CREATE TABLE IF NOT EXISTS system_transfer_with_seed AS base_events
COMMENT 'System token transfer with seed';
ALTER TABLE system_transfer_with_seed
    ADD COLUMN IF NOT EXISTS source                  String,
    ADD COLUMN IF NOT EXISTS destination             String,
    ADD COLUMN IF NOT EXISTS lamports                UInt64,
    ADD COLUMN IF NOT EXISTS source_base             String COMMENT 'Base account address for the seed.',
    ADD COLUMN IF NOT EXISTS source_owner            String COMMENT 'Owner of the source account.',
    ADD COLUMN IF NOT EXISTS source_seed             String COMMENT 'Seed used to derive the source account.',
    -- Indexes --
    ADD INDEX IF NOT EXISTS idx_source (source) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_destination (destination) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_source_base (source_base) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_source_owner (source_owner) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_source_seed (source_seed) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_lamports (lamports) TYPE minmax GRANULARITY 1;

-- WithdrawNonceAccount --
CREATE TABLE IF NOT EXISTS system_withdraw_nonce_account AS base_events
COMMENT 'System token withdraw nonce account';
ALTER TABLE system_withdraw_nonce_account
    ADD COLUMN IF NOT EXISTS destination             String,
    ADD COLUMN IF NOT EXISTS lamports                UInt64,
    ADD COLUMN IF NOT EXISTS nonce_account           String COMMENT 'Nonce account address.',
    ADD COLUMN IF NOT EXISTS nonce_authority         String COMMENT 'Nonce authority account address.',
    -- Indexes --
    ADD INDEX IF NOT EXISTS idx_destination (destination) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_nonce_account (nonce_account) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_nonce_authority (nonce_authority) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_lamports (lamports) TYPE minmax GRANULARITY 1;
