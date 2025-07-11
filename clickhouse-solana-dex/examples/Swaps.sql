-- Count all DEXs --
SELECT
    program_name,
    amm_name,
    count()
FROM swaps
GROUP BY
    program_name,
    amm_name
ORDER BY count() DESC
LIMIT 50;

-- Find all AMMs with Unknown name --
SELECT
    amm,
    count()
FROM swaps
WHERE amm_name = 'Unknown'
GROUP BY amm
ORDER BY count() DESC
LIMIT 100

-- Price by Pool WSOL/USDC --
WITH (
    input_mint <= output_mint AS dir,
    input_amount / output_amount AS px
)
SELECT
    input_amount,
    output_amount,
    input_mint,
    output_mint,
    if (dir, 1/px, px) * 1000 AS price
FROM swaps
WHERE amm_pool = '58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2' LIMIT 20;

-- Query Swap --
SELECT
    amm_name,
    input_amount,
    input_mint,
    output_amount,
    output_mint
FROM swaps
ORDER BY timestamp DESC
LIMIT 20

-- Query Pump.fun - Detail --
SELECT * FROM pumpfun_amm_buy LIMIT 1\G;