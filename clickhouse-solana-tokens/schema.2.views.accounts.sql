CREATE TABLE IF NOT EXISTS accounts (
    -- block --
    block_num           UInt32,
    timestamp           DateTime(0, 'UTC'),
    version             UInt64,

    -- account/mint --
    account   String,
    owner     Nullable(String),
    mint      Nullable(String),
    closed    Nullable(UInt8) COMMENT '1 ⇒ account closed',
    frozen    Nullable(UInt8) COMMENT '1 ⇒ account frozen',

    INDEX idx_owner (owner) TYPE bloom_filter(0.005) GRANULARITY 1,
    INDEX idx_mint (mint) TYPE bloom_filter(0.005) GRANULARITY 1
)
ENGINE = CoalescingMergeTree
ORDER BY account
COMMENT 'SPL Token Accounts (one current row per open account)';

ALTER TABLE accounts MODIFY SETTING deduplicate_merge_projection_mode = 'rebuild';
ALTER TABLE accounts
    ADD PROJECTION IF NOT EXISTS prj_owner (SELECT * ORDER BY owner),
    ADD PROJECTION IF NOT EXISTS prj_mint (SELECT * ORDER BY mint);

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_initialize_account
TO accounts AS
SELECT
    block_num,
    timestamp,
    version,
    account,
    owner,
    mint,
    0            AS closed,
    0            AS frozen
FROM initialize_account;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_close_account
TO accounts AS
SELECT
    block_num,
    timestamp,
    version,
    account,
    Null::Nullable(String) AS owner,      -- keep previous owner
    Null::Nullable(String) AS mint,       -- keep previous mint
    1           AS closed,
    Null::Nullable(UInt8) AS frozen
FROM close_account;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_set_authority
TO accounts AS
SELECT
    block_num,
    timestamp,
    version,
    account,
    new_authority AS owner,    -- changed column
    Null::Nullable(String)          AS mint,     -- unchanged, let the engine reuse prior value
    0             AS closed,
    Null::Nullable(UInt8)            AS frozen
FROM set_authority
WHERE authority_type = 'AccountOwner';

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_thaw_account
TO accounts AS
SELECT
    block_num,
    timestamp,
    version,
    account,
    Null::Nullable(String) AS owner,
    Null::Nullable(String) AS mint,
    Null::Nullable(UInt8)  AS closed,
    0           AS frozen
FROM thaw_account;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_freeze_account
TO accounts AS
SELECT
    block_num,
    timestamp,
    version,
    account,
    Null::Nullable(String) AS owner,
    Null::Nullable(String) AS mint,
    Null::Nullable(UInt8)  AS closed,
    1 AS frozen
FROM freeze_account;
