CREATE TABLE IF NOT EXISTS TEMPLATE_ACCOUNTS_STATE (
    account      String,
    version      UInt64,
    is_deleted   UInt8,
    block_num    UInt32,
    timestamp    DateTime('UTC'),

    -- indexes --
    INDEX idx_block_num (block_num) TYPE minmax GRANULARITY 1,
    INDEX idx_timestamp (timestamp) TYPE minmax GRANULARITY 1
)
ENGINE = ReplacingMergeTree(version, is_deleted)
ORDER BY (account);

ALTER TABLE TEMPLATE_ACCOUNTS_STATE
  MODIFY SETTING deduplicate_merge_projection_mode = 'rebuild';
ALTER TABLE TEMPLATE_ACCOUNTS_STATE
  MODIFY SETTING allow_experimental_replacing_merge_with_cleanup = 1;

-- OWNER
CREATE TABLE IF NOT EXISTS owner_state_latest AS TEMPLATE_ACCOUNTS_STATE;
ALTER TABLE owner_state_latest
    ADD COLUMN IF NOT EXISTS owner String,
    MODIFY COLUMN is_deleted UInt8 MATERIALIZED if(owner = '', 1, 0),
    ADD PROJECTION IF NOT EXISTS prj_owner (SELECT * ORDER BY owner);

-- MINT
CREATE TABLE IF NOT EXISTS mint_state_latest AS TEMPLATE_ACCOUNTS_STATE;
ALTER TABLE mint_state_latest
    ADD COLUMN IF NOT EXISTS mint LowCardinality(String),
    MODIFY COLUMN is_deleted UInt8 MATERIALIZED if(mint = '', 1, 0),
    ADD PROJECTION IF NOT EXISTS prj_mint (SELECT * ORDER BY (mint, account));

-- CLOSED (0/1)
CREATE TABLE IF NOT EXISTS closed_state_latest AS TEMPLATE_ACCOUNTS_STATE;
ALTER TABLE closed_state_latest
    ADD COLUMN IF NOT EXISTS closed UInt8,
    MODIFY COLUMN is_deleted UInt8 MATERIALIZED if(closed = 0, 1, 0),
    ADD PROJECTION IF NOT EXISTS prj_closed (SELECT * ORDER BY (closed, account));

-- FROZEN (0/1)
CREATE TABLE IF NOT EXISTS frozen_state_latest AS TEMPLATE_ACCOUNTS_STATE;
ALTER TABLE frozen_state_latest
    ADD COLUMN IF NOT EXISTS frozen UInt8,
    MODIFY COLUMN is_deleted UInt8 MATERIALIZED if(frozen = 0, 1, 0),
    ADD PROJECTION IF NOT EXISTS prj_frozen (SELECT * ORDER BY (frozen, account));

-- INITIALIZE
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_owner_state_initialize_owner
TO owner_state_latest AS
SELECT
  account,
  owner,
  version,
  block_num,
  timestamp
FROM initialize_account;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_mint_state_initialize_mint
TO mint_state_latest AS
SELECT
  account,
  mint,
  version,
  block_num,
  timestamp
FROM initialize_account;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_closed_state_initialize_closed0
TO closed_state_latest AS
SELECT
  account,
  0 as closed,
  version,
  block_num,
  timestamp
FROM initialize_account;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_frozen_state_initialize_frozen0
TO frozen_state_latest AS
SELECT
  account,
  0 as frozen,
  version,
  block_num,
  timestamp
FROM initialize_account;

-- CLOSE -> closed = 1 (do not touch others)
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_closed_state_close_account
TO closed_state_latest AS
SELECT
  account,
  1 as closed,
  version,
  block_num,
  timestamp
FROM close_account;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_closed_state_close_account_owner
TO owner_state_latest AS
SELECT
  account,
  '' as owner,
  version,
  block_num,
  timestamp
FROM close_account;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_closed_state_close_account_mint
TO mint_state_latest AS
SELECT
  account,
  '' as mint,
  version,
  block_num,
  timestamp
FROM close_account;

-- SET AUTHORITY (owner)
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_owner_state_set_authority_owner
TO owner_state_latest AS
SELECT
  account,
  new_authority as owner,
  version,
  block_num,
  timestamp
FROM set_authority
WHERE authority_type = 'AccountOwner' AND new_authority IS NOT NULL;

-- FREEZE / THAW
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_frozen_state_freeze_account
TO frozen_state_latest AS
SELECT
  account,
  1 as frozen,
  version,
  block_num,
  timestamp
FROM freeze_account;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_frozen_state_thaw_account
TO frozen_state_latest AS
SELECT
  account,
  0 as frozen,
  version,
  block_num,
  timestamp
FROM thaw_account;

