# Cursor Management & Reorg Handling

The two most critical aspects of building production-grade Substreams sinks.

## Cursor Overview

### What is a Cursor?

A cursor is an **opaque string** that represents the exact position in a Substreams stream. It encodes:

- Block number and block hash
- Module execution state
- Position within the block's output

**You should never parse or modify a cursor** - treat it as an opaque value.

### Why Cursors Matter

Without proper cursor handling:
- Restarting your sink processes the same data again (duplicates)
- Or worse, skips data if you save cursors incorrectly (data loss)
- You cannot resume from where you left off

## The Golden Rules

```
RULE #1: Persist cursor AFTER successful data processing, never before
RULE #2: On restart, load the persisted cursor and pass it to the stream
RULE #3: Empty/blank cursor means "start from the beginning"
RULE #4: One cursor per stream - NEVER mix cursors between streams
```

### Rule #1: Persist After Processing

**CORRECT:**
```
1. Receive block data
2. Process/save the data
3. Persist the cursor
```

**WRONG:**
```
1. Receive block data
2. Persist the cursor     <- WRONG: What if step 3 fails?
3. Process/save the data
```

If you persist the cursor before processing and then crash, you'll skip that block on restart.

### Rule #2: Resume From Cursor

When creating a new stream, always pass the last persisted cursor:

```go
// Go
cursor, err := loadCursor()  // Returns blank cursor if first run
if err != nil {
    return fmt.Errorf("load cursor: %w", err)
}
sinker.Run(ctx, cursor, handlers)
```

```javascript
// JavaScript
const cursor = await getCursor();  // Returns undefined if first run
const request = createRequest({
    startCursor: cursor ?? undefined,
    // ...
});
```

```python
# Python
cursor = load_cursor()  # Returns None if first run
request = service_pb2.Request(
    start_cursor=cursor if cursor else "",
    # ...
)
```

```rust
// Rust
let cursor = load_cursor()?;  // Returns None if first run
let stream = SubstreamsStream::new(
    endpoint,
    cursor,  // Pass Option<String>
    // ...
);
```

### Rule #3: Blank Cursor = Start From Beginning

When no cursor exists (first run), the stream starts from:
- The `initialBlock` specified in the module manifest, OR
- The `start_block` you provide in the request

### Rule #4: One Cursor Per Stream

**Critical:** Each Substreams stream MUST have its own dedicated cursor. Never share or mix cursors between different streams.

**Why this matters:**

A cursor is tied to a specific stream context:
- The network/chain being consumed
- The specific Substreams package and module
- The endpoint being used

**Multi-network sinks:**

When building a sink that consumes from multiple networks (e.g., Ethereum + Polygon + Arbitrum), you need:
- One stream connection per network
- One cursor per network
- Separate storage for each cursor

```go
// CORRECT: Separate cursors per network
cursors := map[string]*sink.Cursor{
    "ethereum": loadCursor("ethereum"),
    "polygon":  loadCursor("polygon"),
    "arbitrum": loadCursor("arbitrum"),
}

// Each network gets its own stream with its own cursor
for network, cursor := range cursors {
    go runStream(network, cursor)
}
```

```sql
-- Database schema for multi-network cursor storage
CREATE TABLE cursors (
    sink_id VARCHAR(255),
    network VARCHAR(255),
    cursor TEXT NOT NULL,
    block_num BIGINT,
    updated_at TIMESTAMP DEFAULT NOW(),
    PRIMARY KEY (sink_id, network)  -- Composite key!
);
```

**What happens if you mix cursors:**

- Block numbers never align across networks (Ethereum block 17M ≠ Polygon block 17M)
- Using an Ethereum cursor on a Polygon stream causes undefined behavior
- Data corruption, skipped blocks, or processing wrong data
- Subtle bugs that are extremely hard to diagnose

**Common mistake:**

```go
// WRONG: Single cursor for multiple networks
cursor := loadCursor()  // Which network is this for?

// This will cause problems!
streamEthereum(cursor)
streamPolygon(cursor)   // Using Ethereum's cursor on Polygon!
```

## Storage Patterns

