CREATE TABLE IF NOT EXISTS transfers AS spl_transfer
COMMENT 'SPL 2022 & Native token transfers';

-- SPL Token Transfers --
ALTER TABLE transfers
    -- require `decimals` to be present for token transfers
    DROP COLUMN IF EXISTS decimals,
    DROP COLUMN IF EXISTS decimals_raw,
    ADD COLUMN decimals Nullable(UInt8),
    -- require `mint` to be present for token transfers
    DROP COLUMN IF EXISTS mint,
    DROP COLUMN IF EXISTS mint_raw,
    ADD COLUMN mint LowCardinality(String);

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_spl_transfer
TO transfers AS
SELECT
    * EXCEPT (decimals_raw, mint_raw),

    -- mint --
    mint AS mint,
    decimals AS decimals
FROM spl_transfer
-- ignore 0 transfers
WHERE amount > 0 AND mint IS NOT NULL;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_system_transfer
TO transfers AS
SELECT
    * EXCEPT (lamports),
    lamports AS amount,
    'So11111111111111111111111111111111111111111' AS mint, -- native
    toUInt8(9) AS decimals,
    source AS authority
FROM system_transfer
-- ignore 0 transfers
WHERE lamports > 0;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_system_transfer_with_seed
TO transfers AS
SELECT
    * EXCEPT (lamports, source, destination, source_base, source_seed, source_owner),
    lamports AS amount,
    'So11111111111111111111111111111111111111111' AS mint, -- native
    toUInt8(9) AS decimals,
    source_base AS authority,
    source_seed AS source,
    destination AS destination
FROM system_transfer_with_seed
-- ignore 0 transfers
WHERE lamports > 0;