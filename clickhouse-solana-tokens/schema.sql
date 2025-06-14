-- This file is generated. Do not edit.

-- Native balance by account --
-- There can only be a single Native balance change per block for a given address  --
CREATE TABLE IF NOT EXISTS native_balance_changes  (
    -- block --
    block_num            UInt32,
    block_hash           FixedString(66),
    timestamp            DateTime(0, 'UTC'),

    -- event --
    address              FixedString(42),
    balance              UInt256,

    -- indexes --
    INDEX idx_block_num          (block_num)           TYPE minmax GRANULARITY 4,
    INDEX idx_timestamp          (timestamp)           TYPE minmax GRANULARITY 4,

    -- indexes (event) --
    INDEX idx_address            (address)             TYPE bloom_filter GRANULARITY 4,
    INDEX idx_balance            (balance)             TYPE minmax GRANULARITY 4
)
ENGINE = ReplacingMergeTree
ORDER BY (address, block_num);

-- Exclude gas fees from primary Clickhouse DB --
CREATE TABLE IF NOT EXISTS native_balance_changes_from_gas AS native_balance_changes
ENGINE = ReplacingMergeTree
ORDER BY (address, block_num);


-- Native transfers --
CREATE TABLE IF NOT EXISTS native_transfers  (
    -- block --
    block_num            UInt32,
    block_hash           FixedString(66),
    timestamp            DateTime(0, 'UTC'),

    -- ordering --
    `index`              UInt64, -- relative index
    global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)

    -- transaction --
    tx_hash              FixedString(66),

    -- event --
    `from`               FixedString(42) COMMENT 'sender address', -- log.topics[1]
    `to`                 FixedString(42) COMMENT 'recipient address', -- log.topics[2]
    value                UInt256 COMMENT 'transfer value', -- log.data

    -- indexes --
    INDEX idx_tx_hash            (tx_hash)            TYPE bloom_filter GRANULARITY 4,

    -- indexes (event) --
    INDEX idx_from               (`from`)             TYPE bloom_filter GRANULARITY 4,
    INDEX idx_to                 (`to`)               TYPE bloom_filter GRANULARITY 4,
    INDEX idx_value              (value)              TYPE minmax GRANULARITY 4
)
ENGINE = ReplacingMergeTree
ORDER BY (timestamp, block_num, `index`);

-- Exclude gas fees from primary Clickhouse DB --
CREATE TABLE IF NOT EXISTS native_transfers_from_fees AS native_transfers
ENGINE = ReplacingMergeTree
ORDER BY (timestamp, block_num, `index`);


-- SPL balance by account --
-- There can only be a single SPL balance change per block for a given address / contract pair --
CREATE TABLE IF NOT EXISTS spl_balance_changes  (
    -- block --
    block_num            UInt32,
    block_hash           FixedString(66),
    timestamp            DateTime(0, 'UTC'),

    -- event --
    contract             FixedString(42),
    address              FixedString(42),
    balance              UInt256,

    -- indexes --
    INDEX idx_block_num          (block_num)           TYPE minmax GRANULARITY 4,
    INDEX idx_timestamp          (timestamp)           TYPE minmax GRANULARITY 4,

    -- indexes (event) --
    INDEX idx_contract           (contract)            TYPE set(64) GRANULARITY 4,
    INDEX idx_address            (address)             TYPE bloom_filter GRANULARITY 4,
    INDEX idx_balance            (balance)             TYPE minmax GRANULARITY 4
)
ENGINE = ReplacingMergeTree
ORDER BY (contract, address, block_num);


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


