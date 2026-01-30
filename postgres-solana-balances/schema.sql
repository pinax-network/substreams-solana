-- SPL Token balances table for PostgreSQL
-- There can only be a single SPL Token balance per account / mint pair (latest balance)
CREATE TABLE IF NOT EXISTS spl_token_balances (
    -- block --
    block_num            INTEGER NOT NULL,
    block_hash           TEXT NOT NULL,
    timestamp            TIMESTAMP NOT NULL,

    -- balance --
    mint                 TEXT NOT NULL,
    account              TEXT NOT NULL,
    owner                TEXT NOT NULL,
    balance              NUMERIC NOT NULL,

    PRIMARY KEY (mint, account)
);

CREATE INDEX IF NOT EXISTS idx_spl_token_balances_account ON spl_token_balances (account);
CREATE INDEX IF NOT EXISTS idx_spl_token_balances_owner ON spl_token_balances (owner);
CREATE INDEX IF NOT EXISTS idx_spl_token_balances_block_num ON spl_token_balances (block_num);
CREATE INDEX IF NOT EXISTS idx_spl_token_balances_balance ON spl_token_balances (balance);


-- Native SOL balances table for PostgreSQL
-- There can only be a single native SOL balance per address (latest balance)
CREATE TABLE IF NOT EXISTS native_balances (
    -- block --
    block_num            INTEGER NOT NULL,
    block_hash           TEXT NOT NULL,
    timestamp            TIMESTAMP NOT NULL,

    -- balance --
    address              TEXT PRIMARY KEY,
    balance              NUMERIC NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_native_balances_block_num ON native_balances (block_num);
CREATE INDEX IF NOT EXISTS idx_native_balances_balance ON native_balances (balance);
