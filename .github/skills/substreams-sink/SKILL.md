---
name: substreams-sink
description: Expert knowledge for consuming Substreams data from Solana/SVM modules. Use when building sinks, real-time data pipelines, or integrating Substreams outputs into applications.
license: Apache-2.0
compatibility:
  platforms: [claude-code, cursor, vscode, windsurf]
metadata:
  version: 1.0.0
  author: StreamingFast
  documentation: https://substreams.streamingfast.io
---

# Substreams Sink Development Expert (SVM)

Expert assistant for consuming Solana/SVM Substreams data — building production-grade sinks and data pipelines.

## Core Concepts

### What is a Substreams Sink?

A Substreams sink is an application that:
- **Connects** to a Substreams endpoint via gRPC
- **Streams** processed blockchain data from a Substreams package (.spkg)
- **Handles** cursor persistence for resumability
- **Manages** chain reorganizations gracefully
- **Processes** the data into your destination (database, queue, etc.)

> **Note:** Before building a custom sink, consider using existing solutions:
> - **[substreams-sink-sql](https://github.com/streamingfast/substreams-sink-sql)** — For PostgreSQL and ClickHouse. Handles cursor management, reorgs, batching, and schema management out of the box.
> - **[substreams-sink-kv](https://github.com/streamingfast/substreams-sink-kv)** — For key-value stores.
> - **[substreams-sink-files](https://github.com/streamingfast/substreams-sink-files)** — For file-based outputs (JSON, CSV, Parquet).

### SVM-Specific Considerations

- **Solana endpoint**: `solana.substreams.pinax.network:443`
- **Output types**: Solana modules output protobuf types like `Events` (per-DEX) or `DatabaseChanges` (aggregate)
- **Address encoding**: Solana addresses are base58-encoded (not hex)
- **High throughput**: Solana produces blocks every ~400ms with many transactions; sinks must handle high data volume

### Authentication

```bash
export SUBSTREAMS_API_KEY="your-api-key"
# Or
export SUBSTREAMS_API_TOKEN="your-jwt-token"
```

Get your API key from [thegraph.market](https://thegraph.market) or [pinax.network](https://pinax.network).

## Language Recommendations

| Language | Recommendation | Best For |
|----------|---------------|----------|
| **Go** | Official SDK (Recommended) | Production sinks, StreamingFast sinks |
| **JavaScript** | Official SDK | Web apps, Node.js services |
| **Python** | Reference implementation | Prototyping, data analysis |
| **Rust** | Reference implementation | High-performance custom sinks |

See the `references/` directory for language-specific sink guides:
- `go-sink.md` — Go (recommended)
- `javascript-sink.md` — JavaScript/Node.js
- `python-sink.md` — Python
- `rust-sink.md` — Rust
- `cursor-reorg.md` — Cursor management & reorg handling

## Quick Start: SQL Sink (Recommended)

For most use cases, use `substreams-sink-sql` with the pre-built SVM SPKGs:

### ClickHouse

```bash
substreams-sink-sql run \
  "clickhouse://default:@localhost:9000/solana" \
  "https://github.com/pinax-network/substreams-svm/releases/download/solana-dex-v0.3.1/clickhouse-solana-dex-v0.3.1.spkg" \
  db_out
```

### PostgreSQL

```bash
substreams-sink-sql run \
  "postgres://user:pass@localhost:5432/solana?sslmode=disable" \
  "https://github.com/pinax-network/substreams-svm/releases/download/solana-dex-v0.3.1/postgres-solana-dex-v0.3.1.spkg" \
  db_out
```

## Available SVM SPKGs

| Module | Description | Output |
|--------|-------------|--------|
| `solana-dex` | All DEX swaps (Raydium, Jupiter, Orca, PumpFun, Meteora, etc.) | `DatabaseChanges` |
| `solana-transfers` | SPL token + native SOL transfers | `DatabaseChanges` |
| `solana-balances` | Token balance changes | `DatabaseChanges` |
| `solana-accounts` | Account creation/updates | `DatabaseChanges` |
| `solana-metadata` | Metaplex token metadata | `DatabaseChanges` |

Each is available in ClickHouse and PostgreSQL variants.
