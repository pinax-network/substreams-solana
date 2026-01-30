# Database Changes (CDC) Reference

The Database Changes approach streams individual database operations to maintain real-time consistency with the blockchain.

## Core Concepts

### Change Data Capture (CDC)

CDC tracks and streams individual row-level changes to database tables:
- **CREATE**: Insert new rows
- **UPDATE**: Modify existing rows  
- **DELETE**: Remove rows
- **ORDINAL**: Ordering for reorg handling

### Table Operations

Each change specifies:
- **Table name**: Target database table
- **Primary key**: Unique identifier for the row
- **Operation type**: CREATE, UPDATE, or DELETE
- **Field changes**: New and old values for modified columns

## Protobuf Schema

### Complete Schema Definition

```protobuf
syntax = "proto3";

package db_out;

message DatabaseChanges {
  repeated TableChange table_changes = 1;
}

message TableChange {
  string table = 1;           // Table name
  string pk = 2;              // Primary key value
  uint64 ordinal = 3;         // Ordering for reorgs
  Operation operation = 4;    // Type of change
  repeated Field fields = 5;  // Changed fields

  enum Operation {
    UNSPECIFIED = 0;
    CREATE = 1;     // INSERT
    UPDATE = 2;     // UPDATE  
    DELETE = 3;     // DELETE
  }
}

message Field {
  string name = 1;        // Column name
  string new_value = 2;   // New value (for CREATE/UPDATE)
  string old_value = 3;   // Previous value (for UPDATE/DELETE)
}
```

### Usage Examples

**Creating Records**:
```rust
tables
    .create_row("transfers", format!("{}-{}", tx_hash, log_index))
    .set("tx_hash", tx_hash)
    .set("from_addr", transfer.from)
    .set("to_addr", transfer.to)
    .set("amount", transfer.amount.to_string())
    .set("block_number", block.number);
```

**Updating Records**:
```rust
tables
    .update_row("balances", address.clone())
    .set("balance", new_balance.to_string())
    .set("last_updated", block.number);
```

**Deleting Records**:
```rust
tables
    .delete_row("expired_orders", order_id);
```

## Implementation Patterns

### ERC20 Transfer Processing

```rust
use substreams::prelude::*;
use substreams_database_change::pb::sf::substreams::sink::database::v1::DatabaseChanges;
use substreams_database_change::tables::Tables;
use substreams_ethereum::pb::eth::v2::Block;

#[substreams::handlers::map]
pub fn db_out(events: Events) -> Result<DatabaseChanges, Error> {
    let mut tables = Tables::new();

    for transfer in events.erc20_transfers {
        // Create transfer record
        let transfer_id = format!("{}-{}", transfer.tx_hash, transfer.log_index);
        tables
            .create_row("erc20_transfers", transfer_id)
            .set("tx_hash", &transfer.tx_hash)
            .set("log_index", transfer.log_index)
            .set("contract_address", &transfer.contract)
            .set("from_addr", &transfer.from)
            .set("to_addr", &transfer.to)  
            .set("amount", transfer.amount.to_string())
            .set("block_number", transfer.block_number)
            .set("block_timestamp", transfer.timestamp);

        // Update sender balance
        if let Some(from_balance) = transfer.from_balance {
            tables
                .update_row("token_balances", format!("{}:{}", transfer.contract, transfer.from))
                .set("balance", from_balance.to_string())
                .set("last_updated", transfer.block_number);
        }

        // Update receiver balance  
        if let Some(to_balance) = transfer.to_balance {
            tables
                .update_row("token_balances", format!("{}:{}", transfer.contract, transfer.to))
                .set("balance", to_balance.to_string())
                .set("last_updated", transfer.block_number);
        }
    }

    Ok(tables.to_database_changes())
}
```

### Complex State Management

```rust
#[substreams::handlers::map]
pub fn db_out(
    pool_events: PoolEvents, 
    price_store: StoreGetProto<TokenPrice>
) -> Result<DatabaseChanges, Error> {
    let mut tables = Tables::new();

    for swap in pool_events.swaps {
        // Record swap transaction
        let swap_id = format!("{}:{}:{}", swap.tx_hash, swap.log_index, swap.pool_address);
        tables
            .create_row("pool_swaps", swap_id)
            .set("pool_address", &swap.pool_address)
            .set("tx_hash", &swap.tx_hash)
            .set("user", &swap.user)
            .set("token_in", &swap.token_in)
            .set("token_out", &swap.token_out)
            .set("amount_in", swap.amount_in.to_string())
            .set("amount_out", swap.amount_out.to_string())
            .set("price", swap.price.to_string())
            .set("block_number", swap.block_number);

        // Update pool reserves
        tables
            .update_row("pool_reserves", &swap.pool_address)
            .set("token0_reserve", swap.new_reserve0.to_string())
            .set("token1_reserve", swap.new_reserve1.to_string())
            .set("total_liquidity", swap.total_liquidity.to_string())
            .set("last_swap_block", swap.block_number);

        // Update volume statistics
        let daily_key = format!("{}:{}", swap.pool_address, swap.date);
        tables
            .update_row("daily_pool_stats", daily_key)
            .set("volume_usd", swap.daily_volume_usd.to_string())
            .set("tx_count", swap.daily_tx_count)
            .set("unique_users", swap.daily_unique_users);
    }

    // Handle liquidity provision/removal
    for lp_event in pool_events.liquidity_events {
        match lp_event.event_type {
            LiquidityEventType::Add => {
                tables
                    .create_row("liquidity_positions", &lp_event.position_id)
                    .set("pool_address", &lp_event.pool_address)
                    .set("user", &lp_event.user)
                    .set("liquidity_amount", lp_event.amount.to_string())
                    .set("token0_amount", lp_event.token0_amount.to_string())
                    .set("token1_amount", lp_event.token1_amount.to_string())
                    .set("created_block", lp_event.block_number);
            },
            LiquidityEventType::Remove => {
                tables
                    .delete_row("liquidity_positions", &lp_event.position_id);
            }
        }
    }

    Ok(tables.to_database_changes())
}
```

