CREATE TABLE IF NOT EXISTS mints (
    block_num           UInt32,
    mint                FixedString(44),
    mint_authority      FixedString(44),
    freeze_authority    Nullable(FixedString(44)),
    decimals            UInt8,

    -- indexes --
    INDEX idx_mint_authority (mint_authority) TYPE bloom_filter(0.005) GRANULARITY 1,
    INDEX idx_freeze_authority (freeze_authority) TYPE bloom_filter(0.005) GRANULARITY 1,
    INDEX idx_decimals (decimals) TYPE minmax GRANULARITY 1,
    INDEX idx_block_num (block_num) TYPE minmax GRANULARITY 1

) ENGINE = ReplacingMergeTree(block_num)
ORDER BY (mint, mint_authority)
COMMENT 'SPL Token Mints';

ALTER TABLE mints MODIFY SETTING deduplicate_merge_projection_mode = 'rebuild';
ALTER TABLE mints
    ADD PROJECTION IF NOT EXISTS prj_mint_authority (SELECT * ORDER BY mint_authority),
    ADD PROJECTION IF NOT EXISTS prj_freeze_authority (SELECT * ORDER BY freeze_authority);

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_initialize_mint
TO mints AS
SELECT
    block_num,
    mint,
    mint_authority,
    freeze_authority,
    decimals
FROM initialize_mint;