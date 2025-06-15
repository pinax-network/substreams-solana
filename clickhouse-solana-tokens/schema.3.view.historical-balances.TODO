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
