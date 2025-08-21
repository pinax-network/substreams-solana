CREATE OR REPLACE VIEW accounts AS
WITH
  /* Latest OWNER by version */
  ow AS (
    SELECT
      account,
      argMax(owner, version) AS owner
    FROM owner_state_latest
    GROUP BY account
  ),

  /* Latest MINT by version */
  mt AS (
    SELECT
      account,
      argMax(mint, version) AS mint
    FROM mint_state_latest
    GROUP BY account
  ),

  /* Latest CLOSED by version */
  cl AS (
    SELECT
      account,
      argMax(closed, version) AS closed
    FROM closed_state_latest
    GROUP BY account
  ),

  /* Latest FROZEN by version */
  fr AS (
    SELECT
      account,
      argMax(frozen, version) AS frozen
    FROM frozen_state_latest
    GROUP BY account
  )

SELECT
  acc.account as account,

  /* final states */
  owner,
  mint,
  closed,
  frozen

FROM
  /* union of all accounts seen in any table */
  (
    SELECT account FROM owner_state_latest
    UNION DISTINCT
    SELECT account FROM mint_state_latest
    UNION DISTINCT
    SELECT account FROM closed_state_latest
    UNION DISTINCT
    SELECT account FROM frozen_state_latest
  ) AS acc
LEFT JOIN ow ON ow.account = acc.account
LEFT JOIN mt ON mt.account = acc.account
LEFT JOIN cl ON cl.account = acc.account
LEFT JOIN fr ON fr.account = acc.account;
