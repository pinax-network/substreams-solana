# Schema Design Patterns

Common schema patterns and design principles for blockchain data in SQL databases.

## Design Principles

### 1. Blockchain Data Characteristics

Understanding blockchain data helps inform schema design:
- **Immutable**: Data rarely changes once written
- **Sequential**: Block/transaction ordering is critical
- **High volume**: Millions of transactions per day
- **Nested structures**: Transactions contain logs, traces, etc.
- **Variable content**: Different transaction types have different data

### 2. Normalization vs Denormalization

**When to Normalize**:
- ✅ Operational systems requiring consistency
- ✅ Storage cost is a primary concern  
- ✅ Complex relationships need referential integrity
- ✅ Multiple applications need consistent data

**When to Denormalize**:
- ✅ Analytics and reporting systems
- ✅ Query performance is critical
- ✅ Read-heavy workloads
- ✅ Time-series analysis requirements

## Core Entity Patterns

### Block-Transaction-Log Hierarchy

```sql
-- Normalized approach (PostgreSQL)
CREATE TABLE blocks (
    number BIGINT PRIMARY KEY,
    hash VARCHAR(66) NOT NULL UNIQUE,
    parent_hash VARCHAR(66) NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    gas_limit BIGINT NOT NULL,
    gas_used BIGINT NOT NULL,
    miner VARCHAR(42),
    difficulty NUMERIC(78,0),
    total_difficulty NUMERIC(78,0),
    size_bytes BIGINT,
    
    -- Computed fields
    gas_utilization DECIMAL(5,4) GENERATED ALWAYS AS (gas_used::decimal / gas_limit) STORED,
    
    -- Constraints
    CONSTRAINT valid_block_number CHECK (number >= 0),
    CONSTRAINT valid_gas_used CHECK (gas_used <= gas_limit),
    CONSTRAINT future_timestamp CHECK (timestamp <= NOW() + INTERVAL '1 hour')
);

CREATE TABLE transactions (
    hash VARCHAR(66) PRIMARY KEY,
    block_number BIGINT NOT NULL REFERENCES blocks(number),
    tx_index INTEGER NOT NULL,
    from_addr VARCHAR(42) NOT NULL,
    to_addr VARCHAR(42), -- NULL for contract creation
    value NUMERIC(78,0) NOT NULL DEFAULT 0,
    gas_limit BIGINT NOT NULL,
    gas_used BIGINT NOT NULL,
    gas_price NUMERIC(78,0) NOT NULL,
    nonce BIGINT NOT NULL,
    input_data BYTEA,
    status INTEGER NOT NULL, -- 0 = failed, 1 = success
    
    -- Computed fields
    tx_fee NUMERIC(78,0) GENERATED ALWAYS AS (gas_used * gas_price) STORED,
    is_contract_creation BOOLEAN GENERATED ALWAYS AS (to_addr IS NULL) STORED,
    
    UNIQUE(block_number, tx_index),
    
    -- Constraints
    CONSTRAINT valid_status CHECK (status IN (0, 1)),
    CONSTRAINT valid_gas CHECK (gas_used <= gas_limit),
    CONSTRAINT positive_gas_price CHECK (gas_price >= 0),
    CONSTRAINT positive_value CHECK (value >= 0)
);

CREATE TABLE transaction_logs (
    id SERIAL PRIMARY KEY,
    tx_hash VARCHAR(66) NOT NULL REFERENCES transactions(hash),
    log_index INTEGER NOT NULL,
    address VARCHAR(42) NOT NULL,
    topic0 VARCHAR(66),
    topic1 VARCHAR(66),
    topic2 VARCHAR(66),
    topic3 VARCHAR(66),
    data BYTEA,
    removed BOOLEAN DEFAULT FALSE,
    
    UNIQUE(tx_hash, log_index),
    
    -- Constraint for topic structure
    CONSTRAINT topic_ordering CHECK (
        (topic0 IS NOT NULL) OR 
        (topic0 IS NULL AND topic1 IS NULL AND topic2 IS NULL AND topic3 IS NULL)
    )
);
```

