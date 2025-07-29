CREATE TABLE IF NOT EXISTS accounts (
    block_num       UInt32,
    is_closed       Boolean DEFAULT false COMMENT 'true = closed, false = open',
    account         FixedString(44),
    owner           FixedString(44),
    mint            FixedString(44)
) ENGINE = ReplacingMergeTree(block_num)
ORDER BY (mint, account)
COMMENT 'Solana Accounts, used by SPL Tokens';

ALTER TABLE accounts MODIFY SETTING deduplicate_merge_projection_mode = 'drop';
ALTER TABLE accounts ADD PROJECTION IF NOT EXISTS prj_owner (SELECT * ORDER BY owner);

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