-- OHLCV prices --
CREATE TABLE IF NOT EXISTS ohlc_prices (
    timestamp               UInt32 COMMENT 'beginning of the bar',
    datetime                DateTime('UTC', 0) MATERIALIZED toDateTime(timestamp, 'UTC'),

    -- OrderBy --
    program_id              LowCardinality(FixedString(44)),
    program_name            LowCardinality(String) MATERIALIZED program_names(program_id),
    amm                     LowCardinality(FixedString(44)),
    amm_name                LowCardinality(String) MATERIALIZED program_names(amm),
    amm_pool                LowCardinality(FixedString(44)),
    mint0                   LowCardinality(FixedString(44)),
    mint0_type              LowCardinality(String) MATERIALIZED token_types(mint0),
    mint0_name              LowCardinality(String) MATERIALIZED token_names(mint0),
    mint1                   LowCardinality(FixedString(44)),
    mint1_type              LowCardinality(String) MATERIALIZED token_types(mint1),
    mint1_name              LowCardinality(String) MATERIALIZED token_names(mint1),

    -- Aggregate --
    open0                   AggregateFunction(argMin, Float64, UInt64),
    quantile0               AggregateFunction(quantileDeterministic, Float64, UInt64),
    close0                  AggregateFunction(argMax, Float64, UInt64),

    -- volume --
    gross_volume0           SimpleAggregateFunction(sum, Int128) COMMENT 'gross volume of token0 in the window',
    gross_volume1           SimpleAggregateFunction(sum, Int128) COMMENT 'gross volume of token1 in the window',
    net_flow0               SimpleAggregateFunction(sum, Int128) COMMENT 'net flow of token0 in the window',
    net_flow1               SimpleAggregateFunction(sum, Int128) COMMENT 'net flow of token1 in the window',

    -- universal --
    uaw                     AggregateFunction(uniq, FixedString(44)) COMMENT 'unique wallet addresses in the window',
    transactions            SimpleAggregateFunction(sum, UInt64) COMMENT 'number of transactions in the window',

    -- indexes --
    INDEX idx_timestamp         (timestamp)                 TYPE minmax         GRANULARITY 1,
    INDEX idx_program_id        (program_id)                TYPE set(8)         GRANULARITY 4,
    INDEX idx_amm               (amm)                       TYPE set(128)       GRANULARITY 2,
    INDEX idx_amm_pool          (amm_pool)                  TYPE set(512)       GRANULARITY 1,
    INDEX idx_mint0             (mint0)                     TYPE set(512)       GRANULARITY 1,
    INDEX idx_mint1             (mint1)                     TYPE set(512)       GRANULARITY 1,
    INDEX idx_mint_pair         (mint0, mint1)              TYPE set(512)       GRANULARITY 1,
    INDEX idx_mint0_type        (mint0_type)                TYPE set(4)         GRANULARITY 1, -- USD,ETH,BTC,SOL
    INDEX idx_mint1_type        (mint1_type)                TYPE set(4)         GRANULARITY 1, -- USD,ETH,BTC,SOL
    INDEX idx_mint0_name        (mint0_name)                TYPE set(16)        GRANULARITY 1,
    INDEX idx_mint1_name        (mint1_name)                TYPE set(16)        GRANULARITY 1,

    -- indexes (volume) --
    INDEX idx_gross_volume0     (gross_volume0)             TYPE minmax         GRANULARITY 1,
    INDEX idx_gross_volume1     (gross_volume1)             TYPE minmax         GRANULARITY 1,
    INDEX idx_net_flow0         (net_flow0)                 TYPE minmax         GRANULARITY 1,
    INDEX idx_net_flow1         (net_flow1)                 TYPE minmax         GRANULARITY 1,
    INDEX idx_transactions      (transactions)              TYPE minmax         GRANULARITY 1
)
ENGINE = AggregatingMergeTree
ORDER BY (program_id, amm, amm_pool, mint0, mint1, timestamp)
COMMENT 'OHLCV prices for AMM pools, aggregated by hour';

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_ohlc_prices
REFRESH EVERY 1 MINUTE APPEND
TO ohlc_prices
AS
WITH
    (input_mint <= output_mint) AS dir,
    if (dir, input_mint,  output_mint) AS mint0,
    if (dir, output_mint, input_mint) AS mint1,
    if (dir, input_amount,  output_amount) AS amount0,
    if (dir, output_amount, input_amount) AS amount1,
    toFloat64(amount1) / amount0 AS price,
    abs(amount0) AS gv0,
    abs(amount1) AS gv1,
    -- net flow of mint0: +in, -out
    if(dir, toInt128(input_amount), -toInt128(output_amount))  AS nf0,
    -- net flow of mint1: +in, -out (signs flipped vs. your original)
    if(dir, -toInt128(output_amount), toInt128(input_amount))  AS nf1

SELECT
    toStartOfHour(datetime)    AS timestamp,
    program_id, amm, amm_pool, mint0, mint1,

    /* OHLC */
    argMinState(price, toUInt64(timestamp))                 AS open0,
    quantileDeterministicState(price, toUInt64(timestamp))  AS quantile0,
    argMaxState(price, toUInt64(timestamp))                 AS close0,

    /* volumes & flows (all in canonical orientation) */
    sum(gv0)                AS gross_volume0,
    sum(gv1)                AS gross_volume1,
    sum(nf0)                AS net_flow0,
    sum(nf1)                AS net_flow1,

    /* universal */
    uniqState(user)         AS uaw,
    count()                 AS transactions
FROM swaps
GROUP BY program_id, amm, amm_pool, mint0, mint1, timestamp;

