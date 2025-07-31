-- Current open accounts (no FINAL)
SELECT
  account,
  argMax(owner,  version) AS owner,
  argMax(mint,   version) AS mint
FROM accounts
GROUP BY account
HAVING sum(sign) = 1;  -- open == net +1
