-- SPL Token Balances --
CREATE TABLE IF NOT EXISTS spl_balances (
    block_num       UInt32,
    program_id      FixedString(44),
    account         FixedString(44),
    amount          UInt64,
    mint            FixedString(44),
    decimals        UInt8,

    -- indexes --
    INDEX idx_account (account) TYPE bloom_filter(0.005) GRANULARITY 1,
    INDEX idx_amount (amount) TYPE minmax GRANULARITY 1

)
ENGINE = ReplacingMergeTree(block_num)
ORDER BY (program_id, mint, account)
COMMENT 'SPL token balances (SPL tokens, not native SOL)';

ALTER TABLE spl_balances MODIFY SETTING deduplicate_merge_projection_mode = 'drop';
ALTER TABLE spl_balances
    ADD PROJECTION IF NOT EXISTS prj_amount (SELECT * ORDER BY (amount, account)),
    ADD PROJECTION IF NOT EXISTS prj_account (SELECT * ORDER BY (account));

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_post_token_balances
TO spl_balances AS
SELECT
    block_num,
    program_id,
    account,
    amount,
    mint,
    decimals
FROM post_token_balances;

-- Native SOL Balances --
CREATE TABLE IF NOT EXISTS native_balances (
    block_num       UInt32,
    program_id      FixedString(44) MATERIALIZED '11111111111111111111111111111111',
    account         FixedString(44),
    amount          UInt64,
    mint            FixedString(44) MATERIALIZED 'So11111111111111111111111111111111111111111',
    decimals        UInt8 MATERIALIZED 9,

    -- indexes --
    INDEX idx_account (account) TYPE bloom_filter(0.005) GRANULARITY 1,
    INDEX idx_amount (amount) TYPE minmax GRANULARITY 1
)
ENGINE = ReplacingMergeTree(block_num)
ORDER BY (account)
COMMENT 'Native SOL balances (lamports, not SPL tokens)';

ALTER TABLE native_balances MODIFY SETTING deduplicate_merge_projection_mode = 'drop';
ALTER TABLE native_balances
    ADD PROJECTION IF NOT EXISTS prj_amount (SELECT * ORDER BY (amount, account));

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_system_post_balances
TO native_balances AS
SELECT
    block_num,
    account,
    amount
FROM system_post_balances;

-- All Balances --
CREATE TABLE IF NOT EXISTS balances AS spl_balances
COMMENT 'All token balances (SPL and native SOL)';

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_spl_balances
TO balances AS
SELECT *
FROM spl_balances;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_native_balances
TO balances AS
SELECT
    *,
    program_id,
    mint,
    decimals
FROM native_balances;