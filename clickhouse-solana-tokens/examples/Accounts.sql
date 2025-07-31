-- Current open accounts (no FINAL)
SELECT
  account,
  argMax(owner,  version) AS owner,
  argMax(mint,   version) AS mint
FROM accounts
GROUP BY account
HAVING sum(sign) = 1;  -- open == net +1

-- Using views --
SELECT * FROM accounts_current LIMIT 10;

-- Query `accounts` by `owner`
EXPLAIN indexes = 1
SELECT
    account,
    argMax(mint, version) AS mint
FROM accounts
WHERE owner = '8vFajALwc4r79zXRQqVt9QkwhXARV4QBkjzf526QVSdr'
GROUP BY account
HAVING sum(sign) = 1