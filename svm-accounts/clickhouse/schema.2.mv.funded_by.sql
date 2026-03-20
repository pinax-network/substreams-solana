-- FUNDED BY (who funded the account creation)
CREATE TABLE IF NOT EXISTS funded_by_state AS TEMPLATE_ACCOUNTS_STATE;
ALTER TABLE funded_by_state
    ADD COLUMN IF NOT EXISTS funded_by String COMMENT 'Account that funded the creation',
    ADD COLUMN IF NOT EXISTS space UInt64 DEFAULT 0 COMMENT 'Allocated data size in bytes',
    ADD COLUMN IF NOT EXISTS lamports UInt64 DEFAULT 0 COMMENT 'Initial balance in lamports',
    MODIFY COLUMN is_deleted UInt8 MATERIALIZED if(funded_by = '', 1, 0),
    ADD PROJECTION IF NOT EXISTS prj_funded_by (SELECT * ORDER BY (funded_by, account));

-- system_create_account → funded_by_state
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_funded_by_state_create_account
TO funded_by_state AS
SELECT
    program_id,
    new_account AS account,
    source AS funded_by,
    space,
    lamports,
    version,
    block_num,
    timestamp
FROM system_create_account;

-- system_create_account_with_seed → funded_by_state
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_funded_by_state_create_account_with_seed
TO funded_by_state AS
SELECT
    program_id,
    new_account AS account,
    source AS funded_by,
    space,
    lamports,
    version,
    block_num,
    timestamp
FROM system_create_account_with_seed;
