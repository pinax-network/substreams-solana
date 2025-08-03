CREATE TABLE IF NOT EXISTS transfers AS spl_transfer
COMMENT 'SPL 2022 & Native token transfers';

-- SPL Token Transfers --
ALTER TABLE transfers
    -- JOIN `*_owner` with `initialize_accounts` TABLE --
    ADD COLUMN IF NOT EXISTS source_owner            String,
    ADD COLUMN IF NOT EXISTS destination_owner       String,
    -- require `decimals` to be present for token transfers
    DROP COLUMN IF EXISTS decimals,
    DROP COLUMN IF EXISTS decimals_raw,
    ADD COLUMN decimals UInt8,
    -- require `mint` to be present for token transfers
    DROP COLUMN IF EXISTS mint,
    DROP COLUMN IF EXISTS mint_raw,
    ADD COLUMN mint LowCardinality(String),
    -- Indexes --
    ADD INDEX IF NOT EXISTS idx_source_owner (source_owner) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_destination_owner (destination_owner) TYPE bloom_filter(0.005) GRANULARITY 1;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_spl_transfer
TO transfers AS
SELECT
    * EXCEPT (block_num, mint_raw, mint, is_closed, account, owner, decimals_raw, decimals, mint_authority, freeze_authority, version, sign),

    -- base fields --
    t.block_num AS block_num,

    -- mint --
    ifNull(t.decimals, m.decimals) AS decimals,
    t.mint AS mint,

    -- JOIN fields --
    a1.owner AS source_owner,
    a2.owner AS destination_owner
FROM spl_transfer AS t
JOIN accounts AS a1 ON (t.mint = a1.mint AND t.source = a1.account)
JOIN accounts AS a2 ON (t.mint = a2.mint AND t.destination = a2.account)
JOIN mints AS m ON m.mint = t.mint
-- ignore 0 transfers
WHERE t.amount > 0 AND t.mint IS NOT NULL;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_system_transfer
TO transfers AS
SELECT
    * EXCEPT (lamports),
    lamports AS amount,
    'So11111111111111111111111111111111111111111' AS mint, -- native
    toUInt8(9) AS decimals,
    source as authority,
    source AS source_owner,
    destination AS destination_owner
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
    destination AS destination,
    source_seed AS source_owner,
    destination AS destination_owner
FROM system_transfer_with_seed
-- ignore 0 transfers
WHERE lamports > 0;