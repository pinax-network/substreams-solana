---
name: substreams-testing
description: Expert knowledge for testing Solana/SVM Substreams applications. Covers unit testing, integration testing, and performance testing patterns.
license: Apache-2.0
compatibility:
  platforms: [claude-code, cursor, vscode, windsurf]
metadata:
  version: 1.0.0
  author: Pinax Network
  documentation: https://substreams.streamingfast.io
---

# Substreams Testing Expert (SVM)

Expert assistant for testing Solana/SVM Substreams applications.

## Testing Philosophy

1. **Test early and often** — Catch issues in development, not production
2. **Test with real data** — Solana edge cases are numerous (failed txs, inner instructions, CPI)
3. **Test at multiple scales** — Single blocks, small ranges, and large datasets
4. **Performance testing** — Solana's high throughput (400ms blocks, thousands of txs) demands it

## Unit Tests

### Testing IDL Decoders

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use substreams_solana_idls::raydium;

    #[test]
    fn test_unpack_swap_base_in() {
        // Known instruction data for a SwapBaseIn
        let instruction_data: Vec<u8> = vec![/* raw bytes */];

        let result = raydium::amm::v4::instructions::unpack(&instruction_data);
        assert!(result.is_ok());

        match result.unwrap() {
            raydium::amm::v4::instructions::RaydiumV4Instruction::SwapBaseIn(swap) => {
                assert!(swap.amount_in > 0);
                assert!(swap.minimum_amount_out > 0);
            }
            _ => panic!("Expected SwapBaseIn instruction"),
        }
    }

    #[test]
    fn test_unpack_invalid_data() {
        let invalid_data: Vec<u8> = vec![0xFF, 0xFF];
        let result = raydium::amm::v4::instructions::unpack(&invalid_data);
        assert!(result.is_err());
    }
}
```

### Testing Helper Functions

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use substreams_solana::base58;

    #[test]
    fn test_base58_encoding() {
        let bytes = vec![0u8; 32];
        let encoded = base58::encode(&bytes);
        assert_eq!(encoded, "11111111111111111111111111111111");
    }

    #[test]
    fn test_to_global_sequence() {
        let clock = Clock {
            number: 100,
            ..Default::default()
        };
        let seq = to_global_sequence(&clock, 5);
        assert_eq!(seq, (100 << 32) + 5);
    }

    #[test]
    fn test_common_key_v2() {
        let clock = Clock {
            id: "abc123".to_string(),
            ..Default::default()
        };
        let keys = common_key_v2(&clock, 0, 1);
        assert_eq!(keys[0], ("block_hash", "abc123".to_string()));
        assert_eq!(keys[1], ("transaction_index", "0".to_string()));
        assert_eq!(keys[2], ("instruction_index", "1".to_string()));
    }
}
```

### Testing DatabaseChanges Output

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use substreams_database_change::tables::Tables;

    #[test]
    fn test_db_row_creation() {
        let mut tables = Tables::new();
        tables.create_row("raydium_amm_v4_swap", [
            ("block_hash", "test_hash".to_string()),
            ("transaction_index", "0".to_string()),
            ("instruction_index", "0".to_string()),
        ])
        .set("signature", "test_sig")
        .set("fee", 5000u64);

        let changes = tables.to_database_changes();
        assert_eq!(changes.table_changes.len(), 1);
        assert_eq!(changes.table_changes[0].table, "raydium_amm_v4_swap");
    }
}
```

## Integration Testing

### Running Against Live Data

```bash
# Test a single DEX module with a small block range
substreams run ./raydium/amm-v4/substreams.yaml \
  map_events \
  --start-block 250000000 \
  --stop-block +100

# Test the aggregate db_out module
substreams run ./db-solana-dex/substreams.yaml \
  db_out \
  --start-block 250000000 \
  --stop-block +100
```

### Verifying Output

```bash
# Output as JSON for inspection
substreams run ./raydium/amm-v4/substreams.yaml \
  map_events \
  --start-block 250000000 \
  --stop-block +10 \
  --output json
```

## Build Verification

### Full Build Test

```bash
# Build all workspace crates
cargo build --target wasm32-unknown-unknown --release

# Run all unit tests (native target)
cargo test

# Build specific crate
cargo build --target wasm32-unknown-unknown --release -p raydium-amm-v4
```

### Schema Validation

For ClickHouse sinks with split schema files, verify schemas concatenate correctly:

```bash
cd db-solana-dex-clickhouse
make schema  # Concatenates split files into schema.sql
```

## Performance Testing

### Block Range Recommendations

| Test Type | Block Range | Purpose |
|-----------|-------------|---------|
| Smoke test | `+10` blocks | Verify basic functionality |
| Unit range | `+100` blocks | Check output correctness |
| Integration | `+1000` blocks | Verify cross-block consistency |
| Performance | `+10000` blocks | Measure throughput |
| Soak test | `+100000` blocks | Long-running stability |

### Solana-Specific Considerations

- **Vote transactions**: Use `blocks_without_votes` to exclude ~80% of transactions
- **High instruction count**: Some Solana blocks have thousands of instructions
- **CPI depth**: Inner instructions (CPI) can be deeply nested
- **Failed transactions**: Always verify failed transaction handling

## Common Issues

### Build Failures

```bash
# Missing wasm target
rustup target add wasm32-unknown-unknown

# Outdated dependencies
cargo update

# Protobuf generation issues
cd proto && make protogen
```

### Runtime Issues

- **Empty output**: Check program ID filter matches expected program
- **Missing instructions**: Verify `walk_instructions()` covers inner instructions (CPI)
- **Base58 encoding**: Ensure addresses are base58-encoded, not hex
- **Failed txs in output**: Add `is_failed()` check if you want to skip them
