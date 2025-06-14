-- latest SPL Metadata --
CREATE TABLE IF NOT EXISTS spl_metadata  (
    -- block --
    block_num            SimpleAggregateFunction(max, UInt32) COMMENT 'block number',
    timestamp            SimpleAggregateFunction(max, DateTime(0, 'UTC')),

    -- contract --
    address              FixedString(42) COMMENT 'ERC-20 contract address',
    decimals             SimpleAggregateFunction(anyLast, UInt8) COMMENT 'ERC-20 contract decimals (typically 18)',
    name                 SimpleAggregateFunction(anyLast, Nullable(String)) COMMENT 'ERC-20 contract name (typically 3-8 characters)',
    symbol               SimpleAggregateFunction(anyLast, Nullable(String)) COMMENT 'ERC-20 contract symbol (typically 3-4 characters)'
)
ENGINE = AggregatingMergeTree
ORDER BY address;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_spl_metadata_initialize
TO spl_metadata AS
SELECT
    -- block --
    block_num,
    timestamp,

    -- event--
    address,
    decimals,

    -- replace empty strings with NULLs --
    IF (name = '', Null, name) AS name,
    IF (symbol = '', Null, symbol) AS symbol
FROM spl_metadata_initialize;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_spl_metadata_changes
TO spl_metadata AS
SELECT
    -- block --
    c.block_num as block_num,
    c.timestamp as timestamp,

    -- event--
    c.address AS address,

    -- replace empty strings with NULLs --
    IF (c.name = '', Null, c.name) AS name,
    IF (c.symbol = '', Null, c.symbol) AS symbol
FROM spl_metadata_changes AS c
JOIN spl_metadata_initialize USING (address); -- address must already be initialized

-- one time INSERT to populate Native contract --
INSERT INTO spl_metadata (
    -- block --
    block_num,
    timestamp,
    -- event --
    address,
    name,
    symbol,
    decimals
)
VALUES (
    0,
    toDateTime(0, 'UTC'),
    '0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee',
    'Native',
    'Native',
    18
);