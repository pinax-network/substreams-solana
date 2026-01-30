# PostgreSQL Patterns and Best Practices

Advanced patterns and optimization strategies for PostgreSQL with Substreams.

## Schema Design Patterns

### Normalized Schema for Ethereum Data

```sql
-- Core blockchain entities
CREATE TABLE blocks (
    number BIGINT PRIMARY KEY,
    hash VARCHAR(66) NOT NULL UNIQUE,
    parent_hash VARCHAR(66) NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    gas_limit BIGINT NOT NULL,
    gas_used BIGINT NOT NULL,
    tx_count INTEGER DEFAULT 0,
    size_bytes BIGINT,
    
    -- Indexing
    CONSTRAINT valid_block_number CHECK (number >= 0),
    CONSTRAINT valid_timestamp CHECK (timestamp > '2015-07-30'::timestamp)
);

-- Transactions with proper relationships
CREATE TABLE transactions (
    hash VARCHAR(66) PRIMARY KEY,
    block_number BIGINT NOT NULL REFERENCES blocks(number) ON DELETE CASCADE,
    tx_index INTEGER NOT NULL,
    from_addr VARCHAR(42) NOT NULL,
    to_addr VARCHAR(42),
    value NUMERIC(78,0) NOT NULL DEFAULT 0,
    gas_limit BIGINT NOT NULL,
    gas_used BIGINT NOT NULL,
    gas_price NUMERIC(78,0) NOT NULL,
    nonce BIGINT NOT NULL,
    input_data BYTEA,
    status INTEGER NOT NULL,
    
    -- Composite uniqueness
    UNIQUE(block_number, tx_index),
    
    -- Validation constraints
    CONSTRAINT valid_status CHECK (status IN (0, 1)),
    CONSTRAINT valid_gas CHECK (gas_used <= gas_limit),
    CONSTRAINT positive_value CHECK (value >= 0)
);

-- Event logs with proper indexing
CREATE TABLE transaction_logs (
    id SERIAL PRIMARY KEY,
    tx_hash VARCHAR(66) NOT NULL REFERENCES transactions(hash) ON DELETE CASCADE,
    log_index INTEGER NOT NULL,
    address VARCHAR(42) NOT NULL,
    topic0 VARCHAR(66),
    topic1 VARCHAR(66),
    topic2 VARCHAR(66), 
    topic3 VARCHAR(66),
    data BYTEA,
    
    -- Composite uniqueness for logs
    UNIQUE(tx_hash, log_index)
);

-- Token-specific tables
CREATE TABLE erc20_tokens (
    address VARCHAR(42) PRIMARY KEY,
    symbol VARCHAR(20),
    name VARCHAR(100),
    decimals INTEGER,
    total_supply NUMERIC(78,0),
    discovered_block BIGINT NOT NULL,
    
    CONSTRAINT valid_decimals CHECK (decimals >= 0 AND decimals <= 77)
);

CREATE TABLE erc20_transfers (
    id SERIAL PRIMARY KEY,
    tx_hash VARCHAR(66) NOT NULL REFERENCES transactions(hash) ON DELETE CASCADE,
    log_index INTEGER NOT NULL,
    contract_address VARCHAR(42) NOT NULL REFERENCES erc20_tokens(address),
    from_addr VARCHAR(42) NOT NULL,
    to_addr VARCHAR(42) NOT NULL,
    amount NUMERIC(78,0) NOT NULL,
    block_number BIGINT NOT NULL,
    
    -- Composite uniqueness
    UNIQUE(tx_hash, log_index),
    
    -- Foreign key to blocks
    FOREIGN KEY (block_number) REFERENCES blocks(number) ON DELETE CASCADE,
    
    -- Constraints
    CONSTRAINT positive_amount CHECK (amount > 0),
    CONSTRAINT different_addresses CHECK (from_addr != to_addr)
);
```

### Indexing Strategy

