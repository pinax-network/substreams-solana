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
