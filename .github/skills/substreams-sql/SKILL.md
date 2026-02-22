---
name: substreams-sql
description: Expert knowledge for building SQL database sinks from Solana/SVM Substreams. Covers DatabaseChanges, ClickHouse schemas, PostgreSQL schemas, and materialized views for Solana DEX/token data.
license: Apache-2.0
compatibility:
  platforms: [claude-code, cursor, vscode, windsurf]
metadata:
  version: 1.0.0
  author: Pinax Network
  documentation: https://substreams.streamingfast.io
---

# Substreams SQL Expert (SVM)

Expert assistant for building SQL database sinks from Solana/SVM Substreams data.

## Prerequisites

- **substreams-sink-sql**: Required CLI tool. Install from [releases](https://github.com/streamingfast/substreams-sink-sql/releases).

## Core Concepts

### DatabaseChanges Approach

SVM Substreams use the CDC (Change Data Capture) approach via `DatabaseChanges`:

```rust
use substreams_database_change::pb::sf::substreams::sink::database::v1::DatabaseChanges;
use substreams_database_change::tables::Tables;

#[substreams::handlers::map]
pub fn db_out(
    clock: Clock,
    raydium_events: raydium::Events,
    jupiter_events: jupiter::Events,
) -> Result<DatabaseChanges, Error> {
    let mut tables = Tables::new();

    // Insert block metadata
    let seconds = clock.timestamp.as_ref().unwrap().seconds;
    tables.create_row("blocks", [("block_num", clock.number.to_string())])
        .set("block_hash", &clock.id)
        .set("timestamp", seconds.to_string());

    // Insert DEX swap events
    for (tx_index, tx) in raydium_events.transactions.iter().enumerate() {
        for (instr_index, instruction) in tx.instructions.iter().enumerate() {
            let keys = common::db::common_key_v2(&clock, tx_index, instr_index);
            let row = tables.create_row("raydium_amm_v4_swap", keys);
            common::db::set_raydium_transaction_v2(tx, row);
            common::db::set_raydium_instruction_v2(instruction, row);
            // Set event-specific fields...
        }
    }

    Ok(tables.to_database_changes())
}
```

### Primary Key Pattern

SVM tables use a composite primary key of `(block_hash, transaction_index, instruction_index)`:

```rust
pub fn common_key_v2(clock: &Clock, transaction_index: usize, instruction_index: usize) -> [(&'static str, String); 3] {
    [
        ("block_hash", clock.id.to_string()),
        ("transaction_index", transaction_index.to_string()),
        ("instruction_index", instruction_index.to_string()),
    ]
}
```

## ClickHouse Schema Patterns

### Split Schema Files

The SVM repo uses split schema files for maintainability:

```
db-solana-dex-clickhouse/
├── schema.0.blocks.sql                    # Block metadata table
├── schema.0.functions.helpers.sql         # Helper functions
├── schema.0.functions.programs.sql        # Program ID mappings
├── schema.0.functions.tokens.sql          # Token metadata functions
├── schema.0.template.sql                  # Template for new DEX tables
├── schema.1.events.raydium-amm-v4.sql    # Raydium AMM V4 events
├── schema.1.events.jupiter.sql            # Jupiter swap events
├── schema.1.events.pumpfun.sql            # PumpFun events
├── schema.1.events.orca.sql               # Orca Whirlpool events
├── schema.2.view.swaps.sql                # Unified swaps view
├── schema.3.view.ohlc_prices.sql          # OHLC price materialized view
└── schema.3.view.pool_activity_summary.sql
```

### ClickHouse Table Template

```sql
CREATE TABLE IF NOT EXISTS raydium_amm_v4_swap
(
    -- block --
    block_num                   UInt64,
    block_hash                  FixedString(64),
    timestamp                   DateTime,

    -- ordering --
    transaction_index           UInt32,
    instruction_index           UInt32,

    -- transaction --
    signature                   String,
    fee_payer                   String,
    signers_raw                 String DEFAULT '',
    fee                         UInt64 DEFAULT 0,
    compute_units_consumed      UInt64 DEFAULT 0,

    -- instruction --
    program_id                  String,
    stack_height                UInt32,

    -- event-specific --
    amount_in                   UInt64,
    minimum_amount_out          UInt64,
    pool_coin_token             String,
    pool_pc_token               String,

    -- versioning --
    version                     UInt64 MATERIALIZED toUInt64(block_num) * 4294967296 + toUInt64(transaction_index) * 65536 + toUInt64(instruction_index)
)
ENGINE = ReplacingMergeTree(version)
PRIMARY KEY (block_hash, transaction_index, instruction_index)
ORDER BY (block_hash, transaction_index, instruction_index);
```

### Common ClickHouse Indexes

```sql
CREATE INDEX IF NOT EXISTS idx_block_num ON raydium_amm_v4_swap (block_num) TYPE minmax;
CREATE INDEX IF NOT EXISTS idx_timestamp ON raydium_amm_v4_swap (timestamp) TYPE minmax;
CREATE INDEX IF NOT EXISTS idx_signature ON raydium_amm_v4_swap (signature) TYPE bloom_filter;
CREATE INDEX IF NOT EXISTS idx_fee_payer ON raydium_amm_v4_swap (fee_payer) TYPE bloom_filter;
```

### Unified Swaps View

A materialized view combines all DEX swap tables into a single `swaps` view:

```sql
CREATE TABLE IF NOT EXISTS swaps (
    block_num UInt64,
    timestamp DateTime,
    signature String,
    fee_payer String,
    program_id String,
    dex String,
    input_mint String,
    input_amount UInt64,
    output_mint String,
    output_amount UInt64
) ENGINE = MergeTree()
ORDER BY (timestamp, signature);
```

## PostgreSQL Schema Patterns

### PostgreSQL Table Template

```sql
CREATE TABLE IF NOT EXISTS raydium_amm_v4_swap (
    -- block --
    block_num                   INTEGER NOT NULL,
    block_hash                  TEXT NOT NULL,
    timestamp                   TIMESTAMP NOT NULL,

    -- ordering --
    transaction_index           INTEGER NOT NULL,
    instruction_index           INTEGER NOT NULL,

    -- transaction --
    signature                   TEXT NOT NULL,
    fee_payer                   TEXT NOT NULL,
    signers_raw                 TEXT NOT NULL DEFAULT '',
    fee                         BIGINT NOT NULL DEFAULT 0,
    compute_units_consumed      BIGINT NOT NULL DEFAULT 0,

    -- instruction --
    program_id                  TEXT NOT NULL,
    stack_height                INTEGER NOT NULL,

    -- event-specific --
    amount_in                   BIGINT NOT NULL,
    minimum_amount_out          BIGINT NOT NULL,

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_raydium_swap_block_num ON raydium_amm_v4_swap (block_num);
CREATE INDEX IF NOT EXISTS idx_raydium_swap_timestamp ON raydium_amm_v4_swap (timestamp);
CREATE INDEX IF NOT EXISTS idx_raydium_swap_signature ON raydium_amm_v4_swap (signature);
CREATE INDEX IF NOT EXISTS idx_raydium_swap_fee_payer ON raydium_amm_v4_swap (fee_payer);
```

## Manifest Configuration

### Base DB Module

```yaml
imports:
  database_changes: ../spkg/substreams-sink-database-changes-v4.0.0.spkg
  sql: ../spkg/substreams-sink-sql-protodefs-v1.0.7.spkg
```

### Sink Module

```yaml
sink:
  module: db_out
  type: sf.substreams.sink.sql.v1.Service
  config:
    schema: "./schema.sql"
    engine: clickhouse  # or "postgres"
    postgraphile_frontend:
      enabled: false     # true for postgres
```

## Running the Sink

```bash
# ClickHouse
substreams-sink-sql run \
  "clickhouse://default:@localhost:9000/solana" \
  ./db-solana-dex-clickhouse/substreams.yaml

# PostgreSQL
substreams-sink-sql run \
  "postgres://user:pass@localhost:5432/solana?sslmode=disable" \
  ./db-solana-dex-postgres/substreams.yaml
```
