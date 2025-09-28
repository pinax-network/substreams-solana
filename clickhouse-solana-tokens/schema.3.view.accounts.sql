/* OWNER (spine) */
CREATE OR REPLACE VIEW accounts_owner_view AS
SELECT
  account,
  max(version)   AS version,
  max(block_num) AS block_num,
  max(timestamp) AS timestamp,
  argMax(a.owner, a.version) AS owner
FROM owner_state_latest AS a
WHERE is_deleted = 0
GROUP BY account;

/* Optional fields kept as separate views (same pattern) */
CREATE OR REPLACE VIEW accounts_mint_view AS
SELECT
  account,
  max(version)   AS version,
  max(block_num) AS block_num,
  max(timestamp) AS timestamp,
  argMax(a.mint, a.version) AS mint
FROM mint_state_latest AS a
WHERE is_deleted = 0
GROUP BY account;

CREATE OR REPLACE VIEW accounts_closed_view AS
SELECT
  account,
  max(version)   AS version,
  max(block_num) AS block_num,
  max(timestamp) AS timestamp,
  argMax(a.closed, a.version) AS closed
FROM closed_state_latest AS a
WHERE is_deleted = 0
GROUP BY account;

CREATE OR REPLACE VIEW accounts_frozen_view AS
SELECT
  account,
  max(version)   AS version,
  max(block_num) AS block_num,
  max(timestamp) AS timestamp,
  argMax(a.frozen, a.version) AS frozen
FROM frozen_state_latest AS a
WHERE is_deleted = 0
GROUP BY account;

/* COMBINED VIEW — owner is required, others are optional */
CREATE OR REPLACE VIEW accounts_view AS
SELECT
  o.account,
  o.block_num,                 -- authoritative block_num from owner
  o.timestamp,                 -- authoritative timestamp from owner
  o.owner AS owner,
  if(empty(m.mint),  NULL, m.mint)  AS mint,
  c.closed AS closed,
  f.frozen AS frozen
FROM accounts_owner_view AS o
LEFT JOIN accounts_mint_view   AS m USING (account)
LEFT JOIN accounts_closed_view AS c USING (account)
LEFT JOIN accounts_frozen_view AS f USING (account);
