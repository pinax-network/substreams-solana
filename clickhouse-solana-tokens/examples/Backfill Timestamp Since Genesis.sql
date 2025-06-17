-- removes the old definition
ALTER TABLE blocks
DROP INDEX idx_timestamp_since_genesis;

ALTER TABLE blocks
DROP COLUMN timestamp_since_genesis;

-- Add a new column to store the timestamp since genesis
ALTER TABLE blocks
ADD COLUMN timestamp_since_genesis DateTime('UTC')
MATERIALIZED if (
    timestamp = 0,
    toDateTime(1584332940 + intDiv(block_num * 2, 5), 'UTC'),
    timestamp
);

-- Add index for the new column
ALTER TABLE blocks
ADD INDEX idx_timestamp_since_genesis (timestamp_since_genesis) TYPE minmax GRANULARITY 4;

-- rebuild existing parts
ALTER TABLE blocks
MATERIALIZE COLUMN timestamp_since_genesis;

-- Force recreation of ALL secondary indexes by rewriting parts
OPTIMIZE TABLE blocks FINAL;