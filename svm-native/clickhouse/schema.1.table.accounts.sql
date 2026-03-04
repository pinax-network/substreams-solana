-- CreateAccount (native) --
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

-- CreateAccountWithSeed (native) --
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
