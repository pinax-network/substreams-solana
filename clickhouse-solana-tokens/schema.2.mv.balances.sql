-- SPL Token Balances --
CREATE TABLE IF NOT EXISTS balances (
    -- block --
    block_num       UInt32,
    timestamp       DateTime(0, 'UTC'),

    -- balance --
    program_id      LowCardinality(String),
    account         String,
    amount          UInt64,
    mint            Nullable(String),
    decimals        Nullable(UInt8),

    -- indexes --
    INDEX idx_program_id (program_id) TYPE set(2) GRANULARITY 1,
    INDEX idx_amount (amount) TYPE minmax GRANULARITY 1,
    INDEX idx_mint (mint) TYPE bloom_filter(0.005) GRANULARITY 1,
    INDEX idx_decimals (decimals) TYPE minmax GRANULARITY 1
)
ENGINE = ReplacingMergeTree(block_num)
ORDER BY (account)
COMMENT 'SPL Token balances (single balance per-block per-account/mint)';

ALTER TABLE balances MODIFY SETTING deduplicate_merge_projection_mode = 'rebuild';
ALTER TABLE balances
    ADD PROJECTION IF NOT EXISTS prj_mint (SELECT * ORDER BY (mint, account));

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_post_token_balances
TO balances AS
SELECT
    block_num,
    timestamp,
    program_id,
    account,
    amount,
    mint,
    decimals
FROM post_token_balances;

-- Set account to 0 balance on CloseAccount --
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_close_account_balances
TO balances AS
SELECT
    block_num,
    timestamp,
    program_id,
    account,
    0 AS amount,
    CAST(NULL AS Nullable(String)) AS mint,
    CAST(NULL AS Nullable(UInt8)) AS decimals
FROM close_account;