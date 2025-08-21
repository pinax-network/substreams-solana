-- OWNER
CREATE TABLE IF NOT EXISTS owner_state_latest (
    account    String,
    owner      String,
    version    UInt64,
    block_num  UInt32,
    timestamp  DateTime('UTC')
)
ENGINE = ReplacingMergeTree(version)
ORDER BY (account);

ALTER TABLE owner_state_latest MODIFY SETTING deduplicate_merge_projection_mode = 'rebuild';
ALTER TABLE owner_state_latest
    ADD PROJECTION IF NOT EXISTS prj_owner (SELECT * ORDER BY owner);

-- MINT
CREATE TABLE IF NOT EXISTS mint_state_latest (
    account    String,
    mint       LowCardinality(String),
    version    UInt64,
    block_num  UInt32,
    timestamp  DateTime('UTC')
)
ENGINE = ReplacingMergeTree(version)
ORDER BY (account);

-- CLOSED (0/1)
CREATE TABLE IF NOT EXISTS closed_state_latest (
    account    String,
    closed     UInt8,
    version    UInt64,
    block_num  UInt32,
    timestamp  DateTime('UTC')
)
ENGINE = ReplacingMergeTree(version)
ORDER BY (account);

-- FROZEN (0/1)
CREATE TABLE IF NOT EXISTS frozen_state_latest (
    account    String,
    frozen     UInt8,
    version    UInt64,
    block_num  UInt32,
    timestamp  DateTime('UTC')
)
ENGINE = ReplacingMergeTree(version)
ORDER BY (account);

-- INITIALIZE
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_initialize_owner
TO owner_state_latest AS
SELECT
  account,
  owner,
  version,
  block_num,
  timestamp
FROM initialize_account
WHERE owner IS NOT NULL;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_initialize_mint
TO mint_state_latest AS
SELECT
  account,
  mint,
  version,
  block_num,
  timestamp
FROM initialize_account
WHERE mint IS NOT NULL;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_initialize_closed0
TO closed_state_latest AS
SELECT
  account,
  0 as closed,
  version,
  block_num,
  timestamp
FROM initialize_account;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_initialize_frozen0
TO frozen_state_latest AS
SELECT
  account,
  0 as frozen,
  version,
  block_num,
  timestamp
FROM initialize_account;

-- CLOSE -> closed = 1 (do not touch others)
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_close_account
TO closed_state_latest AS
SELECT
  account,
  1 as closed,
  version,
  block_num,
  timestamp
FROM close_account;

-- SET AUTHORITY (owner)
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_set_authority_owner
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
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_freeze_account
TO frozen_state_latest AS
SELECT
  account,
  1 as frozen,
  version,
  block_num,
  timestamp
FROM freeze_account;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_thaw_account
TO frozen_state_latest AS
SELECT
  account,
  0 as frozen,
  version,
  block_num,
  timestamp
FROM thaw_account;

