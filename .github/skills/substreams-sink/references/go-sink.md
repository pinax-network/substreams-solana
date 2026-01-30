# Go Substreams Sink Reference

Complete guide for building production-grade Substreams sinks in Go using the official SDK.

## Why Go?

Go is the **recommended language** for building Substreams sinks because:

- Official SDK maintained by StreamingFast
- Battle-tested in all StreamingFast production sinks
- Built-in retry logic, cursor management, and error handling
- Automatic endpoint discovery from manifest
- Comprehensive CLI flag support out of the box

## Installation

```bash
go get github.com/streamingfast/substreams/sink
```

## Key Dependencies

```go
import (
    "github.com/streamingfast/substreams/sink"
    pbsubstreamsrpc "github.com/streamingfast/substreams/pb/sf/substreams/rpc/v2"
)
```

## Basic Sink Structure

```go
package main

import (
    "context"
    "fmt"
    "os"

    "github.com/spf13/cobra"
    "github.com/spf13/pflag"
    "github.com/streamingfast/cli"
    . "github.com/streamingfast/cli"
    "github.com/streamingfast/logging"
    pbsubstreamsrpc "github.com/streamingfast/substreams/pb/sf/substreams/rpc/v2"
    "github.com/streamingfast/substreams/sink"

    // Import your output module's protobuf type
    pb "your-package/pb"
)

// Expected output type from the Substreams module
var expectedOutputModuleType = string(new(pb.YourOutputType).ProtoReflect().Descriptor().FullName())

var zlog, tracer = logging.RootLogger("sink", "github.com/your-org/your-sink")

func main() {
    logging.InstantiateLoggers()

    Run(
        "your-sink",
        "Description of your sink",

        Command(sinkRunE,
            "sink [<manifest> [<output_module>]]",
            "Run the sink",
            RangeArgs(0, 2),
            Flags(func(flags *pflag.FlagSet) {
                sink.AddFlagsToSet(flags)
            }),
        ),

        OnCommandErrorLogAndExit(zlog),
    )
}

func sinkRunE(cmd *cobra.Command, args []string) error {
    manifestPath := "substreams.yaml"
    outputModuleName := sink.InferOutputModuleFromPackage

    if len(args) > 0 {
        manifestPath = args[0]
    }
    if len(args) > 1 {
        outputModuleName = args[1]
    }

    sinker, err := sink.NewFromViper(
        cmd,
        expectedOutputModuleType,
        manifestPath,
        outputModuleName,
        "your-sink/1.0.0",
        zlog,
        tracer,
    )
    if err != nil {
        return fmt.Errorf("unable to create sinker: %w", err)
    }

    sinker.OnTerminating(func(err error) {
        cli.NoError(err, "unexpected sinker error")
        zlog.Info("sink is terminating")
    })

    // Load cursor from persistent storage
    cursor, err := loadCursor()
    if err != nil {
        return fmt.Errorf("load cursor: %w", err)
    }

    // Blocking call - runs until termination
    sinker.Run(
        context.Background(),
        cursor,
        sink.NewSinkerHandlers(handleBlockScopedData, handleBlockUndoSignal),
    )

    return nil
}
```

## Handler Functions

### Block Scoped Data Handler

Called for each new block of data:

```go
func handleBlockScopedData(
    ctx context.Context,
    data *pbsubstreamsrpc.BlockScopedData,
    isLive *bool,
    cursor *sink.Cursor,
) error {
    // 1. Unmarshal the output data
    output := &pb.YourOutputType{}
    if err := data.Output.MapOutput.UnmarshalTo(output); err != nil {
        return fmt.Errorf("unmarshal error: %w", err)
    }

    // 2. Access block metadata
    blockNum := data.Clock.Number
    blockID := data.Clock.Id
    blockTime := data.Clock.Timestamp.AsTime()

    // 3. Process the data
    if err := processData(output, blockNum); err != nil {
        return fmt.Errorf("process error: %w", err)
    }

    // 4. CRITICAL: Persist cursor AFTER successful processing
    if err := persistCursor(cursor); err != nil {
        return fmt.Errorf("cursor persist error: %w", err)
    }

    // 5. Optional: Check liveness
    if isLive != nil && *isLive {
        zlog.Info("processing live block", zap.Uint64("block", blockNum))
    }

    return nil
}
```

### Block Undo Signal Handler

Called when a chain reorganization occurs:

```go
func handleBlockUndoSignal(
    ctx context.Context,
    undoSignal *pbsubstreamsrpc.BlockUndoSignal,
    cursor *sink.Cursor,
) error {
    // 1. Get the last valid block info
    lastValidBlock := undoSignal.LastValidBlock.Number
    lastValidBlockID := undoSignal.LastValidBlock.Id

    zlog.Warn("chain reorg detected",
        zap.Uint64("rewind_to_block", lastValidBlock),
        zap.String("block_id", lastValidBlockID),
    )

    // 2. Delete/revert data for blocks > lastValidBlock
    if err := rewindData(lastValidBlock); err != nil {
        return fmt.Errorf("rewind error: %w", err)
    }

    // 3. CRITICAL: Persist the cursor from the undo signal
    if err := persistCursor(cursor); err != nil {
        return fmt.Errorf("cursor persist error: %w", err)
    }

    return nil
}
```

