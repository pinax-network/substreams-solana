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
  ├── db-svm-dex/             → svm-dex-v*.spkg
  ├── db-svm-transfers/       → svm-transfers-v*.spkg
  ├── db-svm-balances/        → svm-balances-v*.spkg
  ├── db-svm-accounts/        → svm-accounts-v*.spkg
  └── db-svm-metadata/        → svm-metadata-v*.spkg

Level 2 — Sink layer (imports Level 1 .spkg files):
  ├── db-svm-dex-clickhouse/       → clickhouse-svm-dex-v*.spkg
  ├── db-svm-dex-postgres/         → postgres-svm-dex-v*.spkg
  ├── db-svm-transfers-clickhouse/ → clickhouse-svm-transfers-v*.spkg
  ├── db-svm-transfers-postgres/   → postgres-svm-transfers-v*.spkg
  ├── db-svm-balances-clickhouse/  → clickhouse-svm-balances-v*.spkg
  ├── db-svm-balances-postgres/    → postgres-svm-balances-v*.spkg
  ├── db-svm-accounts-clickhouse/  → clickhouse-svm-accounts-v*.spkg
  ├── db-svm-accounts-postgres/    → postgres-svm-accounts-v*.spkg
  ├── db-svm-metadata-clickhouse/  → clickhouse-svm-metadata-v*.spkg
  └── db-svm-metadata-postgres/    → postgres-svm-metadata-v*.spkg
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
  -p db-svm-dex
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
cd db-svm-dex       && substreams pack && cp svm-dex-v*.spkg ../spkg/
cd db-svm-transfers && substreams pack && cp svm-transfers-v*.spkg ../spkg/
cd db-svm-balances  && substreams pack && cp svm-balances-v*.spkg ../spkg/
```

### 5. Pack Level 2 (Sink layer)

These import Level 1 `.spkg` files, so Level 1 must be packed first.

```bash
cd db-svm-dex-clickhouse && substreams pack && cp clickhouse-svm-dex-v*.spkg ../spkg/
cd db-svm-dex-postgres   && substreams pack && cp postgres-svm-dex-v*.spkg ../spkg/
```

## Important Notes

- **Always build WASM before packing**: `substreams pack` reads compiled `.wasm` files
- **Remove old SPKGs** from `./spkg/` when updating versions
- **Binary type**: SVM uses `wasm/rust-v1+wasm-bindgen-shims` (not plain `wasm/rust-v1`)
- **Nested crate paths**: DEX modules are nested (e.g., `raydium/amm-v4/`), so SPKG copy paths use `../../spkg/`
