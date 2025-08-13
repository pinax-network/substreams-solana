CREATE TABLE IF NOT EXISTS mints (
    -- block --
    block_num           UInt32,
    timestamp           DateTime(0, 'UTC'),
    version             UInt64,

    -- mint --
    mint              LowCardinality(String),
    mint_authority    String,
    freeze_authority  Nullable(String),
    decimals          UInt8,

    -- indexes --
    INDEX idx_mint_authority (mint_authority) TYPE bloom_filter(0.005) GRANULARITY 1,
    INDEX idx_freeze_authority (freeze_authority) TYPE bloom_filter(0.005) GRANULARITY 1,
    INDEX idx_decimals (decimals) TYPE minmax GRANULARITY 1

) ENGINE = ReplacingMergeTree(version)
ORDER BY mint
COMMENT 'SPL Token Mints';

ALTER TABLE mints MODIFY SETTING deduplicate_merge_projection_mode = 'rebuild';
ALTER TABLE mints
    ADD PROJECTION IF NOT EXISTS prj_mint_authority (SELECT * ORDER BY mint_authority),
    ADD PROJECTION IF NOT EXISTS prj_freeze_authority (SELECT * ORDER BY freeze_authority);

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_initialize_mint
TO mints AS
SELECT
    block_num,
    timestamp,
    version,
    mint,
    mint_authority,
    freeze_authority,
    decimals
FROM initialize_mint;