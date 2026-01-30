# Substreams Module Types Guide

Deep dive into the three core module types: Map, Store, and Index.

> **Note:** Code examples below assume the following imports unless stated otherwise:
> ```rust
> use substreams::errors::Error;
> use substreams::prelude::*;
> use substreams::Hex;
> use substreams_ethereum::pb::eth::v2::{Block, TransactionTrace};
> ```

## Map Modules

Map modules transform input data into output data. They are stateless and process one block at a time.

### Characteristics

- **Stateless**: No memory between blocks
- **Pure functions**: Same input always produces same output
- **Parallelizable**: Can process multiple blocks simultaneously
- **Cacheable**: Outputs can be cached and reused

### Use Cases

- Extract events from transaction logs
- Transform block data into custom formats
- Filter and process transactions
- Decode contract calls and events
- Enrich data with external information

### Example: Event Extraction

```rust
#[substreams::handlers::map]
pub fn map_transfers(block: Block) -> Result<Transfers, Error> {
    let mut transfers = Transfers::default();
    
    for trx in block.transactions() {
        for (log, _call) in trx.logs_with_calls() {
            if is_erc20_transfer(log) {
                transfers.items.push(Transfer {
                    tx_hash: Hex::encode(&trx.hash),
                    from: extract_address(&log.topics[1]),
                    to: extract_address(&log.topics[2]),
                    amount: extract_amount(&log.data),
                    token: Hex::encode(&log.address),
                    block_num: block.number,
                });
            }
        }
    }
    
    Ok(transfers)
}
```

### Performance Tips

- Avoid cloning large structures
- Use references when possible
- Filter early to reduce processing
- Minimize allocations in hot paths

## Store Modules

Store modules maintain state across blocks. They aggregate, accumulate, or track data over time.

### Characteristics

- **Stateful**: Maintains data between blocks
- **Sequential**: Must process blocks in order
- **Persistent**: State survives restarts
- **Queryable**: Can be read by other modules

### Update Policies

#### Set Policy
Replaces existing values:

```rust
#[substreams::handlers::store]
pub fn store_latest_price(transfers: Transfers, store: StoreSetString) {
    for transfer in transfers.items {
        let key = format!("token:{}", transfer.token);
        store.set(0, &key, &transfer.amount);
    }
}
```

#### Add Policy
Accumulates numeric values:

```rust
#[substreams::handlers::store]
pub fn store_volume(transfers: Transfers, store: StoreAddBigInt) {
    for transfer in transfers.items {
        let key = format!("daily:{}", transfer.token);
        store.add(0, &key, &BigInt::from_str(&transfer.amount).unwrap());
    }
}
```

#### Append Policy
Concatenates byte values:

```rust
#[substreams::handlers::store]
pub fn store_history(events: Events, store: StoreAppendBytes) {
    for event in events.items {
        let key = format!("history:{}", event.contract);
        store.append(0, &key, &event.data);
    }
}
```

### Value Types

- `string`: UTF-8 strings
- `bytes`: Raw byte arrays
- `int64`: 64-bit signed integers
- `bigint`: Arbitrary precision integers
- `float64`: 64-bit floating point
- `bigdecimal`: Arbitrary precision decimals
- `proto:Type`: Custom protobuf messages

### Reading from Stores

```rust
#[substreams::handlers::map]
pub fn map_enriched(
    transfers: Transfers, 
    metadata_store: StoreGetString
) -> Result<EnrichedTransfers, Error> {
    let mut enriched = EnrichedTransfers::default();
    
    for transfer in transfers.items {
        let key = format!("metadata:{}", transfer.token);
        let metadata = metadata_store.get_last(&key);
        
        enriched.items.push(EnrichedTransfer {
            transfer: Some(transfer),
            metadata,
        });
    }
    
    Ok(enriched)
}
```

### Store Access Modes

#### Default Mode (Read/Write)
Full access to store:

```yaml
inputs:
  - store: my_store
```

#### Get Mode (Read-Only)
Read-only access:

```yaml
inputs:
  - store: my_store
    mode: get
```

#### Deltas Mode
Only receive changes:

```yaml
inputs:
  - store: my_store, mode: deltas
```

### Best Practices

- Use appropriate value types
- Design efficient key schemas
- Minimize store size
- Consider data retention needs
- Use deltas mode when possible

## Index Modules

Index modules filter blocks to improve query performance. They identify which blocks contain relevant data.

