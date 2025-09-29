CREATE TABLE IF NOT EXISTS TEMPLATE_MINTS_STATE (
    mint         String,
    version      UInt64,
    is_deleted   UInt8,
    block_num    UInt32,
    timestamp    DateTime(0, 'UTC'),

    -- indexes --
    INDEX idx_block_num (block_num) TYPE minmax GRANULARITY 1,
    INDEX idx_timestamp (timestamp) TYPE minmax GRANULARITY 1,
    INDEX idx_is_deleted (is_deleted) TYPE set(2) GRANULARITY 1
)
ENGINE = ReplacingMergeTree(version, is_deleted)
ORDER BY (mint);

-- TTL to clean up deleted rows after 0 seconds (immediate cleanup on merge)
ALTER TABLE TEMPLATE_MINTS_STATE
  MODIFY SETTING allow_experimental_replacing_merge_with_cleanup = 1;
ALTER TABLE TEMPLATE_MINTS_STATE
  MODIFY SETTING deduplicate_merge_projection_mode = 'rebuild';

-- DECIMALS
CREATE TABLE IF NOT EXISTS decimals_state_latest AS TEMPLATE_MINTS_STATE;
ALTER TABLE decimals_state_latest
    ADD COLUMN IF NOT EXISTS decimals UInt8,
    ADD INDEX IF NOT EXISTS idx_decimals (decimals) TYPE minmax GRANULARITY 1;

-- MINT AUTHORITY
CREATE TABLE IF NOT EXISTS mint_authority_state_latest AS TEMPLATE_MINTS_STATE;
ALTER TABLE mint_authority_state_latest
    ADD COLUMN IF NOT EXISTS mint_authority String,
    MODIFY COLUMN is_deleted UInt8 MATERIALIZED if(mint_authority = '', 1, 0),
    ADD PROJECTION IF NOT EXISTS prj_mint_authority (SELECT * ORDER BY (mint_authority, mint));

-- FREEZE AUTHORITY
CREATE TABLE IF NOT EXISTS freeze_authority_state_latest AS TEMPLATE_MINTS_STATE;
ALTER TABLE freeze_authority_state_latest
    ADD COLUMN IF NOT EXISTS freeze_authority String,
    MODIFY COLUMN is_deleted UInt8 MATERIALIZED if(freeze_authority = '', 1, 0),
    ADD PROJECTION IF NOT EXISTS prj_freeze_authority (SELECT * ORDER BY (freeze_authority, mint));

-- CLOSED MINT (0/1)
CREATE TABLE IF NOT EXISTS close_mint_state_latest AS TEMPLATE_MINTS_STATE;
ALTER TABLE close_mint_state_latest
    ADD COLUMN IF NOT EXISTS closed UInt8,
    MODIFY COLUMN is_deleted UInt8 MATERIALIZED if(closed = 0, 1, 0);

-- INITIALIZE
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_decimals_state_initialize_mint
TO decimals_state_latest AS
SELECT
  mint,
  decimals,
  version,
  block_num,
  timestamp
FROM initialize_mint;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_mint_authority_state_initialize_mint
TO mint_authority_state_latest AS
SELECT
  mint,
  mint_authority,
  version,
  block_num,
  timestamp
FROM initialize_mint;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_freeze_authority_state_initialize_mint
TO freeze_authority_state_latest AS
SELECT
  mint,
  freeze_authority,
  version,
  block_num,
  timestamp
FROM initialize_mint;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_close_mint_state_initialize_closed0
TO close_mint_state_latest AS
SELECT
  mint,
  0 as closed,
  version,
  block_num,
  timestamp
FROM initialize_mint;
