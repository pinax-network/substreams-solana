-- InitializeTokenMetadata --
CREATE TABLE IF NOT EXISTS initialize_token_metadata AS base_events
COMMENT 'SPL Token InitializeTokenMetadata events';
ALTER TABLE initialize_token_metadata
    ADD COLUMN IF NOT EXISTS metadata                LowCardinality(String),
    ADD COLUMN IF NOT EXISTS update_authority        LowCardinality(String),
    ADD COLUMN IF NOT EXISTS mint                    LowCardinality(String),
    ADD COLUMN IF NOT EXISTS mint_authority          LowCardinality(String),
    ADD COLUMN IF NOT EXISTS name                    String,
    ADD COLUMN IF NOT EXISTS symbol                  LowCardinality(String),
    ADD COLUMN IF NOT EXISTS uri                     String;

-- UpdateTokenMetadataField --
CREATE TABLE IF NOT EXISTS update_token_metadata_field AS base_events
COMMENT 'SPL Token UpdateTokenMetadataField events';
ALTER TABLE update_token_metadata_field
    ADD COLUMN IF NOT EXISTS metadata                String,
    ADD COLUMN IF NOT EXISTS update_authority        String,
    ADD COLUMN IF NOT EXISTS field                   LowCardinality(String),
    ADD COLUMN IF NOT EXISTS value                   String;

-- UpdateTokenMetadataAuthority --
CREATE TABLE IF NOT EXISTS update_token_metadata_authority AS base_events
COMMENT 'SPL Token UpdateTokenMetadataAuthority events';
ALTER TABLE update_token_metadata_authority
    ADD COLUMN IF NOT EXISTS metadata                String,
    ADD COLUMN IF NOT EXISTS update_authority        String,
    ADD COLUMN IF NOT EXISTS new_authority           String;

-- RemoveTokenMetadataField --
CREATE TABLE IF NOT EXISTS remove_token_metadata_field AS base_events
COMMENT 'SPL Token RemoveTokenMetadataField events';
ALTER TABLE remove_token_metadata_field
    ADD COLUMN IF NOT EXISTS metadata                String,
    ADD COLUMN IF NOT EXISTS update_authority        String,
    ADD COLUMN IF NOT EXISTS `key`                   String,
    ADD COLUMN IF NOT EXISTS idempotent              Bool DEFAULT false COMMENT 'Whether the removal is idempotent';
