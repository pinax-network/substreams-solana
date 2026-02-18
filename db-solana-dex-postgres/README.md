# Solana PostgreSQL DEX

DEX swap events for Solana stored in PostgreSQL. Includes Jupiter, Pump.fun, Raydium, and Meteora.

## Prerequisites

- [Docker](https://docs.docker.com/get-docker/) and Docker Compose
- [substreams-sink-sql](https://github.com/streamingfast/substreams-sink-sql)
- [Substreams CLI](https://substreams.streamingfast.io/getting-started/installing-the-cli)

## Quick Start

### 1. Start PostgreSQL & pgweb

```bash
docker compose up -d
```

This starts:
- **PostgreSQL 16** on port `5432`
- **pgweb** (web UI) on port `8081` → http://localhost:8081

PostgreSQL credentials:
- **User**: `dev-node`
- **Password**: `insecure-change-me-in-prod`
- **Database**: `dev-node`

### 2. Setup Schema

```bash
make setup
```

This creates all tables, indexes, and upsert rules in the database.

### 3. Run the Sink

```bash
make dev
```

This streams DEX swap event data from Solana mainnet to your PostgreSQL database.

## Web UI

Access **pgweb** at http://localhost:8081 - auto-connects to the database, no setup needed!

## Querying PostgreSQL

### Using Docker exec

```bash
# Connect to psql shell
docker exec -it substreams-postgres psql -U dev-node -d dev-node

# Run a single query
docker exec substreams-postgres psql -U dev-node -d dev-node -c "SELECT * FROM jupiter_swap LIMIT 10;"
```

### Using psql directly (if installed locally)

```bash
psql "postgresql://dev-node:insecure-change-me-in-prod@localhost:5432/dev-node"
```

### Example Queries

```sql
-- Count swaps by protocol
SELECT COUNT(*) FROM jupiter_swap;
SELECT COUNT(*) FROM pumpfun_buy;
SELECT COUNT(*) FROM raydium_clmm_swap;

-- Recent Jupiter swaps
SELECT signature, input_mint, input_amount, output_mint, output_amount, timestamp
FROM jupiter_swap
ORDER BY timestamp DESC
LIMIT 100;

-- Top Pump.fun tokens by buy volume
SELECT mint, COUNT(*) as buy_count, SUM(sol_amount) as total_sol
FROM pumpfun_buy
GROUP BY mint
ORDER BY total_sol DESC
LIMIT 100;

-- List all tables
\dt

-- Describe a table
\d jupiter_swap
```

## Docker Compose Commands

```bash
# Start PostgreSQL & pgweb
docker compose up -d

# View logs
docker compose logs -f postgres

# Stop (keeps data)
docker compose down

# Stop and remove all data (reset)
docker compose down -v

# Check container status
docker compose ps
```

## Configuration

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `ENDPOINT` | `solana.substreams.pinax.network:443` | Substreams endpoint |
| `START_BLOCK` | `350000000` | Starting block number |
| `STOP_BLOCK` | `350002000` | Ending block number |
| `PG_DSN` | `psql://dev-node:...@localhost:5432/dev-node?sslmode=disable` | PostgreSQL connection string |

### Customizing Block Range

```bash
# Sync a specific block range
make dev START_BLOCK=300000000 STOP_BLOCK=300001000
```

## Schema

### Tables

| Table | Description |
|-------|-------------|
| `jupiter_swap` | Jupiter V4 & V6 swap events |
| `pumpfun_buy` | Pump.fun bonding curve buy events |
| `pumpfun_sell` | Pump.fun bonding curve sell events |
| `pumpfun_amm_buy` | Pump.fun AMM buy events |
| `pumpfun_amm_sell` | Pump.fun AMM sell events |
| `raydium_amm_v4_swap_base_in` | Raydium AMM V4 swap base in events |
| `raydium_amm_v4_swap_base_out` | Raydium AMM V4 swap base out events |
| `raydium_clmm_swap` | Raydium CLMM swap events |
| `raydium_cpmm_swap_base_in` | Raydium CPMM swap base in events |
| `raydium_cpmm_swap_base_out` | Raydium CPMM swap base out events |
| `raydium_launchpad_buy` | Raydium Launchpad buy events |
| `raydium_launchpad_sell` | Raydium Launchpad sell events |
| `meteora_dllm_swap` | Meteora DLLM swap events |
| `meteora_daam_swap` | Meteora DAAM swap events |
| `meteora_amm_swap` | Meteora AMM swap events |
| `blocks` | Solana blocks |

## Troubleshooting

### Connection refused

Make sure PostgreSQL is running:
```bash
docker compose ps
docker compose logs postgres
```

### Permission denied

Check that the credentials match:
```bash
docker exec substreams-postgres psql -U dev-node -d dev-node -c "SELECT 1;"
```

### Reset everything

```bash
docker compose down -v
docker compose up -d
make setup
```

## License

[MIT](../LICENSE)
