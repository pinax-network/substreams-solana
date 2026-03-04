-- InitializeAccount --
CREATE TABLE IF NOT EXISTS initialize_account AS base_events
COMMENT 'SPL Token InitializeAccount events';
ALTER TABLE initialize_account
    ADD COLUMN IF NOT EXISTS account                 String,
    ADD COLUMN IF NOT EXISTS mint                    LowCardinality(String),
    ADD COLUMN IF NOT EXISTS owner                   String,

    -- Indexes --
    ADD INDEX IF NOT EXISTS idx_account (account) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_mint (mint) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_owner (owner) TYPE bloom_filter(0.005) GRANULARITY 1;

-- InitializeMint --
CREATE TABLE IF NOT EXISTS initialize_mint AS base_events
COMMENT 'SPL Token InitializeMint events';
ALTER TABLE initialize_mint
    ADD COLUMN IF NOT EXISTS mint                    LowCardinality(String),
    ADD COLUMN IF NOT EXISTS mint_authority          String,
    ADD COLUMN IF NOT EXISTS freeze_authority_raw    String,
    ADD COLUMN IF NOT EXISTS freeze_authority        Nullable(String) MATERIALIZED string_or_null(freeze_authority_raw),
    ADD COLUMN IF NOT EXISTS decimals                UInt8,

    -- Indexes --
    ADD INDEX IF NOT EXISTS idx_mint (mint) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_mint_authority (mint_authority) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_freeze_authority (freeze_authority) TYPE bloom_filter(0.005) GRANULARITY 1;

-- InitializeImmutableOwner --
CREATE TABLE IF NOT EXISTS initialize_immutable_owner AS base_events
COMMENT 'SPL Token InitializeImmutableOwner events';
ALTER TABLE initialize_immutable_owner
    ADD COLUMN IF NOT EXISTS account                 String,

    -- Indexes --
    ADD INDEX IF NOT EXISTS idx_account (account) TYPE bloom_filter(0.005) GRANULARITY 1;

-- SetAuthority --
CREATE TABLE IF NOT EXISTS set_authority AS base_events
COMMENT 'SPL Token SetAuthority events';
ALTER TABLE set_authority
    ADD COLUMN IF NOT EXISTS account                 String,
    ADD COLUMN IF NOT EXISTS authority_type          LowCardinality(String),
    ADD COLUMN IF NOT EXISTS new_authority_raw       String,
    ADD COLUMN IF NOT EXISTS new_authority           Nullable(String) MATERIALIZED string_or_null(new_authority_raw),
    ADD COLUMN IF NOT EXISTS authority               String,
    ADD COLUMN IF NOT EXISTS multisig_authority_raw  String,
    ADD COLUMN IF NOT EXISTS multisig_authority      Array(String) MATERIALIZED string_to_array(multisig_authority_raw),

    -- Indexes --
    ADD INDEX IF NOT EXISTS idx_account (account) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_new_authority (new_authority) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_authority (authority) TYPE bloom_filter(0.005) GRANULARITY 1;

-- CloseAccount --
CREATE TABLE IF NOT EXISTS close_account AS base_events
COMMENT 'SPL Token CloseAccount events';
ALTER TABLE close_account
    ADD COLUMN IF NOT EXISTS account                 String,
    ADD COLUMN IF NOT EXISTS destination             String,
    ADD COLUMN IF NOT EXISTS authority               String,
    ADD COLUMN IF NOT EXISTS multisig_authority_raw  String,
    ADD COLUMN IF NOT EXISTS multisig_authority      Array(String) MATERIALIZED string_to_array(multisig_authority_raw),

    -- Indexes --
    ADD INDEX IF NOT EXISTS idx_account (account) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_destination (destination) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_authority (authority) TYPE bloom_filter(0.005) GRANULARITY 1;
