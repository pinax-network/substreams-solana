-- SPL Token Balances --
CREATE TABLE IF NOT EXISTS balances (
    -- block --
    block_num       UInt32,
    timestamp       DateTime(0, 'UTC'),

    -- balance --
    program_id      LowCardinality(String),
    account         String,
    amount          UInt64,
    mint            LowCardinality(String),
    decimals        UInt8,

    -- indexes --
    INDEX idx_program_id (program_id) TYPE set(2) GRANULARITY 1,
    INDEX idx_amount (amount) TYPE minmax GRANULARITY 1,
    INDEX idx_decimals (decimals) TYPE minmax GRANULARITY 1
)
ENGINE = ReplacingMergeTree(block_num)
ORDER BY (mint, account)
COMMENT 'SPL Token balances (single balance per-block per-account/mint)';

ALTER TABLE balances MODIFY SETTING deduplicate_merge_projection_mode = 'rebuild';
ALTER TABLE balances
    ADD PROJECTION IF NOT EXISTS prj_account (SELECT * ORDER BY (account));

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