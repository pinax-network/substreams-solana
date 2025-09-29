-- SetAuthority --
CREATE TABLE IF NOT EXISTS set_authority AS base_events
COMMENT 'SPL Token SetAuthority events';
ALTER TABLE set_authority
    ADD COLUMN IF NOT EXISTS account                 String,
    ADD COLUMN IF NOT EXISTS authority_type          LowCardinality(String), -- AuthorityType enum as string
    ADD COLUMN IF NOT EXISTS new_authority_raw       String,
    ADD COLUMN IF NOT EXISTS new_authority           Nullable(String) MATERIALIZED string_or_null(new_authority_raw),
    ADD COLUMN IF NOT EXISTS authority               String,
    ADD COLUMN IF NOT EXISTS multisig_authority_raw  String,
    ADD COLUMN IF NOT EXISTS multisig_authority      Array(String) MATERIALIZED string_to_array(multisig_authority_raw),

    -- Indexes --
    ADD INDEX IF NOT EXISTS idx_account (account) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_new_authority (new_authority) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_authority (authority) TYPE bloom_filter(0.005) GRANULARITY 1;

-- FREEZE AUTHORITY
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_set_authority_freeze_authority
TO freeze_authority_state_latest AS
SELECT
  account AS mint,
  new_authority_raw AS freeze_authority,
  version,
  block_num,
  timestamp
FROM set_authority
WHERE authority_type = 'AUTHORITY_TYPE_FREEZE_ACCOUNT';

-- MINT AUTHORITY
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_set_authority_mint_authority
TO mint_authority_state_latest AS
SELECT
  account AS mint,
  new_authority_raw AS mint_authority,
  version,
  block_num,
  timestamp
FROM set_authority
WHERE authority_type = 'AUTHORITY_TYPE_MINT_TOKENS';

-- OWNER (UPDATE AUTHORITY)
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_set_authority_owner
TO owner_state_latest AS
SELECT
  account,
  new_authority_raw AS owner,
  version,
  block_num,
  timestamp
FROM set_authority
WHERE authority_type = 'AUTHORITY_TYPE_ACCOUNT_OWNER';

-- CLOSE ACCOUNT
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_closed_state_set_authority_owner
TO owner_state_latest AS
SELECT
  account,
  '' as owner,
  version,
  block_num,
  timestamp
FROM set_authority
WHERE authority_type = 'AUTHORITY_TYPE_CLOSE_ACCOUNT';

-- CLOSE MINT
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_close_mint_state_set_authority_close_mint
TO close_mint_state_latest AS
SELECT
  account AS mint,
  1 AS closed,
  version,
  block_num,
  timestamp
FROM set_authority
WHERE authority_type = 'AUTHORITY_TYPE_CLOSE_MINT';

--     ┌─authority_type──────────────────────────────────┬───count()─┐
--  1. │ AUTHORITY_TYPE_ACCOUNT_OWNER                    │ 123645372 │
--  2. │ AUTHORITY_TYPE_CLOSE_ACCOUNT                    │ 110497166 │
--  3. │ AUTHORITY_TYPE_MINT_TOKENS                      │  66885099 │
--  4. │ AUTHORITY_TYPE_FREEZE_ACCOUNT                   │  41076484 │
--  5. │ AUTHORITY_TYPE_METADATA_POINTER                 │     32059 │
--  6. │ AUTHORITY_TYPE_PERMANENT_DELEGATE               │     15154 │
--  7. │ AUTHORITY_TYPE_CLOSE_MINT                       │     14368 │
--  8. │ AUTHORITY_TYPE_TRANSFER_FEE_CONFIG              │      8904 │
--  9. │ AUTHORITY_TYPE_WITHHELD_WITHDRAW                │      2800 │
-- 10. │ AUTHORITY_TYPE_TRANSFER_HOOK_PROGRAM_ID         │       186 │
-- 11. │ AUTHORITY_TYPE_CONFIDENTIAL_TRANSFER_MINT       │        67 │
-- 12. │ AUTHORITY_TYPE_CONFIDENTIAL_TRANSFER_FEE_CONFIG │        12 │
-- 13. │ AUTHORITY_TYPE_INTEREST_RATE                    │         9 │
--     └─────────────────────────────────────────────────┴───────────┘