```sql
-- Denormalized approach (ClickHouse)
CREATE TABLE blockchain_events (
    -- Block context
    block_number UInt64,
    block_timestamp DateTime,
    block_hash String,
    
    -- Transaction context
    tx_hash String,
    tx_index UInt32,
    tx_from String,
    tx_to String,
    tx_value UInt256,
    tx_status UInt8,
    
    -- Log context
    log_index UInt32,
    log_address String,
    
    -- Event data
    event_signature String,
    event_name LowCardinality(String),
    topics Array(String),
    data String,
    
    -- Decoded event parameters (JSONB-like)
    decoded_params Map(String, String),
    
    -- Time dimensions for analytics
    hour DateTime,
    date Date,
    week_start Date,
    month_start Date
    
) ENGINE = MergeTree()
PARTITION BY toYYYYMM(date)
ORDER BY (event_name, log_address, block_timestamp)
SETTINGS index_granularity = 8192;
```

### Token-Centric Schemas

```sql
-- ERC20 token system
CREATE TABLE erc20_tokens (
    address VARCHAR(42) PRIMARY KEY,
    symbol VARCHAR(20) NOT NULL,
    name VARCHAR(100) NOT NULL,
    decimals INTEGER NOT NULL,
    total_supply NUMERIC(78,0),
    
    -- Discovery metadata
    discovered_block BIGINT NOT NULL,
    discovered_timestamp TIMESTAMP NOT NULL,
    creator_address VARCHAR(42),
    
    -- Token classification
    category VARCHAR(50), -- 'stablecoin', 'governance', 'utility', etc.
    is_verified BOOLEAN DEFAULT FALSE,
    
    -- Constraints
    CONSTRAINT valid_decimals CHECK (decimals >= 0 AND decimals <= 77),
    CONSTRAINT valid_supply CHECK (total_supply >= 0),
    CONSTRAINT valid_discovery CHECK (discovered_block > 0)
);

-- Transfer events with balance tracking
CREATE TABLE erc20_transfers (
    id SERIAL PRIMARY KEY,
    tx_hash VARCHAR(66) NOT NULL,
    log_index INTEGER NOT NULL,
    contract_address VARCHAR(42) NOT NULL REFERENCES erc20_tokens(address),
    from_addr VARCHAR(42) NOT NULL,
    to_addr VARCHAR(42) NOT NULL,
    amount NUMERIC(78,0) NOT NULL,
    
    -- Context
    block_number BIGINT NOT NULL,
    block_timestamp TIMESTAMP NOT NULL,
    tx_index INTEGER NOT NULL,
    
    -- Computed fields
    amount_formatted DECIMAL GENERATED ALWAYS AS (
        amount::DECIMAL / POWER(10, (SELECT decimals FROM erc20_tokens WHERE address = contract_address))
    ) STORED,
    
    UNIQUE(tx_hash, log_index),
    
    -- Constraints
    CONSTRAINT positive_amount CHECK (amount > 0),
    CONSTRAINT different_addresses CHECK (from_addr != to_addr)
);

-- Current balances (materialized view approach)
CREATE MATERIALIZED VIEW current_erc20_balances AS
WITH balance_changes AS (
    -- Outgoing transfers (negative)
    SELECT 
        contract_address,
        from_addr as address,
        -amount as balance_change,
        block_number
    FROM erc20_transfers
    
    UNION ALL
    
    -- Incoming transfers (positive)
    SELECT 
        contract_address,
        to_addr as address,
        amount as balance_change,
        block_number
    FROM erc20_transfers
),
current_balances AS (
    SELECT 
        contract_address,
        address,
        SUM(balance_change) as balance,
        MAX(block_number) as last_updated_block
    FROM balance_changes
    GROUP BY contract_address, address
    HAVING SUM(balance_change) > 0  -- Only positive balances
)
SELECT 
    cb.contract_address,
    t.symbol,
    t.name,
    cb.address,
    cb.balance,
    cb.balance::DECIMAL / POWER(10, t.decimals) as balance_formatted,
    cb.last_updated_block,
    
    -- Balance ranking
    RANK() OVER (
        PARTITION BY cb.contract_address 
        ORDER BY cb.balance DESC
    ) as balance_rank,
    
    -- Percentage of total supply
    CASE 
        WHEN t.total_supply > 0 
        THEN (cb.balance::DECIMAL / t.total_supply::DECIMAL) * 100 
        ELSE 0 
    END as supply_percentage
    
FROM current_balances cb
JOIN erc20_tokens t ON cb.contract_address = t.address;

-- Unique index for fast lookups
CREATE UNIQUE INDEX ON current_erc20_balances (contract_address, address);
CREATE INDEX ON current_erc20_balances (contract_address, balance DESC);
```

