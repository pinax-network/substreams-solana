/* ===========================
   MATERIALIZED VIEWS (METAPLEX)
   =========================== */

/* ---------- CREATE: fan-out initial state ---------- */

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_metaplex_create_update_authority
TO metadata_update_authority_state_latest AS
SELECT
    metadata,
    update_authority,
    version,
    block_num,
    timestamp
FROM metaplex_create_metadata_account;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_metaplex_create_mint
TO metadata_mint_state_latest AS
SELECT
    metadata,
    mint,
    version,
    block_num,
    timestamp
FROM metaplex_create_metadata_account;

-- If you later have a distinct "mint_authority" source, add a similar MV
-- For now, Metaplex create schema doesn’t include mint_authority.

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_metaplex_create_name
TO metadata_name_state_latest AS
SELECT
    metadata,
    name,
    version,
    block_num,
    timestamp
FROM metaplex_create_metadata_account
WHERE name != '';

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_metaplex_create_symbol
TO metadata_symbol_state_latest AS
SELECT
    metadata,
    symbol,
    version,
    block_num,
    timestamp
FROM metaplex_create_metadata_account
WHERE symbol != '';

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_metaplex_create_uri
TO metadata_uri_state_latest AS
SELECT
    metadata,
    uri,
    version,
    block_num,
    timestamp
FROM metaplex_create_metadata_account
WHERE uri != '';

/* ---------- UPDATE AUTHORITY ---------- */

-- Only apply if provided (ignore empty string = no-op)
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_metaplex_update_update_authority_latest
TO metadata_update_authority_state_latest AS
SELECT
    metadata,
    update_authority,
    version,
    block_num,
    timestamp
FROM metaplex_update_metadata_account
WHERE update_authority != '';


/* ---------- FIELD UPDATES (name/symbol/uri) ---------- */

-- Only apply if provided (ignore empty string = no-op)
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_metaplex_update_field_name
TO metadata_name_state_latest AS
SELECT
    metadata,
    name,
    version,
    block_num,
    timestamp
FROM metaplex_update_metadata_account
WHERE name != '';

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_metaplex_update_field_symbol
TO metadata_symbol_state_latest AS
SELECT
    metadata,
    symbol,
    version,
    block_num,
    timestamp
FROM metaplex_update_metadata_account
WHERE symbol != '';

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_metaplex_update_field_uri
TO metadata_uri_state_latest AS
SELECT
    metadata,
    uri,
    version,
    block_num,
    timestamp
FROM metaplex_update_metadata_account
WHERE uri != '';
