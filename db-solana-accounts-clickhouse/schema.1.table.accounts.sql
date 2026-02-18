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
    ADD COLUMN IF NOT EXISTS authority_type          LowCardinality(String), -- AuthorityType enum as string
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

-- FreezeAccount --
CREATE TABLE IF NOT EXISTS freeze_account AS base_events
COMMENT 'SPL Token FreezeAccount events';
ALTER TABLE freeze_account
    ADD COLUMN IF NOT EXISTS account                 String,
    ADD COLUMN IF NOT EXISTS mint                    LowCardinality(String),
    ADD COLUMN IF NOT EXISTS authority               String,
    ADD COLUMN IF NOT EXISTS multisig_authority_raw  String,
    ADD COLUMN IF NOT EXISTS multisig_authority      Array(String) MATERIALIZED string_to_array(multisig_authority_raw),

    -- Indexes --
    ADD INDEX IF NOT EXISTS idx_account (account) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_mint (mint) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_authority (authority) TYPE bloom_filter(0.005) GRANULARITY 1;

-- ThawAccount --
CREATE TABLE IF NOT EXISTS thaw_account AS base_events
COMMENT 'SPL Token ThawAccount events';
ALTER TABLE thaw_account
    ADD COLUMN IF NOT EXISTS account                 String,
    ADD COLUMN IF NOT EXISTS mint                    LowCardinality(String),
    ADD COLUMN IF NOT EXISTS authority               String,
    ADD COLUMN IF NOT EXISTS multisig_authority_raw  String,
    ADD COLUMN IF NOT EXISTS multisig_authority      Array(String) MATERIALIZED string_to_array(multisig_authority_raw),

    -- Indexes --
    ADD INDEX IF NOT EXISTS idx_account (account) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_mint (mint) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_authority (authority) TYPE bloom_filter(0.005) GRANULARITY 1;

-- Approve --
CREATE TABLE IF NOT EXISTS approve AS base_events
COMMENT 'SPL Token Approve events';
ALTER TABLE approve
    ADD COLUMN IF NOT EXISTS source                  String,
    ADD COLUMN IF NOT EXISTS mint_raw                String,
    ADD COLUMN IF NOT EXISTS mint                    Nullable(String) MATERIALIZED string_or_null(mint_raw),
    ADD COLUMN IF NOT EXISTS delegate                String,
    ADD COLUMN IF NOT EXISTS owner                   String,
    ADD COLUMN IF NOT EXISTS amount                  UInt64,
    ADD COLUMN IF NOT EXISTS decimals_raw            String,
    ADD COLUMN IF NOT EXISTS decimals                Nullable(UInt8) MATERIALIZED string_to_uint8(decimals_raw),
    ADD COLUMN IF NOT EXISTS authority               String,
    ADD COLUMN IF NOT EXISTS multisig_authority_raw  String,
    ADD COLUMN IF NOT EXISTS multisig_authority      Array(String) MATERIALIZED string_to_array(multisig_authority_raw),

    -- Indexes --
    ADD INDEX IF NOT EXISTS idx_source (source) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_mint (mint) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_delegate (delegate) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_owner (owner) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_amount (amount) TYPE minmax GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_authority (authority) TYPE bloom_filter(0.005) GRANULARITY 1;

-- Revoke --
CREATE TABLE IF NOT EXISTS revoke AS base_events
COMMENT 'SPL Token Revoke events';
ALTER TABLE revoke
    ADD COLUMN IF NOT EXISTS source                  String,
    ADD COLUMN IF NOT EXISTS owner                   String,
    ADD COLUMN IF NOT EXISTS authority               String,
    ADD COLUMN IF NOT EXISTS multisig_authority_raw  String,
    ADD COLUMN IF NOT EXISTS multisig_authority      Array(String) MATERIALIZED string_to_array(multisig_authority_raw),

    -- Indexes --
    ADD INDEX IF NOT EXISTS idx_source (source) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_owner (owner) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_authority (authority) TYPE bloom_filter(0.005) GRANULARITY 1;

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