## Cursor Management

### File-Based Cursor (Simple)

```go
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

### Database Cursor (Production)

> **Note:** The database examples below are for illustration purposes. For sinking to PostgreSQL or ClickHouse, we highly recommend using [substreams-sink-sql](https://github.com/streamingfast/substreams-sink-sql) which handles cursor management, reorg handling, batching, and many edge cases out of the box.

```go
func loadCursor(db *sql.DB, sinkID string) (*sink.Cursor, error) {
    var cursorStr string
    err := db.QueryRow(
        "SELECT cursor FROM cursors WHERE sink_id = $1",
        sinkID,
    ).Scan(&cursorStr)

    if err == sql.ErrNoRows {
        return sink.NewBlankCursor(), nil
    }
    if err != nil {
        return nil, err
    }
    return sink.NewCursor(cursorStr)
}

func persistCursor(db *sql.DB, sinkID string, cursor *sink.Cursor) error {
    _, err := db.Exec(`
        INSERT INTO cursors (sink_id, cursor, updated_at)
        VALUES ($1, $2, NOW())
        ON CONFLICT (sink_id)
        DO UPDATE SET cursor = $2, updated_at = NOW()
    `, sinkID, cursor.String())
    return err
}
```

## CLI Flags Reference

The SDK provides these flags automatically via `sink.AddFlagsToSet()`:

| Flag | Description | Default |
|------|-------------|---------|
| `--api-key-envvar` | Env var name for API key | `SUBSTREAMS_API_KEY` |
| `--api-token-envvar` | Env var name for API token | `SUBSTREAMS_API_TOKEN` |
| `-e, --endpoint` | gRPC endpoint | Auto from manifest |
| `-s, --start-block` | Start block number | Module's initialBlock |
| `-t, --stop-block` | Stop block number | `0` (live) |
| `-p, --params` | Module parameters | None |
| `--development-mode` | Enable dev mode | `false` |
| `--final-blocks-only` | Only finalized blocks | `false` |
| `--max-retries` | Max retry attempts | `3` |
| `--plaintext` | Disable TLS | `false` |
| `--insecure` | Skip cert validation | `false` |

## Usage Examples

```bash
# Basic usage with manifest
go run . sink substreams.yaml map_events

# From package registry
go run . sink substreams_template@v0.1.0

# From URL
go run . sink https://spkg.io/streamingfast/substreams-eth-block-meta-v0.4.3.spkg db_out

# With explicit endpoint
go run . sink manifest.spkg map_events --endpoint mainnet.eth.streamingfast.io:443

# With block range
go run . sink manifest.spkg map_events -s 17000000 -t +1000

# With module parameters
go run . sink manifest.spkg map_events -p "map_events=0xa0b86a33..."

# Final blocks only (no reorgs)
go run . sink manifest.spkg map_events --final-blocks-only

# Development mode (debug output)
go run . sink manifest.spkg map_events --development-mode
```

## Complete Production Example

```go
package main

import (
    "context"
    "database/sql"
    "fmt"
    "os"

    "github.com/spf13/cobra"
    "github.com/spf13/pflag"
    "github.com/streamingfast/cli"
    . "github.com/streamingfast/cli"
    "github.com/streamingfast/logging"
    pbsubstreamsrpc "github.com/streamingfast/substreams/pb/sf/substreams/rpc/v2"
    "github.com/streamingfast/substreams/sink"
    "go.uber.org/zap"

    pb "your-package/pb"
)

var expectedOutputModuleType = string(new(pb.Events).ProtoReflect().Descriptor().FullName())
var zlog, tracer = logging.RootLogger("events-sink", "github.com/your-org/events-sink")

var db *sql.DB

func main() {
    logging.InstantiateLoggers()

    Run(
        "events-sink",
        "Sinks blockchain events to PostgreSQL",

        Command(sinkRunE,
            "sink [<manifest> [<output_module>]]",
            "Run the sink",
            RangeArgs(0, 2),
            Flags(func(flags *pflag.FlagSet) {
                sink.AddFlagsToSet(flags)
                flags.String("dsn", "", "PostgreSQL connection string")
            }),
        ),

        OnCommandErrorLogAndExit(zlog),
    )
}

