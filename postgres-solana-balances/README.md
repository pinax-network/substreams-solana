# Solana PostgreSQL Balances

SPL Token & Native SOL balances for Solana stored in PostgreSQL.

## Prerequisites

- [Docker](https://docs.docker.com/get-docker/) and Docker Compose
- [substreams-sink-sql](https://github.com/streamingfast/substreams-sink-sql)
- [Substreams CLI](https://substreams.streamingfast.io/getting-started/installing-the-cli)

## Quick Start

### 1. Start PostgreSQL

```bash
docker compose up -d
```

This starts a PostgreSQL 16 container with:
- **User**: `dev-node`
- **Password**: `insecure-change-me-in-prod`
- **Database**: `dev-node`
- **Port**: `5432`

### 2. Setup Schema

```bash
make setup
```

This creates all tables, indexes, and upsert rules in the database.

### 3. Run the Sink

```bash
make dev
```

This streams SPL Token and native SOL balance data from Solana mainnet to your PostgreSQL database.

## Querying PostgreSQL

### Using Docker exec

```bash
# Connect to psql shell
docker exec -it substreams-postgres psql -U dev-node -d dev-node

# Run a single query
docker exec substreams-postgres psql -U dev-node -d dev-node -c "SELECT * FROM spl_token_balances LIMIT 10;"
```

### Using psql directly (if installed locally)

```bash
psql "postgresql://dev-node:insecure-change-me-in-prod@localhost:5432/dev-node"
```

### Example Queries

```sql
-- Count all balances
SELECT COUNT(*) FROM spl_token_balances;
SELECT COUNT(*) FROM native_balances;

-- Get top 10 SPL Token balances for a specific mint (e.g., USDC)
SELECT account, owner, balance 
FROM spl_token_balances 
WHERE mint = 'EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v'
ORDER BY balance DESC 
LIMIT 10;

-- Get all SPL Token balances for an owner
SELECT mint, account, balance, timestamp 
FROM spl_token_balances 
WHERE owner = '...your_address...';

-- Get native SOL balance for an address
SELECT balance, timestamp 
FROM native_balances 
WHERE address = '...your_address...';

-- List all tables
\dt

-- Describe a table
\d spl_token_balances
```

## Docker Compose Commands

```bash
# Start PostgreSQL
docker compose up -d

# View logs
docker compose logs -f postgres

# Stop PostgreSQL (keeps data)
docker compose down

# Stop and remove all data (reset)
docker compose down -v

# Check container status
docker compose ps
```

## Configuration

### Environment Variables

You can customize the PostgreSQL connection by setting environment variables in the Makefile or exporting them:

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
| `blocks` | Block metadata (number, hash, timestamp) |
| `spl_token_balances` | Latest SPL Token balances per account/mint |
| `native_balances` | Latest native SOL balances per address |

### SPL Token Balances Schema

```sql
CREATE TABLE spl_token_balances (
    block_num   INTEGER NOT NULL,
    block_hash  TEXT NOT NULL,
    timestamp   TIMESTAMP NOT NULL,
    mint        TEXT NOT NULL,      -- Token mint address
    account     TEXT NOT NULL,      -- Token account address
    owner       TEXT NOT NULL,      -- Owner address
    balance     NUMERIC NOT NULL,   -- Balance in lamports
    PRIMARY KEY (mint, account)
);
```

### Native Balances Schema

```sql
CREATE TABLE native_balances (
    block_num   INTEGER NOT NULL,
    block_hash  TEXT NOT NULL,
    timestamp   TIMESTAMP NOT NULL,
    address     TEXT PRIMARY KEY,   -- Account address
    balance     NUMERIC NOT NULL    -- Balance in lamports
);
```

## How Upserts Work

The schema uses PostgreSQL `RULE`s to handle balance updates. When the same account receives multiple balance changes within a block, only the latest value is stored:

```sql
-- Automatically converts INSERT to UPDATE when key exists
CREATE RULE upsert_spl_token_balances AS
    ON INSERT TO spl_token_balances
    WHERE EXISTS (SELECT 1 FROM spl_token_balances WHERE mint = NEW.mint AND account = NEW.account)
    DO INSTEAD UPDATE ...
```

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
