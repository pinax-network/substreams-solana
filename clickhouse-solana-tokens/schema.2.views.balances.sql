-- SPL Token Balances --
CREATE TABLE IF NOT EXISTS balances (
    block_num       UInt32,
    timestamp       DateTime(0, 'UTC'),
    program_id      LowCardinality(String),
    account         String,
    amount          UInt64,
    mint            LowCardinality(String),
    decimals        UInt8,

    -- indexes --
    INDEX idx_program_id (program_id) TYPE set(3) GRANULARITY 1, -- SPL Token, Token-2022 & Native SOL
    INDEX idx_account (account) TYPE bloom_filter(0.005) GRANULARITY 1,
    INDEX idx_amount (amount) TYPE minmax GRANULARITY 1,
    INDEX idx_decimals (decimals) TYPE minmax GRANULARITY 1
)
ENGINE = ReplacingMergeTree(block_num)
ORDER BY (program_id, mint, account)
COMMENT 'SPL Token & Native SOL balances';

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

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_system_post_balances
TO balances AS
SELECT
    block_num,
    timestamp,
    '11111111111111111111111111111111' AS program_id,
    account,
    amount,
    'So11111111111111111111111111111111111111111' AS mint,
    9 AS decimals
FROM system_post_balances;