-- OHLCV prices --
CREATE TABLE IF NOT EXISTS ohlc_prices (
    timestamp            DateTime(0, 'UTC') COMMENT 'beginning of the bar',

    -- pool --
    pool                  String COMMENT 'pool address',
    protocol             SimpleAggregateFunction(any, LowCardinality(String)),
    program_id           SimpleAggregateFunction(any, FixedString(44)), -- EVM aka `contract`
    -- fee                  SimpleAggregateFunction(anyLast, UInt32),

    -- token0 metadata --
    token0               SimpleAggregateFunction(any, FixedString(44)),
    decimals0            SimpleAggregateFunction(any, UInt8),
    -- symbol0              SimpleAggregateFunction(anyLast, Nullable(String)),
    -- name0                SimpleAggregateFunction(anyLast, Nullable(String)),

    -- token1 metadata --
    token1               SimpleAggregateFunction(any, FixedString(44)),
    decimals1            SimpleAggregateFunction(any, UInt8),
    -- symbol1              SimpleAggregateFunction(anyLast, Nullable(String)),
    -- name1                SimpleAggregateFunction(anyLast, Nullable(String)),

    -- canonical pair (token0, token1) lexicographic order --
    canonical0           SimpleAggregateFunction(any, FixedString(44)),
    canonical1           SimpleAggregateFunction(any, FixedString(44)),

    -- swaps --
    open0                AggregateFunction(argMin, Float64, UInt64),
    quantile0            AggregateFunction(quantileDeterministic, Float64, UInt64),
    close0               AggregateFunction(argMax, Float64, UInt64),

    -- volume --
    gross_volume0        SimpleAggregateFunction(sum, Float64) COMMENT 'gross volume of token0 in the window',
    gross_volume1        SimpleAggregateFunction(sum, Float64) COMMENT 'gross volume of token1 in the window',
    net_flow0            SimpleAggregateFunction(sum, Float64) COMMENT 'net flow of token0 in the window',
    net_flow1            SimpleAggregateFunction(sum, Float64) COMMENT 'net flow of token1 in the window',

    -- universal --
    uaw                  AggregateFunction(uniq, FixedString(44)) COMMENT 'unique wallet addresses in the window',
    transactions         SimpleAggregateFunction(sum, UInt64) COMMENT 'number of transactions in the window',

    -- indexes --
    INDEX idx_protocol          (protocol)                  TYPE set(4)         GRANULARITY 4,
    INDEX idx_program_id        (program_id)                TYPE set(64)        GRANULARITY 4,
    -- INDEX idx_fee               (fee)                       TYPE minmax         GRANULARITY 4,
    INDEX idx_token0            (token0)                    TYPE set(64)        GRANULARITY 4,
    INDEX idx_token1            (token1)                    TYPE set(64)        GRANULARITY 4,
    INDEX idx_token_pair        (token0, token1)            TYPE set(64)        GRANULARITY 4,

    -- indexes (volume) --
    INDEX idx_gross_volume0     (gross_volume0)             TYPE minmax         GRANULARITY 4,
    INDEX idx_gross_volume1     (gross_volume1)             TYPE minmax         GRANULARITY 4,
    INDEX idx_net_flow0         (net_flow0)                 TYPE minmax         GRANULARITY 4,
    INDEX idx_net_flow1         (net_flow1)                 TYPE minmax         GRANULARITY 4,
    INDEX idx_transactions      (transactions)              TYPE minmax         GRANULARITY 4,

    -- -- indexes (canonical pair) --
    -- INDEX idx_canonical_pair    (canonical0, canonical1)    TYPE set(64)        GRANULARITY 4,
    -- INDEX idx_canonical_pair0   (canonical0)                TYPE set(64)        GRANULARITY 4,
    -- INDEX idx_canonical_pair1   (canonical1)                TYPE set(64)        GRANULARITY 4
)
ENGINE = AggregatingMergeTree
ORDER BY (pool, timestamp);

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_ohlc_prices
-- REFRESH EVERY 1 HOUR OFFSET 5 MINUTE APPEND
TO ohlc_prices
AS
WITH
    any(s.token0) AS t0,
    any(s.token1) AS t1,
    pow(10, m0.decimals) AS scale0,
    pow(10, m1.decimals) AS scale1
SELECT
    toStartOfHour(s.timestamp)  AS timestamp,
    s.pool                      AS pool,
    any(s.protocol)             AS protocol,
    any(s.program_id)           AS program_id,

    -- token0 metadata --
    t0                      AS token0,
    any(m0.decimals)        AS decimals0,
    -- anyLast(m0.symbol)      AS symbol0,
    -- anyLast(m0.name)        AS name0,

    -- token1 metadata --
    t1                      AS token1,
    any(m1.decimals)        AS decimals1,
    -- anyLast(m1.symbol)      AS symbol1,
    -- anyLast(m1.name)        AS name1,

    -- -- canonical pair --
    -- if(t0 < t1, t0, t1) AS canonical0,
    -- if(t0 < t1, t1, t0) AS canonical1,

    -- swaps --
    argMinState(s.price * scale0 / scale1, s.global_sequence)                AS open0,
    quantileDeterministicState(s.price * scale0 / scale1, s.global_sequence) AS quantile0,
    argMaxState(s.price * scale0 / scale1, s.global_sequence)                AS close0,

    -- volume --
    sum(abs(s.amount0) / scale0)        AS gross_volume0,
    sum(abs(s.amount1) / scale1)        AS gross_volume1,
    sum(s.amount0 / scale0)             AS net_flow0,
    sum(s.amount1 / scale1)             AS net_flow1,

    -- universal --
    uniqState(s.sender)                AS uaw,
    count()                             AS transactions
FROM swaps AS s
LEFT JOIN mints AS m0 ON m0.mint = s.token0
LEFT JOIN mints AS m1 ON m1.mint = s.token1
GROUP BY pool, timestamp;