### DeFi Protocol Patterns

```sql
-- Uniswap V2/V3 liquidity pools
CREATE TABLE liquidity_pools (
    address VARCHAR(42) PRIMARY KEY,
    factory_address VARCHAR(42) NOT NULL,
    protocol VARCHAR(20) NOT NULL, -- 'uniswap_v2', 'uniswap_v3', etc.
    protocol_version VARCHAR(10) NOT NULL,
    
    -- Token pair
    token0_address VARCHAR(42) NOT NULL,
    token1_address VARCHAR(42) NOT NULL,
    
    -- Pool parameters
    fee_tier INTEGER, -- For V3: 500, 3000, 10000 (basis points)
    tick_spacing INTEGER, -- For V3
    
    -- Creation info
    created_block BIGINT NOT NULL,
    created_timestamp TIMESTAMP NOT NULL,
    creator_address VARCHAR(42),
    
    -- Current state (updated via triggers)
    reserve0 NUMERIC(78,0) DEFAULT 0,
    reserve1 NUMERIC(78,0) DEFAULT 0,
    total_liquidity NUMERIC(78,0) DEFAULT 0,
    
    -- Metadata
    is_active BOOLEAN DEFAULT TRUE,
    last_updated_block BIGINT,
    
    UNIQUE(token0_address, token1_address, fee_tier, protocol),
    
    -- Ensure token0 < token1 (canonical ordering)
    CONSTRAINT canonical_token_order CHECK (token0_address < token1_address)
);

-- Pool events (swaps, mints, burns)
CREATE TABLE pool_events (
    id SERIAL PRIMARY KEY,
    pool_address VARCHAR(42) NOT NULL REFERENCES liquidity_pools(address),
    tx_hash VARCHAR(66) NOT NULL,
    log_index INTEGER NOT NULL,
    event_type VARCHAR(20) NOT NULL, -- 'swap', 'mint', 'burn', 'sync'
    
    -- Universal fields
    block_number BIGINT NOT NULL,
    block_timestamp TIMESTAMP NOT NULL,
    
    -- Swap-specific fields
    sender_address VARCHAR(42),
    recipient_address VARCHAR(42),
    amount0_in NUMERIC(78,0),
    amount1_in NUMERIC(78,0),
    amount0_out NUMERIC(78,0),
    amount1_out NUMERIC(78,0),
    
    -- Liquidity-specific fields  
    liquidity_delta NUMERIC(78,0),
    
    -- Price impact
    price_before DECIMAL,
    price_after DECIMAL,
    
    -- Reserves after event
    reserve0_after NUMERIC(78,0),
    reserve1_after NUMERIC(78,0),
    
    UNIQUE(tx_hash, log_index),
    CONSTRAINT valid_event_type CHECK (event_type IN ('swap', 'mint', 'burn', 'sync'))
);

-- Liquidity positions (for tracking individual LP positions)
CREATE TABLE liquidity_positions (
    id SERIAL PRIMARY KEY,
    pool_address VARCHAR(42) NOT NULL REFERENCES liquidity_pools(address),
    owner_address VARCHAR(42) NOT NULL,
    
    -- Position bounds (V3)
    tick_lower INTEGER,
    tick_upper INTEGER,
    
    -- Position state
    liquidity NUMERIC(78,0) NOT NULL DEFAULT 0,
    token0_owed NUMERIC(78,0) DEFAULT 0,
    token1_owed NUMERIC(78,0) DEFAULT 0,
    
    -- Lifecycle
    created_block BIGINT NOT NULL,
    created_timestamp TIMESTAMP NOT NULL,
    closed_block BIGINT,
    closed_timestamp TIMESTAMP,
    
    -- Status
    is_active BOOLEAN DEFAULT TRUE,
    last_updated_block BIGINT,
    
    -- For V3: unique position per owner/pool/tick range
    UNIQUE(pool_address, owner_address, tick_lower, tick_upper),
    
    CONSTRAINT valid_ticks CHECK (tick_lower < tick_upper),
    CONSTRAINT positive_liquidity CHECK (liquidity >= 0)
);
```

## Time-Series Patterns

