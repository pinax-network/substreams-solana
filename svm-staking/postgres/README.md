# Solana PostgreSQL Liquid Staking

Liquid staking protocol events for Solana stored in PostgreSQL. Includes Marinade Finance.

## Prerequisites

- [Docker](https://docs.docker.com/get-docker/) and Docker Compose
- [substreams-sink-sql](https://github.com/streamingfast/substreams-sink-sql)
- [Substreams CLI](https://substreams.streamingfast.io/getting-started/installing-the-cli)

## Quick Start

### 1. Start PostgreSQL

```bash
docker compose up -d
```

### 2. Setup Schema

```bash
make setup
```

### 3. Run the Sink

```bash
make dev
```

## Schema

### Tables

| Table | Description |
|-------|-------------|
| `marinade_deposit` | Marinade Finance SOL → mSOL deposits |
| `marinade_deposit_stake_account` | Marinade Finance stake account deposits |
| `marinade_liquid_unstake` | Marinade Finance mSOL → SOL liquid unstake |
| `marinade_add_liquidity` | Marinade Finance add SOL liquidity |
| `marinade_remove_liquidity` | Marinade Finance remove liquidity |
| `marinade_withdraw_stake_account` | Marinade Finance withdraw stake account |
| `blocks` | Solana blocks |

## License

[Apache-2.0](../../LICENSE)
