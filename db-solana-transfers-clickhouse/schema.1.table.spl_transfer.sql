-- SPL Token Transfers --
CREATE TABLE IF NOT EXISTS spl_transfer AS base_events
COMMENT 'SPL Token Transfer/Burn/Mint events';
ALTER TABLE spl_transfer
    -- authority --
    ADD COLUMN IF NOT EXISTS authority               String,
    ADD COLUMN IF NOT EXISTS multisig_authority_raw  String,
    ADD COLUMN IF NOT EXISTS multisig_authority      Array(String) MATERIALIZED string_to_array(multisig_authority_raw),

    -- events --
    ADD COLUMN IF NOT EXISTS source                  String,
    ADD COLUMN IF NOT EXISTS destination             String,
    ADD COLUMN IF NOT EXISTS amount                  UInt64,
    ADD COLUMN IF NOT EXISTS mint                    LowCardinality(String),

    -- Optional
    ADD COLUMN IF NOT EXISTS decimals_raw            String,
    ADD COLUMN IF NOT EXISTS decimals                Nullable(UInt8) MATERIALIZED string_to_uint8(decimals_raw),

    -- Indexes --
    ADD INDEX IF NOT EXISTS idx_authority (authority) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_source (source) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_destination (destination) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_mint (mint) TYPE bloom_filter(0.005) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_amount (amount) TYPE minmax GRANULARITY 1,

    -- Projections --
    ADD PROJECTION IF NOT EXISTS prj_authority (SELECT authority, timestamp, _part_offset ORDER BY (authority, timestamp)),
    ADD PROJECTION IF NOT EXISTS prj_source (SELECT source, timestamp, _part_offset ORDER BY (source, timestamp)),
    ADD PROJECTION IF NOT EXISTS prj_destination (SELECT destination, timestamp, _part_offset ORDER BY (destination, timestamp)),
    ADD PROJECTION IF NOT EXISTS prj_mint (SELECT mint, timestamp, _part_offset ORDER BY (mint, timestamp));
