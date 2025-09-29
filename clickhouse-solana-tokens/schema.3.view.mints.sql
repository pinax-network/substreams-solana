/* MINT (spine) */
CREATE OR REPLACE VIEW decimals_view AS
SELECT
  mint,
  max(version)   AS version,
  max(block_num) AS block_num,
  max(timestamp) AS timestamp,
  argMax(a.decimals, a.version) AS decimals
FROM decimals_state_latest AS a
GROUP BY mint;

CREATE OR REPLACE VIEW mint_authority_view AS
SELECT
  mint,
  max(version)   AS version,
  max(block_num) AS block_num,
  max(timestamp) AS timestamp,
  argMax(a.mint_authority, a.version) AS mint_authority
FROM mint_authority_state_latest AS a
GROUP BY mint;

CREATE OR REPLACE VIEW freeze_authority_view AS
SELECT
  mint,
  max(version)   AS version,
  max(block_num) AS block_num,
  max(timestamp) AS timestamp,
  argMax(a.freeze_authority, a.version) AS freeze_authority
FROM freeze_authority_state_latest AS a
GROUP BY mint;

/* COMBINED VIEW — decimals is required, others are optional */
CREATE OR REPLACE VIEW mints AS
SELECT
  m.mint as mint,
  m.block_num as block_num,                 -- authoritative block_num from decimals
  m.timestamp as timestamp,                 -- authoritative timestamp from decimals
  if(empty(o.owner), NULL, o.owner) AS update_authority, -- can be null (belongs to system contract)
  if(empty(m.mint_authority), NULL, m.mint_authority) AS mint_authority, -- can be null (non-mintable)
  if(empty(f.freeze_authority), NULL, f.freeze_authority) AS freeze_authority, -- can be null (non-freezable)
  d.decimals AS decimals,
  i.immutable AS immutable
FROM decimals_view AS d
LEFT JOIN accounts_owner_view AS o ON account = d.mint
LEFT JOIN mint_authority_view AS m USING (mint)
LEFT JOIN freeze_authority_view AS f USING (mint)
LEFT JOIN accounts_immutable_view AS i ON (account = d.mint);
