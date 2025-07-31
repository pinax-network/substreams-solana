CREATE TABLE IF NOT EXISTS accounts (
    version         UInt64,
    sign            Int8 COMMENT '-1 = closed, +1 = open',
    account         String,
    mint            LowCardinality(String),
    owner           String,

    -- indexes --
    INDEX idx_owner (owner) TYPE bloom_filter(0.005) GRANULARITY 1,
    INDEX idx_mint (mint) TYPE bloom_filter(0.005) GRANULARITY 1
) ENGINE = VersionedCollapsingMergeTree(sign, version)
ORDER BY account
COMMENT 'SPL Token Accounts (one current row per open account)';

ALTER TABLE accounts MODIFY SETTING deduplicate_merge_projection_mode = 'rebuild';
ALTER TABLE accounts
    ADD PROJECTION IF NOT EXISTS prj_owner (SELECT * ORDER BY owner),
    ADD PROJECTION IF NOT EXISTS prj_mint (SELECT * ORDER BY mint);

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_initialize_account
TO accounts AS
SELECT
    to_version(block_num, transaction_index, instruction_index) AS version,
    1 AS sign,
    account,
    owner,
    mint
FROM initialize_account;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_close_account
TO accounts AS
SELECT
    to_version(block_num, transaction_index, instruction_index) AS version,
    -1 AS sign,
    account,
    '' AS mint,
    '' AS owner
FROM close_account;