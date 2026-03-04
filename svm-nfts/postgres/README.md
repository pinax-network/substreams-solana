# Solana PostgreSQL NFT Marketplaces

NFT marketplace events for Solana stored in PostgreSQL. Includes Magic Eden M2, Magic Eden M3, and Tensor.

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
| `magiceden_m2_sell` | Magic Eden M2 NFT listings |
| `magiceden_m2_execute_sale` | Magic Eden M2 NFT sales |
| `magiceden_m3_fulfill_buy` | Magic Eden M3 pool bid fills (NFT sold to pool) |
| `magiceden_m3_fulfill_sell` | Magic Eden M3 pool listing fills (NFT bought from pool) |
| `tensor_list` | Tensor NFT listings |
| `tensor_buy` | Tensor NFT buy instructions |
| `tensor_bid` | Tensor NFT bids |
| `tensor_take` | Tensor NFT take events (actual sales) |
| `blocks` | Solana blocks |

## License

[Apache-2.0](../../LICENSE)
