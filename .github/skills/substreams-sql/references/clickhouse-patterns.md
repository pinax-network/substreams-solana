# ClickHouse Patterns and Optimization

Advanced patterns for high-performance analytics with ClickHouse and Substreams.

## ClickHouse-Optimized Schema Design

### MergeTree Engine Configuration

```sql
-- Primary table for ERC20 transfers (analytics-optimized)
CREATE TABLE erc20_transfers (
    block_number UInt64,
    block_timestamp DateTime,
    tx_hash String,
    log_index UInt32,
    contract_address String,
    from_addr String,
    to_addr String,
    amount UInt256,
    
    -- Pre-computed time dimensions for analytics
    hour DateTime,
    date Date,
    week_start Date,
    month_start Date,
    
    -- Contract metadata (denormalized for performance)
    token_symbol LowCardinality(String),
    token_decimals UInt8,
    
    -- Calculated fields
    amount_formatted Float64 MATERIALIZED amount / pow(10, token_decimals),
    is_large_transfer UInt8 MATERIALIZED if(amount > 1000000000000000000, 1, 0)
    
) ENGINE = MergeTree()
PARTITION BY toYYYYMM(date)
ORDER BY (contract_address, block_timestamp, block_number, log_index)
SETTINGS index_granularity = 8192;

-- Blocks table optimized for time-series queries  
CREATE TABLE blocks (
    number UInt64,
    hash String,
    parent_hash String,
    timestamp DateTime,
    gas_limit UInt64,
    gas_used UInt64,
    tx_count UInt32,
    size_bytes UInt64,
    
    -- Time dimensions
    hour DateTime MATERIALIZED toStartOfHour(timestamp),
    date Date MATERIALIZED toDate(timestamp),
    
    -- Derived metrics
    gas_utilization Float32 MATERIALIZED gas_used / gas_limit,
    avg_gas_per_tx UInt64 MATERIALIZED gas_used / tx_count
    
) ENGINE = MergeTree()
ORDER BY number
SETTINGS index_granularity = 8192;

-- Transaction table with optimized sorting
CREATE TABLE transactions (
    hash String,
    block_number UInt64,
    tx_index UInt32,
    from_addr String,
    to_addr String,
    value UInt256,
    gas_limit UInt64,
    gas_used UInt64,
    gas_price UInt256,
    status UInt8,
    
    -- Time context (denormalized)
    block_timestamp DateTime,
    date Date MATERIALIZED toDate(block_timestamp),
    
    -- Calculated fields
    tx_fee UInt256 MATERIALIZED gas_used * gas_price,
    gas_efficiency Float32 MATERIALIZED gas_used / gas_limit
    
) ENGINE = MergeTree()
PARTITION BY toYYYYMM(date)
ORDER BY (block_number, tx_index)
SETTINGS index_granularity = 8192;
```

### Specialized Analytics Tables

```sql
-- Hourly aggregated token metrics
CREATE TABLE token_hourly_stats (
    contract_address String,
    hour DateTime,
    
    -- Transfer metrics
    transfer_count UInt64,
    unique_senders UInt32,
    unique_receivers UInt32,
    unique_addresses UInt32,
    
    -- Volume metrics  
    total_volume UInt256,
    avg_transfer_size Float64,
    median_transfer_size Float64,
    max_transfer_size UInt256,
    
    -- Price data (if available)
    price_usd Float64,
    volume_usd Float64,
    
    -- Derived metrics
    velocity Float64,  -- volume / circulating supply
    concentration Float64  -- top 10% holder concentration
    
) ENGINE = MergeTree()
PARTITION BY toYYYYMM(toDate(hour))
ORDER BY (contract_address, hour)
SETTINGS index_granularity = 8192;

-- Real-time materialized view for hourly stats
CREATE MATERIALIZED VIEW token_hourly_stats_mv TO token_hourly_stats AS
SELECT
    contract_address,
    toStartOfHour(block_timestamp) as hour,
    count() as transfer_count,
    uniq(from_addr) as unique_senders,
    uniq(to_addr) as unique_receivers,
    uniq(from_addr, to_addr) as unique_addresses,
    sum(amount) as total_volume,
    avg(amount) as avg_transfer_size,
    quantile(0.5)(amount) as median_transfer_size,
    max(amount) as max_transfer_size,
    -- Add price lookups here when available
    0 as price_usd,  -- Placeholder
    0 as volume_usd,
    0 as velocity,
    0 as concentration
FROM erc20_transfers
GROUP BY contract_address, toStartOfHour(block_timestamp);
```

