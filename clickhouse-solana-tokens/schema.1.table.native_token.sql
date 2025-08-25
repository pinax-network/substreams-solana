-- SPL Token Transfers --
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

-- CreateAccount --
CREATE TABLE IF NOT EXISTS system_create_account AS base_events
COMMENT 'System token create account';
ALTER TABLE system_create_account
    ADD COLUMN IF NOT EXISTS source                  String COMMENT 'Funding account address.',
    ADD COLUMN IF NOT EXISTS new_account             String COMMENT 'New account address.',
    ADD COLUMN IF NOT EXISTS owner                   String COMMENT 'Owner program account address',
    ADD COLUMN IF NOT EXISTS lamports                UInt64 COMMENT 'Initial balance in lamports.',
    ADD COLUMN IF NOT EXISTS space                   UInt64 COMMENT 'Space allocated for the new account.',
    -- Indexes --
    ADD INDEX IF NOT EXISTS idx_source (source) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_new_account (new_account) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_owner (owner) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_space (space) TYPE set(32) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_lamports (lamports) TYPE minmax GRANULARITY 1;

-- CreateAccountWithSeed --
CREATE TABLE IF NOT EXISTS system_create_account_with_seed AS base_events
COMMENT 'System token create account with seed';
ALTER TABLE system_create_account_with_seed
    ADD COLUMN IF NOT EXISTS source                  String COMMENT 'Funding account address.',
    ADD COLUMN IF NOT EXISTS new_account             String COMMENT 'New account address.',
    ADD COLUMN IF NOT EXISTS base                    String COMMENT 'Primary base account address used for deriving the seed.',
    ADD COLUMN IF NOT EXISTS base_account_raw        String COMMENT 'Optional secondary account related to the base.',
    ADD COLUMN IF NOT EXISTS base_account            Nullable(String) MATERIALIZED if(empty(base_account_raw), NULL, base_account_raw),
    ADD COLUMN IF NOT EXISTS owner                   String COMMENT 'Owner program account address',
    ADD COLUMN IF NOT EXISTS lamports                UInt64 COMMENT 'Initial balance in lamports.',
    ADD COLUMN IF NOT EXISTS space                   UInt64 COMMENT 'Space allocated for the new account.',
    ADD COLUMN IF NOT EXISTS seed                    String COMMENT 'Seed used to derive the new account.',
    -- Indexes --
    ADD INDEX IF NOT EXISTS idx_source (source) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_new_account (new_account) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_base (base) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_base_account (base_account) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_owner (owner) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_space (space) TYPE set(32) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_lamports (lamports) TYPE minmax GRANULARITY 1;

-- System Post Balance --
CREATE TABLE IF NOT EXISTS system_post_balances AS base_transactions
COMMENT 'System post balances (only last transaction in block which effects the balance)';
ALTER TABLE system_post_balances
    ADD COLUMN IF NOT EXISTS account                  String COMMENT 'Account address.',
    ADD COLUMN IF NOT EXISTS amount                   UInt64 COMMENT 'Balance amount in lamports.',
    -- Indexes --
    ADD INDEX IF NOT EXISTS idx_account (account) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_amount (amount) TYPE minmax GRANULARITY 1;
