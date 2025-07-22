-- When you run ALTER TABLE … ADD INDEX … ClickHouse only stores the metadata for the new skip-index; existing data parts are left unchanged.
-- To build (or rebuild) the index files for data that was already on disk you have to trigger a mutation:

-- remove from every shard / replica
ALTER TABLE transfers
    ON CLUSTER 'tokenapis-b'
    DROP INDEX IF EXISTS idx_mint
    SETTINGS mutations_sync = 2;   -- wait until finished on all replicas

-- create the index description (metadata only)
ALTER TABLE transfers ON CLUSTER 'tokenapis-b'
    ADD INDEX idx_mint (mint) TYPE set(512) GRANULARITY 1
    SETTINGS mutations_sync = 2;   -- wait until finished on all replicas

-- How to Let the Process Run for Longer
SET distributed_ddl_task_timeout = 600; -- sets timeout to 10 minutes

-- now materialise it for all historical data
ALTER TABLE transfers ON CLUSTER 'tokenapis-b'
    MATERIALIZE INDEX idx_mint   -- optional: IN PARTITION '202507'
    SETTINGS mutations_sync = 2; -- wait until the mutation finishes

-- check the index
EXPLAIN indexes = 1
SELECT * FROM transfers WHERE mint = '2wwDDbLtfvd7xy18hGkwGRmiF3wTQgQed9sfiNsipump' LIMIT 10;

-- check queue
SELECT query, status FROM system.distributed_ddl_queue
WHERE status != 'Finished'

-- detect how many unique values
WITH t AS (
    SELECT signer, * FROM swaps LIMIT 8192 OFFSET 8000000
) SELECT
    uniq(amm),
    uniq(amm_pool),
    uniq(user),
    uniq(signer),
    uniq(fee_payer),
    uniq(input_mint),
    uniq(output_mint)
FROM t

-- check the index sizes
SELECT
    name,
    formatReadableSize(data_compressed_bytes) AS on_disk,
    type,
    granularity
FROM system.data_skipping_indices
WHERE table = 'swaps'
ORDER BY data_compressed_bytes DESC;


SYSTEM FLUSH LOGS;       -- zero the deltas
-- run a representative INSERT batch
SELECT event, value
FROM system.events
WHERE event IN (
    'MergeTreeDataProjectionWriterProjectionsCalculationMicroseconds',
    'MergeTreeDataWriterSkipIndicesCalculationMicroseconds',
    'MergeTreeDataProjectionWriterUncompressedBytes',
    'MergeTreeDataWriterUncompressedBytes',
    'InsertQueryTimeMicroseconds');

-- drop the index
ALTER TABLE swaps
DROP INDEX idx_block_num;
