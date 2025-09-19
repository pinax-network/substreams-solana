/* MINT */
-- 1:1 relationship with metadata account
-- should not have any duplicates, no need for GROUP BY
CREATE OR REPLACE VIEW metadata_mint_view AS
SELECT
  metadata,
  version,
  block_num,
  timestamp,
  mint
FROM metadata_mint_state_latest;

/* NAME */
CREATE OR REPLACE VIEW metadata_name_view AS
SELECT
  metadata,
  max(version) as version,
  max(block_num) as block_num,
  max(timestamp) as timestamp,
  argMax(name, m.version) AS name
FROM metadata_name_state_latest as m
GROUP BY metadata;

/* SYMBOL */
CREATE OR REPLACE VIEW metadata_symbol_view AS
SELECT
  metadata,
  max(version) as version,
  max(block_num) as block_num,
  max(timestamp) as timestamp,
  argMax(symbol, m.version) AS symbol
FROM metadata_symbol_state_latest as m
GROUP BY metadata;

/* URI */
CREATE OR REPLACE VIEW metadata_uri_view AS
SELECT
  metadata,
  max(version) as version,
  max(block_num) as block_num,
  max(timestamp) as timestamp,
  argMax(uri, m.version) AS uri
FROM metadata_uri_state_latest as m
GROUP BY metadata;

/* MINT AUTHORITY */
CREATE OR REPLACE VIEW metadata_mint_authority_view AS
SELECT
  metadata,
  max(version) as version,
  max(block_num) as block_num,
  max(timestamp) as timestamp,
  argMax(mint_authority, m.version) AS mint_authority
FROM metadata_mint_authority_state_latest as m
GROUP BY metadata;

/* UPDATE AUTHORITY */
CREATE OR REPLACE VIEW metadata_update_authority_view AS
SELECT
  metadata,
  max(version) as version,
  max(block_num) as block_num,
  max(timestamp) as timestamp,
  argMax(update_authority, m.version) AS update_authority
FROM metadata_update_authority_state_latest as m
GROUP BY metadata;