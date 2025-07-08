-- Top Uniswap pools --
SELECT
    protocol,
    pool,
    count()
FROM swaps
GROUP BY protocol, pool
ORDER BY count() DESC
LIMIT 10

-- OHLC Prices by Pool --
WITH (
      9 AS decimals0, -- JOIN solana-tokens::initialize_mints
      6 AS decimals1, -- JOIN solana-tokens::initialize_mints
      2 AS precision -- user defined
) SELECT
      timestamp,
      'WSOL/USDC' AS ticker,

      -- OHLC --
      floor(1/argMinMerge(open0) * pow(10, decimals0 - decimals1), precision)                        AS open,
      floor(1/quantileDeterministicMerge(0.99)(quantile0) * pow(10, decimals0 - decimals1), precision)   AS high,
      floor(1/quantileDeterministicMerge(0.01)(quantile0) * pow(10, decimals0 - decimals1), precision)    AS low,
      floor(1/argMaxMerge(close0) * pow(10, decimals0 - decimals1), precision)                       AS close,

      -- volume --
      floor(sum(gross_volume0) / pow(10, decimals0), precision)         AS "gross volume (WSOL)",
      floor(sum(gross_volume1) / pow(10, decimals1), precision)         AS "gross volume (USDC)",
      floor(sum(net_flow0) / pow(10, decimals0), precision)             AS "net flow (WSOL)",
      floor(sum(net_flow1) / pow(10, decimals1), precision)             AS "net flow (USDC)",

      -- universal --
      uniqMerge(uaw)          AS uaw,
      sum(transactions)       AS transactions
FROM ohlc_prices
WHERE amm_pool = '58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2' -- Raydium V4 WSOL/USDC
GROUP BY amm_pool, mint0, mint1, timestamp
ORDER BY timestamp DESC
LIMIT 10;


-- Price by Pool --
WITH (
    input_mint <= output_mint AS dir,
    input_amount / output_amount AS px
)
SELECT
    input_amount,
    output_amount,
    input_mint,
    output_mint,
    if (dir, px, 1 / px) AS price
FROM swaps
WHERE amm_pool = '58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2'