### Advanced Data Types and Compression

```sql
-- Optimized storage with compression
CREATE TABLE erc20_transfers_compressed (
    block_number UInt64 Codec(Delta, ZSTD),  -- Delta compression for sequential data
    block_timestamp DateTime Codec(DoubleDelta, ZSTD),  -- Time series compression
    tx_hash String Codec(ZSTD),
    log_index UInt32 Codec(ZSTD),
    contract_address LowCardinality(String),  -- Limited cardinality optimization
    from_addr String Codec(ZSTD),
    to_addr String Codec(ZSTD),
    amount UInt256 Codec(ZSTD),
    
    -- Nested structures for complex data
    decoded_event Nested (
        name String,
        indexed Bool,
        value String
    )
    
) ENGINE = MergeTree()
PARTITION BY toYYYYMM(toDate(block_timestamp))
ORDER BY (contract_address, block_timestamp)
SETTINGS index_granularity = 8192;

-- Array columns for efficient multi-value storage
CREATE TABLE transaction_logs_optimized (
    tx_hash String,
    block_number UInt64,
    
    -- Arrays for multiple log entries per transaction
    log_indexes Array(UInt32),
    addresses Array(String),
    topics Array(Array(String)),  -- Nested array for topics
    data Array(String),
    
    -- Aggregate metrics
    log_count UInt32 MATERIALIZED length(log_indexes),
    unique_contracts UInt32 MATERIALIZED length(arrayDistinct(addresses))
    
) ENGINE = MergeTree()
ORDER BY (block_number, tx_hash)
SETTINGS index_granularity = 8192;
```

## High-Performance Analytics Queries

### Time-Series Analysis

```sql
-- Token volume trends with moving averages
WITH daily_volumes AS (
    SELECT 
        contract_address,
        toDate(block_timestamp) as date,
        sum(amount_formatted) as daily_volume,
        count() as transfer_count,
        uniq(from_addr, to_addr) as unique_participants
    FROM erc20_transfers
    WHERE date >= today() - INTERVAL 30 DAY
    GROUP BY contract_address, date
)
SELECT 
    contract_address,
    date,
    daily_volume,
    transfer_count,
    unique_participants,
    
    -- Moving averages
    avg(daily_volume) OVER (
        PARTITION BY contract_address 
        ORDER BY date 
        ROWS BETWEEN 6 PRECEDING AND CURRENT ROW
    ) as volume_7d_avg,
    
    avg(transfer_count) OVER (
        PARTITION BY contract_address 
        ORDER BY date 
        ROWS BETWEEN 29 PRECEDING AND CURRENT ROW
    ) as transfers_30d_avg,
    
    -- Growth rates
    (daily_volume - lag(daily_volume, 7) OVER (PARTITION BY contract_address ORDER BY date)) 
    / lag(daily_volume, 7) OVER (PARTITION BY contract_address ORDER BY date) as volume_7d_growth
    
FROM daily_volumes
WHERE contract_address IN (
    SELECT contract_address 
    FROM erc20_transfers 
    WHERE date >= today() - INTERVAL 7 DAY
    GROUP BY contract_address 
    HAVING sum(amount_formatted) > 1000000  -- Focus on high-volume tokens
)
ORDER BY contract_address, date;

-- Hourly activity patterns
SELECT 
    toHour(block_timestamp) as hour_of_day,
    toDayOfWeek(block_timestamp) as day_of_week,
    count() as transfer_count,
    sum(amount_formatted) as total_volume,
    uniq(from_addr) as unique_senders,
    avg(amount_formatted) as avg_transfer_size,
    
    -- Percentile analysis
    quantile(0.5)(amount_formatted) as median_transfer,
    quantile(0.95)(amount_formatted) as p95_transfer,
    quantile(0.99)(amount_formatted) as p99_transfer
    
FROM erc20_transfers
WHERE block_timestamp >= now() - INTERVAL 7 DAY
GROUP BY hour_of_day, day_of_week
ORDER BY day_of_week, hour_of_day;
```

### Advanced Aggregation Patterns

