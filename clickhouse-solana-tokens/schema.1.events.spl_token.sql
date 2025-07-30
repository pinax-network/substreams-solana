-- SPL Token Transfers --
CREATE TABLE IF NOT EXISTS spl_transfer AS base_events
COMMENT 'SPL Token Transfer/Burn/Mint events';
ALTER TABLE spl_transfer
    -- authority --
    ADD COLUMN IF NOT EXISTS authority               FixedString(44),
    ADD COLUMN IF NOT EXISTS multisig_authority_raw  String,
    ADD COLUMN IF NOT EXISTS multisig_authority      Array(String) MATERIALIZED string_to_array(multisig_authority_raw),

    -- events --
    ADD COLUMN IF NOT EXISTS source                  FixedString(44),
    ADD COLUMN IF NOT EXISTS destination             FixedString(44),
    ADD COLUMN IF NOT EXISTS amount                  UInt64,

    -- Optional
    ADD COLUMN IF NOT EXISTS mint_raw                String,
    ADD COLUMN IF NOT EXISTS mint                    Nullable(FixedString(44)) MATERIALIZED fixed_string_or_null(mint_raw),
    ADD COLUMN IF NOT EXISTS decimals_raw            String,
    ADD COLUMN IF NOT EXISTS decimals                Nullable(UInt8) MATERIALIZED string_to_uint8(decimals_raw);

-- InitializeAccount --
CREATE TABLE IF NOT EXISTS initialize_account AS base_events
COMMENT 'SPL Token InitializeAccount events';
ALTER TABLE initialize_account
    ADD COLUMN IF NOT EXISTS account                 FixedString(44),
    ADD COLUMN IF NOT EXISTS mint                    FixedString(44),
    ADD COLUMN IF NOT EXISTS owner                   FixedString(44);

-- InitializeMint --
CREATE TABLE IF NOT EXISTS initialize_mint AS base_events
COMMENT 'SPL Token InitializeMint events';
ALTER TABLE initialize_mint
    ADD COLUMN IF NOT EXISTS mint                    FixedString(44),
    ADD COLUMN IF NOT EXISTS mint_authority          FixedString(44),
    ADD COLUMN IF NOT EXISTS freeze_authority_raw    String,
    ADD COLUMN IF NOT EXISTS freeze_authority        Nullable(FixedString(44)) MATERIALIZED fixed_string_or_null(freeze_authority_raw),
    ADD COLUMN IF NOT EXISTS decimals                UInt8;

-- InitializeImmutableOwner --
CREATE TABLE IF NOT EXISTS initialize_immutable_owner AS base_events
COMMENT 'SPL Token InitializeImmutableOwner events';
ALTER TABLE initialize_immutable_owner
    ADD COLUMN IF NOT EXISTS account                 FixedString(44);

-- SetAuthority --
CREATE TABLE IF NOT EXISTS set_authority AS base_events
COMMENT 'SPL Token SetAuthority events';
ALTER TABLE set_authority
    ADD COLUMN IF NOT EXISTS account                 FixedString(44),
    ADD COLUMN IF NOT EXISTS authority_type          LowCardinality(String), -- AuthorityType enum as string
    ADD COLUMN IF NOT EXISTS new_authority_raw       String,
    ADD COLUMN IF NOT EXISTS new_authority           Nullable(FixedString(44)) MATERIALIZED fixed_string_or_null(new_authority_raw),
    ADD COLUMN IF NOT EXISTS authority               FixedString(44),
    ADD COLUMN IF NOT EXISTS multisig_authority_raw  String,
    ADD COLUMN IF NOT EXISTS multisig_authority      Array(String) MATERIALIZED string_to_array(multisig_authority_raw);

-- CloseAccount --
CREATE TABLE IF NOT EXISTS close_account AS base_events
COMMENT 'SPL Token CloseAccount events';
ALTER TABLE close_account
    ADD COLUMN IF NOT EXISTS account                 FixedString(44),
    ADD COLUMN IF NOT EXISTS destination             FixedString(44),
    ADD COLUMN IF NOT EXISTS authority               FixedString(44),
    ADD COLUMN IF NOT EXISTS multisig_authority_raw  String,
    ADD COLUMN IF NOT EXISTS multisig_authority      Array(String) MATERIALIZED string_to_array(multisig_authority_raw);

-- FreezeAccount --
CREATE TABLE IF NOT EXISTS freeze_account AS base_events
COMMENT 'SPL Token FreezeAccount events';
ALTER TABLE freeze_account
    ADD COLUMN IF NOT EXISTS account                 FixedString(44),
    ADD COLUMN IF NOT EXISTS mint                    FixedString(44),
    ADD COLUMN IF NOT EXISTS authority               FixedString(44),
    ADD COLUMN IF NOT EXISTS multisig_authority_raw  String,
    ADD COLUMN IF NOT EXISTS multisig_authority      Array(String) MATERIALIZED string_to_array(multisig_authority_raw);

-- ThawAccount --
CREATE TABLE IF NOT EXISTS thaw_account AS base_events
COMMENT 'SPL Token ThawAccount events';
ALTER TABLE thaw_account
    ADD COLUMN IF NOT EXISTS account                 FixedString(44),
    ADD COLUMN IF NOT EXISTS mint                    FixedString(44),
    ADD COLUMN IF NOT EXISTS authority               FixedString(44),
    ADD COLUMN IF NOT EXISTS multisig_authority_raw  String,
    ADD COLUMN IF NOT EXISTS multisig_authority      Array(String) MATERIALIZED string_to_array(multisig_authority_raw);