> **Note:** The database examples in this section are for illustration purposes. For production sinks targeting PostgreSQL or ClickHouse, we highly recommend using [substreams-sink-sql](https://github.com/streamingfast/substreams-sink-sql) which handles cursor management, reorg handling, batching, and many edge cases out of the box.

### File-Based (Development)

Simple but limited to single-instance deployments:

```go
// Go
const cursorFile = "cursor.txt"

func loadCursor() (*sink.Cursor, error) {
    data, err := os.ReadFile(cursorFile)
    if err != nil {
        if os.IsNotExist(err) {
            return sink.NewBlankCursor(), nil
        }
        return nil, fmt.Errorf("read cursor: %w", err)
    }
    cursor, err := sink.NewCursor(string(data))
    if err != nil {
        return nil, fmt.Errorf("parse cursor: %w", err)
    }
    return cursor, nil
}

func persistCursor(cursor *sink.Cursor) error {
    return os.WriteFile(cursorFile, []byte(cursor.String()), 0644)
}
```

### Database (Production)

Recommended for production deployments:

```sql
CREATE TABLE cursors (
    sink_id VARCHAR(255) PRIMARY KEY,
    cursor TEXT NOT NULL,
    block_num BIGINT,
    updated_at TIMESTAMP DEFAULT NOW()
);
```

**Atomic cursor + data updates** (best practice):

```go
// Go with database transaction
func handleBlock(ctx context.Context, data *BlockScopedData, cursor *sink.Cursor) error {
    tx, err := db.BeginTx(ctx, nil)
    if err != nil {
        return err
    }
    defer tx.Rollback()

    // 1. Insert your data
    for _, event := range data.Events {
        _, err := tx.ExecContext(ctx,
            "INSERT INTO events (block_num, ...) VALUES ($1, ...)",
            data.Clock.Number, ...
        )
        if err != nil {
            return err
        }
    }

    // 2. Update cursor in same transaction
    _, err = tx.ExecContext(ctx, `
        INSERT INTO cursors (sink_id, cursor, block_num, updated_at)
        VALUES ($1, $2, $3, NOW())
        ON CONFLICT (sink_id)
        DO UPDATE SET cursor = $2, block_num = $3, updated_at = NOW()
    `, "my-sink", cursor.String(), data.Clock.Number)
    if err != nil {
        return err
    }

    // 3. Commit both together
    return tx.Commit()
}
```

### Redis/Key-Value (High Availability)

For high-frequency cursor updates:

```python
import redis

r = redis.Redis()

def load_cursor(sink_id: str) -> Optional[str]:
    cursor = r.get(f"cursor:{sink_id}")
    return cursor.decode('utf-8') if cursor else None

def persist_cursor(sink_id: str, cursor: str) -> None:
    r.set(f"cursor:{sink_id}", cursor)
```

## Chain Reorganization (Reorg) Handling

### What is a Reorg?

A chain reorganization happens when the blockchain's canonical chain changes. Blocks that were previously considered valid are now invalid, and different blocks take their place.

```
Before reorg:
Block 5 -> Block 6a -> Block 7a -> Block 8a (you processed these)

After reorg:
Block 5 -> Block 6b -> Block 7b (chain switched to this fork)

Your sink has data from blocks 6a, 7a, 8a that no longer exist!
```

### BlockUndoSignal

When a reorg occurs, Substreams sends a `BlockUndoSignal`:

```protobuf
message BlockUndoSignal {
    BlockRef last_valid_block = 1;  // The last block that's still valid
    string last_valid_cursor = 2;   // Cursor at that block
}

message BlockRef {
    uint64 number = 1;  // Block number
    string id = 2;      // Block hash
}
```

### Handling Undo Signals

You **must** handle undo signals to maintain data consistency:

```go
// Go
func handleBlockUndoSignal(ctx context.Context, signal *pbsubstreamsrpc.BlockUndoSignal, cursor *sink.Cursor) error {
    lastValidBlock := signal.LastValidBlock.Number

    // 1. Delete/revert data for invalid blocks
    _, err := db.ExecContext(ctx,
        "DELETE FROM events WHERE block_num > $1",
        lastValidBlock,
    )
    if err != nil {
        return err
    }

    // 2. Persist the valid cursor
    return persistCursor(cursor)
}
```

```javascript
// JavaScript
const handleBlockUndoSignal = async (signal) => {
    const lastValidBlock = signal.lastValidBlock.num;

    // 1. Revert data
    await db.query('DELETE FROM events WHERE block_num > $1', [lastValidBlock]);

    // 2. Persist valid cursor
    await writeCursor(signal.lastValidCursor);
};
```

```python
# Python
def handle_block_undo_signal(signal) -> None:
    last_valid = signal.last_valid_block.number

    # 1. Revert data
    cursor.execute("DELETE FROM events WHERE block_num > %s", (last_valid,))
    conn.commit()

    # 2. Persist valid cursor
    persist_cursor(signal.last_valid_cursor)
```

```rust
// Rust
fn process_undo(signal: &BlockUndoSignal) -> Result<(), Error> {
    let last_valid = signal.last_valid_block.as_ref()
        .expect("missing last_valid_block")
        .number;

    // 1. Revert data
    db.execute("DELETE FROM events WHERE block_num > ?", [last_valid])?;

    // 2. Persist valid cursor
    persist_cursor(&signal.last_valid_cursor)?;

    Ok(())
}
```

### Reorg Handling Strategies

#### Strategy 1: Delete and Re-process (Simple)

Best for append-only data:

```sql
DELETE FROM events WHERE block_num > :last_valid_block;
```

#### Strategy 2: Soft Delete with Block Reference

Keep history but mark as reverted:

```sql
-- Add a reverted column
ALTER TABLE events ADD COLUMN reverted_at TIMESTAMP;

-- On undo:
UPDATE events SET reverted_at = NOW()
WHERE block_num > :last_valid_block AND reverted_at IS NULL;

-- Query only valid events:
SELECT * FROM events WHERE reverted_at IS NULL;
```

#### Strategy 3: Final Blocks Only (Avoid Reorgs)

The simplest solution - only process finalized blocks:

```go
// Go - Using the SDK flag
go run . sink manifest.spkg module --final-blocks-only
```

```rust
// Rust - In the request
Request {
    final_blocks_only: true,
    // ...
}
```

**Trade-off:** ~2-3 minute delay from chain tip, but no reorg handling needed.

## Error Recovery

### Cursor Recovery After Crash

If your sink crashes:

1. On restart, load the last persisted cursor
2. **Clean up partial data** (see below)
3. Pass cursor to the stream
4. Processing resumes exactly where it left off

#### Handling Partial Data (Non-Atomic Inserts)

**Critical:** If your sink cannot atomically insert all data for a block in a single transaction, you may have partial data from an incomplete block after a crash.

**Recommended approach:** Before resuming, delete all data with a block number higher than your last persisted cursor's block. This ensures any partial data from an interrupted block is cleaned up before reprocessing.

```go
func main() {
    cursor, err := loadCursor()
    if err != nil {
        log.Fatalf("load cursor: %v", err)
    }

    // Clean up any partial data from blocks after the cursor
    // This handles the case where we crashed mid-block
    if !cursor.IsBlank() {
        lastBlock := cursor.Block().Num()
        if err := cleanupPartialData(lastBlock); err != nil {
            log.Fatalf("cleanup partial data: %v", err)
        }
        log.Info("Cleaned up partial data", "above_block", lastBlock)
    }

    if cursor.IsBlank() {
        log.Info("Starting from beginning (no cursor found)")
    } else {
        log.Info("Resuming from cursor", "block", cursor.Block())
    }

    sinker.Run(ctx, cursor, handlers)
}

func cleanupPartialData(lastValidBlock uint64) error {
    // Delete any data from blocks after the last persisted cursor
    // This ensures we don't have partial data from an interrupted block
    _, err := db.Exec("DELETE FROM events WHERE block_num > $1", lastValidBlock)
    return err
}
```

**When is this needed?**
- Sinks that insert multiple rows per block without transactions
- Sinks writing to systems without transactional guarantees (some queues, file systems)
- Any case where a crash can leave partial block data

**When is this NOT needed?**
- Sinks using database transactions that commit cursor + data atomically
- Sinks processing one record per block
- Sinks with idempotent writes (INSERT ON CONFLICT DO NOTHING)

### Handling Cursor Persistence Failures

Cursor persistence failures are **fatal** - you must stop processing:

```javascript
const writeCursor = async (cursor) => {
    try {
        await fs.promises.writeFile(CURSOR_FILE, cursor);
    } catch (e) {
        // This is fatal - cannot continue safely
        throw new Error("CURSOR_PERSISTENCE_FAILED");
    }
};

// In error handler:
if (e.message === "CURSOR_PERSISTENCE_FAILED") {
    console.error("Fatal: Could not persist cursor");
    process.exit(1);
}
```

### Idempotent Processing

Design your data processing to be idempotent (safe to repeat):

```sql
-- Use INSERT ... ON CONFLICT for idempotency
INSERT INTO events (id, block_num, data)
VALUES ($1, $2, $3)
ON CONFLICT (id) DO NOTHING;
```

This way, if you replay the same block, no duplicate data is created.

## Common Patterns

### High Water Mark Tracking

Track the highest processed block for monitoring:

```go
type CursorState struct {
    Cursor    string
    BlockNum  uint64
    UpdatedAt time.Time
}

func persistState(state CursorState) error {
    _, err := db.Exec(`
        INSERT INTO sink_state (sink_id, cursor, block_num, updated_at)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (sink_id)
        DO UPDATE SET cursor = $2, block_num = $3, updated_at = $4
    `, "my-sink", state.Cursor, state.BlockNum, state.UpdatedAt)
    return err
}
```

### Batched Cursor Updates

For high-throughput sinks, batch cursor updates:

```go
var pendingBlocks []BlockData
const batchSize = 100

func handleBlock(data *BlockScopedData, cursor *sink.Cursor) error {
    pendingBlocks = append(pendingBlocks, data)

    if len(pendingBlocks) >= batchSize {
        // Process batch
        if err := processBatch(pendingBlocks); err != nil {
            return err
        }

        // Persist cursor after batch
        if err := persistCursor(cursor); err != nil {
            return err
        }

        pendingBlocks = nil
    }
    return nil
}
```

**Warning:** Batching means reprocessing more blocks after a crash. Balance throughput vs recovery time.

### Multi-Module Cursor Management

When consuming multiple modules, track cursors per module:

```go
type ModuleCursor struct {
    SinkID     string
    ModuleName string
    Cursor     string
    BlockNum   uint64
}

func persistModuleCursor(mc ModuleCursor) error {
    _, err := db.Exec(`
        INSERT INTO module_cursors (sink_id, module_name, cursor, block_num)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (sink_id, module_name)
        DO UPDATE SET cursor = $3, block_num = $4
    `, mc.SinkID, mc.ModuleName, mc.Cursor, mc.BlockNum)
    return err
}
```

## Troubleshooting

### Duplicate Data After Restart

**Cause:** Cursor persisted before data processing completed.

**Fix:** Always persist cursor AFTER successful processing:

```go
// WRONG
persistCursor(cursor)
processData(data)  // If this fails, cursor already advanced

// CORRECT
processData(data)
persistCursor(cursor)  // Only persist after success
```

### Missing Data After Restart

**Cause:** Data processed but cursor not persisted before crash.

**Fix:** Use atomic transactions:

```go
tx.Begin()
processData(data)      // In transaction
persistCursor(cursor)  // In same transaction
tx.Commit()            // Both succeed or both fail
```

### Reorg Data Inconsistency

**Cause:** Undo signal not handled correctly.

**Fix:** Ensure all data from invalidated blocks is removed:

```sql
-- Make sure to catch all related tables
DELETE FROM events WHERE block_num > :last_valid;
DELETE FROM transfers WHERE block_num > :last_valid;
DELETE FROM balances_updates WHERE block_num > :last_valid;
```

### Cursor Not Advancing

**Cause:** Error in processing loop prevents cursor update.

**Fix:** Ensure errors are propagated and cursor only updates on success:

```go
for block := range stream {
    if err := processBlock(block); err != nil {
        return err  // Stop processing, don't update cursor
    }
    persistCursor(block.Cursor)  // Only reached on success
}
```

## Summary

| Aspect | Best Practice |
|--------|--------------|
| Persist timing | AFTER successful processing |
| Storage | Database for production, file for development |
| Atomicity | Use transactions for cursor + data |
| Reorg handling | Delete invalid blocks OR use final_blocks_only |
| Recovery | Load cursor on startup, resume from there |
| Failures | Cursor persist failures are fatal |
