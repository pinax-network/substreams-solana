CREATE OR REPLACE VIEW metadata AS
WITH
  ua AS (SELECT metadata, argMax(update_authority, version) AS update_authority FROM metadata_update_authority_state_latest GROUP BY metadata),
  mt AS (SELECT metadata, argMax(mint, version) AS mint FROM metadata_mint_state_latest GROUP BY metadata),
  ma AS (SELECT metadata, argMax(mint_authority, version) AS mint_authority FROM metadata_mint_authority_state_latest GROUP BY metadata),
  nm AS (SELECT metadata, argMax(name, version) AS name FROM metadata_name_state_latest GROUP BY metadata),
  sb AS (SELECT metadata, argMax(symbol, version) AS symbol FROM metadata_symbol_state_latest GROUP BY metadata),
  ur AS (SELECT metadata, argMax(uri, version) AS uri FROM metadata_uri_state_latest GROUP BY metadata)
SELECT
  acc.metadata AS metadata,
  ua.update_authority,
  mt.mint,
  ma.mint_authority,
  nm.name,
  sb.symbol,
  ur.uri
FROM
  (
    SELECT metadata FROM metadata_update_authority_state_latest
    UNION DISTINCT SELECT metadata FROM metadata_mint_state_latest
    UNION DISTINCT SELECT metadata FROM metadata_mint_authority_state_latest
    UNION DISTINCT SELECT metadata FROM metadata_name_state_latest
    UNION DISTINCT SELECT metadata FROM metadata_symbol_state_latest
    UNION DISTINCT SELECT metadata FROM metadata_uri_state_latest
  ) AS acc
LEFT JOIN ua ON ua.metadata = acc.metadata
LEFT JOIN mt ON mt.metadata = acc.metadata
LEFT JOIN ma ON ma.metadata = acc.metadata
LEFT JOIN nm ON nm.metadata = acc.metadata
LEFT JOIN sb ON sb.metadata = acc.metadata
LEFT JOIN ur ON ur.metadata = acc.metadata;
