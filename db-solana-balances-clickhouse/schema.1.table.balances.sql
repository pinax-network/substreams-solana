-- SPL Token Post Balance --
CREATE TABLE IF NOT EXISTS post_token_balances AS base_transactions
COMMENT 'SPL Token Post Balance events (only last transaction in block which effects the balance)';
ALTER TABLE post_token_balances
    ADD COLUMN IF NOT EXISTS program_id         LowCardinality(String) COMMENT 'Program ID of the SPL Token program.',
    ADD COLUMN IF NOT EXISTS account            String COMMENT 'Account address.',
    ADD COLUMN IF NOT EXISTS mint               String COMMENT 'Mint address',
    ADD COLUMN IF NOT EXISTS amount             UInt64 COMMENT 'Balance amount in lamports.',
    ADD COLUMN IF NOT EXISTS decimals           UInt8,

    -- Indexes --
    ADD INDEX IF NOT EXISTS idx_account (account) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_mint (mint) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_amount (amount) TYPE minmax GRANULARITY 1;

-- System Post Balance --
CREATE TABLE IF NOT EXISTS system_post_balances AS base_transactions
COMMENT 'System post balances (only last transaction in block which effects the balance)';
ALTER TABLE system_post_balances
    ADD COLUMN IF NOT EXISTS account                  String COMMENT 'Account address.',
    ADD COLUMN IF NOT EXISTS amount                   UInt64 COMMENT 'Balance amount in lamports.',
    -- Indexes --
    ADD INDEX IF NOT EXISTS idx_account (account) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_amount (amount) TYPE minmax GRANULARITY 1;
