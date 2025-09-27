# GitHub Copilot Instructions for Substreams Solana

## Project Overview

This repository contains Substreams modules for tracking Solana blockchain data, specifically focused on:

- **Token tracking**: SPL tokens, SPL-2022 tokens, and native SOL
- **DEX protocols**: Raydium, Pump.fun, Jupiter, Meteora, and other AMMs
- **Data sinks**: ClickHouse integration for data storage and analytics

## Architecture

### Workspace Structure

This is a Rust workspace with multiple crates organized by functionality:

- **Token modules**: `spl-token`, `native-token`, `clickhouse-solana-tokens`
- **DEX modules**: `raydium/*`, `jupiter/*`, `meteora/*`, `pumpfun/*`, etc.
- **Common utilities**: `common`, `proto`
- **Database integration**: `clickhouse-solana-dex`

### Key Technologies

- **Substreams**: StreamingFast's data processing framework
- **Rust**: Core language with WebAssembly compilation target
- **Protocol Buffers**: For data serialization and type definitions
- **ClickHouse**: Primary data sink for analytics

## Development Workflow

### Building

```bash
# Build all workspace members
cargo build --target wasm32-unknown-unknown --release

# Check without building
cargo check

# Build specific module
cd <module-directory>
make build
```

### Testing

```bash
# Run unit tests
cargo test

# Test specific module
cd <module-directory>
make noop  # Test with no-op sink
make gui   # Test with GUI visualization
```

### Substreams Development

Each module typically contains:
- `src/lib.rs` - Main module logic with substreams handlers
- `substreams.yaml` - Module configuration and dependencies
- `Makefile` - Build and test commands
- `proto/` - Protocol buffer definitions (if needed)

### Common Makefile Targets

- `make build` - Build the module for WASM target
- `make pack` - Package the substream module
- `make noop` - Test with substreams-sink-noop
- `make gui` - Run with substreams GUI for visualization
- `make prod` - Run in production mode
- `make dev` - Run development mode with SQL sink

## Code Style and Conventions

### Rust Code Style

- Follow standard Rust formatting (`rustfmt` is configured)
- Use `cargo clippy` for linting
- Follow existing patterns in the codebase
- Use descriptive variable names, especially for blockchain data

### Substreams Patterns

1. **Handler Functions**: Use `#[substreams::handlers::map]` or `#[substreams::handlers::store]`
2. **Error Handling**: Use `Result<T, Error>` and proper error propagation
3. **Data Structures**: Define clear protobuf messages for module outputs
4. **Logging**: Use `substreams::log!` for debugging

### Protocol Buffer Conventions

- Use semantic versioning for proto packages (e.g., `v1`, `v2`)
- Group related messages in logical modules
- Follow naming conventions: `PascalCase` for messages, `snake_case` for fields

## Testing Guidelines

### Unit Tests

- Test core business logic with standard Rust `#[test]` functions
- Mock external dependencies where possible
- Focus on edge cases and error conditions

### Integration Tests

- Use `make noop` for basic functionality verification
- Use `make gui` for visual verification of data processing
- Test against known block ranges with expected outcomes

### Performance Tests

- Monitor processing speed and memory usage
- Use production mode for performance testing
- Validate parallel processing capabilities

## Documentation Standards

### Code Documentation

- Document all public functions and modules
- Use doc comments (`///`) for API documentation
- Include examples in documentation where helpful

### README Files

- Each module should have a comprehensive README
- Document supported program IDs and instruction types
- Include usage examples and configuration options
- List known limitations or todos

### Comments

- Use inline comments sparingly, prefer self-documenting code
- Comment complex blockchain logic and data transformations
- Explain business logic specific to DEX or token protocols

## Blockchain-Specific Guidelines

### Solana Development

- Understand Solana's account model and program structure
- Use proper public key handling with `bs58` encoding
- Handle instruction data parsing carefully
- Account for different program versions and upgrades

### DEX Integration

- Each DEX has unique instruction formats and data structures
- Use appropriate IDL definitions from `substreams-solana-idls`
- Handle swap events, liquidity changes, and pool operations
- Map program IDs to human-readable protocol names

### Data Processing

- Process pre/post token balances for accurate accounting
- Handle instruction failures and partial executions
- Aggregate data appropriately for analytics use cases
- Maintain referential integrity across related events

## Common Patterns

### Module Dependencies

```yaml
# substreams.yaml example
imports:
  sol: https://github.com/streamingfast/substreams-foundational-modules/releases/download/substreams-v0.3.3/solana-common-v0.3.3.spkg

modules:
  - name: map_events
    kind: map
    inputs:
      - source: sf.solana.type.v1.Block
    output:
      type: proto:myprotocol.v1.Events
```

### Error Handling

```rust
use substreams::errors::Error;

#[substreams::handlers::map]
pub fn map_events(block: Block) -> Result<Events, Error> {
    // Processing logic
    Ok(events)
}
```

### Database Integration

```rust
use substreams_database_change::pb::database::DatabaseChanges;

// Transform events to database changes
pub fn to_database_changes(events: &Events) -> DatabaseChanges {
    // Implementation
}
```

## Debugging and Troubleshooting

### Common Issues

1. **Build failures**: Check Rust toolchain and WASM target installation
2. **Runtime errors**: Use `substreams::log!` for debugging
3. **Performance issues**: Consider data filtering and processing efficiency
4. **Data inconsistencies**: Verify instruction parsing and event handling

### Useful Commands

```bash
# Debug specific block range
make gui START_BLOCK=365000000

# Check module health
make noop

# Validate output format
substreams pack && substreams info substreams.yaml
```

## Contributing Guidelines

1. **Fork and branch**: Create feature branches for new development
2. **Test thoroughly**: Ensure all tests pass and new functionality works
3. **Follow conventions**: Adhere to existing code style and patterns
4. **Document changes**: Update READMEs and comments as needed
5. **Review dependencies**: Be mindful of adding new dependencies

## Security Considerations

- Validate all input data from blockchain
- Handle large numbers and potential overflows
- Be cautious with external dependencies
- Follow Rust security best practices

## Performance Optimization

- Use efficient data structures for high-throughput processing
- Consider memory usage in long-running processes
- Optimize protocol buffer serialization
- Leverage parallel processing where appropriate