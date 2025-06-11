# SPL Token program

> Substreams for tracking SPL and SPL-2022 tokens.

## Includes

- `transfers`
  - **Transfer**
  - **TransferChecked**
- `mints` (Transfer)
  - **MintTo**
  - **MintToChecked**
- `burns` (Transfer)
  - **Burn**
  - **BurnChecked**
- `initialize_mints`
  - **InitializeMint**
  - **InitializeMint2**
- `initialize_accounts`
  - **InitializeAccount**
  - **InitializeAccount2**
  - **InitializeAccount3**
- `approves`
  - **Approve**
  - **ApproveChecked**
- `revokes`
  - **Revoke**

## Ordering & Indexing

All events are ordered by the following fields:

| Field Name | Description |
|------------|-------------|
| `execution_index` | Running counter of every SPL-Token instruction encountered across the whole block, incremented each time one is processed. |
| `instruction_index` | Position of the current instruction inside its transaction (root + inner), counted from the start of that transaction. |
| `inner_instruction_index` | position of the instruction among inner (non-root) instructions within the same transaction; stays 0 for root instructions and increments only for nested ones. |