### Temporal Data Organization

```sql
-- Time-partitioned transaction table
CREATE TABLE transactions_partitioned (
    hash VARCHAR(66),
    block_number BIGINT NOT NULL,
    block_timestamp TIMESTAMP NOT NULL,
    from_addr VARCHAR(42) NOT NULL,
    to_addr VARCHAR(42),
    value NUMERIC(78,0) NOT NULL,
    gas_used BIGINT NOT NULL,
    
    -- Time dimension for partitioning
    tx_date DATE GENERATED ALWAYS AS (DATE(block_timestamp)) STORED
) PARTITION BY RANGE (tx_date);

-- Create monthly partitions
CREATE TABLE transactions_2024_01 PARTITION OF transactions_partitioned
    FOR VALUES FROM ('2024-01-01') TO ('2024-02-01');
    
CREATE TABLE transactions_2024_02 PARTITION OF transactions_partitioned
    FOR VALUES FROM ('2024-02-01') TO ('2024-03-01');

-- Automatic partition management
CREATE OR REPLACE FUNCTION create_transaction_partition(partition_date DATE)
RETURNS VOID AS $$
DECLARE
    partition_name TEXT;
    start_date DATE;
    end_date DATE;
BEGIN
    partition_name := 'transactions_' || TO_CHAR(partition_date, 'YYYY_MM');
    start_date := DATE_TRUNC('month', partition_date);
    end_date := start_date + INTERVAL '1 month';
    
    EXECUTE format('CREATE TABLE %I PARTITION OF transactions_partitioned 
                    FOR VALUES FROM (%L) TO (%L)',
                   partition_name, start_date, end_date);
                   
    -- Create indexes on new partition
    EXECUTE format('CREATE INDEX ON %I (from_addr)', partition_name);
    EXECUTE format('CREATE INDEX ON %I (to_addr)', partition_name);
    EXECUTE format('CREATE INDEX ON %I (block_number)', partition_name);
END;
$$ LANGUAGE plpgsql;
```

### Aggregation Tables

```sql
-- Multi-resolution time aggregations
CREATE TABLE token_hourly_stats (
    contract_address VARCHAR(42) NOT NULL,
    hour_timestamp TIMESTAMP NOT NULL,
    
    -- Transfer metrics
    transfer_count INTEGER NOT NULL DEFAULT 0,
    unique_senders INTEGER NOT NULL DEFAULT 0,
    unique_receivers INTEGER NOT NULL DEFAULT 0,
    total_unique_addresses INTEGER NOT NULL DEFAULT 0,
    
    -- Volume metrics
    total_volume NUMERIC(78,0) NOT NULL DEFAULT 0,
    volume_formatted DECIMAL NOT NULL DEFAULT 0,
    avg_transfer_size DECIMAL NOT NULL DEFAULT 0,
    median_transfer_size DECIMAL NOT NULL DEFAULT 0,
    max_transfer_size NUMERIC(78,0) NOT NULL DEFAULT 0,
    
    -- Price data (if available)
    price_usd DECIMAL,
    volume_usd DECIMAL,
    market_cap_usd DECIMAL,
    
    PRIMARY KEY (contract_address, hour_timestamp)
);

CREATE TABLE token_daily_stats (
    contract_address VARCHAR(42) NOT NULL,
    date DATE NOT NULL,
    
    -- Aggregated from hourly
    transfer_count INTEGER NOT NULL DEFAULT 0,
    unique_senders INTEGER NOT NULL DEFAULT 0,
    unique_receivers INTEGER NOT NULL DEFAULT 0,
    total_unique_addresses INTEGER NOT NULL DEFAULT 0,
    total_volume NUMERIC(78,0) NOT NULL DEFAULT 0,
    volume_formatted DECIMAL NOT NULL DEFAULT 0,
    
    -- Daily-specific metrics
    opening_price DECIMAL,
    closing_price DECIMAL,
    high_price DECIMAL,
    low_price DECIMAL,
    volume_weighted_price DECIMAL,
    
    -- Growth metrics
    transfer_count_change INTEGER DEFAULT 0,
    volume_change_pct DECIMAL DEFAULT 0,
    price_change_pct DECIMAL DEFAULT 0,
    
    -- Moving averages (computed)
    volume_7d_avg DECIMAL,
    volume_30d_avg DECIMAL,
    price_7d_avg DECIMAL,
    price_30d_avg DECIMAL,
    
    PRIMARY KEY (contract_address, date)
);

-- Automatic aggregation using triggers
CREATE OR REPLACE FUNCTION update_token_hourly_stats()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO token_hourly_stats (
        contract_address,
        hour_timestamp,
        transfer_count,
        total_volume,
        volume_formatted
    )
    SELECT 
        NEW.contract_address,
        DATE_TRUNC('hour', NEW.block_timestamp),
        1,
        NEW.amount,
        NEW.amount::DECIMAL / POWER(10, (SELECT decimals FROM erc20_tokens WHERE address = NEW.contract_address))
    ON CONFLICT (contract_address, hour_timestamp) DO UPDATE SET
        transfer_count = token_hourly_stats.transfer_count + 1,
        total_volume = token_hourly_stats.total_volume + EXCLUDED.total_volume,
        volume_formatted = token_hourly_stats.volume_formatted + EXCLUDED.volume_formatted;
        
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER erc20_transfer_hourly_aggregation
    AFTER INSERT ON erc20_transfers
    FOR EACH ROW EXECUTE FUNCTION update_token_hourly_stats();
```