## Advanced Features

### Conditional Updates

```rust
// Only update if value actually changed
if new_balance != old_balance {
    tables
        .update_row("balances", &address)
        .set("balance", new_balance.to_string())
        .set("updated_at", block.timestamp);
}

// Conditional record creation
if transfer.amount > DUST_THRESHOLD {
    tables
        .create_row("significant_transfers", transfer_id)
        .set("from_addr", &transfer.from)
        .set("to_addr", &transfer.to)
        .set("amount", transfer.amount.to_string());
}
```

### Bulk Operations

```rust
// Batch related changes together
for (address, balance_change) in balance_changes {
    if balance_change.new_balance == 0 {
        // Remove zero balances
        tables.delete_row("active_balances", &address);
    } else {
        tables
            .update_row("active_balances", &address) 
            .set("balance", balance_change.new_balance.to_string())
            .set("last_tx", &balance_change.last_tx_hash);
    }
}
```

### Ordinal-based Consistency

Ordinals are automatically managed by the `Tables` struct. Each call to `create_row`, `update_row`, `upsert_row`, or `delete_row` increments an internal ordinal counter, ensuring correct ordering for reorg handling. You do not need to set ordinals manually.

```rust
#[substreams::handlers::map]
pub fn db_out(block: Block) -> Result<DatabaseChanges, Error> {
    let mut tables = Tables::new();

    for trx in block.transactions() {
        for (log, _call) in trx.logs_with_calls() {
            let log_id = format!("{}:{}", Hex::encode(&trx.hash), log.index);

            // Ordinal is assigned automatically by Tables
            tables
                .create_row("all_logs", log_id)
                .set("tx_hash", Hex::encode(&trx.hash))
                .set("log_index", log.index)
                .set("address", Hex::encode(&log.address))
                .set("data", Hex::encode(&log.data));
        }
    }

    Ok(tables.to_database_changes())
}
```

## Schema Considerations

### Primary Key Design

**Good Primary Keys**:
```rust
// Composite keys for uniqueness
format!("{}:{}", tx_hash, log_index)          // Transfer events
format!("{}:{}:{}", pool, user, position)     // LP positions  
format!("{}:{}", contract, holder)            // Token balances
format!("{}:{}", block_number, tx_index)      // Transaction ordering
```

**Avoid**:
```rust
// Don't use auto-incrementing IDs - not reorg safe
tables.create_row("transfers", "AUTO_INCREMENT")  // BAD

// Don't use non-deterministic keys  
tables.create_row("events", uuid::new())          // BAD
```

### Field Types and Validation

```rust
// Proper type handling — all values go through the set() method
// which accepts any type implementing ToDatabaseValue
tables
    .create_row("transactions", &tx_hash)
    .set("hash", &tx_hash)                    // String
    .set("block_number", tx.block_number)     // i64
    .set("gas_used", tx.gas_used.to_string()) // BigInt as string
    .set("success", tx.status == 1)           // Boolean
    .set("timestamp", tx.timestamp)           // Unix timestamp
    .set("input_data", Hex::encode(&tx.input)); // Binary as hex string
```

### Handling NULL Values

```rust
// Optional fields — set() is called on a Row, not on Tables directly
let row = tables.create_row("contracts", &contract_id);

if let Some(name) = contract_name {
    row.set("name", name);
} else {
    row.set("name", ""); // Empty string as fallback
}

// For optional numeric fields, use set_if_null with delta updates
tables
    .upsert_row("stats", &key)
    .set_if_null("first_value", &value)  // Only set if column is NULL
    .set("latest_value", &value);
```

## Error Handling

### Graceful Degradation

