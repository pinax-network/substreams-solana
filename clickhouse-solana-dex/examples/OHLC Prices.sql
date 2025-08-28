-- Top Transactions by AMM Pool --
SELECT
      amm_name,
      mint0,
      mint1,
      sum(transactions) AS transactions
FROM ohlc_prices
WHERE amm_pool != ''
GROUP BY amm_name, amm_pool, mint0, mint1
ORDER BY transactions DESC
LIMIT 20

-- OHLC Prices by Pool --
WITH (
      9 AS decimals0, -- JOIN solana-tokens::initialize_mints
      6 AS decimals1, -- JOIN solana-tokens::initialize_mints
      2 AS precision -- user defined
) SELECT
      timestamp,
      'WSOL/USDC' AS ticker,

      -- OHLC --
      floor(argMinMerge(open0) * pow(10, decimals0 - decimals1), precision)                        AS open,
      floor(quantileDeterministicMerge(0.99)(quantile0) * pow(10, decimals0 - decimals1), precision)   AS high,
      floor(quantileDeterministicMerge(0.01)(quantile0) * pow(10, decimals0 - decimals1), precision)    AS low,
      floor(argMaxMerge(close0) * pow(10, decimals0 - decimals1), precision)                       AS close,

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

-- Minimal OHLC Prices --
SELECT
      timestamp,
      argMinMerge(open0) * 1000 AS open,
      argMaxMerge(close0) * 1000 AS close
FROM ohlc_prices
WHERE amm_pool = '58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2' -- Raydium V4 WSOL/USDC
GROUP BY amm_pool, timestamp
ORDER BY timestamp DESC
LIMIT 10;
