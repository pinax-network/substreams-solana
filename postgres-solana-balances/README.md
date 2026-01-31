# Solana PostgreSQL Balances

SPL Token & Native SOL balances for Solana stored in PostgreSQL.

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

This streams SPL Token and native SOL balance data from Solana mainnet to your PostgreSQL database.

## Web UI

Access **pgweb** at http://localhost:8081 - auto-connects to the database, no setup needed!

## Querying PostgreSQL

### Using Docker exec

```bash
# Connect to psql shell
docker exec -it substreams-postgres psql -U dev-node -d dev-node

# Run a single query
docker exec substreams-postgres psql -U dev-node -d dev-node -c "SELECT * FROM balances LIMIT 10;"
```

### Using psql directly (if installed locally)

```bash
psql "postgresql://dev-node:insecure-change-me-in-prod@localhost:5432/dev-node"
```

### Example Queries

```sql
-- Count all balances
SELECT COUNT(*) FROM balances;
SELECT COUNT(*) FROM balances_native;

-- Top 100 holders for a specific mint (e.g., Wrapped SOL)
SELECT account, amount, block_num, timestamp
FROM balances
WHERE mint = 'So11111111111111111111111111111111111111112'
ORDER BY amount DESC
LIMIT 100;

-- Top 100 holders (non-zero only)
SELECT account, amount, block_num, timestamp
FROM balances
WHERE mint = 'So11111111111111111111111111111111111111112'
  AND amount != 0
ORDER BY amount DESC
LIMIT 100;

-- Get all token balances for an account (by token account address)
SELECT mint, amount, decimals, timestamp 
FROM balances 
WHERE account = '...token_account_address...'
  AND amount != 0;

-- Get all tokens held by an account (find all token accounts)
SELECT mint, account, amount, decimals, timestamp 
FROM balances 
WHERE account IN (
    SELECT account FROM balances WHERE account LIKE '...owner_pattern...'
)
AND amount != 0
ORDER BY amount DESC;

-- Get native SOL balance for an account
SELECT amount, timestamp 
FROM balances_native 
WHERE account = '...your_address...';

-- List all tables
\dt

-- Describe a table
\d balances
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
| `balances` | Latest SPL Token balances per account/mint |
| `balances_native` | Latest native SOL balances per account |

### SPL Token Balances Schema

```sql
CREATE TABLE balances (
    -- block --
    block_num   INTEGER NOT NULL,
    block_hash  TEXT NOT NULL,
    timestamp   TIMESTAMP NOT NULL,

    -- balance --
    program_id  TEXT NOT NULL,      -- Token program ID
    mint        TEXT NOT NULL,      -- Token mint address
    account     TEXT NOT NULL,      -- Token account address
    amount      NUMERIC NOT NULL,   -- Balance amount
    decimals    SMALLINT NOT NULL,  -- Token decimals

    PRIMARY KEY (mint, account)
);
```

### Native SOL Balances Schema

```sql
CREATE TABLE balances_native (
    -- block --
    block_num   INTEGER NOT NULL,
    block_hash  TEXT NOT NULL,
    timestamp   TIMESTAMP NOT NULL,

    -- balance --
    account     TEXT PRIMARY KEY,   -- Account address
    amount      NUMERIC NOT NULL    -- Balance in lamports
);
```

### Indexes

The schema includes optimized indexes for common queries:

**Block indexes** - for filtering by time/block:
- `idx_balances_block_num`, `idx_balances_timestamp`
- `idx_balances_native_block_num`, `idx_balances_native_timestamp`

**Single column indexes** - for lookups:
- `idx_balances_program_id`, `idx_balances_account`, `idx_balances_amount`
- `idx_balances_native_amount`

**Composite indexes** (non-zero balances only):
- `idx_balances_nonzero` - (mint, account)
- `idx_balances_account_mint` - (account, mint) for finding all tokens by account

**Sorted indexes** (non-zero balances only) - for top/bottom holders:
- `idx_balances_mint_amount_desc` - top holders per mint
- `idx_balances_mint_amount_asc` - bottom holders per mint
- `idx_balances_native_amount_desc` - top SOL holders
- `idx_balances_native_amount_asc` - bottom SOL holders

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
