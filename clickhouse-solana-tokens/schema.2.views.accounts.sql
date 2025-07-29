CREATE TABLE IF NOT EXISTS accounts (
    block_num       UInt32,
    sign            Int8,      -- +1 = open, -1 = close
    account         String,
    owner           String,
    mint            String
) ENGINE = VersionedCollapsingMergeTree(block_num, sign)
ORDER BY (account, mint)
COMMENT 'Solana Accounts, used by SPL Tokens';

ALTER TABLE accounts ADD PROJECTION IF NOT EXISTS prj_owner (SELECT * ORDER BY owner);

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_initialize_account
TO accounts AS
SELECT
    block_num,
    1 AS sign,  -- +1 = open
    account,
    owner,
    mint
FROM initialize_account;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_initialize_immutable_owner
TO accounts AS
SELECT
    block_num,
    1 AS sign,  -- +1 = open
    account,
    account AS owner
FROM initialize_immutable_owner;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_close_account
TO accounts AS
SELECT
    block_num,
    -1 AS sign,  -- -1 = close
    account,
    owner
FROM close_account;