-- SPL Total Suppy changes --
-- There can only be a single SPL supply change per block per contract  --
CREATE TABLE IF NOT EXISTS spl_total_supply_changes  (
    -- block --
    block_num               UInt32,
    block_hash              FixedString(66),
    timestamp               DateTime(0, 'UTC'),

    -- event --
    contract                FixedString(42),
    total_supply            UInt256,

    -- indexes --
    INDEX idx_block_num           (block_num)             TYPE minmax GRANULARITY 4,
    INDEX idx_timestamp           (timestamp)             TYPE minmax GRANULARITY 4,

    -- indexes (event) --
    INDEX idx_contract            (contract)              TYPE bloom_filter GRANULARITY 4,
    INDEX idx_total_supply        (total_supply)          TYPE minmax GRANULARITY 4
)
ENGINE = ReplacingMergeTree
ORDER BY (contract, block_num);


-- SPL transfers --
CREATE TABLE IF NOT EXISTS spl_transfers  (
    -- block --
    block_num            UInt32,
    block_hash           FixedString(66),
    timestamp            DateTime(0, 'UTC'),

    -- ordering --
    `index`              UInt64, -- relative index
    global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)

    -- transaction --
    tx_hash              FixedString(66),

    -- call --
    caller               FixedString(42),

    -- log --
    contract             FixedString(42),
    ordinal              UInt64, -- log.ordinal

    -- event --
    `from`               FixedString(42) COMMENT 'sender address',
    `to`                 FixedString(42) COMMENT 'recipient address',
    value                UInt256 COMMENT 'transfer value',

    -- indexes --
    INDEX idx_tx_hash            (tx_hash)            TYPE bloom_filter GRANULARITY 4,
    INDEX idx_caller             (caller)             TYPE bloom_filter GRANULARITY 4,

    -- indexes (event) --
    INDEX idx_contract           (contract)           TYPE set(64) GRANULARITY 4,
    INDEX idx_from               (`from`)             TYPE bloom_filter GRANULARITY 4,
    INDEX idx_to                 (`to`)               TYPE bloom_filter GRANULARITY 4,
    INDEX idx_value              (value)              TYPE minmax GRANULARITY 4
)
ENGINE = ReplacingMergeTree
ORDER BY (timestamp, block_num, `index`);

-- SPL approvals --
CREATE TABLE IF NOT EXISTS spl_approvals  (
    -- block --
    block_num            UInt32,
    block_hash           FixedString(66),
    timestamp            DateTime(0, 'UTC'),

    -- ordering --
    `index`              UInt64, -- relative index
    global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)

    -- transaction --
    tx_hash              FixedString(66),

    -- call --
    caller               FixedString(42),

    -- log --
    contract             FixedString(42),
    ordinal              UInt64, -- log.ordinal

    -- event --
    owner                FixedString(42),
    spender              FixedString(42),
    value                UInt256,

    -- indexes --
    INDEX idx_tx_hash            (tx_hash)            TYPE bloom_filter GRANULARITY 4,
    INDEX idx_caller             (caller)             TYPE bloom_filter GRANULARITY 4,

    -- indexes (event) --
    INDEX idx_contract           (contract)           TYPE set(64) GRANULARITY 4,
    INDEX idx_owner              (owner)              TYPE bloom_filter GRANULARITY 4,
    INDEX idx_spender            (spender)            TYPE bloom_filter GRANULARITY 4,
    INDEX idx_value              (value)              TYPE minmax GRANULARITY 4
)
ENGINE = ReplacingMergeTree
ORDER BY (timestamp, block_num, `index`);

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

-- latest balances by owner/contract --
CREATE TABLE IF NOT EXISTS balances  (
    -- block --
    block_num            UInt32,
    block_hash           FixedString(66),
    timestamp            DateTime(0, 'UTC'),

    -- event --
    contract             FixedString(42),
    address              FixedString(42),
    balance              Float64,
    balance_raw          UInt256,

    -- metadata --
    decimals             UInt8,
    symbol               Nullable(String),
    name                 Nullable(String),

    -- indexes --
    INDEX idx_block_num          (block_num)           TYPE minmax GRANULARITY 4,
    INDEX idx_timestamp          (timestamp)           TYPE minmax GRANULARITY 4,

    -- indexes (event) --
    INDEX idx_contract           (contract)            TYPE set(64) GRANULARITY 4,
    INDEX idx_address            (address)             TYPE bloom_filter GRANULARITY 4,
    INDEX idx_balance            (balance)             TYPE minmax GRANULARITY 4,

    -- indexes (metadata) --
    INDEX idx_decimals           (decimals)            TYPE set(32) GRANULARITY 4,
    INDEX idx_symbol             (symbol)              TYPE set(64) GRANULARITY 4,
    INDEX idx_name               (name)                TYPE set(64) GRANULARITY 4
)
ENGINE = ReplacingMergeTree(block_num)
ORDER BY (address, contract);