## Cross-Chain Patterns

### Multi-Chain Schema Design

```sql
-- Chain registry
CREATE TABLE blockchain_networks (
    chain_id INTEGER PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    symbol VARCHAR(10) NOT NULL,
    rpc_url VARCHAR(200),
    explorer_url VARCHAR(200),
    
    -- Chain characteristics
    block_time_seconds INTEGER,
    consensus_mechanism VARCHAR(20), -- 'pow', 'pos', 'poa'
    native_currency_symbol VARCHAR(10),
    native_currency_decimals INTEGER DEFAULT 18,
    
    -- Status
    is_active BOOLEAN DEFAULT TRUE,
    is_testnet BOOLEAN DEFAULT FALSE,
    
    -- Metadata
    launch_date DATE,
    description TEXT
);

-- Multi-chain blocks table
CREATE TABLE blocks_multichain (
    chain_id INTEGER NOT NULL REFERENCES blockchain_networks(chain_id),
    number BIGINT NOT NULL,
    hash VARCHAR(66) NOT NULL,
    parent_hash VARCHAR(66) NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    
    -- Chain-specific fields
    gas_limit BIGINT,
    gas_used BIGINT,
    difficulty NUMERIC(78,0),
    miner VARCHAR(42),
    
    -- Universal identifier
    global_block_id UUID DEFAULT gen_random_uuid(),
    
    PRIMARY KEY (chain_id, number),
    UNIQUE (chain_id, hash)
);

-- Multi-chain token transfers
CREATE TABLE token_transfers_multichain (
    id SERIAL PRIMARY KEY,
    chain_id INTEGER NOT NULL REFERENCES blockchain_networks(chain_id),
    
    -- Transaction context
    tx_hash VARCHAR(66) NOT NULL,
    log_index INTEGER NOT NULL,
    block_number BIGINT NOT NULL,
    block_timestamp TIMESTAMP NOT NULL,
    
    -- Token details
    contract_address VARCHAR(42) NOT NULL,
    token_standard VARCHAR(10) NOT NULL, -- 'ERC20', 'BEP20', etc.
    
    -- Transfer details
    from_addr VARCHAR(42) NOT NULL,
    to_addr VARCHAR(42) NOT NULL,
    amount NUMERIC(78,0) NOT NULL,
    amount_formatted DECIMAL,
    
    -- Cross-chain linking
    bridge_tx_hash VARCHAR(66), -- If this is part of a bridge operation
    origin_chain_id INTEGER,
    destination_chain_id INTEGER,
    
    UNIQUE(chain_id, tx_hash, log_index),
    
    -- Foreign key to blocks
    FOREIGN KEY (chain_id, block_number) REFERENCES blocks_multichain(chain_id, number)
);

-- Cross-chain bridge events
CREATE TABLE bridge_events (
    id SERIAL PRIMARY KEY,
    
    -- Source transaction
    source_chain_id INTEGER NOT NULL,
    source_tx_hash VARCHAR(66) NOT NULL,
    source_block_number BIGINT NOT NULL,
    source_log_index INTEGER,
    
    -- Destination transaction (may be NULL if pending)
    dest_chain_id INTEGER NOT NULL,
    dest_tx_hash VARCHAR(66),
    dest_block_number BIGINT,
    dest_log_index INTEGER,
    
    -- Bridge details
    bridge_protocol VARCHAR(50) NOT NULL,
    token_address VARCHAR(42) NOT NULL,
    amount NUMERIC(78,0) NOT NULL,
    sender_address VARCHAR(42) NOT NULL,
    recipient_address VARCHAR(42) NOT NULL,
    
    -- Status tracking
    status VARCHAR(20) DEFAULT 'pending', -- 'pending', 'completed', 'failed'
    initiated_at TIMESTAMP NOT NULL,
    completed_at TIMESTAMP,
    
    -- Fees
    source_fee NUMERIC(78,0),
    dest_fee NUMERIC(78,0),
    bridge_fee NUMERIC(78,0),
    
    UNIQUE(source_chain_id, source_tx_hash, source_log_index),
    
    CONSTRAINT valid_status CHECK (status IN ('pending', 'completed', 'failed')),
    CONSTRAINT different_chains CHECK (source_chain_id != dest_chain_id)
);
```

