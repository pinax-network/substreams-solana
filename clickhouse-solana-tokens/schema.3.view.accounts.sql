/* OWNER (spine) */
CREATE OR REPLACE VIEW accounts_owner_view AS
SELECT
  account,
  max(version)   AS version,
  max(block_num) AS block_num,
  max(timestamp) AS timestamp,
  argMax(a.owner, a.version) AS owner
FROM owner_state_latest AS a
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
GROUP BY account;

CREATE OR REPLACE VIEW accounts_closed_view AS
SELECT
  account,
  max(version)   AS version,
  max(block_num) AS block_num,
  max(timestamp) AS timestamp,
  argMax(a.closed, a.version) AS closed
FROM closed_state_latest AS a
GROUP BY account;

CREATE OR REPLACE VIEW accounts_frozen_view AS
SELECT
  account,
  max(version)   AS version,
  max(block_num) AS block_num,
  max(timestamp) AS timestamp,
  argMax(a.frozen, a.version) AS frozen
FROM frozen_state_latest AS a
GROUP BY account;

CREATE OR REPLACE VIEW accounts_immutable_view AS
SELECT
  account,
  max(version)   AS version,
  max(block_num) AS block_num,
  max(timestamp) AS timestamp,
  argMax(a.immutable, a.version) AS immutable
FROM immutable_owner_state_latest AS a
GROUP BY account;

/* COMBINED VIEW — owner is required, others are optional */
CREATE OR REPLACE VIEW accounts AS
SELECT
  m.account as account,
  m.block_num as block_num,                 -- authoritative block_num from owner
  m.timestamp as timestamp,                 -- authoritative timestamp from owner
  if(empty(o.owner), NULL, o.owner) AS owner, -- can be null (belongs to system contract)
  if(empty(m.mint), NULL, m.mint) AS mint,
  c.closed AS closed,
  f.frozen AS frozen,
  i.immutable AS immutable
FROM accounts_mint_view AS m
LEFT JOIN accounts_owner_view AS o USING (account)
LEFT JOIN accounts_closed_view AS c USING (account)
LEFT JOIN accounts_frozen_view AS f USING (account)
LEFT JOIN accounts_immutable_view AS i USING (account);
