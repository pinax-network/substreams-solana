/* How many active projection parts exist?  */
SELECT name,
    sum(rows) AS total_rows,
    formatReadableSize(sum(data_compressed_bytes)) AS on_disk
FROM system.projection_parts
WHERE database = currentDatabase()
  AND table    = 'swaps' AND active = 1
GROUP BY name
ORDER BY sum(data_compressed_bytes) DESC;

-- Table size --
SELECT
    table,
    sum(rows) AS total_rows,
    formatReadableSize(sum(data_compressed_bytes)) AS on_disk
FROM system.parts
WHERE table = 'swaps' AND active
GROUP BY table
ORDER BY sum(data_compressed_bytes) DESC;

-- check the index sizes
SELECT
    name,
    formatReadableSize(data_compressed_bytes) AS on_disk,
    type,
    granularity
FROM system.data_skipping_indices
WHERE table = 'swaps'
ORDER BY data_compressed_bytes DESC;

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


-- Add Projection to the Swaps table
ALTER TABLE swaps DROP PROJECTION prj_timestamp;

ALTER TABLE swaps
ADD PROJECTION prj_timestamp
(
  SELECT *
  ORDER BY timestamp
);

ALTER TABLE swaps
ADD PROJECTION prj_timestamp
(
  SELECT timestamp, _part_offset
  ORDER BY timestamp
);

-- Materialise existing data (one‑off, runs in the background)
ALTER TABLE swaps MATERIALIZE PROJECTION prj_timestamp;

-- Check the projection settings
SELECT
    name,
    value
FROM system.settings
WHERE name LIKE '%projection%'

-- Enable trace
SET send_logs_level='trace';
EXPLAIN PIPELINE
SELECT ...;

-- Query signature
EXPLAIN indexes = 1
SELECT signature
FROM swaps
WHERE signature = (SELECT signature FROM swaps ORDER BY rand() LIMIT 1);


EXPLAIN indexes=1
SELECT timestamp, block_num
FROM swaps
WHERE timestamp >= timestamp('2025-06-28 03:00:00Z') + INTERVAL 300 MINUTE
ORDER BY timestamp;

EXPLAIN indexes=1
SELECT timestamp, block_num
FROM swaps_min
WHERE block_num <= 20001000
ORDER BY timestamp;

EXPLAIN indexes = 1
SELECT input_mint
FROM swaps
WHERE input_mint = (SELECT input_mint FROM swaps ORDER BY rand() LIMIT 1);

EXPLAIN indexes = 1
SELECT *
FROM swaps
WHERE user = (SELECT user FROM swaps ORDER BY rand() LIMIT 1);


SELECT
    name,
    value
FROM system.settings
WHERE name IN (
  'optimize_use_projections',
  'optimize_use_projection_filtering',
  'query_plan_optimize_projection',
  'optimize_use_implicit_projections',
  'allow_experimental_projection_optimization',
  'use_skip_indexes',
  'enable_shared_storage_snapshot_in_query'
);

   ┌─name──────────────────────────────┬─value─┐
1. │ optimize_use_projections          │ 1     │
2. │ optimize_use_projection_filtering │ 1     │
3. │ query_plan_optimize_projection    │ 1     │
   └───────────────────────────────────┴───────┘

EXPLAIN indexes = 1
SELECT *
FROM swaps
WHERE user = (SELECT user FROM swaps ORDER BY rand() LIMIT 1);

EXPLAIN indexes =1
SELECT *
FROM swaps
WHERE _part_starting_offset + _part_offset IN (
    SELECT _part_starting_offset + _part_offset
    FROM swaps
    WHERE user = (SELECT user FROM swaps ORDER BY rand() LIMIT 1)
)
SETTINGS enable_shared_storage_snapshot_in_query = 1;

EXPLAIN indexes =1
SELECT *
FROM swaps
WHERE _part_starting_offset + _part_offset IN (
    SELECT _part_starting_offset + _part_offset
    FROM swaps
    WHERE signature = (SELECT signature FROM swaps ORDER BY rand() LIMIT 1)
);

EXPLAIN indexes =1
SELECT *
FROM pumpfun_amm_buy
WHERE fee_payer = (SELECT fee_payer FROM pumpfun_amm_buy ORDER BY rand() LIMIT 1);

EXPLAIN indexes =1
SELECT *
FROM pumpfun_amm_buy
WHERE _part_starting_offset + _part_offset IN (
    SELECT _part_starting_offset + _part_offset
    FROM pumpfun_amm_buy
    WHERE signature = (SELECT signature FROM pumpfun_amm_buy ORDER BY rand() LIMIT 1)
);

EXPLAIN indexes =1
SELECT *
FROM swaps
ORDER BY timestamp DESC LIMIT 10;

EXPLAIN indexes = 1
SELECT * FROM swaps
WHERE program_id = (SELECT program_id FROM swaps ORDER BY rand() LIMIT 1)
ORDER BY timestamp DESC LIMIT 10;

EXPLAIN indexes = 1
SELECT * FROM swaps
WHERE amm = (SELECT amm FROM swaps ORDER BY rand() LIMIT 1)
ORDER BY timestamp DESC LIMIT 10;

EXPLAIN indexes = 1
SELECT * FROM swaps
WHERE amm_pool = (SELECT amm_pool FROM swaps WHERE amm_pool !=''  ORDER BY rand() LIMIT 1)
ORDER BY timestamp DESC LIMIT 10;

EXPLAIN indexes = 1
SELECT * FROM swaps
WHERE timestamp <= (SELECT min(timestamp) + 100 FROM swaps)
ORDER BY timestamp DESC LIMIT 10;

EXPLAIN projections = 1
SELECT * FROM swaps
WHERE program_id = (SELECT program_id FROM swaps ORDER BY rand() LIMIT 1)
AND timestamp <= (SELECT min(timestamp) + 100 FROM swaps)
ORDER BY timestamp DESC LIMIT 10;


EXPLAIN indexes = 1
SELECT _part_starting_offset + _part_offset
FROM swaps
WHERE
    input_mint = (SELECT input_mint FROM swaps ORDER BY rand() LIMIT 1)


EXPLAIN indexes =1
SELECT *
FROM swaps
WHERE _part_starting_offset + _part_offset IN (
    SELECT _part_starting_offset + _part_offset
    FROM swaps
    WHERE input_mint = (SELECT input_mint FROM swaps ORDER BY rand() LIMIT 1)
) AND _part_starting_offset + _part_offset IN (
    SELECT _part_starting_offset + _part_offset
    FROM swaps
    WHERE amm = (SELECT amm FROM swaps ORDER BY rand() LIMIT 1)
)
SETTINGS enable_shared_storage_snapshot_in_query = 1;