## Analytics-Optimized Patterns

### Star Schema for BI Tools

```sql
-- Fact table for transfers
CREATE TABLE fact_token_transfers (
    transfer_id SERIAL PRIMARY KEY,
    
    -- Dimension keys
    token_dim_key INTEGER NOT NULL,
    sender_dim_key INTEGER NOT NULL,
    receiver_dim_key INTEGER NOT NULL,
    time_dim_key INTEGER NOT NULL,
    block_dim_key INTEGER NOT NULL,
    
    -- Measures
    amount_raw NUMERIC(78,0) NOT NULL,
    amount_formatted DECIMAL NOT NULL,
    gas_used BIGINT,
    gas_price NUMERIC(78,0),
    transaction_fee NUMERIC(78,0),
    
    -- Degenerate dimensions (high cardinality)
    tx_hash VARCHAR(66) NOT NULL,
    log_index INTEGER NOT NULL,
    
    -- Calculated measures
    amount_usd DECIMAL,
    fee_usd DECIMAL
);

-- Token dimension
CREATE TABLE dim_tokens (
    token_dim_key SERIAL PRIMARY KEY,
    address VARCHAR(42) NOT NULL UNIQUE,
    symbol VARCHAR(20) NOT NULL,
    name VARCHAR(100) NOT NULL,
    decimals INTEGER NOT NULL,
    
    -- Classification
    category VARCHAR(50),
    subcategory VARCHAR(50),
    is_stablecoin BOOLEAN DEFAULT FALSE,
    is_governance_token BOOLEAN DEFAULT FALSE,
    
    -- Metadata
    total_supply NUMERIC(78,0),
    market_cap_usd DECIMAL,
    launch_date DATE,
    
    -- SCD Type 2 fields
    effective_from TIMESTAMP DEFAULT NOW(),
    effective_to TIMESTAMP DEFAULT '9999-12-31'::TIMESTAMP,
    is_current BOOLEAN DEFAULT TRUE
);

-- Address dimension
CREATE TABLE dim_addresses (
    address_dim_key SERIAL PRIMARY KEY,
    address VARCHAR(42) NOT NULL UNIQUE,
    
    -- Classification
    address_type VARCHAR(20), -- 'eoa', 'contract', 'exchange', 'bridge'
    category VARCHAR(50), -- 'exchange', 'defi_protocol', 'nft_marketplace'
    subcategory VARCHAR(50),
    
    -- Metadata
    label VARCHAR(100),
    name VARCHAR(200),
    is_verified BOOLEAN DEFAULT FALSE,
    risk_score DECIMAL(3,2), -- 0.00 to 1.00
    
    -- Activity metrics (periodically updated)
    first_seen_block BIGINT,
    last_seen_block BIGINT,
    total_transactions BIGINT DEFAULT 0,
    total_volume_usd DECIMAL DEFAULT 0,
    
    -- Flags
    is_exchange BOOLEAN DEFAULT FALSE,
    is_contract BOOLEAN DEFAULT FALSE,
    is_dex BOOLEAN DEFAULT FALSE,
    is_bridge BOOLEAN DEFAULT FALSE
);

-- Time dimension
CREATE TABLE dim_time (
    time_dim_key SERIAL PRIMARY KEY,
    date DATE NOT NULL UNIQUE,
    
    -- Date parts
    year INTEGER NOT NULL,
    quarter INTEGER NOT NULL,
    month INTEGER NOT NULL,
    day INTEGER NOT NULL,
    day_of_week INTEGER NOT NULL,
    day_of_year INTEGER NOT NULL,
    week_of_year INTEGER NOT NULL,
    
    -- Text representations
    year_month VARCHAR(7), -- '2024-01'
    quarter_text VARCHAR(6), -- '2024Q1'
    month_name VARCHAR(12),
    day_name VARCHAR(12),
    
    -- Business calendar
    is_weekend BOOLEAN,
    is_holiday BOOLEAN,
    is_trading_day BOOLEAN,
    
    -- Relative indicators
    is_current_date BOOLEAN DEFAULT FALSE,
    days_from_today INTEGER
);

-- Block dimension
CREATE TABLE dim_blocks (
    block_dim_key SERIAL PRIMARY KEY,
    block_number BIGINT NOT NULL UNIQUE,
    block_hash VARCHAR(66) NOT NULL,
    
    -- Block metadata
    timestamp TIMESTAMP NOT NULL,
    gas_limit BIGINT,
    gas_used BIGINT,
    gas_utilization DECIMAL(5,4),
    transaction_count INTEGER,
    
    -- Size metrics
    size_bytes BIGINT,
    
    -- Performance metrics
    avg_gas_price NUMERIC(78,0),
    median_gas_price NUMERIC(78,0)
);
```

