---
name: substreams-dev
description: Expert knowledge for developing Substreams modules on Solana/SVM. Covers manifest spec, Rust modules, IDL-based decoders, protobuf schemas, and Solana-specific data processing patterns.
license: Apache-2.0
compatibility:
  platforms: [claude-code, cursor, vscode, windsurf]
metadata:
  version: 1.0.0
  author: Pinax Network
  documentation: https://substreams.streamingfast.io
---

# Substreams SVM Development Expert

Expert assistant for building Substreams projects on Solana/SVM — high-performance blockchain data indexing for Solana transactions, instructions, and program events.

## Core Concepts

### What is Substreams?

Substreams is a powerful blockchain indexing technology that enables:
- **Parallel processing** of blockchain data with high performance
- **Composable modules** written in Rust (map, store, index types)
- **Protobuf schemas** for typed data structures
- **Streaming-first** architecture with cursor-based reorg handling

### SVM vs EVM Key Differences

| Aspect | EVM | SVM (Solana) |
|--------|-----|--------------|
| Data source | `sf.ethereum.type.v2.Block` | `sf.solana.type.v1.Block` |
| Decoding | ABI-based (`substreams-ethereum`) | IDL-based (`substreams-solana-idls`) |
| Transactions | Transaction traces with logs | Confirmed transactions with instructions |
| Events | Log topics + data | Program logs + instruction data |
| Addresses | 20-byte hex | 32-byte base58 |
| Filtering | Contract address + event signature | Program ID |
| Common import | `substreams-ethereum` crate | `substreams-solana` crate |

### Key Components

1. **Manifest** (`substreams.yaml`): Defines modules, networks, dependencies
2. **Modules**: Map (transform), Store (aggregate), Index (filter)
3. **Protobuf**: Type-safe schemas for inputs and outputs
4. **WASM**: Rust code compiled to WebAssembly for execution
5. **IDL Decoders**: Anchor/native IDL-based instruction decoders from `substreams-solana-idls`

## Project Structure

```
my-solana-substreams/
├── substreams.yaml          # Manifest
├── proto/
│   └── v1/my-events.proto   # Schema definitions
├── src/
│   └── lib.rs               # Rust module code
├── Cargo.toml               # Rust dependencies
└── target/                   # Build output (gitignored)
```

### Monorepo Structure (substreams-svm)

```
substreams-svm/
├── Cargo.toml                    # Workspace root
├── proto/                        # Shared protobuf definitions
│   └── v1/*.proto
├── common/                       # Shared Rust utilities
│   └── src/{lib.rs, db.rs, solana.rs}
├── spkg/                         # Pre-built SPKG dependencies
│
├── # Individual DEX modules
├── raydium/amm-v4/              # Raydium AMM V4
├── raydium/clmm/                # Raydium CLMM
├── raydium/cpmm/                # Raydium CPMM
├── raydium/launchpad/           # Raydium Launchpad
├── jupiter/v4/                  # Jupiter V4
├── jupiter/v6/                  # Jupiter V6
├── orca/whirlpool/              # Orca Whirlpool
├── meteora/{amm,daam,dllm}/     # Meteora pools
├── pumpfun/bonding_curve/       # PumpFun bonding curve
├── pumpfun/amm/                 # PumpFun AMM
├── pumpswap/                    # PumpSwap
├── moonshot/                    # Moonshot
├── pancakeswap/                 # PancakeSwap
├── lifinity/                    # Lifinity
├── phoenix/                     # Phoenix
├── openbook/                    # OpenBook
├── stabble/                     # Stabble
├── darklake/                    # Darklake
├── bonk-swap/                   # Bonk Swap
│
├── # Token modules
├── spl-token/                   # SPL Token events
├── native-token/                # SOL native transfers
├── metaplex/                    # Metaplex metadata
│
├── # Aggregate DB modules
├── db-solana-dex/               # DEX DatabaseChanges (all DEXs)
├── db-solana-transfers/         # Transfer DatabaseChanges
├── db-solana-balances/          # Balance DatabaseChanges
├── db-solana-accounts/          # Account DatabaseChanges
├── db-solana-metadata/          # Metadata DatabaseChanges
│
├── # Sink modules (ClickHouse / Postgres)
├── db-solana-dex-clickhouse/
├── db-solana-dex-postgres/
├── db-solana-transfers-clickhouse/
├── db-solana-transfers-postgres/
├── db-solana-balances-clickhouse/
├── db-solana-balances-postgres/
├── db-solana-accounts-clickhouse/
├── db-solana-accounts-postgres/
├── db-solana-metadata-clickhouse/
└── db-solana-metadata-postgres/
```

## Prerequisites

### Required CLI Tools

- **substreams**: Core CLI for building, running, and deploying
- **buf**: Required by `substreams build` for protobuf code generation

### Authentication

```bash
substreams auth  # Interactive authentication
# Or set SUBSTREAMS_API_TOKEN environment variable
```

## Rust Dependencies (Cargo.toml)

```toml
[dependencies]
substreams = "0.7.3"
substreams-solana = "0.14.2"
substreams-solana-idls = { git = "https://github.com/pinax-network/substreams-solana-idls", tag = "v1.0.0" }
substreams-database-change = "4.0.0"
prost = "0.13"
prost-types = "0.13"
bs58 = "0.5.1"
```

## Common Workflows

### Creating a New DEX Module