-- Approve --
CREATE TABLE IF NOT EXISTS approve AS base_events
COMMENT 'SPL Token Approve events';
ALTER TABLE approve
    ADD COLUMN IF NOT EXISTS source                  FixedString(44),
    ADD COLUMN IF NOT EXISTS mint                    FixedString(44), -- can be empty
    ADD COLUMN IF NOT EXISTS delegate                FixedString(44),
    ADD COLUMN IF NOT EXISTS owner                   FixedString(44),
    ADD COLUMN IF NOT EXISTS amount                  UInt64,
    ADD COLUMN IF NOT EXISTS decimals_raw            String,
    ADD COLUMN IF NOT EXISTS decimals                Nullable(UInt8) MATERIALIZED string_to_uint8(decimals_raw),
    ADD COLUMN IF NOT EXISTS authority               FixedString(44),
    ADD COLUMN IF NOT EXISTS multisig_authority_raw  String,
    ADD COLUMN IF NOT EXISTS multisig_authority      Array(String) MATERIALIZED string_to_array(multisig_authority_raw);

-- Revoke --
CREATE TABLE IF NOT EXISTS revoke AS base_events
COMMENT 'SPL Token Revoke events';
ALTER TABLE revoke
    ADD COLUMN IF NOT EXISTS source                  FixedString(44),
    ADD COLUMN IF NOT EXISTS owner                   FixedString(44),
    ADD COLUMN IF NOT EXISTS authority               FixedString(44),
    ADD COLUMN IF NOT EXISTS multisig_authority_raw  String,
    ADD COLUMN IF NOT EXISTS multisig_authority      Array(String) MATERIALIZED string_to_array(multisig_authority_raw);

-- InitializeTokenMetadata --
CREATE TABLE IF NOT EXISTS initialize_token_metadata AS base_events
COMMENT 'SPL Token InitializeTokenMetadata events';
ALTER TABLE initialize_token_metadata
    ADD COLUMN IF NOT EXISTS metadata                FixedString(44),
    ADD COLUMN IF NOT EXISTS update_authority        FixedString(44),
    ADD COLUMN IF NOT EXISTS mint                    FixedString(44),
    ADD COLUMN IF NOT EXISTS mint_authority          FixedString(44),
    ADD COLUMN IF NOT EXISTS name                    String,
    ADD COLUMN IF NOT EXISTS symbol                  String,
    ADD COLUMN IF NOT EXISTS uri                     String;

-- UpdateTokenMetadataField --
CREATE TABLE IF NOT EXISTS update_token_metadata_field AS base_events
COMMENT 'SPL Token UpdateTokenMetadataField events';
ALTER TABLE update_token_metadata_field
    ADD COLUMN IF NOT EXISTS metadata                FixedString(44),
    ADD COLUMN IF NOT EXISTS update_authority        FixedString(44),
    ADD COLUMN IF NOT EXISTS field                   String,
    ADD COLUMN IF NOT EXISTS value                   String;

-- UpdateTokenMetadataAuthority --
CREATE TABLE IF NOT EXISTS update_token_metadata_authority AS base_events
COMMENT 'SPL Token UpdateTokenMetadataAuthority events';
ALTER TABLE update_token_metadata_authority
    ADD COLUMN IF NOT EXISTS metadata                FixedString(44),
    ADD COLUMN IF NOT EXISTS update_authority        FixedString(44),
    ADD COLUMN IF NOT EXISTS new_authority           FixedString(44);

-- RemoveTokenMetadataField --
CREATE TABLE IF NOT EXISTS remove_token_metadata_field AS base_events
COMMENT 'SPL Token RemoveTokenMetadataField events';
ALTER TABLE remove_token_metadata_field
    ADD COLUMN IF NOT EXISTS metadata                FixedString(44),
    ADD COLUMN IF NOT EXISTS update_authority        FixedString(44),
    ADD COLUMN IF NOT EXISTS `key`                   String,
    ADD COLUMN IF NOT EXISTS idempotent              Bool DEFAULT false COMMENT 'Whether the removal is idempotent';

-- SPL Token Post Balance --
CREATE TABLE IF NOT EXISTS post_token_balances AS base_transactions
COMMENT 'SPL Token Post Balance events';
ALTER TABLE post_token_balances
    ADD COLUMN IF NOT EXISTS program_id         FixedString(44) COMMENT 'Program ID of the SPL Token program.',
    ADD COLUMN IF NOT EXISTS account            FixedString(44) COMMENT 'Account address.',
    ADD COLUMN IF NOT EXISTS mint               FixedString(44) COMMENT 'Mint address',
    ADD COLUMN IF NOT EXISTS amount             UInt64 COMMENT 'Balance amount in lamports.',
    ADD COLUMN IF NOT EXISTS decimals           UInt8;

-- SPL Token Pre Balance --
CREATE TABLE IF NOT EXISTS pre_token_balances AS post_token_balances
COMMENT 'SPL Token Pre Balance events';
