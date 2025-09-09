CREATE TABLE IF NOT EXISTS metadata (
    metadata         String,
    mint             LowCardinality(String),
    mint_authority   String,
    update_authority String,
    name             LowCardinality(String),
    symbol           LowCardinality(String),
    uri              String,

    -- indexes --
    INDEX idx_mint_authority (mint_authority) TYPE bloom_filter(0.005) GRANULARITY 1,
    INDEX idx_update_authority (update_authority) TYPE bloom_filter(0.005) GRANULARITY 1,
    INDEX idx_name (name) TYPE bloom_filter(0.005) GRANULARITY 1,
    INDEX idx_symbol (symbol) TYPE bloom_filter(0.005) GRANULARITY 1,
    INDEX idx_uri (uri) TYPE bloom_filter(0.005) GRANULARITY 1
)
ENGINE = ReplacingMergeTree
ORDER BY (metadata);

-- optimize for lookups by mint
ALTER TABLE metadata MODIFY SETTING deduplicate_merge_projection_mode = 'rebuild';
ALTER TABLE metadata
    ADD PROJECTION IF NOT EXISTS prj_mint (SELECT * ORDER BY (mint));

-- 1) MV: refresh on name changes
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_metadata_by_mint_from_name
TO metadata AS
SELECT v.mint, v.metadata as metadata, v.update_authority, v.mint_authority, v.name, v.symbol, v.uri
FROM (SELECT DISTINCT metadata FROM metadata_name_state_latest) AS changed
INNER JOIN metadata_view AS v ON v.metadata = changed.metadata;

-- 2) MV: refresh on symbol changes
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_metadata_by_mint_from_symbol
TO metadata AS
SELECT v.mint, v.metadata as metadata, v.update_authority, v.mint_authority, v.name, v.symbol, v.uri
FROM (SELECT DISTINCT metadata FROM metadata_symbol_state_latest) AS changed
INNER JOIN metadata_view AS v ON v.metadata = changed.metadata;

-- 3) MV: refresh on uri changes
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_metadata_by_mint_from_uri
TO metadata AS
SELECT v.mint, v.metadata as metadata, v.update_authority, v.mint_authority, v.name, v.symbol, v.uri
FROM (SELECT DISTINCT metadata FROM metadata_uri_state_latest) AS changed
INNER JOIN metadata_view AS v ON v.metadata = changed.metadata;

-- 4) MV: refresh on update authority changes
CREATE MATERIALIZED VIEW mv_metadata_by_mint_from_update_authority
TO metadata AS
SELECT v.mint, v.metadata as metadata, v.update_authority, v.mint_authority, v.name, v.symbol, v.uri
FROM (SELECT DISTINCT metadata FROM metadata_update_authority_state_latest) AS changed
INNER JOIN metadata_view AS v ON v.metadata = changed.metadata;

-- 5) MV: refresh on mint authority changes
CREATE MATERIALIZED VIEW mv_metadata_by_mint_from_mint_authority
TO metadata AS
SELECT v.mint, v.metadata as metadata, v.update_authority, v.mint_authority, v.name, v.symbol, v.uri
FROM (SELECT DISTINCT metadata FROM metadata_mint_authority_state_latest) AS changed
INNER JOIN metadata_view AS v ON v.metadata = changed.metadata;
