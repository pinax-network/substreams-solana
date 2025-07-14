/* How many active projection parts exist?  */
SELECT name,
       sum(rows)  AS rows_in_proj,
       sum(data_compressed_bytes) / 1e6 AS size_mb
FROM system.projection_parts
WHERE database = currentDatabase()
  AND table    = 'swaps'
GROUP BY name;


-- Make the optimiser show its hand with EXPLAIN
EXPLAIN PLAN indexes=1
SELECT *
FROM swaps
ORDER BY timestamp, block_num
LIMIT 100
SETTINGS allow_experimental_projection_optimization = 1;

-- with projection (default)
SELECT count() FROM (
    SELECT *
    FROM swaps
    ORDER BY timestamp, block_num
    LIMIT 10000
);

-- disable projection for a control run
SET optimize_use_projections = 0;
SELECT count() FROM (
    SELECT *
    FROM swaps
    ORDER BY timestamp, block_num
    LIMIT 10000
);

-- with projection (default)
SELECT count()
FROM (SELECT * FROM swaps ORDER BY timestamp, block_num LIMIT 10000);

-- disable projections
SET optimize_use_projections = 0;
SELECT count()
FROM (SELECT * FROM swaps ORDER BY timestamp, block_num LIMIT 10000);
