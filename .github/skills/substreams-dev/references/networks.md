# Supported Networks

Complete list of blockchain networks supported by Substreams, derived from [The Graph Networks Registry](https://networks-registry.thegraph.com/TheGraphNetworksRegistry.json).

## EVM Networks

### Ethereum Networks
- `mainnet` - Ethereum Mainnet
- `sepolia` - Ethereum Sepolia Testnet
- `holesky` - Ethereum Holesky Testnet
- `hoodi` - Ethereum Hoodi Testnet

### Layer 2 Networks
- `optimism` - OP Mainnet
- `optimism-sepolia` - OP Sepolia Testnet
- `arbitrum-one` - Arbitrum One Mainnet
- `arbitrum-sepolia` - Arbitrum Sepolia Testnet
- `arbitrum-nova` - Arbitrum Nova Mainnet
- `base` - Base Chain
- `base-sepolia` - Base Sepolia Testnet
- `polygon-zkevm` - Polygon zkEVM Mainnet
- `matic` - Polygon Mainnet
- `polygon-amoy` - Polygon Amoy Testnet
- `zksync-era` - zkSync Mainnet
- `scroll` - Scroll Mainnet
- `scroll-sepolia` - Scroll Sepolia Testnet
- `linea` - Linea Mainnet
- `linea-sepolia` - Linea Sepolia Testnet
- `mode-mainnet` - Mode Mainnet
- `blast-mainnet` - Blast Mainnet
- `blast-testnet` - Blast Sepolia Testnet
- `zora` - Zora Network
- `boba` - Boba Network

### BSC Networks
- `bsc` - BNB Smart Chain Mainnet
- `chapel` - BNB Smart Chain Chapel Testnet
- `bnb-op` - opBNB Mainnet

### Other EVM Networks
- `avalanche` - Avalanche C-Chain
- `fantom` - Fantom Opera Mainnet
- `fuse` - Fuse Mainnet
- `moonbeam` - Moonbeam Mainnet
- `moonriver` - Moonriver Mainnet
- `unichain` - Unichain Mainnet
- `unichain-testnet` - Unichain Sepolia Testnet
- `sei-mainnet` - Sei Network
- `monad` - Monad Mainnet
- `injective-evm` - Injective EVM Mainnet
- `injective-evm-testnet` - Injective EVM Testnet
- `soneium` - Soneium Mainnet
- `soneium-testnet` - Soneium Minato Testnet
- `ronin` - Ronin Mainnet
- `etherlink-mainnet` - Etherlink Mainnet
- `worldchain` - World Chain Mainnet
- `ink` - Ink Mainnet
- `xai` - Xai Mainnet
- `tron-evm` - TRON EVM Mainnet
- `katana` - Katana Mainnet
- `berachain` - Berachain Mainnet
- `chiliz` - Chiliz Mainnet
- `chiliz-testnet` - Chiliz Spicy Testnet

## Non-EVM Networks

### Antelope/EOSIO Networks
- `wax` - WAX Mainnet
- `wax-testnet` - WAX Testnet
- `telos` - Telos Mainnet
- `telos-testnet` - Telos Testnet
- `kylin` - Vaulta Kylin Testnet
- `jungle4` - Vaulta Jungle4 Testnet
- `ultra` - Ultra Mainnet
- `eos` - Vaulta Mainnet

### Consensus Layer Networks
- `mainnet-cl` - Ethereum Consensus Layer
- `gnosis-cl` - Gnosis Consensus Layer
- `gnosis-chiado-cl` - Gnosis Chiado Consensus Layer
- `sepolia-cl` - Ethereum Sepolia Consensus Layer
- `hoodi-cl` - Ethereum Hoodi Consensus Layer

### Bitcoin Networks
- `btc` - Bitcoin Mainnet
- `litecoin` - Litecoin Mainnet

### Cosmos Networks
- `injective` - Injective Mainnet
- `injective-testnet` - Injective Testnet
- `mantra-mainnet` - Mantra Mainnet
- `mantra-testnet` - Mantra Dukong Testnet

### NEAR Networks
- `near` - Near Mainnet
- `near-testnet` - Near Testnet

### Solana Networks
- `solana` - Solana Mainnet
- `solana-accounts` - Solana Mainnet (Accounts data)
- `solana-devnet` - Solana Devnet
- `bnb-svm` - svmBNB Mainnet

### Starknet Networks
- `starknet` - Starknet Mainnet
- `starknet-testnet` - Starknet Sepolia Testnet

### Other Networks
- `stellar` - Stellar Mainnet
- `stellar-testnet` - Stellar Testnet
- `tron` - TRON Mainnet

## Usage in Manifest

Specify the network in your `substreams.yaml`:

```yaml
specVersion: v0.1.0
package:
  name: my-substreams
  version: v1.0.0

network: mainnet  # Use network ID from above

modules:
  - name: map_events
    kind: map
    inputs:
      - source: sf.ethereum.type.v2.Block  # For EVM networks
      # - source: sf.solana.type.v1.Block   # For Solana
      # - source: sf.near.type.v1.Block     # For NEAR
```

## Network-Specific Considerations

### EVM Networks
- Use `sf.ethereum.type.v2.Block` as source input
- Transaction structure is consistent across EVM networks
- Gas mechanics may vary (e.g., Polygon uses MATIC, BSC uses BNB)

### Solana
- Use `sf.solana.type.v1.Block` as source input
- Different transaction structure (accounts, instructions)
- No gas concept (uses compute units and fees)

### NEAR Protocol
- Use `sf.near.type.v1.Block` as source input
- Account-based model with function calls
- Different fee structure

### Cosmos Networks
- Use `sf.cosmos.type.v1.Block` as source input
- Message-based transactions
- Different consensus mechanism (Tendermint)

## Running Substreams

Specify the network when running using its network alias (the `-e` flag accepts both network aliases and full endpoint URLs):

```bash
# Ethereum Mainnet
substreams run -e mainnet substreams.yaml module_name

# Polygon
substreams run -e matic substreams.yaml module_name

# Solana
substreams run -e solana substreams.yaml module_name

# Or use explicit endpoint URLs
substreams run -e mainnet.eth.streamingfast.io:443 substreams.yaml module_name
```

## Network Configuration

### Authentication
All networks require authentication:

```bash
# Recommended: API key
export SUBSTREAMS_API_KEY="your-api-key"

# Alternative: JWT token (legacy)
export SUBSTREAMS_API_TOKEN="your-api-token"
```

### Rate Limits
- Free tier: Limited requests per minute
- Pro tier: Higher rate limits
- Enterprise: Custom rate limits

### Data Availability
- **Real-time**: Latest blocks available within seconds
- **Historical**: Full historical data available
- **Reorganizations**: Handled automatically with cursors

## Best Practices

1. **Choose the right network**: Consider transaction volume and costs
2. **Test on local network or testnets**: Use Local Development Environment ([Ethereum](https://docs.substreams.dev/how-to-guides/develop-your-own-substreams/on-evm/local-development), [Solana](https://docs.substreams.dev/how-to-guides/develop-your-own-substreams/solana/local-development)) or some testnets like Sepolia or Holesky for Ethereum testing
3. **Monitor performance**: Different networks have different characteristics
4. **Handle network-specific features**: Some features may not be available on all networks
5. **Consider data costs**: Historical data usage may incur costs

## Getting Access

1. Sign up at [thegraph.market](https://thegraph.market)
2. Create API key
3. Configure authentication
4. Start building!

For enterprise needs or additional networks, contact [StreamingFast support](mailto:support@streamingfast.io).