```sql
-- Multi-dimensional token analysis
SELECT 
    contract_address,
    token_symbol,
    
    -- Volume metrics
    sum(amount_formatted) as total_volume,
    count() as transfer_count,
    
    -- User metrics
    uniq(from_addr) as unique_senders,
    uniq(to_addr) as unique_receivers, 
    uniq(from_addr, to_addr) as total_unique_users,
    
    -- Distribution metrics
    quantile(0.1)(amount_formatted) as p10_transfer,
    quantile(0.5)(amount_formatted) as median_transfer,
    quantile(0.9)(amount_formatted) as p90_transfer,
    
    -- Concentration metrics
    sum(if(amount_formatted > quantile(0.9)(amount_formatted), amount_formatted, 0)) 
    / sum(amount_formatted) as top_10pct_volume_share,
    
    -- Activity patterns
    countIf(toHour(block_timestamp) BETWEEN 9 AND 17) / count() as business_hours_ratio,
    countIf(toDayOfWeek(block_timestamp) IN (6, 7)) / count() as weekend_ratio,
    
    -- Growth metrics
    countIf(block_timestamp >= now() - INTERVAL 1 DAY) as transfers_24h,
    countIf(block_timestamp >= now() - INTERVAL 7 DAY) as transfers_7d,
    countIf(block_timestamp >= now() - INTERVAL 30 DAY) as transfers_30d,
    
    -- Velocity (daily volume / market cap estimate)
    sum(amount_formatted) / (max(block_number) - min(block_number)) * 7200 as daily_velocity_estimate
    
FROM erc20_transfers
WHERE block_timestamp >= now() - INTERVAL 30 DAY
GROUP BY contract_address, token_symbol
HAVING transfer_count > 100  -- Filter for active tokens
ORDER BY total_volume DESC
LIMIT 100;

-- Cross-token correlation analysis  
WITH token_daily_volumes AS (
    SELECT 
        contract_address,
        toDate(block_timestamp) as date,
        sum(amount_formatted) as daily_volume
    FROM erc20_transfers
    WHERE toDate(block_timestamp) >= today() - INTERVAL 30 DAY
    GROUP BY contract_address, date
),
token_pairs AS (
    SELECT 
        a.contract_address as token_a,
        b.contract_address as token_b, 
        a.date,
        a.daily_volume as volume_a,
        b.daily_volume as volume_b
    FROM token_daily_volumes a
    JOIN token_daily_volumes b ON a.date = b.date
    WHERE a.contract_address < b.contract_address  -- Avoid duplicate pairs
      AND a.daily_volume > 10000 AND b.daily_volume > 10000  -- High volume only
)
SELECT 
    token_a,
    token_b,
    count() as days_active,
    corr(volume_a, volume_b) as volume_correlation,
    
    -- Synchronized activity detection
    countIf(volume_a > avgIf(volume_a, volume_a > 0) AND volume_b > avgIf(volume_b, volume_b > 0)) 
    / count() as high_activity_sync_ratio
    
FROM token_pairs
GROUP BY token_a, token_b
HAVING days_active >= 20  -- Sufficient data points
ORDER BY abs(volume_correlation) DESC;
```

### Real-time Analytics with Projections

```sql
-- Create projection for common query patterns
ALTER TABLE erc20_transfers ADD PROJECTION hourly_token_stats (
    SELECT 
        contract_address,
        toStartOfHour(block_timestamp) as hour,
        count() as transfer_count,
        sum(amount) as total_volume,
        uniq(from_addr) as unique_senders,
        uniq(to_addr) as unique_receivers
    GROUP BY contract_address, hour
    ORDER BY contract_address, hour
);

-- Materialize the projection
ALTER TABLE erc20_transfers MATERIALIZE PROJECTION hourly_token_stats;

-- Query automatically uses projection
SELECT 
    contract_address,
    hour,
    transfer_count,
    total_volume,
    unique_senders + unique_receivers as total_unique_addresses
FROM erc20_transfers
WHERE contract_address = '0xA0b86a33E6Fe17d67C8c086c6c4c0E3C8E3B7EC2'  -- USDC
  AND hour >= now() - INTERVAL 24 HOUR
ORDER BY hour DESC;
```

## Distributed and Sharded Deployments

### Cluster Configuration

```sql
-- Create distributed table across cluster
CREATE TABLE erc20_transfers_distributed AS erc20_transfers
ENGINE = Distributed('clickhouse_cluster', 'default', 'erc20_transfers', cityHash64(contract_address));

-- Replicated MergeTree for high availability  
CREATE TABLE erc20_transfers_replicated (
    -- Same schema as before
    block_number UInt64,
    block_timestamp DateTime,
    contract_address String,
    -- ... other columns
) ENGINE = ReplicatedMergeTree('/clickhouse/tables/{layer}-{shard}/erc20_transfers', '{replica}')
PARTITION BY toYYYYMM(toDate(block_timestamp))
ORDER BY (contract_address, block_timestamp, block_number)
SETTINGS index_granularity = 8192;

-- Sharding strategy by token contract
CREATE TABLE erc20_transfers_sharded AS erc20_transfers_replicated
ENGINE = Distributed('clickhouse_cluster', 'default', 'erc20_transfers_replicated', 
                     intHash64(cityHash64(contract_address)));
```

