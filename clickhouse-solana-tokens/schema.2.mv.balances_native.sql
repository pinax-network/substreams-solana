-- Native Token Balances --
CREATE TABLE IF NOT EXISTS balances_native (
    -- block --
    block_num       UInt32,
    timestamp       DateTime(0, 'UTC'),

    -- balance --
    account         String,
    lamports        UInt64,

    -- indexes --
    INDEX idx_lamports (lamports) TYPE minmax GRANULARITY 1
)
ENGINE = ReplacingMergeTree(block_num)
ORDER BY (account)
COMMENT 'Native SOL balances (single balance per-block per-account)';

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_system_post_balances
TO balances_native AS
SELECT
    block_num,
    timestamp,
    account,
    amount as lamports
FROM system_post_balances;