-- OHLCV prices --
CREATE TABLE IF NOT EXISTS ohlc_prices (
    timestamp               DateTime(0, 'UTC') COMMENT 'beginning of the bar',

    -- OrderBy --
    program_id              LowCardinality(FixedString(44)),
    program_name            LowCardinality(FixedString(44)),
    amm                     LowCardinality(FixedString(44)),
    amm_pool                LowCardinality(FixedString(44)),
    amm_name                LowCardinality(String),
    mint0                   LowCardinality(FixedString(44)),
    mint1                   LowCardinality(FixedString(44)),

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
    INDEX idx_program_id        (program_id)                TYPE set(8)         GRANULARITY 4,
    INDEX idx_program_name      (program_name)              TYPE set(8)         GRANULARITY 4,
    INDEX idx_amm               (amm)                       TYPE set(128)       GRANULARITY 4,
    INDEX idx_amm_pool          (amm_pool)                  TYPE set(128)       GRANULARITY 4,
    INDEX idx_amm_name          (amm_name)                  TYPE set(128)       GRANULARITY 4,
    INDEX idx_mint0             (mint0)                     TYPE set(128)       GRANULARITY 4,
    INDEX idx_mint1             (mint1)                     TYPE set(128)       GRANULARITY 4,
    INDEX idx_mint_pair         (mint0, mint1)              TYPE set(128)       GRANULARITY 4,

    -- indexes (volume) --
    INDEX idx_gross_volume0     (gross_volume0)             TYPE minmax         GRANULARITY 4,
    INDEX idx_gross_volume1     (gross_volume1)             TYPE minmax         GRANULARITY 4,
    INDEX idx_net_flow0         (net_flow0)                 TYPE minmax         GRANULARITY 4,
    INDEX idx_net_flow1         (net_flow1)                 TYPE minmax         GRANULARITY 4,
    INDEX idx_transactions      (transactions)              TYPE minmax         GRANULARITY 4
)
ENGINE = AggregatingMergeTree
ORDER BY (program_id, program_name, amm, amm_pool, amm_name, mint0, mint1);

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_ohlc_prices
REFRESH EVERY 10 MINUTE APPEND
TO ohlc_prices
AS
WITH
    (input_mint <= output_mint) AS dir,
    if (dir, input_mint,  output_mint) AS mint0,
    if (dir, output_mint, input_mint) AS mint1,
    if (dir, input_amount,  output_amount) AS amt0,
    if (dir, output_amount, input_amount) AS amt1,
    toFloat64(amt0) / amt1 AS px,
    abs(amt0) AS gv0,
    abs(amt1) AS gv1,
    if (dir,  toInt128(input_amount), -toInt128(output_amount)) AS nf0,
    if (dir,  toInt128(output_amount), -toInt128(input_amount)) AS nf1

SELECT
    toStartOfHour(timestamp)                               AS timestamp,
    program_id, program_name, amm, amm_pool, amm_name,
    mint0, mint1,

    /* OHLC */
    argMinState(px,  toUInt64(block_num))                  AS open0,
    quantileDeterministicState(px, toUInt64(block_num))    AS quantile0,
    argMaxState(px,  toUInt64(block_num))                  AS close0,

    /* volumes & flows (all in canonical orientation) */
    sum(gv0)                                               AS gross_volume0,
    sum(gv1)                                               AS gross_volume1,
    sum(nf0)                                               AS net_flow0,
    sum(nf1)                                               AS net_flow1,

    /* universal */
    uniqState(user)                                        AS uaw,
    count()                                                AS transactions
FROM swaps
GROUP BY timestamp, program_id, program_name, amm, amm_pool, amm_name, mint0, mint1;

