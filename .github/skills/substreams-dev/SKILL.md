---
name: substreams-dev
description: Expert knowledge for developing, building, and debugging Substreams projects on any blockchain. Use when working with substreams.yaml manifests, Rust modules, protobuf schemas, or blockchain data processing.
license: Apache-2.0
compatibility:
  platforms: [claude-code, cursor, vscode, windsurf]
metadata:
  version: 1.0.0
  author: StreamingFast
  documentation: https://substreams.streamingfast.io
---

# Substreams Development Expert

Expert assistant for building Substreams projects - high-performance blockchain data indexing and transformation.

## Core Concepts

### What is Substreams?

Substreams is a powerful blockchain indexing technology that enables:
- **Parallel processing** of blockchain data with high performance
- **Composable modules** written in Rust (map, store, index types)
- **Protobuf schemas** for typed data structures
- **Streaming-first** architecture with cursor-based reorg handling

### Key Components

1. **Manifest** (`substreams.yaml`): Defines modules, networks, dependencies
2. **Modules**: Map (transform), Store (aggregate), Index (filter)
3. **Protobuf**: Type-safe schemas for inputs and outputs
4. **WASM**: Rust code compiled to WebAssembly for execution

## Project Structure

```
my-substreams/
├── substreams.yaml          # Manifest
├── proto/
│   └── events.proto         # Schema definitions
├── src/
│   └── lib.rs               # Rust module code
├── Cargo.toml               # Rust dependencies
└── build/                   # Generated files (gitignored)
```

## Prerequisites

### Required CLI Tools

- **substreams**: Core CLI for building, running, and deploying
- **buf**: Required by `substreams build` for protobuf code generation

### Authentication

Running `substreams run` against hosted endpoints requires authentication:

```bash
substreams auth  # Interactive authentication
# Or set SUBSTREAMS_API_TOKEN environment variable
```

## Common Workflows

### Creating a New Project

1. **Initialize**: Use `substreams init` or create manifest manually
2. **Define schema**: Create `.proto` files for your data structures
3. **Implement modules**: Write Rust handlers in `src/lib.rs`
4. **Build**: Run `substreams build` to compile to `.spkg`
5. **Test**: Run `substreams run` with small block range (recommended: 1000 blocks)
6. **Deploy**: Publish to registry or deploy as service

### Module Types

**Map Module** - Transforms input to output
```yaml
- name: map_events
  kind: map
  inputs:
    - source: sf.ethereum.type.v2.Block
  output:
    type: proto:my.types.Events
```

**Store Module** - Aggregates data across blocks
```yaml
- name: store_totals
  kind: store
  updatePolicy: add
  valueType: int64
  inputs:
    - map: map_events
```

**Index Module** - Filters blocks for efficient querying
```yaml
- name: index_transfers
  kind: index
  inputs:
    - map: map_events
  output:
    type: proto:sf.substreams.index.v1.Keys
```

### Debugging Checklist

When modules produce unexpected results:

1. **Validate manifest**: `substreams graph` to visualize dependencies
2. **Test small range**: Run 100-1000 blocks, inspect outputs carefully
3. **Check logs**: Look for WASM panics, protobuf decode errors
4. **Verify schema**: Ensure proto types match expected data
5. **Review inputs**: Confirm input modules produce correct data
6. **Initial block**: Check `initialBlock` is set appropriately

### Performance Optimization

* **Use indexes** to skip irrelevant blocks
* **Minimize store size** by storing only necessary data
* **Production mode** enables parallel execution: `--production-mode`
* **Module granularity**: Smaller, focused modules perform better
* **Avoid deep nesting**: Flatten module dependencies when possible

## Manifest Reference

See [references/manifest-spec.md](./references/manifest-spec.md) for complete specification.

### Key Sections

**Package metadata**:
```yaml
specVersion: v0.1.0
package:
  name: my-substreams
  version: v1.0.0
  description: Description of what this substreams does
```

**Protobuf imports**:
```yaml
protobuf:
  files:
    - events.proto
  importPaths:
    - ./proto
```

**Binary reference** (WASM code):
```yaml
binaries:
  default:
    type: wasm/rust-v1
    file: ./target/wasm32-unknown-unknown/release/my_substreams.wasm
```

**Network configuration**:
```yaml
network: mainnet
```

Supported networks: See [references/networks.md](./references/networks.md)

## Rust Module Development

### Map Handler Example

