CREATE TABLE IF NOT EXISTS metadata (
    -- block --
    block_num               UInt32,
    timestamp               DateTime(0, 'UTC'),
    version                 UInt64,

    -- metadata --
    metadata                String,
    update_authority        Nullable(String),
    mint                    Nullable(String) COMMENT 'typically same as metadata',
    mint_authority          Nullable(String),
    name                    Nullable(String),
    symbol                  Nullable(String),
    uri                     Nullable(String),

    -- indexes --
    INDEX idx_block_num (block_num) TYPE minmax GRANULARITY 1,
    INDEX idx_mint (mint) TYPE bloom_filter(0.005) GRANULARITY 1,
    INDEX idx_mint_authority (mint_authority) TYPE bloom_filter(0.005) GRANULARITY 1,
    INDEX idx_update_authority (update_authority) TYPE bloom_filter(0.005) GRANULARITY 1,
    INDEX idx_metadata (metadata) TYPE bloom_filter(0.005) GRANULARITY 1,
    INDEX idx_name (name) TYPE bloom_filter(0.005) GRANULARITY 1,
    INDEX idx_symbol (symbol) TYPE bloom_filter(0.005) GRANULARITY 1,
    INDEX idx_uri (uri) TYPE bloom_filter(0.005) GRANULARITY 1

) ENGINE = CoalescingMergeTree
ORDER BY metadata
COMMENT 'SPL Token Metadata';

ALTER TABLE metadata MODIFY SETTING deduplicate_merge_projection_mode = 'rebuild';
ALTER TABLE metadata
    ADD PROJECTION IF NOT EXISTS prj_mint (SELECT * ORDER BY mint),
    ADD PROJECTION IF NOT EXISTS prj_mint_authority (SELECT * ORDER BY mint_authority),
    ADD PROJECTION IF NOT EXISTS prj_update_authority (SELECT * ORDER BY update_authority),
    ADD PROJECTION IF NOT EXISTS prj_name (SELECT * ORDER BY name),
    ADD PROJECTION IF NOT EXISTS prj_symbol (SELECT * ORDER BY symbol),
    ADD PROJECTION IF NOT EXISTS prj_uri (SELECT * ORDER BY uri);

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_initialize_token_metadata
TO metadata AS
SELECT
    block_num,
    timestamp,
    version,
    metadata,
    update_authority,
    mint,
    mint_authority,
    name,
    symbol,
    uri
FROM initialize_token_metadata;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_update_token_metadata_authority
TO metadata AS
SELECT
    block_num,
    timestamp,
    version,
    metadata,
    update_authority,
    Null::Nullable(String) AS mint,
    Null::Nullable(String) AS mint_authority,
    Null::Nullable(String) AS name,
    Null::Nullable(String) AS symbol,
    Null::Nullable(String) AS uri
FROM update_token_metadata_authority;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_update_token_metadata_field
TO metadata AS
SELECT
    block_num,
    timestamp,
    version,
    metadata,
    Null::Nullable(String) AS update_authority,
    Null::Nullable(String) AS mint,
    Null::Nullable(String) AS mint_authority,
    CASE field
        WHEN 'name' THEN value
        ELSE Null::Nullable(String)
    END AS name,
    CASE field
        WHEN 'symbol' THEN value
        ELSE Null::Nullable(String)
    END AS symbol,
    CASE field
        WHEN 'uri' THEN value
        ELSE Null::Nullable(String)
    END AS uri
FROM update_token_metadata_field
WHERE field IN ('name', 'symbol', 'uri');

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_remove_token_metadata_field
TO metadata AS
SELECT
    block_num,
    timestamp,
    version,
    metadata,
    Null::Nullable(String) AS update_authority,
    Null::Nullable(String) AS mint,
    Null::Nullable(String) AS mint_authority,
    CASE `key`
        WHEN 'name' THEN ''
        ELSE Null::Nullable(String)
    END AS name,
    CASE `key`
        WHEN 'symbol' THEN ''
        ELSE Null::Nullable(String)
    END AS symbol,
    CASE `key`
        WHEN 'uri' THEN ''
        ELSE Null::Nullable(String)
    END AS uri
FROM remove_token_metadata_field
WHERE `key` IN ('name', 'symbol', 'uri');