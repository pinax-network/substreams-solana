/* MINT */
CREATE TABLE IF NOT EXISTS metadata_mint_state_latest (
    metadata   String,
    mint       LowCardinality(String),        -- '' means removed
    version    UInt64,
    block_num  UInt32,
    timestamp  DateTime('UTC'),

    INDEX idx_mint (mint) TYPE bloom_filter(0.005) GRANULARITY 1
)
ENGINE = ReplacingMergeTree(version)
ORDER BY (metadata);

/* MINT AUTHORITY */
CREATE TABLE IF NOT EXISTS metadata_mint_authority_state_latest (
    metadata       String,
    mint_authority String,                    -- '' means removed
    version        UInt64,
    block_num      UInt32,
    timestamp      DateTime('UTC'),

    INDEX idx_mint_authority (mint_authority) TYPE bloom_filter(0.005) GRANULARITY 1
)
ENGINE = ReplacingMergeTree(version)
ORDER BY (metadata);

/* NAME */
CREATE TABLE IF NOT EXISTS metadata_name_state_latest (
    metadata   String,
    name       LowCardinality(String),        -- '' means removed
    version    UInt64,
    block_num  UInt32,
    timestamp  DateTime('UTC'),

    INDEX idx_name (name) TYPE bloom_filter(0.005) GRANULARITY 1
)
ENGINE = ReplacingMergeTree(version)
ORDER BY (metadata);

/* SYMBOL */
CREATE TABLE IF NOT EXISTS metadata_symbol_state_latest (
    metadata   String,
    symbol     LowCardinality(String),        -- '' means removed
    version    UInt64,
    block_num  UInt32,
    timestamp  DateTime('UTC'),

    INDEX idx_symbol (symbol) TYPE bloom_filter(0.005) GRANULARITY 1
)
ENGINE = ReplacingMergeTree(version)
ORDER BY (metadata);

/* URI */
CREATE TABLE IF NOT EXISTS metadata_uri_state_latest (
    metadata   String,
    uri        String,                        -- '' means removed
    version    UInt64,
    block_num  UInt32,
    timestamp  DateTime('UTC'),

    INDEX idx_uri (uri) TYPE bloom_filter(0.005) GRANULARITY 1
)
ENGINE = ReplacingMergeTree(version)
ORDER BY (metadata);

/* UPDATE AUTHORITY */
CREATE TABLE IF NOT EXISTS metadata_update_authority_state_latest (
    metadata          String,
    update_authority  String,                 -- '' means removed
    version           UInt64,
    block_num         UInt32,
    timestamp         DateTime('UTC'),

    INDEX idx_update_authority (update_authority) TYPE bloom_filter(0.005) GRANULARITY 1
)
ENGINE = ReplacingMergeTree(version)
ORDER BY (metadata);

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