```rust
use substreams::errors::Error;
use substreams::prelude::*;
use substreams_ethereum::pb::eth::v2::Block;

#[substreams::handlers::map]
pub fn map_events(block: Block) -> Result<Events, Error> {
    let mut events = Events::default();

    for trx in block.transactions() {
        for (log, _call) in trx.logs_with_calls() {
            // Process logs, extract events
            if is_transfer_event(log) {
                events.transfers.push(extract_transfer(log));
            }
        }
    }

    Ok(events)
}
```

### Store Handler Example

```rust
#[substreams::handlers::store]
pub fn store_totals(events: Events, store: StoreAddInt64) {
    for transfer in events.transfers {
        store.add(0, &transfer.token, transfer.amount as i64);
    }
}
```

### Best Practices

* **Handle errors gracefully**: Use `Result<T, Error>` returns
* **Log sparingly**: Excessive logging impacts performance
* **Validate inputs**: Check for null/empty data before processing
* **Use substreams helpers**: Leverage `substreams-ethereum` crate
* **Test locally first**: Always test with `substreams run` before deploying
* **Avoid excessive cloning**: Use ownership transfer (see Performance section below)

## Performance: Avoiding Excessive Cloning

**CRITICAL:** One of the greatest performance impacts in Substreams is excessive cloning of data structures.

### The Problem

Cloning large data structures is expensive:

* ❌ **Cloning a Transaction**: Copies all fields, logs, traces
* ❌ **Cloning a Block**: Copies the entire block including all transactions (EXTREMELY expensive)
* ❌ **Cloning in loops**: Multiplies the cost by number of iterations

### The Solution: Ownership Transfer

Use Rust's ownership system to transfer or borrow data instead of cloning.

#### Bad Example (Excessive Cloning)

```rust
#[substreams::handlers::map]
pub fn map_events(block: Block) -> Result<Events, Error> {
    let mut events = Events::default();

    for trx in block.transactions() {
        // ❌ BAD: Cloning entire transaction
        let transaction = trx.clone();

        for (log, _call) in transaction.logs_with_calls() {
            // ❌ BAD: Cloning log
            let log_copy = log.clone();
            if is_transfer_event(&log_copy) {
                events.transfers.push(extract_transfer(&log_copy));
            }
        }
    }

    Ok(events)
}
```

#### Good Example (Ownership Transfer)

```rust
#[substreams::handlers::map]
pub fn map_events(block: Block) -> Result<Events, Error> {
    let mut events = Events::default();

    // ✅ GOOD: Iterate by reference
    for trx in block.transactions() {
        // ✅ GOOD: Borrow, don't clone
        for (log, _call) in trx.logs_with_calls() {
            if is_transfer_event(log) {
                // ✅ GOOD: Only extract what you need
                events.transfers.push(extract_transfer(log));
            }
        }
    }

    Ok(events)
}

fn is_transfer_event(log: &Log) -> bool {
    // Use reference, no cloning
    !log.topics.is_empty() &&
    log.topics[0] == TRANSFER_EVENT_SIGNATURE
}

fn extract_transfer(log: &Log) -> Transfer {
    // Extract only the fields you need
    Transfer {
        from: Hex::encode(&log.topics[1]),
        to: Hex::encode(&log.topics[2]),
        amount: Hex::encode(&log.data),
        // Don't copy the entire log
    }
}
```

### When Cloning is Acceptable

Clone only small, necessary data:

```rust
// ✅ OK: Cloning small strings
let token_address = Hex::encode(&log.address).clone();

// ✅ OK: Cloning primitive types
let block_number = block.number.clone();

// ❌ BAD: Cloning entire structures
let block_copy = block.clone(); // Never do this!
let trx_copy = transaction.clone(); // Avoid this!
```

### Performance Tips

1. **Use `logs_with_calls()`**: Iterate logs without cloning
   ```rust
   for (log, _call) in trx.logs_with_calls() { } // Good
   for log in trx.receipt.as_ref().unwrap().logs.clone() { } // Bad
   ```

2. **Use references when appropriate**: Pass references to avoid unnecessary cloning
   ```rust
   fn process_log(log: &Log) { } // Good for read-only access
   fn process_log(log: Log) { } // Good when consuming/transforming data
   ```

3. **Extract minimal data**: Only copy what you actually need
   ```rust
   // Good: Extract only needed fields
   let amount = parse_amount(&log.data);

   // Bad: Copy entire log just to get one field
   let log_copy = log.clone();
   let amount = parse_amount(&log_copy.data);
   ```