```sql
-- Primary access patterns
CREATE INDEX idx_blocks_timestamp ON blocks(timestamp);
CREATE INDEX idx_blocks_hash ON blocks(hash);

-- Transaction queries
CREATE INDEX idx_transactions_from ON transactions(from_addr);
CREATE INDEX idx_transactions_to ON transactions(to_addr) WHERE to_addr IS NOT NULL;
CREATE INDEX idx_transactions_block_idx ON transactions(block_number, tx_index);
CREATE INDEX idx_transactions_timestamp ON transactions(block_number) 
    INCLUDE (hash, from_addr, to_addr, value);

-- Log queries
CREATE INDEX idx_logs_address ON transaction_logs(address);
CREATE INDEX idx_logs_topic0 ON transaction_logs(topic0) WHERE topic0 IS NOT NULL;
CREATE INDEX idx_logs_topic1 ON transaction_logs(topic1) WHERE topic1 IS NOT NULL;
CREATE INDEX idx_logs_address_topic0 ON transaction_logs(address, topic0) 
    WHERE topic0 IS NOT NULL;

-- ERC20 specific indexes
CREATE INDEX idx_transfers_contract ON erc20_transfers(contract_address);
CREATE INDEX idx_transfers_from ON erc20_transfers(from_addr);
CREATE INDEX idx_transfers_to ON erc20_transfers(to_addr);
CREATE INDEX idx_transfers_contract_block ON erc20_transfers(contract_address, block_number);
CREATE INDEX idx_transfers_addresses ON erc20_transfers(from_addr, to_addr);

-- Composite indexes for common queries
CREATE INDEX idx_transfers_token_time ON erc20_transfers(contract_address, block_number DESC)
    INCLUDE (from_addr, to_addr, amount);
    
-- Partial indexes for active data
CREATE INDEX idx_active_tokens ON erc20_tokens(address) WHERE total_supply > 0;
CREATE INDEX idx_recent_transfers ON erc20_transfers(block_number DESC) 
    WHERE block_number > (SELECT MAX(number) - 1000 FROM blocks);
```

### Partitioning Strategies

```sql
-- Time-based partitioning for large tables
CREATE TABLE erc20_transfers (
    -- columns as above
    block_number BIGINT NOT NULL,
    created_date DATE GENERATED ALWAYS AS (
        DATE(to_timestamp(
            (SELECT timestamp FROM blocks WHERE number = block_number)::bigint
        ))
    ) STORED
) PARTITION BY RANGE (created_date);

-- Create monthly partitions
CREATE TABLE erc20_transfers_2024_01 PARTITION OF erc20_transfers
    FOR VALUES FROM ('2024-01-01') TO ('2024-02-01');
    
CREATE TABLE erc20_transfers_2024_02 PARTITION OF erc20_transfers  
    FOR VALUES FROM ('2024-02-01') TO ('2024-03-01');

-- Auto-create future partitions
CREATE OR REPLACE FUNCTION create_monthly_partition(table_name TEXT, start_date DATE)
RETURNS VOID AS $$
DECLARE
    partition_name TEXT;
    end_date DATE;
BEGIN
    partition_name := table_name || '_' || to_char(start_date, 'YYYY_MM');
    end_date := start_date + INTERVAL '1 month';
    
    EXECUTE format('CREATE TABLE %I PARTITION OF %I FOR VALUES FROM (%L) TO (%L)',
                   partition_name, table_name, start_date, end_date);
END;
$$ LANGUAGE plpgsql;
```

## Advanced Query Patterns

### Token Analytics Queries

