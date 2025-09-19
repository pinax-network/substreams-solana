CREATE TABLE IF NOT EXISTS TEMPLATE_METADATA (
    metadata   String,
    version    UInt64,
    block_num  UInt32,
    timestamp  DateTime('UTC')
)
ENGINE = ReplacingMergeTree(version)
ORDER BY (metadata);
ALTER TABLE TEMPLATE_METADATA MODIFY SETTING deduplicate_merge_projection_mode = 'rebuild';

/* MINT */
-- 1:1 relationship with metadata account
-- should not have any duplicates, no need for GROUP BY with custom view
CREATE TABLE IF NOT EXISTS metadata_mint_state_latest AS TEMPLATE_METADATA;
ALTER TABLE metadata_mint_state_latest
    ADD COLUMN IF NOT EXISTS mint LowCardinality(String) AFTER metadata,
    ADD PROJECTION IF NOT EXISTS prj_mint (SELECT * ORDER BY (mint));

/* MINT AUTHORITY */
CREATE TABLE IF NOT EXISTS metadata_mint_authority_state_latest AS TEMPLATE_METADATA;
ALTER TABLE metadata_mint_authority_state_latest
    ADD COLUMN IF NOT EXISTS mint_authority String AFTER metadata,
    ADD PROJECTION IF NOT EXISTS prj_mint_authority (SELECT * ORDER BY (mint_authority, metadata));

/* NAME */
CREATE TABLE IF NOT EXISTS metadata_name_state_latest AS TEMPLATE_METADATA;
ALTER TABLE metadata_name_state_latest
    ADD COLUMN IF NOT EXISTS name LowCardinality(String) AFTER metadata,
    ADD PROJECTION IF NOT EXISTS prj_name (SELECT * ORDER BY (name, metadata));

/* SYMBOL */
CREATE TABLE IF NOT EXISTS metadata_symbol_state_latest AS TEMPLATE_METADATA;
ALTER TABLE metadata_symbol_state_latest
    ADD COLUMN IF NOT EXISTS symbol LowCardinality(String) AFTER metadata,
    ADD PROJECTION IF NOT EXISTS prj_symbol (SELECT * ORDER BY (symbol, metadata));


/* URI */
CREATE TABLE IF NOT EXISTS metadata_uri_state_latest AS TEMPLATE_METADATA;
ALTER TABLE metadata_uri_state_latest
    ADD COLUMN IF NOT EXISTS uri LowCardinality(String) AFTER metadata,
    ADD PROJECTION IF NOT EXISTS prj_uri (SELECT * ORDER BY (uri, metadata));

/* UPDATE AUTHORITY */
CREATE TABLE IF NOT EXISTS metadata_update_authority_state_latest AS TEMPLATE_METADATA;
ALTER TABLE metadata_update_authority_state_latest
    ADD COLUMN IF NOT EXISTS update_authority String AFTER metadata,
    ADD PROJECTION IF NOT EXISTS prj_update_authority (SELECT * ORDER BY (update_authority, metadata));

/* ===========================
   MATERIALIZED VIEWS (ROUTING)
   =========================== */

/* INITIALIZE fan-out */
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_initialize_token_metadata_update_authority
TO metadata_update_authority_state_latest AS
SELECT metadata, update_authority, version, block_num, timestamp
FROM initialize_token_metadata;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_initialize_token_metadata_mint
TO metadata_mint_state_latest AS
SELECT metadata, mint, version, block_num, timestamp
FROM initialize_token_metadata;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_initialize_token_metadata_mint_authority
TO metadata_mint_authority_state_latest AS
SELECT metadata, mint_authority, version, block_num, timestamp
FROM initialize_token_metadata;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_initialize_token_metadata_name
TO metadata_name_state_latest AS
SELECT metadata, name, version, block_num, timestamp
FROM initialize_token_metadata
WHERE name != '';

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_initialize_token_metadata_symbol
TO metadata_symbol_state_latest AS
SELECT metadata, symbol, version, block_num, timestamp
FROM initialize_token_metadata
WHERE symbol != '';

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_initialize_token_metadata_uri
TO metadata_uri_state_latest AS
SELECT metadata, uri, version, block_num, timestamp
FROM initialize_token_metadata
WHERE uri != '';

/* UPDATE AUTHORITY */
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_update_token_metadata_authority_latest
TO metadata_update_authority_state_latest AS
SELECT metadata, update_authority, version, block_num, timestamp
FROM update_token_metadata_authority;

/* FIELD UPDATES */
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_update_token_metadata_field_name
TO metadata_name_state_latest AS
SELECT metadata, value AS name, version, block_num, timestamp
FROM update_token_metadata_field
WHERE field = 'name';

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_update_token_metadata_field_symbol
TO metadata_symbol_state_latest AS
SELECT metadata, value AS symbol, version, block_num, timestamp
FROM update_token_metadata_field
WHERE field = 'symbol';

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_update_token_metadata_field_uri
TO metadata_uri_state_latest AS
SELECT metadata, value AS uri, version, block_num, timestamp
FROM update_token_metadata_field
WHERE field = 'uri';

/* FIELD REMOVALS -> emit '' */
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_remove_token_metadata_field_name
TO metadata_name_state_latest AS
SELECT metadata, '' AS name, version, block_num, timestamp
FROM remove_token_metadata_field
WHERE `key` = 'name';

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_remove_token_metadata_field_symbol
TO metadata_symbol_state_latest AS
SELECT metadata, '' AS symbol, version, block_num, timestamp
FROM remove_token_metadata_field
WHERE `key` = 'symbol';

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_remove_token_metadata_field_uri
TO metadata_uri_state_latest AS
SELECT metadata, '' AS uri, version, block_num, timestamp
FROM remove_token_metadata_field
WHERE `key` = 'uri';

