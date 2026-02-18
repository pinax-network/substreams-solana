# Solana PostgreSQL Transfers

SPL Token, SPL-2022 & Native SOL transfers for Solana stored in PostgreSQL.

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

This streams SPL Token and native SOL transfer data from Solana mainnet to your PostgreSQL database.

## Web UI

Access **pgweb** at http://localhost:8081 - auto-connects to the database, no setup needed!

## Querying PostgreSQL

### Using Docker exec

```bash
# Connect to psql shell
docker exec -it substreams-postgres psql -U dev-node -d dev-node

# Run a single query
docker exec substreams-postgres psql -U dev-node -d dev-node -c "SELECT * FROM spl_transfer LIMIT 10;"
```

### Using psql directly (if installed locally)

```bash
psql "postgresql://dev-node:insecure-change-me-in-prod@localhost:5432/dev-node"
```

### Example Queries

```sql
-- Count transfers by type
SELECT COUNT(*) FROM spl_transfer;
SELECT COUNT(*) FROM system_transfer;

-- Recent SPL transfers for a specific mint
SELECT source, destination, amount, signature, timestamp
FROM spl_transfer
WHERE mint = 'So11111111111111111111111111111111111111112'
ORDER BY timestamp DESC
LIMIT 100;

-- Top SOL transfers by amount
SELECT source, destination, lamports, signature, timestamp
FROM system_transfer
ORDER BY lamports DESC
LIMIT 100;

-- List all tables
\dt

-- Describe a table
\d spl_transfer
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
| `spl_transfer` | SPL Token & SPL-2022 transfer events |
| `system_transfer` | Native SOL transfer events |
| `system_transfer_with_seed` | Native SOL transfer with seed events |
| `system_withdraw_nonce_account` | Nonce account withdrawal events |
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