4. **Use** `into()` for consumption: When you need to consume data
   ```rust
   // When you truly need to take ownership
   events.transfers.push(Transfer {
       from: topics[1].into(), // Consumes the data
       to: topics[2].into(),
   });
   ```

### Common Pitfalls

**Pitfall #1: Cloning in filters**
```rust
// ❌ BAD
block.transactions()
    .iter()
    .filter(|trx| trx.clone().to == target) // Clone every transaction!

// ✅ GOOD
block.transactions()
    .iter()
    .filter(|trx| trx.to == target) // Just compare
```

**Pitfall #2: Unnecessary defensive copies**
```rust
// ❌ BAD
let block_copy = block.clone();
for trx in block_copy.transactions() { } // Why clone the whole block?

// ✅ GOOD
for trx in block.transactions() { } // Use the block directly
```

**Pitfall #3: Cloning for mutation**
```rust
// ❌ BAD
let mut trx_copy = trx.clone();
trx_copy.value = process(trx_copy.value); // Clone just to mutate

// ✅ GOOD
let new_value = process(&trx.value); // Process reference, create new value
```

### Measuring Impact

Use `substreams run` with timing to measure performance:

```bash
# Test with cloning (slow)
time substreams run -s 17000000 -t +1000 map_events

# Test without cloning (fast)
time substreams run -s 17000000 -t +1000 map_events

# You should see significant speedup (2-10x) by avoiding clones
```

### Remember

* 🎯 **Measure performance impact**: Use timing with `substreams run` to identify bottlenecks
* 🎯 **Clone only when necessary**: Most of the time, borrowing is sufficient
* 🎯 **Block cloning is almost never needed**: This is the #1 performance killer
* 🎯 **Transaction cloning should be rare**: Extract only the data you need

## Common Patterns

See [references/patterns.md](./references/patterns.md) for detailed examples:

* Event extraction from logs
* Store aggregation patterns
* Multi-module composition
* Parameterized modules
* Dynamic data sources
* **Database sink patterns** (delta updates, composite keys, sink SQL workflow)

## Querying Chain Head Block

To get the current head block of a chain (useful for determining the latest block number):

**Using Substreams:**
```bash
# Quick head block lookup for a network
substreams run common@latest -s -1 --network mainnet

# Or with explicit endpoint
substreams run common@latest -e=<network-id-alias-or-host> -s -1 -o jsonl
```
Read the first line of output to get the head block information. The `-s -1` flag starts from the latest block.

**Using firecore:**
```bash
# JSON output (use jq for further processing if available)
firecore tools firehose-client <network-id-alias-or-host> -o json -- -1

# Text output (less detail), first line looks like:
# Block #24327807 (14b58bd3fa091c05a46d084bba1e78090d52556d29f4312da77b7aa3220423f4)
firecore tools firehose-client <network-id-alias-or-host> -o text -- -1
```
Read the first line of output to get the head block information.

## Development Tips

1. **Start small**: Begin with 1000 block range for testing
1. **Use GUI**: `substreams gui` for visual debugging (when available)
1. **Version control**: Commit `.spkg` files for reproducibility
1. **Document modules**: Add `doc:` fields in manifest for clarity

## Troubleshooting

**Build fails**:

* Check Rust toolchain: `rustup target add wasm32-unknown-unknown`
* Ensure `buf` CLI is installed (required for proto generation)
* Verify proto imports are correct
* Add `protobuf.excludePaths` with `sf/substreams` and `google` when importing spkgs
* Ensure binary path in manifest matches build output

**Empty output**:

* Confirm `initialBlock` is before first relevant block
* Check module isn't filtered out by upstream index
* Verify input data exists in block range

**Performance issues**:

* Add indexes to skip irrelevant blocks
* Use `--production-mode` for large ranges

## Resources

* [Official Documentation](https://substreams.streamingfast.io)
* [Module Types Guide](./references/module-types.md)
* [Manifest Specification](./references/manifest-spec.md)
* [Common Patterns](./references/patterns.md)
* [Supported Networks](./references/networks.md)

## Getting Help

* [Discord Community](https://discord.gg/streamingfast)
* [GitHub Issues](https://github.com/streamingfast/substreams/issues)
* [Documentation](https://substreams.streamingfast.io)