### Optimized Ingestion Patterns

```yaml
# Manifest sink configuration for ClickHouse
# Note: DSN is passed on the CLI, not in the manifest
sink:
  module: db_out
  type: sf.substreams.sink.sql.v1.Service
  config:
    schema: ./clickhouse-schema.sql
    engine: clickhouse
```

```bash
# Run the sink with ClickHouse DSN
substreams-sink-sql setup "clickhouse://default:@clickhouse-cluster:9000/blockchain" my-substreams.spkg
substreams-sink-sql run "clickhouse://default:@clickhouse-cluster:9000/blockchain" my-substreams.spkg
```

### Bulk Data Operations

```sql
-- Efficient bulk insertion using INSERT SELECT
INSERT INTO erc20_transfers 
SELECT 
    block_number,
    block_timestamp,
    tx_hash,
    log_index,
    contract_address,
    from_addr,
    to_addr,
    amount,
    toStartOfHour(block_timestamp) as hour,
    toDate(block_timestamp) as date,
    toMonday(toDate(block_timestamp)) as week_start,
    toStartOfMonth(toDate(block_timestamp)) as month_start,
    -- Token metadata from dictionary lookup
    dictGetString('token_metadata', 'symbol', contract_address) as token_symbol,
    dictGetUInt8('token_metadata', 'decimals', contract_address) as token_decimals
FROM staging_transfers
WHERE processed = 0;

-- Optimize for bulk operations
SET max_insert_block_size = 1048576;
SET max_threads = 16;
SET max_memory_usage = 10000000000;  -- 10GB for large operations

-- Use async INSERT for high throughput
INSERT INTO erc20_transfers SETTINGS async_insert=1, wait_for_async_insert=0
VALUES (...);
```

## Performance Optimization Strategies

### Query Optimization

```sql
-- Use SAMPLE for large dataset analysis
SELECT 
    contract_address,
    count() * 10 as estimated_transfers,  -- Scale up sample
    avg(amount_formatted) as avg_transfer
FROM erc20_transfers SAMPLE 0.1  -- 10% sample
WHERE block_timestamp >= now() - INTERVAL 30 DAY
GROUP BY contract_address
ORDER BY estimated_transfers DESC;

-- Optimize GROUP BY with pre-filtering
SELECT 
    contract_address,
    sum(amount_formatted) as total_volume
FROM erc20_transfers
WHERE contract_address IN (
    -- Pre-filter to top contracts
    SELECT contract_address 
    FROM (
        SELECT contract_address, count() as tx_count
        FROM erc20_transfers 
        WHERE block_timestamp >= now() - INTERVAL 1 DAY
        GROUP BY contract_address 
        ORDER BY tx_count DESC 
        LIMIT 100
    )
)
AND block_timestamp >= now() - INTERVAL 30 DAY
GROUP BY contract_address
ORDER BY total_volume DESC;

-- Use LIMIT BY for top-N per group efficiently
SELECT 
    contract_address,
    from_addr,
    sum(amount_formatted) as total_sent
FROM erc20_transfers
WHERE block_timestamp >= now() - INTERVAL 7 DAY
GROUP BY contract_address, from_addr
ORDER BY contract_address, total_sent DESC
LIMIT 10 BY contract_address;  -- Top 10 senders per token
```

### Memory and Resource Management

```sql
-- Optimize aggregation memory usage
SET max_memory_usage = 20000000000;  -- 20GB
SET group_by_two_level_threshold = 100000;
SET group_by_two_level_threshold_bytes = 1000000000;

-- Use external sorting for large results
SET max_bytes_before_external_group_by = 10000000000;
SET max_bytes_before_external_sort = 10000000000;

-- Parallel query execution
SET max_threads = 16;
SET max_distributed_connections = 16;

-- Memory-efficient window functions
SELECT 
    contract_address,
    block_timestamp,
    amount_formatted,
    -- Use frame-limited window functions
    sum(amount_formatted) OVER (
        PARTITION BY contract_address 
        ORDER BY block_timestamp 
        ROWS BETWEEN 1000 PRECEDING AND CURRENT ROW
    ) as running_volume_1000_transfers
FROM erc20_transfers
WHERE contract_address = '0xA0b86a33E6Fe17d67C8c086c6c4c0E3C8E3B7EC2'
ORDER BY block_timestamp;
```