```rust
#[substreams::handlers::map]
pub fn db_out(events: Events) -> Result<DatabaseChanges, Error> {
    let mut tables = Tables::new();
    let mut errors = Vec::new();

    for transfer in events.transfers {
        match process_transfer(&transfer) {
            Ok(transfer_data) => {
                tables
                    .create_row("transfers", &transfer_data.id)
                    .set("tx_hash", &transfer_data.tx_hash)
                    .set("amount", &transfer_data.amount);
            },
            Err(e) => {
                errors.push(format!("Transfer {}: {}", transfer.tx_hash, e));
                
                // Create error record for debugging
                tables
                    .create_row("processing_errors", &transfer.tx_hash)
                    .set("error_type", "transfer_processing")
                    .set("error_message", &e.to_string())
                    .set("block_number", transfer.block_number)
                    .set("raw_data", &format!("{:?}", transfer));
            }
        }
    }

    if !errors.is_empty() {
        substreams::log::warn!("Processed with {} errors: {:?}", errors.len(), errors);
    }

    Ok(tables.to_database_changes())
}
```

### Data Validation

```rust
fn validate_transfer(transfer: &Transfer) -> Result<(), String> {
    if transfer.amount == BigInt::zero() {
        return Err("Zero amount transfer".to_string());
    }
    
    if transfer.from == transfer.to {
        return Err("Self-transfer detected".to_string());  
    }
    
    if transfer.from.len() != 42 || transfer.to.len() != 42 {
        return Err("Invalid address format".to_string());
    }
    
    Ok(())
}
```

## Performance Optimization

### Batching Strategies

```rust
// Group related changes
let mut balance_updates = HashMap::new();
let mut new_transfers = Vec::new();

// Collect all changes first
for event in events {
    match event {
        Event::Transfer(t) => {
            new_transfers.push(t);
            balance_updates.entry(t.from).or_insert(Vec::new()).push(t.clone());
            balance_updates.entry(t.to).or_insert(Vec::new()).push(t.clone());
        }
    }
}

// Apply in batches
for transfer in new_transfers {
    tables.create_row("transfers", &transfer.id)
        .set("from_addr", &transfer.from)
        .set("to_addr", &transfer.to)
        .set("amount", &transfer.amount);
}

for (address, transfers) in balance_updates {
    let final_balance = calculate_final_balance(&transfers);
    tables.update_row("balances", &address).set("balance", final_balance);
}
```

### Memory Efficiency

```rust
// Process transactions efficiently — Tables handles batching internally.
// Use references to avoid cloning large structures.
for trx in block.transactions() {
    for (log, _call) in trx.logs_with_calls() {
        if is_relevant_event(log) {
            // Only extract the fields you need
            tables
                .create_row("events", format!("{}:{}", Hex::encode(&trx.hash), log.index))
                .set("tx_hash", Hex::encode(&trx.hash))
                .set("address", Hex::encode(&log.address));
        }
    }
}
```

## Testing CDC Implementation

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer_processing() {
        let transfer = create_test_transfer();
        let result = process_transfer_to_cdc(&transfer).unwrap();
        
        assert_eq!(result.table_changes.len(), 3); // transfer + 2 balance updates
        
        let transfer_change = &result.table_changes[0];
        assert_eq!(transfer_change.table, "erc20_transfers");
        assert_eq!(transfer_change.operation, Operation::Create);
        assert_eq!(transfer_change.fields.len(), 7);
    }

    #[test] 
    fn test_balance_update_deduplication() {
        let transfers = vec![
            create_transfer("0xAAA", "0xBBB", 100),
            create_transfer("0xAAA", "0xCCC", 200), // Same sender
        ];
        
        let result = process_transfers_to_cdc(&transfers).unwrap();
        
        // Should only have one balance update for 0xAAA (sender)
        let balance_updates: Vec<_> = result.table_changes.iter()
            .filter(|c| c.table == "balances" && c.pk.contains("0xAAA"))
            .collect();
            
        assert_eq!(balance_updates.len(), 1);
    }
}
```

### Integration Testing

```bash
# Test with small block range
substreams run -s 18000000 -t +100 db_out

# Validate database state
psql $DSN -c "SELECT COUNT(*) FROM erc20_transfers WHERE block_number BETWEEN 18000000 AND 18000100;"

# Test reorg handling
substreams run -s 18000000 -t +100 db_out --production-mode=false
# Check that duplicate processing produces identical results
```

## Best Practices

### DO

✅ **Use deterministic primary keys**
✅ **Handle reorgs with ordinals**
✅ **Validate data before creating changes**
✅ **Batch related operations**
✅ **Log processing errors gracefully**
✅ **Test with small block ranges first**

### DON'T

❌ **Use auto-incrementing primary keys**
❌ **Ignore ordinal ordering**
❌ **Create changes for invalid data**
❌ **Process each event in isolation**
❌ **Panic on data errors**
❌ **Skip integration testing**

### Performance Tips

1. **Minimize change operations**: Batch updates where possible
2. **Use specific primary keys**: Avoid overly long composite keys
3. **Validate early**: Check data before creating table changes
4. **Handle errors gracefully**: Don't fail entire blocks for single bad records
5. **Monitor change volume**: Track operations per block for performance tuning