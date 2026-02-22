# Supported Solana Networks

Solana networks supported by Substreams, derived from [The Graph Networks Registry](https://networks-registry.thegraph.com/TheGraphNetworksRegistry.json).

## Solana Networks

| Network ID | Description |
|-----------|-------------|
| `solana` | Solana Mainnet |
| `solana-mainnet-beta` | Solana Mainnet Beta (alias used in some manifests) |
| `solana-accounts` | Solana Mainnet (Accounts data) |

## Manifest Usage

```yaml
network: solana
```

## Endpoints

| Network | Endpoint |
|---------|----------|
| Solana Mainnet | `solana.substreams.pinax.network:443` |

## Authentication

All Substreams endpoints require authentication:

```bash
export SUBSTREAMS_API_KEY="your-api-key"
# Or
substreams auth
```

Get your API key from [thegraph.market](https://thegraph.market) or [pinax.network](https://pinax.network).
