/* NAME */
CREATE OR REPLACE VIEW metadata_name_view AS
SELECT
  metadata,
  max(version) as version,
  max(block_num) as block_num,
  max(timestamp) as timestamp,
  argMax(name, m.version) AS name
FROM metadata_name_state as m
GROUP BY metadata;

/* SYMBOL */
CREATE OR REPLACE VIEW metadata_symbol_view AS
SELECT
  metadata,
  max(version) as version,
  max(block_num) as block_num,
  max(timestamp) as timestamp,
  argMax(symbol, m.version) AS symbol
FROM metadata_symbol_state as m
GROUP BY metadata;

/* URI */
CREATE OR REPLACE VIEW metadata_uri_view AS
SELECT
  metadata,
  max(version) as version,
  max(block_num) as block_num,
  max(timestamp) as timestamp,
  argMax(uri, m.version) AS uri
FROM metadata_uri_state as m
GROUP BY metadata;

/* MINT AUTHORITY */
CREATE OR REPLACE VIEW metadata_mint_authority_view AS
SELECT
  metadata,
  max(version) as version,
  max(block_num) as block_num,
  max(timestamp) as timestamp,
  argMax(mint_authority, m.version) AS mint_authority
FROM metadata_mint_authority_state as m
GROUP BY metadata;

/* UPDATE AUTHORITY */
CREATE OR REPLACE VIEW metadata_update_authority_view AS
SELECT
  metadata,
  max(version) as version,
  max(block_num) as block_num,
  max(timestamp) as timestamp,
  argMax(update_authority, m.version) AS update_authority
FROM metadata_update_authority_state as m
GROUP BY metadata;

/* COMBINED VIEW */
CREATE OR REPLACE VIEW metadata_view AS
SELECT
    k.mint as mint,
    k.metadata as metadata,
    k.block_num as block_num,
    k.timestamp as timestamp,
    if(empty(n.name), NULL, n.name) AS name,
    if(empty(s.symbol), NULL, s.symbol) AS symbol,
    if(empty(u.uri), NULL, u.uri) AS uri,
    if(empty(ma.mint_authority), NULL, ma.mint_authority) AS mint_authority,
    if(empty(ua.update_authority), NULL, ua.update_authority) AS update_authority
FROM metadata_mint_state AS k
    LEFT JOIN metadata_name_view              AS n  USING (metadata)
    LEFT JOIN metadata_symbol_view            AS s  USING (metadata)
    LEFT JOIN metadata_uri_view               AS u  USING (metadata)
    LEFT JOIN metadata_mint_authority_view    AS ma USING (metadata)
    LEFT JOIN metadata_update_authority_view  AS ua USING (metadata);