CREATE OR REPLACE VIEW mint_authority_view AS
SELECT
  mint,
  max(version)   AS version,
  max(block_num) AS block_num,
  max(timestamp) AS timestamp,
  argMax(a.authority, a.version) AS authority
FROM mint_authority_state AS a
GROUP BY mint;

CREATE OR REPLACE VIEW freeze_authority_view AS
SELECT
  mint,
  max(version)   AS version,
  max(block_num) AS block_num,
  max(timestamp) AS timestamp,
  argMax(a.authority, a.version) AS authority
FROM freeze_authority_state AS a
GROUP BY mint;

-- Ideal for general mints lookups
CREATE OR REPLACE VIEW mints_view AS
SELECT
  d.mint as mint,
  d.program_id as program_id,
  d.block_num as block_num,                 -- authoritative block_num from decimals
  d.timestamp as timestamp,                 -- authoritative timestamp from decimals
  if(empty(u.authority), NULL, u.authority) AS update_authority, -- can be null (belongs to system contract)
  if(empty(m.authority), NULL, m.authority) AS mint_authority, -- can be null (non-mintable)
  if(empty(f.authority), NULL, f.authority) AS freeze_authority, -- can be null (non-freezable)
  d.decimals AS decimals,
  i.immutable AS immutable
FROM decimals_state AS d
LEFT JOIN metadata_mint_state AS mm USING (mint)          -- 1:1 mint -> metadata
LEFT JOIN metadata_update_authority_view AS u USING (metadata)   -- now cheap
LEFT JOIN mint_authority_view AS m USING (mint)
LEFT JOIN freeze_authority_view AS f USING (mint)
LEFT JOIN immutable_owner_view  AS i ON i.account = d.mint;

