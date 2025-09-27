/* OWNER */
CREATE OR REPLACE VIEW accounts_owner_view AS
SELECT
  account,
  max(version) as version,
  max(block_num) as block_num,
  max(timestamp) as timestamp,
  argMax(owner, version) AS owner
FROM owner_state_latest
GROUP BY account;

/* MINT */
CREATE OR REPLACE VIEW accounts_mint_view AS
SELECT
  account,
  max(version) as version,
  max(block_num) as block_num,
  max(timestamp) as timestamp,
  argMax(mint, version) AS mint
FROM mint_state_latest
GROUP BY account;

/* CLOSED */
CREATE OR REPLACE VIEW accounts_closed_view AS
SELECT
  account,
  max(version) as version,
  max(block_num) as block_num,
  max(timestamp) as timestamp,
  argMax(closed, version) AS closed
FROM closed_state_latest
GROUP BY account;

/* FROZEN */
CREATE OR REPLACE VIEW accounts_frozen_view AS
SELECT
  account,
  max(version) as version,
  max(block_num) as block_num,
  max(timestamp) as timestamp,
  argMax(frozen, version) AS frozen
FROM frozen_state_latest
GROUP BY account;

/* COMBINED VIEW */
CREATE OR REPLACE VIEW accounts_view AS
SELECT
  acc.account as account,
  if(empty(ow.owner), NULL, ow.owner) AS owner,
  if(empty(mt.mint), NULL, mt.mint) AS mint,
  cl.closed AS closed,
  fr.frozen AS frozen
FROM
  /* union of all accounts seen in any table */
  (
    SELECT account FROM owner_state_latest
    UNION DISTINCT SELECT account FROM mint_state_latest
    UNION DISTINCT SELECT account FROM closed_state_latest
    UNION DISTINCT SELECT account FROM frozen_state_latest
  ) AS acc
LEFT JOIN accounts_owner_view  AS ow ON ow.account = acc.account
LEFT JOIN accounts_mint_view   AS mt ON mt.account = acc.account
LEFT JOIN accounts_closed_view AS cl ON cl.account = acc.account
LEFT JOIN accounts_frozen_view AS fr ON fr.account = acc.account;

/* BACKWARD COMPATIBILITY - Alias to the new combined view */
CREATE OR REPLACE VIEW accounts AS
SELECT * FROM accounts_view;
