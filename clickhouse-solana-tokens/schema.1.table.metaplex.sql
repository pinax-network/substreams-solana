-- CreateMetadataAccount --
CREATE TABLE IF NOT EXISTS metaplex_create_metadata_account AS base_events
COMMENT 'Metaplex Create Metadata Account events';
ALTER TABLE metaplex_create_metadata_account
    ADD COLUMN IF NOT EXISTS metadata                LowCardinality(String),
    ADD COLUMN IF NOT EXISTS mint                    LowCardinality(String),
    ADD COLUMN IF NOT EXISTS update_authority        LowCardinality(String),
    ADD COLUMN IF NOT EXISTS payer                   String,
    ADD COLUMN IF NOT EXISTS name                    String,
    ADD COLUMN IF NOT EXISTS symbol                  String,
    ADD COLUMN IF NOT EXISTS uri                     String;

-- UpdateMetadataAccount --
CREATE TABLE IF NOT EXISTS metaplex_update_metadata_account AS base_events
COMMENT 'Metaplex Update Metadata Account events';
ALTER TABLE metaplex_update_metadata_account
    ADD COLUMN IF NOT EXISTS metadata                LowCardinality(String),
    ADD COLUMN IF NOT EXISTS update_authority        LowCardinality(String),
    ADD COLUMN IF NOT EXISTS name_raw                String,
    ADD COLUMN IF NOT EXISTS symbol_raw              String,
    ADD COLUMN IF NOT EXISTS uri_raw                 String,
    ADD COLUMN IF NOT EXISTS name                    Nullable(String) MATERIALIZED string_or_null(name_raw),
    ADD COLUMN IF NOT EXISTS symbol                  Nullable(String) MATERIALIZED string_or_null(symbol_raw),
    ADD COLUMN IF NOT EXISTS uri                     Nullable(String) MATERIALIZED string_or_null(uri_raw);
