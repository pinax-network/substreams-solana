-- latest balances by owner/mint --
CREATE TABLE IF NOT EXISTS balances AS balance_changes
ENGINE = ReplacingMergeTree(block_num)
ORDER BY (owner, mint);

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_balances
TO balances AS
SELECT * FROM balance_changes;

-- latest balances by mint/owner --
CREATE TABLE IF NOT EXISTS balances_by_mint AS balance_changes
ENGINE = ReplacingMergeTree(block_num)
ORDER BY (mint, owner);

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_balances_by_mint
TO balances AS
SELECT * FROM balance_changes;