### Characteristics

- **Filtering**: Determines block relevance
- **Efficient**: Enables skipping irrelevant blocks
- **Lightweight**: Minimal data output
- **Composable**: Can be chained together

### Use Cases

- Skip blocks without specific events
- Filter by contract addresses
- Identify blocks with high activity
- Create custom block filters

### Example: Transfer Index

```rust
#[substreams::handlers::index]
pub fn index_transfers(transfers: Transfers) -> Result<Keys, Error> {
    let mut keys = Keys::default();
    
    if !transfers.items.is_empty() {
        // This block contains transfers
        keys.keys.push("has_transfers".to_string());
        
        // Index by token addresses
        for transfer in transfers.items {
            keys.keys.push(format!("token:{}", transfer.token));
        }
    }
    
    Ok(keys)
}
```

### Index Key Strategies

#### Boolean Flags
Simple presence indicators:

```rust
if has_condition {
    keys.keys.push("condition_met".to_string());
}
```

#### Categorical Keys
Group by categories:

```rust
keys.keys.push(format!("category:{}", category));
keys.keys.push(format!("type:{}", event_type));
```

#### Hierarchical Keys
Nested categorization:

```rust
keys.keys.push(format!("dex:uniswap:v3"));
keys.keys.push(format!("token:{}:transfers", token_addr));
```

### Performance Impact

Indexes dramatically improve performance by allowing the system to skip irrelevant blocks:

```bash
# Without index: processes all blocks
substreams run map_transfers -s 17000000 -t +100000

# With index: system automatically uses index to skip irrelevant blocks
# when index_transfers is defined as a dependency
substreams run map_transfers -s 17000000 -t +100000
```

### Best Practices

- Create specific, targeted indexes
- Use consistent key naming
- Avoid overly broad indexes
- Test index effectiveness
- Document key schemas

## Module Composition Patterns

### Linear Pipeline

```yaml
modules:
  - name: map_raw_events
    kind: map
    inputs:
      - source: sf.ethereum.type.v2.Block
  
  - name: map_decoded_events
    kind: map
    inputs:
      - map: map_raw_events
  
  - name: store_aggregates
    kind: store
    inputs:
      - map: map_decoded_events
```

### Fan-Out Pattern

```yaml
modules:
  - name: map_events
    kind: map
    inputs:
      - source: sf.ethereum.type.v2.Block
  
  - name: store_totals
    kind: store
    inputs:
      - map: map_events
  
  - name: store_counts
    kind: store
    inputs:
      - map: map_events
  
  - name: index_activity
    kind: index
    inputs:
      - map: map_events
```

### Enrichment Pattern

```yaml
modules:
  - name: map_events
    kind: map
    inputs:
      - source: sf.ethereum.type.v2.Block
  
  - name: store_metadata
    kind: store
    inputs:
      - map: map_events
  
  - name: map_enriched
    kind: map
    inputs:
      - map: map_events
      - store: store_metadata, mode: get
```

## Error Handling

### Map Module Errors

```rust
#[substreams::handlers::map]
pub fn map_events(block: Block) -> Result<Events, Error> {
    let mut events = Events::default();
    
    for trx in block.transactions() {
        match process_transaction(trx) {
            Ok(event) => events.items.push(event),
            Err(e) => {
                // Log error but continue processing
                substreams::log::info!("Failed to process transaction: {}", e);
            }
        }
    }
    
    Ok(events)
}
```

### Store Module Errors

```rust
#[substreams::handlers::store]
pub fn store_data(events: Events, store: StoreSetString) {
    for event in events.items {
        if let Ok(value) = serialize_event(&event) {
            store.set(0, &event.id, &value);
        } else {
            substreams::log::warn!("Failed to serialize event: {}", event.id);
        }
    }
}
```

## Testing Strategies

### Unit Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_map_transfers() {
        let block = create_test_block();
        let result = map_transfers(block).unwrap();
        assert_eq!(result.items.len(), 2);
    }
}
```

### Integration Testing

```bash
# Test with small block range
substreams run map_transfers -s 17000000 -t +100

# Test with known data
substreams run map_transfers -s 17000000 -t 17000001
```

### Performance Testing

```bash
# Measure processing time
time substreams run map_transfers -s 17000000 -t +1000

# Check memory usage
substreams run map_transfers -s 17000000 -t +1000 --debug
```