### Storage Optimization

```sql
-- TTL for automated data lifecycle
ALTER TABLE erc20_transfers MODIFY TTL toDate(block_timestamp) + INTERVAL 2 YEAR;

-- Tiered storage with different TTLs
ALTER TABLE erc20_transfers MODIFY TTL 
    toDate(block_timestamp) + INTERVAL 90 DAY TO DISK 'hdd',
    toDate(block_timestamp) + INTERVAL 2 YEAR DELETE;

-- Compression optimization by column
ALTER TABLE erc20_transfers 
    MODIFY COLUMN amount Codec(Delta, ZSTD),
    MODIFY COLUMN block_number Codec(Delta, ZSTD),
    MODIFY COLUMN block_timestamp Codec(DoubleDelta, ZSTD);

-- Partitioning optimization
-- Drop old partitions  
ALTER TABLE erc20_transfers DROP PARTITION '202301';  -- Drop January 2023

-- Optimize partition
OPTIMIZE TABLE erc20_transfers PARTITION '202412' FINAL;
```

## Monitoring and Maintenance

### Performance Monitoring

```sql
-- Query performance analysis
SELECT 
    query,
    count() as executions,
    avg(query_duration_ms) as avg_duration_ms,
    sum(read_bytes) as total_read_bytes,
    sum(result_rows) as total_result_rows
FROM system.query_log
WHERE event_time >= now() - INTERVAL 1 HOUR
  AND type = 2  -- QueryFinish
  AND query NOT LIKE '%system.%'
GROUP BY query
ORDER BY avg_duration_ms DESC
LIMIT 20;

-- Table size and compression analysis
SELECT 
    database,
    table,
    sum(rows) as total_rows,
    formatReadableSize(sum(data_uncompressed_bytes)) as uncompressed_size,
    formatReadableSize(sum(data_compressed_bytes)) as compressed_size,
    round(sum(data_compressed_bytes) / sum(data_uncompressed_bytes), 4) as compression_ratio,
    formatReadableSize(sum(bytes_on_disk)) as disk_size
FROM system.parts 
WHERE active = 1
GROUP BY database, table
ORDER BY sum(bytes_on_disk) DESC;

-- Merge performance monitoring
SELECT 
    table,
    count() as merges_in_progress,
    sum(total_size_bytes_compressed) as total_merge_size,
    avg(progress) as avg_progress,
    max(elapsed) as max_elapsed_seconds
FROM system.merges
GROUP BY table;
```

### Index and Projection Usage

```sql
-- Check projection usage
SELECT 
    database,
    table, 
    name as projection_name,
    type,
    query_count,
    exception_count
FROM system.projections
ORDER BY query_count DESC;

-- Analyze primary key efficiency
SELECT 
    table,
    formatReadableSize(primary_key_bytes_in_memory) as pk_memory_size,
    primary_key_bytes_in_memory / marks as avg_pk_size_per_mark
FROM system.parts
WHERE active = 1
  AND table = 'erc20_transfers'
ORDER BY primary_key_bytes_in_memory DESC;
```

## Best Practices Summary

### Schema Design
✅ **Use appropriate MergeTree family engines**  
✅ **Optimize ORDER BY for query patterns**  
✅ **Partition by time dimensions (month/day)**  
✅ **Use LowCardinality for enum-like fields**  
✅ **Denormalize for analytics performance**

### Query Optimization  
✅ **Use SAMPLE for exploratory analysis**  
✅ **Pre-filter with subqueries when beneficial**  
✅ **Leverage projections for common patterns**  
✅ **Use LIMIT BY for top-N per group**  
✅ **Optimize GROUP BY with appropriate settings**

### Data Management
✅ **Implement TTL for data lifecycle**  
✅ **Use tiered storage for cost optimization**  
✅ **Compress columns appropriately**  
✅ **Monitor and optimize partitions**  
✅ **Regular OPTIMIZE TABLE operations**

### Performance
✅ **Batch inserts for high throughput**  
✅ **Use async inserts for real-time ingestion**  
✅ **Tune memory settings for workload**  
✅ **Monitor query performance regularly**  
✅ **Use distributed tables for scale-out**

### Avoid
❌ **Small frequent inserts**  
❌ **UPDATE/DELETE on large datasets**  
❌ **Too many small partitions**  
❌ **Unoptimized JOIN operations**  
❌ **Missing compression on large columns**