```sql
-- Top tokens by transfer volume (last 24 hours)
WITH recent_blocks AS (
    SELECT number, timestamp 
    FROM blocks 
    WHERE timestamp > NOW() - INTERVAL '24 hours'
),
token_volumes AS (
    SELECT 
        t.contract_address,
        tok.symbol,
        tok.name,
        COUNT(*) as transfer_count,
        COUNT(DISTINCT t.from_addr) + COUNT(DISTINCT t.to_addr) as unique_addresses,
        SUM(t.amount::NUMERIC / POWER(10, tok.decimals)) as volume
    FROM erc20_transfers t
    JOIN erc20_tokens tok ON t.contract_address = tok.address
    JOIN recent_blocks b ON t.block_number = b.number
    GROUP BY t.contract_address, tok.symbol, tok.name, tok.decimals
)
SELECT 
    symbol,
    name,
    transfer_count,
    unique_addresses,
    ROUND(volume, 2) as volume_tokens,
    RANK() OVER (ORDER BY volume DESC) as volume_rank
FROM token_volumes
ORDER BY volume DESC
LIMIT 50;

-- Address activity patterns
WITH address_stats AS (
    SELECT 
        addr,
        COUNT(*) as total_transfers,
        COUNT(DISTINCT contract_address) as unique_tokens,
        MIN(block_number) as first_activity,
        MAX(block_number) as last_activity,
        SUM(CASE WHEN from_addr = addr THEN 1 ELSE 0 END) as sends,
        SUM(CASE WHEN to_addr = addr THEN 1 ELSE 0 END) as receives
    FROM (
        SELECT from_addr as addr, contract_address, block_number FROM erc20_transfers
        UNION ALL
        SELECT to_addr as addr, contract_address, block_number FROM erc20_transfers
    ) combined_activity
    GROUP BY addr
)
SELECT 
    addr,
    total_transfers,
    unique_tokens,
    sends,
    receives,
    ROUND(sends::NUMERIC / NULLIF(total_transfers, 0), 3) as send_ratio,
    last_activity - first_activity as activity_span_blocks
FROM address_stats
WHERE total_transfers > 100
ORDER BY total_transfers DESC;
```

### Performance-Optimized Queries

```sql
-- Efficient balance calculation using window functions
WITH transfer_events AS (
    SELECT 
        contract_address,
        from_addr as addr,
        -amount as delta,
        block_number
    FROM erc20_transfers
    WHERE contract_address = $1 AND block_number <= $2
    
    UNION ALL
    
    SELECT 
        contract_address,
        to_addr as addr, 
        amount as delta,
        block_number
    FROM erc20_transfers
    WHERE contract_address = $1 AND block_number <= $2
),
balance_changes AS (
    SELECT 
        addr,
        SUM(delta) OVER (PARTITION BY addr ORDER BY block_number 
                        ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW) as running_balance,
        block_number,
        ROW_NUMBER() OVER (PARTITION BY addr ORDER BY block_number DESC) as rn
    FROM transfer_events
)
SELECT addr, running_balance as final_balance
FROM balance_changes 
WHERE rn = 1 AND running_balance > 0
ORDER BY running_balance DESC;

-- Fast holder count using estimates
SELECT 
    t.contract_address,
    tok.symbol,
    -- Fast approximate count
    (SELECT reltuples::BIGINT 
     FROM pg_class 
     WHERE relname = 'token_balances') as approx_holders,
    -- Exact count when needed (expensive)
    COUNT(*) FILTER (WHERE balance > 0) as exact_active_holders
FROM erc20_tokens t
LEFT JOIN token_balances b ON t.address = b.contract_address
WHERE t.address = $1
GROUP BY t.contract_address, t.symbol;
```

## Materialized Views and Aggregations

### Real-time Token Statistics

