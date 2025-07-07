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
      'WSOL/USDT' AS ticker,

      -- OHLC --
      floor(argMinMerge(open0) * pow(10, decimals0 - decimals1), precision)                        AS open,
      floor(quantileDeterministicMerge(0.99)(quantile0) * pow(10, decimals0 - decimals1), precision)   AS high,
      floor(quantileDeterministicMerge(0.01)(quantile0) * pow(10, decimals0 - decimals1), precision)    AS low,
      floor(argMaxMerge(close0) * pow(10, decimals0 - decimals1), precision)                       AS close,

      -- volume --
      floor(sum(gross_volume0) / pow(10, decimals0), precision)         AS "gross volume (WSOL)",
      floor(sum(gross_volume1) / pow(10, decimals1), precision)         AS "gross volume (USDT)",
      floor(sum(net_flow0) / pow(10, decimals0), precision)             AS "net flow (WSOL)",
      floor(sum(net_flow1) / pow(10, decimals1), precision)             AS "net flow (USDT)",

      -- universal --
      uniqMerge(uaw)          AS uaw,
      sum(transactions)       AS transactions
FROM ohlc_prices
WHERE pool = '7XawhbbxtsRcQA8KTkHT9f9nc6d69UwqCDh6U5EEbEmX' -- Raydium V4 WSOL/USDT
GROUP BY pool, timestamp
ORDER BY timestamp DESC
LIMIT 10;
