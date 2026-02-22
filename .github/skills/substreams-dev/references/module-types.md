# Substreams Module Types Guide (SVM)

Deep dive into module types for Solana/SVM Substreams.

> **Note:** Code examples below assume the following imports unless stated otherwise:
> ```rust
> use substreams::errors::Error;
> use substreams_solana::pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction};
> use substreams_solana::block_view::InstructionView;
> ```

## Map Modules

Map modules transform input data into output data. They are stateless and process one block at a time.

### SVM Event Extraction

```rust
use substreams_solana_idls::raydium;

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, Error> {
    Ok(pb::Events {
        transactions: block.transactions_owned()
            .filter_map(process_transaction)
            .collect(),
    })
}

fn process_transaction(tx: ConfirmedTransaction) -> Option<pb::Transaction> {
    let tx_meta = tx.meta.as_ref()?;
    let instructions: Vec<pb::Instruction> = tx.walk_instructions()
        .filter_map(|iview| process_instruction(&iview))
        .collect();

    if instructions.is_empty() {
        return None;
    }

    Some(pb::Transaction {
        signature: tx.hash().to_vec(),
        fee_payer: common::solana::get_fee_payer(&tx).unwrap_or_default(),
        signers: common::solana::get_signers(&tx).unwrap_or_default(),
        fee: tx_meta.fee,
        compute_units_consumed: tx_meta.compute_units_consumed(),
        instructions,
        ..Default::default()
    })
}

fn process_instruction(instruction: &InstructionView) -> Option<pb::Instruction> {
    let program_id = instruction.program_id().0;
    if program_id != &raydium::amm::v4::PROGRAM_ID {
        return None;
    }

    match raydium::amm::v4::instructions::unpack(instruction.data()) {
        Ok(raydium::amm::v4::instructions::RaydiumV4Instruction::SwapBaseIn(event)) => {
            Some(pb::Instruction {
                program_id: program_id.to_vec(),
                stack_height: instruction.stack_height(),
                instruction: Some(pb::instruction::Instruction::SwapBaseIn(..)),
            })
        }
        _ => None,
    }
}
```

### Accessing Instruction Accounts

```rust
fn get_swap_accounts(instruction: &InstructionView) -> pb::SwapAccounts {
    let accounts = instruction.accounts();
    pb::SwapAccounts {
        amm: accounts.get(0).map(|a| a.0.to_vec()).unwrap_or_default(),
        pool_coin_token: accounts.get(4).map(|a| a.0.to_vec()).unwrap_or_default(),
        pool_pc_token: accounts.get(5).map(|a| a.0.to_vec()).unwrap_or_default(),
        // ... more accounts based on IDL layout
    }
}
```

## Store Modules

Store modules maintain state across blocks, identical to EVM patterns:

```rust
#[substreams::handlers::store]
pub fn store_token_prices(events: pb::Events, store: StoreSetProto<pb::TokenPrice>) {
    for tx in events.transactions {
        for instruction in tx.instructions {
            if let Some(price) = extract_price(&instruction) {
                store.set(0, &format!("token:{}", base58::encode(&price.mint)), &price);
            }
        }
    }
}
```

## DatabaseChanges Module (db_out)

The aggregate `db_out` module combines multiple DEX outputs into `DatabaseChanges`:

```rust
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::sf::substreams::sink::database::v1::DatabaseChanges;
use substreams_database_change::tables::Tables;

#[substreams::handlers::map]
pub fn db_out(
    clock: Clock,
    raydium_events: raydium::Events,
    jupiter_events: jupiter::Events,
    pumpfun_events: pumpfun::Events,
    // ... more DEX inputs
) -> Result<DatabaseChanges, Error> {
    let mut tables = Tables::new();

    // Process each DEX's events into table rows
    for tx in raydium_events.transactions {
        for (i, instruction) in tx.instructions.iter().enumerate() {
            let keys = common::db::common_key_v2(&clock, tx_index, i);
            tables.create_row("raydium_amm_v4_swap", keys)
                .set("signature", base58::encode(&tx.signature))
                .set("amount_in", instruction.amount_in)
                .set("amount_out", instruction.amount_out);
        }
    }

    Ok(tables.to_database_changes())
}
```

### Common Key Patterns

```rust
// Primary key: (block_hash, transaction_index, instruction_index)
pub fn common_key_v2(clock: &Clock, transaction_index: usize, instruction_index: usize) -> [(&'static str, String); 3] {
    [
        ("block_hash", clock.id.to_string()),
        ("transaction_index", transaction_index.to_string()),
        ("instruction_index", instruction_index.to_string()),
    ]
}

// Global sequence for ClickHouse versioning
pub fn to_global_sequence(clock: &Clock, index: u64) -> u64 {
    (clock.number << 32) + index
}
```