1. **Define protobuf schema** in `proto/v1/<dex-name>.proto`
2. **Create crate directory** with `Cargo.toml` and `src/lib.rs`
3. **Add to workspace** in root `Cargo.toml`
4. **Implement `map_events`** handler using IDL decoders
5. **Create `substreams.yaml`** manifest with program ID filter
6. **Build**: `cargo build --target wasm32-unknown-unknown --release`
7. **Test**: `substreams run` with small block range

### Module Types

**Map Module** — Transforms Solana blocks into typed events:
```yaml
- name: map_events
  kind: map
  inputs:
    - map: solana_common:blocks_without_votes
  blockFilter:
    module: solana_common:program_ids_without_votes
    query:
      string: "program:<PROGRAM_ID>"
  output:
    type: proto:my.dex.v1.Events
```

**Store Module** — Aggregates data across blocks:
```yaml
- name: store_totals
  kind: store
  updatePolicy: add
  valueType: int64
  inputs:
    - map: map_events
```

### Block Filtering

Solana Substreams use `blockFilter` with program IDs to efficiently skip irrelevant transactions:

```yaml
blockFilter:
  module: solana_common:program_ids_without_votes
  query:
    string: "program:675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8"
```

### Binary Type

SVM Substreams use `wasm-bindgen-shims` binary type:

```yaml
binaries:
  default:
    type: wasm/rust-v1+wasm-bindgen-shims
    file: ../../target/wasm32-unknown-unknown/release/my_module.wasm
```

### Network

All SVM manifests use:

```yaml
network: solana
```

## IDL-Based Instruction Decoding

Unlike EVM which uses ABI event signatures, Solana Substreams use IDL decoders from the `substreams-solana-idls` crate:

```rust
use substreams_solana_idls::raydium;

// Unpack instruction data using IDL decoder
match raydium::amm::v4::instructions::unpack(instruction.data()) {
    Ok(raydium::amm::v4::instructions::RaydiumV4Instruction::SwapBaseIn(event)) => {
        // Process swap event
        Some(pb::Instruction {
            program_id: program_id.to_vec(),
            instruction: Some(pb::instruction::Instruction::SwapBaseIn(..)),
        })
    }
    _ => None,
}
```

### Available IDL Decoders

The `substreams-solana-idls` crate provides decoders for:
- **Raydium**: `amm::v4`, `clmm`, `cpmm`, `launchpad`
- **Jupiter**: `v4`, `v6`
- **Orca**: `whirlpool`
- **Meteora**: `amm`, `daam`, `dllm`
- **PumpFun**: `bonding_curve`, `amm`
- **PumpSwap**
- **Moonshot**
- **PancakeSwap**
- **Lifinity**
- **Phoenix**
- **OpenBook**
- **Stabble**, **Darklake**, **Bonk Swap**

## Solana Transaction Processing

### Walking Instructions

```rust
use substreams_solana::{
    block_view::InstructionView,
    pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction},
};

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, Error> {
    Ok(pb::Events {
        transactions: block.transactions_owned()
            .filter_map(process_transaction)
            .collect(),
    })
}

fn process_transaction(tx: ConfirmedTransaction) -> Option<pb::Transaction> {
    let tx_meta = tx.meta.as_ref()?;
    let instructions: Vec<pb::Instruction> = tx.walk_instructions()
        .filter_map(|iview| process_instruction(&iview))
        .collect();

    if instructions.is_empty() {
        return None;
    }

    Some(pb::Transaction {
        signature: tx.hash().to_vec(),
        fee_payer: get_fee_payer(&tx).unwrap_or_default(),
        fee: tx_meta.fee,
        compute_units_consumed: tx_meta.compute_units_consumed(),
        instructions,
        ..Default::default()
    })
}
```

### Common Helpers (from `common` crate)

```rust
use common::solana::{get_fee_payer, get_signers, is_failed, is_invoke, is_success, parse_program_id};
```

### Address Encoding

Solana uses base58 encoding for addresses (not hex):

```rust
use substreams_solana::base58;

let address_string = base58::encode(&address_bytes);
```

## Protobuf Schema Patterns

### Standard Transaction Wrapper

```protobuf
syntax = "proto3";
package my.dex.v1;

message Events {
  repeated Transaction transactions = 1;
}

message Transaction {
  bytes signature                   = 1;
  bytes fee_payer                   = 2;
  repeated bytes signers            = 3;
  uint64 fee                        = 4;
  uint64 compute_units_consumed     = 5;
  repeated Instruction instructions = 6;
  repeated Log logs                 = 7;
}

message Instruction {
  bytes  program_id  = 1;
  uint32 stack_height = 2;
  oneof instruction {
    SwapInstruction swap = 3;
    // ... other instruction types
  }
}
```

## Common Program IDs

| Program | ID |
|---------|-----|
| Raydium AMM V4 | `675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8` |
| Raydium CLMM | `CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK` |
| Raydium CPMM | `CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C` |
| Jupiter V6 | `JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4` |
| Orca Whirlpool | `whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc` |
| PumpFun | `6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P` |
| PumpFun AMM | `pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA` |
| PumpSwap | `PSwapMdSai8tjrEXcxFeQth87xC4rRsa4VA5mhGhXkP` |
| Meteora DLMM | `LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo` |
| Metaplex | `metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s` |
| SPL Token | `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA` |
