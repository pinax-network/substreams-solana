ALTER TABLE closed_state ON CLUSTER 'tokenapis-b'
    ADD INDEX IF NOT EXISTS idx_closed (closed) TYPE set(2) GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_block_num (block_num) TYPE minmax GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_timestamp (timestamp) TYPE minmax GRANULARITY 1;

ALTER TABLE closed_state ON CLUSTER 'tokenapis-b'
    MATERIALIZE INDEX idx_closed,
    MATERIALIZE INDEX idx_block_num,
    MATERIALIZE INDEX idx_timestamp;