-- insert SPL balance changes --
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_spl_balances
TO balances AS
SELECT
    -- block --
    b.block_num AS block_num,
    b.block_hash AS block_hash,
    b.timestamp AS timestamp,

    -- event --
    b.contract AS contract,
    b.address AS address,
    b.balance / pow(10, m.decimals) AS balance,
    b.balance AS balance_raw,

    -- erc20 metadata --
    m.decimals AS decimals,
    m.symbol AS symbol,
    m.name AS name

FROM spl_balance_changes AS b
JOIN spl_metadata AS m ON m.address = b.contract;

-- insert Native balance changes --
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_native_balances
TO balances AS
SELECT
    -- block --
    block_num,
    block_hash,
    timestamp,

    -- event --
    '0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee' AS contract,
    address,
    b.balance / pow(10, 18) AS balance,
    b.balance AS balance_raw,

    -- erc20 metadata --
    18 AS decimals,
    'Native' AS symbol,
    'Native' AS name

FROM native_balance_changes as b;

-- latest balances by contract/address --
CREATE TABLE IF NOT EXISTS balances_by_contract AS balances
ENGINE = ReplacingMergeTree(block_num)
ORDER BY (contract, address);

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_balances_by_contract
TO balances_by_contract AS
SELECT * FROM balances;


-- Historical balances by address/contract --
CREATE TABLE IF NOT EXISTS historical_balances (
    -- block --
    block_num            SimpleAggregateFunction(min, UInt32),
    timestamp            DateTime(0, 'UTC') COMMENT 'the start of the aggregate window',

    -- balance change --
    contract             FixedString(42) COMMENT 'contract address',
    address              FixedString(42) COMMENT 'wallet address',

    -- metadata --
    decimals            SimpleAggregateFunction(any, UInt8),
    symbol              SimpleAggregateFunction(anyLast, Nullable(String)),
    name                SimpleAggregateFunction(anyLast, Nullable(String)),

    -- ohlc --
    open                 AggregateFunction(argMin, Float64, UInt32),
    high                 SimpleAggregateFunction(max, Float64),
    low                  SimpleAggregateFunction(min, Float64),
    close                AggregateFunction(argMax, Float64, UInt32),
    uaw                  AggregateFunction(uniq, FixedString(42)) COMMENT 'unique wallet addresses that changed balance in the window',
    transactions         SimpleAggregateFunction(sum, UInt64) COMMENT 'number of transactions in the window',
)
ENGINE = AggregatingMergeTree
ORDER BY (address, contract, timestamp);

-- SPL Balances --
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_historical_spl_balances
TO historical_balances
AS
WITH
    b.balance / pow(10, m.decimals) AS balance
SELECT
    -- block --
    toStartOfHour(timestamp) AS timestamp,
    min(block_num) AS block_num,

    -- balance change --
    address,
    contract,

    -- erc20 metadata --
    any(m.decimals) AS decimals,
    anyLast(m.symbol) AS symbol,
    anyLast(m.name) AS name,

    -- ohlc --
    argMinState(balance, b.block_num) AS open,
    max(balance) AS high,
    min(balance) AS low,
    argMaxState(balance, b.block_num) AS close,
    uniqState(address) AS uaw,
    count() AS transactions
FROM spl_balance_changes AS b
JOIN spl_metadata AS m ON m.address = b.contract
GROUP BY address, contract, timestamp;

