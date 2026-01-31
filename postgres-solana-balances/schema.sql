-- SPL Token balances table for PostgreSQL
-- There can only be a single SPL Token balance per account / mint pair (latest balance)
CREATE TABLE IF NOT EXISTS balances (
    -- block --
    block_num            INTEGER NOT NULL,
    block_hash           TEXT NOT NULL,
    timestamp            TIMESTAMP NOT NULL,

    -- balance --
    program_id           TEXT NOT NULL,
    mint                 TEXT NOT NULL,
    account              TEXT NOT NULL,
    amount               NUMERIC NOT NULL,
    decimals             SMALLINT NOT NULL,

    PRIMARY KEY (mint, account)
);

-- Block indexes
CREATE INDEX IF NOT EXISTS idx_balances_block_num ON balances (block_num);
CREATE INDEX IF NOT EXISTS idx_balances_timestamp ON balances (timestamp);

-- Single column indexes
CREATE INDEX IF NOT EXISTS idx_balances_program_id ON balances (program_id);
CREATE INDEX IF NOT EXISTS idx_balances_account ON balances (account);
CREATE INDEX IF NOT EXISTS idx_balances_amount ON balances (amount);

-- Composite indexes (non-zero balances only)
CREATE INDEX IF NOT EXISTS idx_balances_nonzero ON balances (mint, account) WHERE amount != 0;
CREATE INDEX IF NOT EXISTS idx_balances_account_mint ON balances (account, mint) WHERE amount != 0;

-- Sorted indexes for top/bottom balances per mint
CREATE INDEX IF NOT EXISTS idx_balances_mint_amount_desc ON balances (mint, amount DESC) WHERE amount != 0;
CREATE INDEX IF NOT EXISTS idx_balances_mint_amount_asc ON balances (mint, amount ASC) WHERE amount != 0;


-- Native SOL balances table for PostgreSQL
-- There can only be a single native SOL balance per account (latest balance)
CREATE TABLE IF NOT EXISTS balances_native (
    -- block --
    block_num            INTEGER NOT NULL,
    block_hash           TEXT NOT NULL,
    timestamp            TIMESTAMP NOT NULL,

    -- balance --
    account              TEXT PRIMARY KEY,
    amount               NUMERIC NOT NULL
);

-- Block indexes
CREATE INDEX IF NOT EXISTS idx_balances_native_block_num ON balances_native (block_num);
CREATE INDEX IF NOT EXISTS idx_balances_native_timestamp ON balances_native (timestamp);

-- Single column indexes
CREATE INDEX IF NOT EXISTS idx_balances_native_amount ON balances_native (amount);

-- Partial indexes (non-zero balances only)
CREATE INDEX IF NOT EXISTS idx_balances_native_nonzero ON balances_native (account) WHERE amount != 0;

-- Sorted indexes for top/bottom balances
CREATE INDEX IF NOT EXISTS idx_balances_native_amount_desc ON balances_native (amount DESC) WHERE amount != 0;
CREATE INDEX IF NOT EXISTS idx_balances_native_amount_asc ON balances_native (amount ASC) WHERE amount != 0;
