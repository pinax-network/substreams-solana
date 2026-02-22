---
name: substreams-pack
description: Instructions for building, packing, and distributing Substreams packages (.spkg) in the substreams-svm monorepo. Use when building crates, running `substreams pack`, or copying .spkg files.
license: Apache-2.0
compatibility:
  platforms: [claude-code, cursor, vscode, windsurf]
metadata:
  version: 1.0.0
  author: Pinax Network
  documentation: https://github.com/pinax-network/substreams-svm
---

# Substreams Pack & Release (SVM)

Instructions for building, packing, and copying `.spkg` files in the substreams-svm monorepo.

## Overview

The `./spkg/` directory is the central distribution point for shared Substreams packages. Downstream modules import their dependencies from `../spkg/*.spkg`, so packages **must** be copied there after packing.

## Dependency Chain

Packages must be built **bottom-up** — a package's `.spkg` dependencies must exist in `./spkg/` before it can be packed.

```
Level 0 — Leaf crates (individual DEX/token modules):
  ├── raydium/amm-v4/        → raydium-amm-v4-v*.spkg
  ├── raydium/clmm/          → raydium-clmm-v*.spkg
  ├── raydium/cpmm/          → raydium-cpmm-v*.spkg
  ├── raydium/launchpad/     → raydium-launchpad-v*.spkg
  ├── jupiter/v4/            → jupiter-v4-v*.spkg
  ├── jupiter/v6/            → jupiter-v6-v*.spkg
  ├── orca/whirlpool/        → orca-whirlpool-v*.spkg
  ├── meteora/amm/           → meteora-amm-v*.spkg
  ├── meteora/daam/          → meteora-daam-v*.spkg
  ├── meteora/dllm/          → meteora-dllm-v*.spkg
  ├── pumpfun/bonding_curve/ → pumpfun-bonding-curve-v*.spkg
  ├── pumpfun/amm/           → pumpfun-amm-v*.spkg
  ├── pumpswap/              → pumpswap-v*.spkg
  ├── spl-token/             → spl-token-v*.spkg
  ├── native-token/          → native-token-v*.spkg
  └── ... (moonshot, pancakeswap, lifinity, phoenix, openbook, etc.)

Level 1 — DB layer (imports Level 0 .spkg files):
  ├── db-solana-dex/             → solana-dex-v*.spkg
  ├── db-solana-transfers/       → solana-transfers-v*.spkg
  ├── db-solana-balances/        → solana-balances-v*.spkg
  ├── db-solana-accounts/        → solana-accounts-v*.spkg
  └── db-solana-metadata/        → solana-metadata-v*.spkg

Level 2 — Sink layer (imports Level 1 .spkg files):
  ├── db-solana-dex-clickhouse/       → clickhouse-solana-dex-v*.spkg
  ├── db-solana-dex-postgres/         → postgres-solana-dex-v*.spkg
  ├── db-solana-transfers-clickhouse/ → clickhouse-solana-transfers-v*.spkg
  ├── db-solana-transfers-postgres/   → postgres-solana-transfers-v*.spkg
  ├── db-solana-balances-clickhouse/  → clickhouse-solana-balances-v*.spkg
  ├── db-solana-balances-postgres/    → postgres-solana-balances-v*.spkg
  ├── db-solana-accounts-clickhouse/  → clickhouse-solana-accounts-v*.spkg
  ├── db-solana-accounts-postgres/    → postgres-solana-accounts-v*.spkg
  ├── db-solana-metadata-clickhouse/  → clickhouse-solana-metadata-v*.spkg
  └── db-solana-metadata-postgres/    → postgres-solana-metadata-v*.spkg
```

## Step-by-Step: Full Rebuild

### 1. Regenerate Protobuf Bindings (if .proto files changed)

```bash
cd proto && make protogen
```

This runs `substreams protogen --exclude-paths sf/substreams,google` to regenerate `proto/src/pb/*.rs` from `proto/v1/*.proto`.

### 2. Build All Rust Crates

```bash
cargo build --target wasm32-unknown-unknown --release
```

Or build specific crates:

```bash
cargo build --target wasm32-unknown-unknown --release \
  -p raydium-amm-v4 \
  -p jupiter-v6 \
  -p db-solana-dex
```

### 3. Pack Level 0 (leaf crates)

```bash
cd raydium/amm-v4 && substreams pack && cp raydium-amm-v4-v*.spkg ../../spkg/
cd jupiter/v6     && substreams pack && cp jupiter-v6-v*.spkg ../../spkg/
cd pumpfun/bonding_curve && substreams pack && cp pumpfun-bonding-curve-v*.spkg ../../spkg/
cd spl-token      && substreams pack && cp spl-token-v*.spkg ../spkg/
cd native-token   && substreams pack && cp native-token-v*.spkg ../spkg/
```

### 4. Pack Level 1 (DB layer)

These import Level 0 `.spkg` files, so Level 0 must be packed first.

```bash
cd db-solana-dex       && substreams pack && cp solana-dex-v*.spkg ../spkg/
cd db-solana-transfers && substreams pack && cp solana-transfers-v*.spkg ../spkg/
cd db-solana-balances  && substreams pack && cp solana-balances-v*.spkg ../spkg/
```

### 5. Pack Level 2 (Sink layer)

These import Level 1 `.spkg` files, so Level 1 must be packed first.

```bash
cd db-solana-dex-clickhouse && substreams pack && cp clickhouse-solana-dex-v*.spkg ../spkg/
cd db-solana-dex-postgres   && substreams pack && cp postgres-solana-dex-v*.spkg ../spkg/
```

## Important Notes

- **Always build WASM before packing**: `substreams pack` reads compiled `.wasm` files
- **Remove old SPKGs** from `./spkg/` when updating versions
- **Binary type**: SVM uses `wasm/rust-v1+wasm-bindgen-shims` (not plain `wasm/rust-v1`)
- **Nested crate paths**: DEX modules are nested (e.g., `raydium/amm-v4/`), so SPKG copy paths use `../../spkg/`