-- Native balances --
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_historical_native_balances
TO historical_balances
AS
WITH
    b.balance / pow(10, 18) AS balance
SELECT
    -- block --
    min(block_num) AS block_num,
    toStartOfHour(timestamp) AS timestamp,

    -- balance change --
    '0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee' AS contract,
    address,

    -- erc20 metadata --
    18 AS decimals,
    'Native' AS symbol,
    'Native' AS name,

    -- balance --
    argMinState(balance, b.block_num) AS open,
    max(balance) AS high,
    min(balance) AS low,
    argMaxState(balance, b.block_num) AS close,
    uniqState(address) AS uaw,
    count() AS transactions
FROM native_balance_changes AS b
GROUP BY address, timestamp;

-- Historical balances by contract/address --
CREATE MATERIALIZED VIEW IF NOT EXISTS historical_balances_by_contract
ENGINE = AggregatingMergeTree
ORDER BY (contract, address, timestamp)
AS
SELECT * FROM historical_balances;


-- latest transfers --
CREATE TABLE IF NOT EXISTS transfers (
    -- block --
    block_num            UInt32,
    block_hash           FixedString(66),
    timestamp            DateTime(0, 'UTC'),

    -- ordering --
    `index`             UInt64,
    global_sequence     UInt64,

    -- transaction --
    tx_hash		        FixedString(66),

    -- log --
    contract			FixedString(42),

    -- event --
    `from`				FixedString(42),
    `to`				FixedString(42),
    amount              UInt256,
    value               Float64,

    -- metadata --
    decimals            UInt8,
    symbol              Nullable(String),
    name                Nullable(String),

    -- indexes --
    INDEX idx_tx_hash            (tx_hash)              TYPE bloom_filter GRANULARITY 4,

    -- indexes (event) --
    INDEX idx_contract           (contract)             TYPE set(64) GRANULARITY 4,
    INDEX idx_from               (`from`)               TYPE bloom_filter GRANULARITY 4,
    INDEX idx_to                 (`to`)                 TYPE bloom_filter GRANULARITY 4,
    INDEX idx_value              (value)                TYPE minmax GRANULARITY 4,
    INDEX idx_amount             (amount)               TYPE minmax GRANULARITY 4,

    -- indexes (metadata) --
    INDEX idx_decimals           (decimals)             TYPE set(18) GRANULARITY 4,
    INDEX idx_symbol             (symbol)               TYPE set(64) GRANULARITY 4,
    INDEX idx_name               (name)                 TYPE set(64) GRANULARITY 4
)
ENGINE = ReplacingMergeTree
ORDER BY (timestamp, block_num, `index`);

-- insert SPL transfers --
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_spl_transfers
TO transfers AS
SELECT
    -- block --
    timestamp,
    block_hash,
    block_num,

    -- ordering --
    `index`,
    global_sequence,

    -- transaction --
    tx_hash,

    -- log --
    contract,

    -- event --
    `from`,
    `to`,
    value AS amount,
    value / pow(10, decimals) AS value,

    -- ERC20 metadata --
    decimals,
    symbol,
    name
FROM spl_transfers AS t
JOIN spl_metadata AS c ON c.address = t.contract;

-- insert Native transfers --
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_native_transfers
TO transfers AS
WITH
    '0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee' AS contract,
    18 AS decimals,
    'Native' AS symbol,
    'Native' AS name
SELECT
    -- block --
    timestamp,
    block_hash,
    block_num,

    -- ordering --
    `index`,
    global_sequence,

    -- transaction --
    tx_hash,

    -- log --
    contract,

    -- event --
    `from`,
    `to`,
    value AS amount,
    value / pow(10, decimals) AS value,

    -- ERC20 metadata --
    decimals,
    symbol,
    name
FROM native_transfers;


CREATE TABLE IF NOT EXISTS cursors
(
    id        String,
    cursor    String,
    block_num Int64,
    block_id  String
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (id)
        ORDER BY (id);

