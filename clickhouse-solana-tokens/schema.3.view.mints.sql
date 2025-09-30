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

-- Ideal for general mints lookups
CREATE OR REPLACE VIEW mints_view AS
SELECT
  d.mint as mint,
  d.program_id as program_id,
  d.block_num as block_num,                 -- authoritative block_num from decimals
  d.timestamp as timestamp,                 -- authoritative timestamp from decimals
  if(empty(u.update_authority), NULL, u.update_authority) AS update_authority, -- can be null (belongs to system contract)
  if(empty(m.mint_authority), NULL, m.mint_authority) AS mint_authority, -- can be null (non-mintable)
  if(empty(f.freeze_authority), NULL, f.freeze_authority) AS freeze_authority, -- can be null (non-freezable)
  d.decimals AS decimals,
  i.immutable AS immutable
FROM decimals_state_latest AS d
LEFT JOIN metadata_update_authority_view AS u ON u.metadata = d.mint
LEFT JOIN mint_authority_view AS m USING (mint)
LEFT JOIN freeze_authority_view AS f USING (mint)
LEFT JOIN immutable_owner_view AS i ON i.account = d.mint;
