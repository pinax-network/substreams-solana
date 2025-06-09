-- SPL Metadata Initialize --
CREATE TABLE IF NOT EXISTS spl_metadata_initialize (
    -- block --
    block_num            UInt32,
    block_hash           FixedString(66),
    timestamp            DateTime(0, 'UTC'),

    -- event --
    address              FixedString(42),
    decimals             UInt8,
    name                 Nullable(String),
    symbol               Nullable(String)
)
ENGINE = ReplacingMergeTree
ORDER BY (address);

-- SPL Metadata Changes --
CREATE TABLE IF NOT EXISTS spl_metadata_changes (
    -- block --
    block_num            UInt32,
    block_hash           FixedString(66),
    timestamp            DateTime(0, 'UTC'),

    -- event --
    address              FixedString(42),
    name                 Nullable(String),
    symbol               Nullable(String)
)
ENGINE = ReplacingMergeTree
ORDER BY (address, block_num);
