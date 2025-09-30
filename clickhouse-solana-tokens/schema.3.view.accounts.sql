/* OWNER (spine) */
CREATE OR REPLACE VIEW owner_view AS
SELECT
  account,
  any(program_id) AS program_id,
  max(version)   AS version,
  max(block_num) AS block_num,
  max(timestamp) AS timestamp,
  argMax(a.owner, a.version) AS owner
FROM owner_state_latest AS a
GROUP BY account;

/* Optional fields kept as separate views (same pattern) */
CREATE OR REPLACE VIEW account_mint_view AS
SELECT
  account,
  any(program_id) AS program_id,
  max(version)   AS version,
  max(block_num) AS block_num,
  max(timestamp) AS timestamp,
  argMax(a.mint, a.version) AS mint
FROM account_mint_state_latest AS a
GROUP BY account;

CREATE OR REPLACE VIEW close_account_view AS
SELECT
  account,
  any(program_id) AS program_id,
  max(version)   AS version,
  max(block_num) AS block_num,
  max(timestamp) AS timestamp,
  argMax(a.closed, a.version) AS closed
FROM close_account_state_latest AS a
GROUP BY account;

CREATE OR REPLACE VIEW freeze_account_view AS
SELECT
  account,
  any(program_id) AS program_id,
  max(version)   AS version,
  max(block_num) AS block_num,
  max(timestamp) AS timestamp,
  argMax(a.frozen, a.version) AS frozen
FROM freeze_account_state_latest AS a
GROUP BY account;

CREATE OR REPLACE VIEW immutable_owner_view AS
SELECT
  account,
  any(program_id) AS program_id,
  max(version)   AS version,
  max(block_num) AS block_num,
  max(timestamp) AS timestamp,
  argMax(a.immutable, a.version) AS immutable
FROM immutable_owner_state_latest AS a
GROUP BY account;

-- Ideal for general account lookups
CREATE OR REPLACE VIEW accounts_view AS
SELECT
  a.account AS account,
  a.program_id AS program_id,
  a.block_num AS created_at_block_num,
  a.timestamp AS created_at_timestamp,
  if(empty(o.owner), NULL, o.owner) AS owner, -- can be null (belongs to system contract)
  if(empty(m.mint), NULL, m.mint) AS mint,
  c.closed AS closed,
  f.frozen AS frozen,
  i.immutable AS immutable
FROM account_state_latest AS a
LEFT JOIN account_mint_view AS m USING (account)
LEFT JOIN owner_view AS o USING (account)
LEFT JOIN close_account_view AS c USING (account)
LEFT JOIN freeze_account_view AS f USING (account)
LEFT JOIN immutable_owner_view AS i USING (account);

-- Ideal for owner lookups (get all accounts of an owner)
CREATE OR REPLACE VIEW accounts_from_owner_view AS
SELECT
  o.account AS account,
  o.program_id AS program_id,
  o.block_num AS created_at_block_num,
  o.timestamp AS created_at_timestamp,
  if(empty(o.owner), NULL, o.owner) AS owner, -- can be null (belongs to system contract)
  if(empty(m.mint), NULL, m.mint) AS mint,
  c.closed AS closed,
  f.frozen AS frozen,
  i.immutable AS immutable
FROM owner_view AS o
LEFT JOIN account_mint_view AS m USING (account)
LEFT JOIN close_account_view AS c USING (account)
LEFT JOIN freeze_account_view AS f USING (account)
LEFT JOIN immutable_owner_view AS i USING (account);

-- Ideal for scanning all accounts of a given mint
CREATE OR REPLACE VIEW accounts_from_mint_view AS
SELECT
  m.account AS account,
  m.program_id AS program_id,
  m.block_num AS created_at_block_num,
  m.timestamp AS created_at_timestamp,
  if(empty(o.owner), NULL, o.owner) AS owner, -- can be null (belongs to system contract)
  if(empty(m.mint), NULL, m.mint) AS mint,
  c.closed AS closed,
  f.frozen AS frozen,
  i.immutable AS immutable
FROM account_mint_view AS m
LEFT JOIN owner_view AS o USING (account)
LEFT JOIN close_account_view AS c USING (account)
LEFT JOIN freeze_account_view AS f USING (account)
LEFT JOIN immutable_owner_view AS i USING (account);