func sinkRunE(cmd *cobra.Command, args []string) error {
    // Initialize database
    dsn, _ := cmd.Flags().GetString("dsn")
    if dsn == "" {
        dsn = os.Getenv("DATABASE_URL")
    }

    var err error
    db, err = sql.Open("postgres", dsn)
    if err != nil {
        return fmt.Errorf("database connection: %w", err)
    }
    defer db.Close()

    // Create sinker
    manifestPath := "substreams.yaml"
    outputModuleName := sink.InferOutputModuleFromPackage

    if len(args) > 0 {
        manifestPath = args[0]
    }
    if len(args) > 1 {
        outputModuleName = args[1]
    }

    sinker, err := sink.NewFromViper(
        cmd,
        expectedOutputModuleType,
        manifestPath,
        outputModuleName,
        "events-sink/1.0.0",
        zlog,
        tracer,
    )
    if err != nil {
        return fmt.Errorf("create sinker: %w", err)
    }

    sinker.OnTerminating(func(err error) {
        if err != nil {
            zlog.Error("sinker terminated with error", zap.Error(err))
        }
    })

    // Load cursor
    cursor, err := loadCursorFromDB(db, "events-sink")
    if err != nil {
        return fmt.Errorf("load cursor: %w", err)
    }

    // Run (blocking)
    sinker.Run(
        context.Background(),
        cursor,
        sink.NewSinkerHandlers(handleBlockScopedData, handleBlockUndoSignal),
    )

    return nil
}

func handleBlockScopedData(
    ctx context.Context,
    data *pbsubstreamsrpc.BlockScopedData,
    isLive *bool,
    cursor *sink.Cursor,
) error {
    events := &pb.Events{}
    if err := data.Output.MapOutput.UnmarshalTo(events); err != nil {
        return fmt.Errorf("unmarshal: %w", err)
    }

    // Use transaction for atomicity
    tx, err := db.BeginTx(ctx, nil)
    if err != nil {
        return fmt.Errorf("begin tx: %w", err)
    }
    defer tx.Rollback()

    // Insert events
    for _, event := range events.Items {
        _, err := tx.ExecContext(ctx, `
            INSERT INTO events (block_num, tx_hash, event_type, data)
            VALUES ($1, $2, $3, $4)
        `, data.Clock.Number, event.TxHash, event.Type, event.Data)
        if err != nil {
            return fmt.Errorf("insert event: %w", err)
        }
    }

    // Update cursor in same transaction
    _, err = tx.ExecContext(ctx, `
        INSERT INTO cursors (sink_id, cursor, block_num, updated_at)
        VALUES ($1, $2, $3, NOW())
        ON CONFLICT (sink_id)
        DO UPDATE SET cursor = $2, block_num = $3, updated_at = NOW()
    `, "events-sink", cursor.String(), data.Clock.Number)
    if err != nil {
        return fmt.Errorf("update cursor: %w", err)
    }

    if err := tx.Commit(); err != nil {
        return fmt.Errorf("commit: %w", err)
    }

    zlog.Debug("processed block",
        zap.Uint64("block", data.Clock.Number),
        zap.Int("events", len(events.Items)),
    )

    return nil
}

func handleBlockUndoSignal(
    ctx context.Context,
    undoSignal *pbsubstreamsrpc.BlockUndoSignal,
    cursor *sink.Cursor,
) error {
    lastValidBlock := undoSignal.LastValidBlock.Number

    zlog.Warn("reorg detected, rewinding",
        zap.Uint64("to_block", lastValidBlock),
    )

    tx, err := db.BeginTx(ctx, nil)
    if err != nil {
        return fmt.Errorf("begin tx: %w", err)
    }
    defer tx.Rollback()

    // Delete events after the reorg point
    _, err = tx.ExecContext(ctx,
        "DELETE FROM events WHERE block_num > $1",
        lastValidBlock,
    )
    if err != nil {
        return fmt.Errorf("delete events: %w", err)
    }

    // Update cursor
    _, err = tx.ExecContext(ctx, `
        UPDATE cursors SET cursor = $1, block_num = $2, updated_at = NOW()
        WHERE sink_id = $3
    `, cursor.String(), lastValidBlock, "events-sink")
    if err != nil {
        return fmt.Errorf("update cursor: %w", err)
    }

    return tx.Commit()
}

func loadCursorFromDB(db *sql.DB, sinkID string) (*sink.Cursor, error) {
    var cursorStr string
    err := db.QueryRow(
        "SELECT cursor FROM cursors WHERE sink_id = $1",
        sinkID,
    ).Scan(&cursorStr)

    if err == sql.ErrNoRows {
        return sink.NewBlankCursor(), nil
    }
    if err != nil {
        return nil, err
    }
    return sink.NewCursor(cursorStr)
}
```

## Best Practices

1. **Always persist cursor after data processing** - Never before, never skip
2. **Use transactions** - Group data writes and cursor updates atomically
3. **Handle undo signals** - Implement proper rewind logic or use `--final-blocks-only`
4. **Log progress** - Use structured logging for observability
5. **Set appropriate retries** - Use `--max-retries` based on your needs
6. **Validate output type** - Use `expectedOutputModuleType` to catch mismatches early
7. **Use production mode** - Don't use `--development-mode` in production

## Troubleshooting

**"output module type mismatch" error:**
- Ensure `expectedOutputModuleType` matches the Substreams module output
- Regenerate protobuf bindings if needed

**"sinker terminated with error" on startup:**
- Check API key/token is set correctly
- Verify endpoint is reachable
- Ensure manifest path is correct

**Data loss after restart:**
- Verify cursor is being persisted correctly
- Check cursor file/database has correct permissions
- Ensure cursor is persisted AFTER processing, not before