```sql
-- Materialized view for token metrics
CREATE MATERIALIZED VIEW token_daily_stats AS
WITH daily_transfers AS (
    SELECT 
        t.contract_address,
        DATE(to_timestamp(b.timestamp)) as date,
        COUNT(*) as transfer_count,
        COUNT(DISTINCT t.from_addr) as unique_senders,
        COUNT(DISTINCT t.to_addr) as unique_receivers,
        SUM(t.amount::NUMERIC / POWER(10, tok.decimals)) as volume_tokens
    FROM erc20_transfers t
    JOIN blocks b ON t.block_number = b.number
    JOIN erc20_tokens tok ON t.contract_address = tok.address
    GROUP BY t.contract_address, DATE(to_timestamp(b.timestamp)), tok.decimals
)
SELECT 
    contract_address,
    date,
    transfer_count,
    unique_senders,
    unique_receivers,
    unique_senders + unique_receivers as unique_addresses,
    volume_tokens,
    -- Moving averages
    AVG(transfer_count) OVER (
        PARTITION BY contract_address 
        ORDER BY date 
        ROWS BETWEEN 6 PRECEDING AND CURRENT ROW
    ) as transfer_count_7d_avg,
    AVG(volume_tokens) OVER (
        PARTITION BY contract_address 
        ORDER BY date 
        ROWS BETWEEN 6 PRECEDING AND CURRENT ROW  
    ) as volume_7d_avg
FROM daily_transfers;

-- Unique index for concurrent refresh
CREATE UNIQUE INDEX ON token_daily_stats (contract_address, date);

-- Auto-refresh trigger
CREATE OR REPLACE FUNCTION refresh_token_daily_stats()
RETURNS TRIGGER AS $$
BEGIN
    REFRESH MATERIALIZED VIEW CONCURRENTLY token_daily_stats;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Trigger on new data
CREATE TRIGGER refresh_stats_trigger
    AFTER INSERT ON erc20_transfers
    FOR EACH STATEMENT
    EXECUTE FUNCTION refresh_token_daily_stats();
```

### Aggregated Balance Views

```sql
-- Current token holder balances
CREATE MATERIALIZED VIEW current_token_balances AS
WITH latest_transfers AS (
    SELECT 
        contract_address,
        from_addr as addr,
        -amount as delta,
        block_number,
        ROW_NUMBER() OVER (
            PARTITION BY contract_address, from_addr 
            ORDER BY block_number DESC, log_index DESC
        ) as rn_from
    FROM erc20_transfers
    
    UNION ALL
    
    SELECT 
        contract_address, 
        to_addr as addr,
        amount as delta,
        block_number,
        ROW_NUMBER() OVER (
            PARTITION BY contract_address, to_addr 
            ORDER BY block_number DESC, log_index DESC
        ) as rn_to
    FROM erc20_transfers
),
balance_deltas AS (
    SELECT 
        contract_address,
        addr,
        SUM(delta) as balance
    FROM latest_transfers
    GROUP BY contract_address, addr
    HAVING SUM(delta) > 0  -- Only positive balances
)
SELECT 
    b.contract_address,
    t.symbol,
    t.name,
    b.addr as holder_address,
    b.balance,
    b.balance::NUMERIC / POWER(10, t.decimals) as balance_formatted,
    RANK() OVER (PARTITION BY b.contract_address ORDER BY b.balance DESC) as holder_rank
FROM balance_deltas b
JOIN erc20_tokens t ON b.contract_address = t.address;

-- Index for fast lookups
CREATE UNIQUE INDEX ON current_token_balances (contract_address, holder_address);
CREATE INDEX ON current_token_balances (contract_address, balance DESC);
```

## JSONB and Advanced Data Types

### Storing Complex Blockchain Data

```sql
-- Extended transaction table with JSONB
CREATE TABLE transactions_extended (
    hash VARCHAR(66) PRIMARY KEY,
    -- Basic fields...
    block_number BIGINT NOT NULL,
    
    -- Complex data as JSONB
    receipt JSONB,           -- Transaction receipt
    trace_calls JSONB,       -- Internal calls
    decoded_input JSONB,     -- Decoded function calls
    
    -- Generated columns from JSONB
    gas_used BIGINT GENERATED ALWAYS AS ((receipt->>'gasUsed')::BIGINT) STORED,
    status INTEGER GENERATED ALWAYS AS ((receipt->>'status')::INTEGER) STORED
);

-- JSONB indexes
CREATE INDEX idx_tx_receipt_gin ON transactions_extended USING GIN (receipt);
CREATE INDEX idx_tx_status ON transactions_extended ((receipt->>'status'));
CREATE INDEX idx_tx_contract_creation ON transactions_extended ((receipt->>'contractAddress')) 
    WHERE receipt->>'contractAddress' IS NOT NULL;

-- Complex queries on JSONB data
SELECT 
    hash,
    receipt->>'gasUsed' as gas_used,
    jsonb_array_length(receipt->'logs') as log_count,
    receipt->'logs' @> '[{"address": "0xa0b86a33e6fe17d67c8b086c6c4c0e3c8e3b7ec2"}]' as has_token_event
FROM transactions_extended
WHERE receipt->>'status' = '1'
  AND (receipt->'logs')::jsonb @> '[{"address": "0xa0b86a33e6fe17d67c8b086c6c4c0e3c8e3b7ec2"}]';
```

