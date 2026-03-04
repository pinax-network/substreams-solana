-- SetAuthority MV mappings --

-- FREEZE AUTHORITY
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_set_authority_freeze_authority
TO freeze_authority_state AS
SELECT
  program_id,
  account AS mint,
  new_authority_raw AS authority,
  version,
  block_num,
  timestamp
FROM set_authority
WHERE authority_type = 'AUTHORITY_TYPE_FREEZE_ACCOUNT';

-- MINT AUTHORITY
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_set_authority_mint_authority
TO mint_authority_state AS
SELECT
  program_id,
  account AS mint,
  new_authority_raw AS authority,
  version,
  block_num,
  timestamp
FROM set_authority
WHERE authority_type = 'AUTHORITY_TYPE_MINT_TOKENS';

-- OWNER (UPDATE AUTHORITY)
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_set_authority_owner
TO owner_state AS
SELECT
  program_id,
  account,
  new_authority_raw AS owner,
  version,
  block_num,
  timestamp
FROM set_authority
WHERE authority_type = 'AUTHORITY_TYPE_ACCOUNT_OWNER';

-- CLOSE ACCOUNT
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_set_close_account_authority
TO close_account_authority_state AS
SELECT
  program_id,
  account,
  new_authority_raw AS authority,
  version,
  block_num,
  timestamp
FROM set_authority
WHERE authority_type = 'AUTHORITY_TYPE_CLOSE_ACCOUNT';

-- CLOSE MINT
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_set_close_mint_authority
TO close_mint_authority_state AS
SELECT
  program_id,
  account AS mint,
  new_authority_raw AS authority,
  version,
  block_num,
  timestamp
FROM set_authority
WHERE authority_type = 'AUTHORITY_TYPE_CLOSE_MINT';
