CREATE TABLE IF NOT EXISTS accounts (
    block_num       UInt32,
    is_closed       Boolean DEFAULT false COMMENT 'true = closed, false = open',
    account         String,
    owner           String,
    mint            LowCardinality(String),

    -- indexes --
    INDEX idx_owner (owner) TYPE bloom_filter(0.005) GRANULARITY 1,
    INDEX idx_account (account) TYPE bloom_filter(0.005) GRANULARITY 1,
    INDEX idx_block_num (block_num) TYPE minmax GRANULARITY 1,
    INDEX idx_is_closed (is_closed) TYPE minmax GRANULARITY 1

) ENGINE = ReplacingMergeTree(block_num)
ORDER BY (mint, account)
COMMENT 'SPL Token Accounts';

ALTER TABLE accounts MODIFY SETTING deduplicate_merge_projection_mode = 'rebuild';
ALTER TABLE accounts
    ADD PROJECTION IF NOT EXISTS prj_owner (SELECT * ORDER BY owner),
    ADD PROJECTION IF NOT EXISTS prj_account (SELECT * ORDER BY account);

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_initialize_account
TO accounts AS
SELECT
    block_num,
    false as is_closed,
    account,
    owner,
    mint
FROM initialize_account;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_initialize_immutable_owner
TO accounts AS
SELECT
    block_num,
    false as is_closed,
    account,
    account AS owner
FROM initialize_immutable_owner;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_close_account
TO accounts AS
SELECT
    block_num,
    true as is_closed,
    account,
    destination AS owner
FROM close_account;