### Event Decoding Storage

```sql
-- Decoded events table
CREATE TABLE decoded_events (
    id SERIAL PRIMARY KEY,
    tx_hash VARCHAR(66) NOT NULL,
    log_index INTEGER NOT NULL,
    contract_address VARCHAR(42) NOT NULL,
    event_name VARCHAR(100) NOT NULL,
    event_signature VARCHAR(66) NOT NULL,
    
    -- Decoded parameters as JSONB
    decoded_data JSONB NOT NULL,
    
    -- Block context
    block_number BIGINT NOT NULL,
    block_timestamp TIMESTAMP NOT NULL,
    
    UNIQUE(tx_hash, log_index)
);

-- Specialized indexes
CREATE INDEX idx_decoded_events_contract_name ON decoded_events(contract_address, event_name);
CREATE INDEX idx_decoded_events_signature ON decoded_events(event_signature);
CREATE INDEX idx_decoded_events_data_gin ON decoded_events USING GIN (decoded_data);

-- Query examples
-- Find all Transfer events with amount > 1000 tokens
SELECT 
    tx_hash,
    contract_address,
    decoded_data->>'from' as from_addr,
    decoded_data->>'to' as to_addr,
    (decoded_data->>'value')::NUMERIC as raw_amount
FROM decoded_events 
WHERE event_name = 'Transfer'
  AND (decoded_data->>'value')::NUMERIC > 1000000000000000000; -- > 1 ETH worth

-- Complex event pattern matching
SELECT 
    tx_hash,
    array_agg(event_name ORDER BY log_index) as event_sequence
FROM decoded_events
WHERE tx_hash IN (
    SELECT tx_hash 
    FROM decoded_events 
    WHERE event_name = 'Swap' 
    GROUP BY tx_hash 
    HAVING COUNT(*) > 1  -- Multi-hop swaps
)
GROUP BY tx_hash;
```

## Performance Optimization

### Query Optimization

```sql
-- Use CTEs for complex queries
WITH RECURSIVE token_holder_tree AS (
    -- Base case: direct holders
    SELECT 
        contract_address,
        holder_address,
        balance,
        1 as depth
    FROM current_token_balances
    WHERE balance > 1000000  -- Significant holders only
    
    UNION ALL
    
    -- Recursive case: holders of holders (for wrapped tokens)
    SELECT 
        t.contract_address,
        b.holder_address,
        b.balance,
        t.depth + 1
    FROM token_holder_tree t
    JOIN current_token_balances b ON t.holder_address = b.contract_address
    WHERE t.depth < 3  -- Limit recursion depth
)
SELECT * FROM token_holder_tree ORDER BY balance DESC;

-- Efficient pagination
SELECT *
FROM erc20_transfers
WHERE (block_number, log_index) > ($1, $2)  -- Cursor-based pagination
ORDER BY block_number, log_index
LIMIT 1000;

-- Avoid expensive operations in WHERE clauses
-- BAD: Function in WHERE
SELECT * FROM erc20_transfers 
WHERE EXTRACT(year FROM to_timestamp(block_timestamp)) = 2024;

-- GOOD: Pre-computed or indexed values
SELECT * FROM erc20_transfers t
JOIN blocks b ON t.block_number = b.number
WHERE b.timestamp BETWEEN '2024-01-01' AND '2025-01-01';
```

