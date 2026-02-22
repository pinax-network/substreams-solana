# Common Substreams Patterns (SVM)

Collection of proven patterns and best practices for Solana/SVM Substreams development.

> **Note:** Code examples assume the following imports unless stated otherwise:
> ```rust
> use substreams::errors::Error;
> use substreams_solana::pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction};
> use substreams_solana::block_view::InstructionView;
> use substreams_solana::base58;
> ```

## Instruction Extraction Patterns

### IDL-Based Decoding (Recommended)

Use `substreams-solana-idls` to decode instructions by program ID:

```rust
use substreams_solana_idls::raydium;

fn process_instruction(instruction: &InstructionView) -> Option<pb::Instruction> {
    let program_id = instruction.program_id().0;
    if program_id != &raydium::amm::v4::PROGRAM_ID {
        return None;
    }

    match raydium::amm::v4::instructions::unpack(instruction.data()) {
        Ok(decoded) => match decoded {
            raydium::amm::v4::instructions::RaydiumV4Instruction::SwapBaseIn(event) => {
                Some(build_swap_instruction(instruction, event))
            }
            raydium::amm::v4::instructions::RaydiumV4Instruction::SwapBaseOut(event) => {
                Some(build_swap_out_instruction(instruction, event))
            }
            _ => None, // Skip non-swap instructions
        },
        Err(_) => None,
    }
}
```

### Multi-DEX Processing

```rust
#[substreams::handlers::map]
fn map_events(block: Block) -> Result<pb::Events, Error> {
    Ok(pb::Events {
        transactions: block.transactions_owned()
            .filter_map(|tx| {
                let instructions: Vec<pb::Instruction> = tx.walk_instructions()
                    .filter_map(|iview| {
                        let program_id = iview.program_id().0;
                        match program_id {
                            id if id == &raydium::amm::v4::PROGRAM_ID => process_raydium(&iview),
                            id if id == &jupiter::v6::PROGRAM_ID => process_jupiter(&iview),
                            id if id == &pumpfun::PROGRAM_ID => process_pumpfun(&iview),
                            _ => None,
                        }
                    })
                    .collect();

                if instructions.is_empty() { None }
                else { Some(build_transaction(&tx, instructions)) }
            })
            .collect(),
    })
}
```

### Log Processing

Process program logs emitted during transaction execution:

```rust
fn process_logs(tx_meta: &TransactionStatusMeta, program_id: &[u8]) -> Vec<pb::Log> {
    let mut logs = Vec::new();
    for log_message in &tx_meta.log_messages {
        if let Some(parsed) = parse_program_log(log_message, program_id) {
            logs.push(parsed);
        }
    }
    logs
}
```

## Transaction Helper Patterns

### Fee Payer & Signers

```rust
use common::solana::{get_fee_payer, get_signers};

fn build_transaction(tx: &ConfirmedTransaction, instructions: Vec<pb::Instruction>) -> pb::Transaction {
    let tx_meta = tx.meta.as_ref().unwrap();
    pb::Transaction {
        signature: tx.hash().to_vec(),
        fee_payer: get_fee_payer(tx).unwrap_or_default(),
        signers: get_signers(tx).unwrap_or_default(),
        fee: tx_meta.fee,
        compute_units_consumed: tx_meta.compute_units_consumed(),
        instructions,
        ..Default::default()
    }
}
```

### Failed Transaction Filtering

```rust
use common::solana::is_failed;

fn process_transaction(tx: ConfirmedTransaction) -> Option<pb::Transaction> {
    // Skip failed transactions
    if is_failed(&tx) {
        return None;
    }
    // ... process successful transaction
}
```

## DatabaseChanges Patterns

### Row Creation with Common Keys

```rust
use common::db::{common_key_v2, set_raydium_transaction_v2, set_raydium_instruction_v2};

fn insert_raydium_swap(tables: &mut Tables, clock: &Clock, tx: &raydium::Transaction, tx_index: usize, instruction: &raydium::Instruction, instr_index: usize) {
    let keys = common_key_v2(clock, tx_index, instr_index);
    let row = tables.create_row("raydium_amm_v4_swap", keys);

    // Common transaction fields
    set_raydium_transaction_v2(tx, row);
    set_raydium_instruction_v2(instruction, row);

    // Event-specific fields
    if let Some(ref swap) = instruction.instruction {
        match swap {
            raydium::instruction::Instruction::SwapBaseIn(s) => {
                row.set("amount_in", s.amount_in)
                   .set("minimum_amount_out", s.minimum_amount_out);
            }
            _ => {}
        }
    }
}
```

### Block Table Pattern

Every sink includes a `blocks` table for block-level metadata:

```rust
fn insert_block(tables: &mut Tables, clock: &Clock) {
    let seconds = clock.timestamp.as_ref().unwrap().seconds;
    tables.create_row("blocks", [("block_num", clock.number.to_string())])
        .set("block_hash", &clock.id)
        .set("timestamp", seconds.to_string());
}
```

## Address Encoding

Always use base58 for Solana addresses:

```rust
use substreams_solana::base58;

// Encode bytes to base58 string
let address = base58::encode(&account_bytes);

// For display in DatabaseChanges
row.set("token_mint", base58::encode(&mint_bytes))
   .set("owner", base58::encode(&owner_bytes));
```

## Performance Tips

- Use `blockFilter` with program IDs to skip irrelevant blocks
- Use `blocks_without_votes` to exclude vote transactions (majority of Solana blocks)
- Filter by program ID early in instruction processing
- Avoid cloning transaction data; use references
- Process instructions lazily with iterators and `filter_map`
