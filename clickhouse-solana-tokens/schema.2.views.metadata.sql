CREATE TABLE IF NOT EXISTS metadata (
    block_num               UInt32,
    metadata                String,
    update_authority        String,
    mint                    LowCardinality(String),
    mint_authority          String,
    name                    String,
    symbol                  LowCardinality(String),
    uri                     String,

    -- indexes --
    INDEX idx_block_num (block_num) TYPE minmax GRANULARITY 1,
    INDEX idx_mint_authority (mint_authority) TYPE bloom_filter(0.005) GRANULARITY 1,
    INDEX idx_update_authority (update_authority) TYPE bloom_filter(0.005) GRANULARITY 1,
    INDEX idx_metadata (metadata) TYPE bloom_filter(0.005) GRANULARITY 1,
    INDEX idx_name (name) TYPE bloom_filter(0.005) GRANULARITY 1,
    INDEX idx_symbol (symbol) TYPE bloom_filter(0.005) GRANULARITY 1,
    INDEX idx_uri (uri) TYPE bloom_filter(0.005) GRANULARITY 1

) ENGINE = ReplacingMergeTree(block_num)
ORDER BY (mint)
COMMENT 'SPL Token Metadata';

ALTER TABLE metadata MODIFY SETTING deduplicate_merge_projection_mode = 'rebuild';
ALTER TABLE metadata
    ADD PROJECTION IF NOT EXISTS prj_mint_authority (SELECT * ORDER BY mint_authority),
    ADD PROJECTION IF NOT EXISTS prj_update_authority (SELECT * ORDER BY update_authority),
    ADD PROJECTION IF NOT EXISTS prj_metadata (SELECT * ORDER BY metadata),
    ADD PROJECTION IF NOT EXISTS prj_name (SELECT * ORDER BY name),
    ADD PROJECTION IF NOT EXISTS prj_symbol (SELECT * ORDER BY symbol),
    ADD PROJECTION IF NOT EXISTS prj_uri (SELECT * ORDER BY uri);

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_initialize_token_metadata
TO metadata AS
SELECT
    block_num,
    metadata,
    update_authority,
    mint,
    mint_authority,
    name,
    symbol,
    uri
FROM initialize_token_metadata;