### Connection and Memory Management

```sql
-- Connection pooling settings
-- In postgresql.conf or connection string
max_connections = 100
shared_buffers = 256MB
effective_cache_size = 1GB
work_mem = 4MB
maintenance_work_mem = 64MB

-- Query-specific memory settings
SET work_mem = '100MB';  -- For large sorting operations
SET temp_buffers = '32MB';  -- For temporary tables

-- Monitor query performance
SELECT 
    query,
    calls,
    total_time,
    mean_time,
    rows
FROM pg_stat_statements 
ORDER BY total_time DESC 
LIMIT 20;
```

### Maintenance and Monitoring

```sql
-- Regular maintenance tasks
-- Vacuum and analyze
VACUUM ANALYZE erc20_transfers;
VACUUM ANALYZE blocks;

-- Reindex periodically
REINDEX INDEX CONCURRENTLY idx_transfers_contract_block;

-- Monitor table sizes
SELECT 
    schemaname,
    tablename,
    pg_size_pretty(pg_total_relation_size(schemaname||'.'||tablename)) as size,
    pg_size_pretty(pg_relation_size(schemaname||'.'||tablename)) as table_size,
    pg_size_pretty(pg_total_relation_size(schemaname||'.'||tablename) - pg_relation_size(schemaname||'.'||tablename)) as index_size
FROM pg_tables 
WHERE schemaname = 'public'
ORDER BY pg_total_relation_size(schemaname||'.'||tablename) DESC;

-- Monitor index usage
SELECT 
    t.tablename,
    indexname,
    c.reltuples AS num_rows,
    pg_size_pretty(pg_relation_size(quote_ident(t.tablename)::text)) AS table_size,
    pg_size_pretty(pg_relation_size(quote_ident(indexrelname)::text)) AS index_size,
    CASE WHEN indisunique THEN 'Y' ELSE 'N' END AS UNIQUE,
    idx_scan as number_of_scans,
    idx_tup_read as tuples_read,
    idx_tup_fetch as tuples_fetched
FROM pg_tables t
LEFT OUTER JOIN pg_class c ON c.relname = t.tablename
LEFT OUTER JOIN (
    SELECT c.relname AS ctablename, ipg.relname AS indexname, x.indnatts AS number_of_columns,
           idx_scan, idx_tup_read, idx_tup_fetch, indexrelname, indisunique
    FROM pg_index x
    JOIN pg_class c ON c.oid = x.indrelid
    JOIN pg_class ipg ON ipg.oid = x.indexrelid
    JOIN pg_stat_all_indexes psai ON x.indexrelid = psai.indexrelid
) AS foo ON t.tablename = foo.ctablename
WHERE t.schemaname = 'public'
ORDER BY 1, 2;
```

## Best Practices Summary

### Schema Design
✅ **Use appropriate data types** (NUMERIC for big integers)  
✅ **Add proper constraints and validation**  
✅ **Design efficient primary and foreign keys**  
✅ **Consider partitioning for large tables**  
✅ **Use JSONB for complex structured data**

### Indexing
✅ **Index common query patterns**  
✅ **Use partial indexes for filtered data**  
✅ **Create composite indexes for multi-column queries**  
✅ **Monitor index usage and remove unused ones**  
✅ **Use GIN indexes for JSONB and array data**

### Performance  
✅ **Use materialized views for expensive aggregations**  
✅ **Implement cursor-based pagination**  
✅ **Batch insert/update operations**  
✅ **Monitor query performance with pg_stat_statements**  
✅ **Regular maintenance (VACUUM, ANALYZE, REINDEX)**

### Avoid
❌ **Auto-incrementing IDs for blockchain data**  
❌ **Functions in WHERE clauses without indexes**  
❌ **Large IN clauses (use JOIN with VALUES instead)**  
❌ **Storing large binary data in regular columns**  
❌ **Missing foreign key constraints**