## Schema Evolution Patterns

### Version Control and Migration

```sql
-- Schema version tracking
CREATE TABLE schema_versions (
    version VARCHAR(20) PRIMARY KEY,
    applied_at TIMESTAMP DEFAULT NOW(),
    description TEXT,
    migration_script TEXT,
    rollback_script TEXT
);

-- Add versioning to key tables
ALTER TABLE erc20_tokens ADD COLUMN schema_version VARCHAR(20) DEFAULT '1.0';
ALTER TABLE erc20_transfers ADD COLUMN schema_version VARCHAR(20) DEFAULT '1.0';

-- Backward compatibility views
CREATE VIEW erc20_transfers_v1 AS
SELECT 
    id,
    tx_hash,
    log_index,
    contract_address,
    from_addr,
    to_addr,
    amount,
    block_number,
    block_timestamp
FROM erc20_transfers
WHERE schema_version = '1.0'
   OR schema_version IS NULL;  -- Handle legacy data

-- Forward compatibility (adding new fields)
CREATE VIEW erc20_transfers_latest AS  
SELECT 
    id,
    tx_hash,
    log_index,
    contract_address,
    from_addr,
    to_addr,
    amount,
    block_number,
    block_timestamp,
    -- New fields with defaults for older records
    COALESCE(amount_usd, 0) as amount_usd,
    COALESCE(gas_used, 0) as gas_used,
    COALESCE(transaction_fee, 0) as transaction_fee
FROM erc20_transfers;
```

## Best Practices Summary

### Design Principles
✅ **Understand your query patterns before designing**  
✅ **Choose normalization level based on use case**  
✅ **Use appropriate data types (NUMERIC for big integers)**  
✅ **Add constraints for data validation**  
✅ **Design for time-series access patterns**

### Performance Optimization
✅ **Partition large tables by time dimensions**  
✅ **Create covering indexes for common queries**  
✅ **Use materialized views for expensive aggregations**  
✅ **Denormalize for analytics workloads**  
✅ **Pre-compute common calculations**

### Maintainability  
✅ **Version your schema changes**  
✅ **Create compatibility views**  
✅ **Document business rules in constraints**  
✅ **Use meaningful naming conventions**  
✅ **Plan for schema evolution**

### Multi-Chain Considerations
✅ **Include chain_id in all relevant tables**  
✅ **Handle cross-chain relationships properly**  
✅ **Account for different address formats**  
✅ **Consider chain-specific features**  
✅ **Plan for different block times and finality**

### Avoid
❌ **Auto-incrementing keys for blockchain data**  
❌ **Storing addresses without proper validation**  
❌ **Missing foreign key constraints**  
❌ **Overly normalized schemas for analytics**  
❌ **Ignoring time-zone considerations**