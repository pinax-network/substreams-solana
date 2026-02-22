---
name: release-conventions
description: Naming conventions for tags, releases, and SPKGs in the substreams-svm monorepo. Use when creating releases, tagging versions, or publishing packages.
license: Apache-2.0
compatibility:
  platforms: [claude-code, cursor, vscode, windsurf]
metadata:
  version: 1.0.0
  author: Pinax Network
  documentation: https://github.com/pinax-network/substreams-svm
---

# Release & Packaging Conventions

Naming conventions and release workflow for the Pinax `substreams-svm` monorepo.

## URL Pattern

```
https://github.com/pinax-network/substreams-svm/releases/download/<tag>/<spkg-filename>
```

Examples:
- `https://github.com/pinax-network/substreams-svm/releases/download/solana-dex-v0.3.1/clickhouse-solana-dex-v0.3.1.spkg`
- `https://github.com/pinax-network/substreams-svm/releases/download/solana-balances-v0.2.0/clickhouse-solana-balances-v0.2.0.spkg`

## Tag Format

```
solana-<type>-v<version>
```

Examples: `solana-dex-v0.3.1`, `solana-balances-v0.2.0`, `solana-transfers-v0.1.0`

## Release Name

```
solana-<type> v<version>
```

Examples: `solana-dex v0.3.1`, `solana-balances v0.2.0`

## SPKG Naming

Each crate may produce up to 3 SPKGs. All share the same version number.

| Type | Pattern | Example |
|------|---------|---------|
| Base DB module | `solana-<type>-v<version>.spkg` | `solana-dex-v0.3.1.spkg` |
| ClickHouse sink | `clickhouse-solana-<type>-v<version>.spkg` | `clickhouse-solana-dex-v0.3.1.spkg` |
| PostgreSQL sink | `postgres-solana-<type>-v<version>.spkg` | `postgres-solana-dex-v0.3.1.spkg` |

> **Key**: Engine prefix comes first (`clickhouse-` / `postgres-`), then `solana-<type>`.

## substreams.yaml Package Names

| Type | Pattern | Example |
|------|---------|---------|
| Base DB module | `solana_<type>` | `solana_dex` |
| ClickHouse sink | `solana_<type>_clickhouse` | `solana_dex_clickhouse` |
| PostgreSQL sink | `solana_<type>_postgres` | `solana_dex_postgres` |

## SPKG Distribution

SPKGs must be placed in **two locations**:

1. `./spkg/` folder in the repo (so downstream modules can import them)
2. GitHub release assets (for external consumers / k8s templates)

## Release Workflow

1. **Bump version** in all relevant `substreams.yaml` files (base, clickhouse, postgres)
2. **Update import paths** in clickhouse/postgres `substreams.yaml` to reference the new base spkg version
3. **Build**: `cargo build --target wasm32-unknown-unknown --release -p <crate>`
4. **Pack** each module: `substreams pack` in the base, clickhouse, and postgres directories
5. **Copy SPKGs** to `./spkg/` (with correct naming) and remove old versions
6. **Commit** version bumps + spkg files
7. **Tag**: `git tag -a solana-<type>-v<version> -m "solana-<type> v<version>"`
8. **Push** commit and tag
9. **Create GitHub release** with the tag, attach all 3 SPKGs as assets

## Version Alignment

All 3 SPKGs for a given crate (base, clickhouse, postgres) **must share the same version**.

## Directory Structure

The SVM repo uses a flat structure with `db-solana-*` prefixed directories for sink layers:

```
db-solana-<type>/                    # Base DB module
├── substreams.yaml
├── src/lib.rs
├── Cargo.toml
db-solana-<type>-clickhouse/         # ClickHouse sink
├── substreams.yaml
├── schema.*.sql                     # Split schema files
├── Makefile
db-solana-<type>-postgres/           # PostgreSQL sink
├── substreams.yaml
├── schema.sql
```

## Individual DEX Module SPKGs

Individual DEX modules (e.g., `raydium-amm-v4`, `pumpfun`, `jupiter-v6`) are also published as SPKGs and referenced by the aggregate `db-solana-dex` module via imports:

```
raydium-amm-v4-v0.2.0.spkg
pumpfun-bonding-curve-v0.2.2.spkg
jupiter-v6-v0.2.0.spkg
```
