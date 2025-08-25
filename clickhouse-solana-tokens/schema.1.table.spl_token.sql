-- SPL Token Transfers --
CREATE TABLE IF NOT EXISTS spl_transfer AS base_events
COMMENT 'SPL Token Transfer/Burn/Mint events';
ALTER TABLE spl_transfer
    -- authority --
    ADD COLUMN IF NOT EXISTS authority               String,
    ADD COLUMN IF NOT EXISTS multisig_authority_raw  String,
    ADD COLUMN IF NOT EXISTS multisig_authority      Array(String) MATERIALIZED string_to_array(multisig_authority_raw),

    -- events --
    ADD COLUMN IF NOT EXISTS source                  String,
    ADD COLUMN IF NOT EXISTS destination             String,
    ADD COLUMN IF NOT EXISTS amount                  UInt64,
    ADD COLUMN IF NOT EXISTS mint                    LowCardinality(String),

    -- Optional
    ADD COLUMN IF NOT EXISTS decimals_raw            String,
    ADD COLUMN IF NOT EXISTS decimals                Nullable(UInt8) MATERIALIZED string_to_uint8(decimals_raw),

    -- Indexes --
    ADD INDEX IF NOT EXISTS idx_source (source) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_destination (destination) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_mint (mint) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_amount (amount) TYPE minmax GRANULARITY 1;

-- InitializeAccount --
CREATE TABLE IF NOT EXISTS initialize_account AS base_events
COMMENT 'SPL Token InitializeAccount events';
ALTER TABLE initialize_account
    ADD COLUMN IF NOT EXISTS account                 String,
    ADD COLUMN IF NOT EXISTS mint                    LowCardinality(String),
    ADD COLUMN IF NOT EXISTS owner                   String;

-- InitializeMint --
CREATE TABLE IF NOT EXISTS initialize_mint AS base_events
COMMENT 'SPL Token InitializeMint events';
ALTER TABLE initialize_mint
    ADD COLUMN IF NOT EXISTS mint                    LowCardinality(String),
    ADD COLUMN IF NOT EXISTS mint_authority          String,
    ADD COLUMN IF NOT EXISTS freeze_authority_raw    String,
    ADD COLUMN IF NOT EXISTS freeze_authority        Nullable(String) MATERIALIZED string_or_null(freeze_authority_raw),
    ADD COLUMN IF NOT EXISTS decimals                UInt8;

-- InitializeImmutableOwner --
CREATE TABLE IF NOT EXISTS initialize_immutable_owner AS base_events
COMMENT 'SPL Token InitializeImmutableOwner events';
ALTER TABLE initialize_immutable_owner
    ADD COLUMN IF NOT EXISTS account                 String;

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
    ADD COLUMN IF NOT EXISTS multisig_authority      Array(String) MATERIALIZED string_to_array(multisig_authority_raw);

-- CloseAccount --
CREATE TABLE IF NOT EXISTS close_account AS base_events
COMMENT 'SPL Token CloseAccount events';
ALTER TABLE close_account
    ADD COLUMN IF NOT EXISTS account                 String,
    ADD COLUMN IF NOT EXISTS destination             String,
    ADD COLUMN IF NOT EXISTS authority               String,
    ADD COLUMN IF NOT EXISTS multisig_authority_raw  String,
    ADD COLUMN IF NOT EXISTS multisig_authority      Array(String) MATERIALIZED string_to_array(multisig_authority_raw);

-- FreezeAccount --
CREATE TABLE IF NOT EXISTS freeze_account AS base_events
COMMENT 'SPL Token FreezeAccount events';
ALTER TABLE freeze_account
    ADD COLUMN IF NOT EXISTS account                 String,
    ADD COLUMN IF NOT EXISTS mint                    LowCardinality(String),
    ADD COLUMN IF NOT EXISTS authority               String,
    ADD COLUMN IF NOT EXISTS multisig_authority_raw  String,
    ADD COLUMN IF NOT EXISTS multisig_authority      Array(String) MATERIALIZED string_to_array(multisig_authority_raw);

-- ThawAccount --
CREATE TABLE IF NOT EXISTS thaw_account AS base_events
COMMENT 'SPL Token ThawAccount events';
ALTER TABLE thaw_account
    ADD COLUMN IF NOT EXISTS account                 String,
    ADD COLUMN IF NOT EXISTS mint                    LowCardinality(String),
    ADD COLUMN IF NOT EXISTS authority               String,
    ADD COLUMN IF NOT EXISTS multisig_authority_raw  String,
    ADD COLUMN IF NOT EXISTS multisig_authority      Array(String) MATERIALIZED string_to_array(multisig_authority_raw);

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
    ADD COLUMN IF NOT EXISTS multisig_authority      Array(String) MATERIALIZED string_to_array(multisig_authority_raw);

-- Revoke --
CREATE TABLE IF NOT EXISTS revoke AS base_events
COMMENT 'SPL Token Revoke events';
ALTER TABLE revoke
    ADD COLUMN IF NOT EXISTS source                  String,
    ADD COLUMN IF NOT EXISTS owner                   String,
    ADD COLUMN IF NOT EXISTS authority               String,
    ADD COLUMN IF NOT EXISTS multisig_authority_raw  String,
    ADD COLUMN IF NOT EXISTS multisig_authority      Array(String) MATERIALIZED string_to_array(multisig_authority_raw);

-- SPL Token Post Balance --
CREATE TABLE IF NOT EXISTS post_token_balances AS base_transactions
COMMENT 'SPL Token Post Balance events (only last transaction in block which effects the balance)';
ALTER TABLE post_token_balances
    ADD COLUMN IF NOT EXISTS program_id         LowCardinality(String) COMMENT 'Program ID of the SPL Token program.',
    ADD COLUMN IF NOT EXISTS account            String COMMENT 'Account address.',
    ADD COLUMN IF NOT EXISTS mint               String COMMENT 'Mint address',
    ADD COLUMN IF NOT EXISTS amount             UInt64 COMMENT 'Balance amount in lamports.',
    ADD COLUMN IF NOT EXISTS decimals           UInt8;
