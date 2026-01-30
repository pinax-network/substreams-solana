---
name: substreams-sink
description: Expert knowledge for consuming Substreams data in applications. Use when building sinks, real-time data pipelines, or integrating Substreams outputs into Go, JavaScript, Python, or Rust applications.
license: Apache-2.0
compatibility:
  platforms: [claude-code, cursor, vscode, windsurf]
metadata:
  version: 1.0.0
  author: StreamingFast
  documentation: https://substreams.streamingfast.io
---

# Substreams Sink Development Expert

Expert assistant for consuming Substreams data - building production-grade sinks and data pipelines.

## Core Concepts

### What is a Substreams Sink?

A Substreams sink is an application that:
- **Connects** to a Substreams endpoint via gRPC
- **Streams** processed blockchain data from a Substreams package (.spkg)
- **Handles** cursor persistence for resumability
- **Manages** chain reorganizations (reorgs) gracefully
- **Processes** the data into your destination (database, queue, etc.)

> **Note:** Before building a custom sink, consider using existing solutions:
> - **[substreams-sink-sql](https://github.com/streamingfast/substreams-sink-sql)** - For PostgreSQL and ClickHouse. Handles cursor management, reorgs, batching, and schema management out of the box.
> - **[substreams-sink-kv](https://github.com/streamingfast/substreams-sink-kv)** - For key-value stores.
> - **[substreams-sink-files](https://github.com/streamingfast/substreams-sink-files)** - For file-based outputs (JSON, CSV, Parquet).
>
> The examples in this guide use database code for illustration purposes. For production SQL database sinks, `substreams-sink-sql` is highly recommended as it solves cursor persistence, reorg handling, batching, and many edge cases already.

### Key Components

1. **Endpoint**: gRPC server providing Substreams data (e.g., `mainnet.eth.streamingfast.io:443`)
2. **Package (.spkg)**: Compiled Substreams with modules and protobuf schemas
3. **Module**: The specific output module to stream from
4. **Cursor**: Opaque string for resuming streams at exact position
5. **Block Range**: Start and stop blocks for the stream

### Authentication

All Substreams endpoints require authentication:

```bash
# Set API key (recommended for CLI tools)
export SUBSTREAMS_API_KEY="your-api-key"

# Or set bearer token directly
export SUBSTREAMS_API_TOKEN="your-jwt-token"
```

Get your API key from [thegraph.market](https://thegraph.market) or [pinax.network](https://pinax.network).

## Language Recommendations

| Language | Recommendation | Best For |
|----------|---------------|----------|
| **Go** | Official SDK (Recommended) | Production sinks, StreamingFast sinks |
| **JavaScript** | Official SDK | Web apps, Node.js services |
| **Python** | Reference implementation | Prototyping, data analysis |
| **Rust** | Reference implementation | High-performance custom sinks |

## Quick Start by Language

### Go (Recommended)

```go
package main

import (
    "context"
    "log"

    "github.com/streamingfast/substreams/sink"
)

func main() {
    sinker, err := sink.New(
        sink.NewFromManifest("substreams.spkg", "map_events"),
        sink.WithBlockRange(":+1000"),
    )
    if err != nil {
        log.Fatalf("create sinker: %v", err)
    }

    sinker.Run(ctx, sink.NewSinker(
        handleBlockScopedData,
        handleBlockUndoSignal,
    ))
}

func handleBlockScopedData(ctx context.Context, data *pbsubstreamsrpc.BlockScopedData, isLive *bool, cursor *sink.Cursor) error {
    // Process block data
    // Persist cursor after successful processing
    return nil
}

func handleBlockUndoSignal(ctx context.Context, undoSignal *pbsubstreamsrpc.BlockUndoSignal, cursor *sink.Cursor) error {
    // Handle reorg: rewind data to undoSignal.LastValidBlock
    // Persist undoSignal.LastValidCursor
    return nil
}
```

See [references/go-sink.md](./references/go-sink.md) for complete guide.

### JavaScript (Node.js)

```javascript
import { createRegistry, createRequest } from "@substreams/core";
import { createGrpcTransport } from "@connectrpc/connect-node";

const transport = createGrpcTransport({
    baseUrl: "https://mainnet.eth.streamingfast.io:443",
    httpVersion: "2",
});

const request = createRequest({
    substreamPackage: pkg,
    outputModule: "map_events",
    startBlockNum: 17000000n,
    stopBlockNum: "+1000",
});

for await (const response of stream(request, registry, transport)) {
    if (response.message.case === "blockScopedData") {
        // Process block data
        await persistCursor(response.message.value.cursor);
    } else if (response.message.case === "blockUndoSignal") {
        // Handle reorg
        await handleUndo(response.message.value);
    }
}
```

See [references/javascript-sink.md](./references/javascript-sink.md) for complete guide.

### Python

```python
import grpc
from sf.substreams.rpc.v2 import service_pb2, service_pb2_grpc

creds = grpc.ssl_channel_credentials()
with grpc.secure_channel("mainnet.eth.streamingfast.io:443", creds) as channel:
    stub = service_pb2_grpc.StreamStub(channel)
    metadata = [("authorization", f"Bearer {token}")]

    request = service_pb2.Request(
        start_block_num=17000000,
        stop_block_num=17001000,
        modules=package.modules,
        output_module="map_events",
        production_mode=True,
    )

    for response in stub.Blocks(request, metadata=metadata):
        if response.WhichOneof("message") == "block_scoped_data":
            # Process block data
            pass
        elif response.WhichOneof("message") == "block_undo_signal":
            # Handle reorg
            pass
```

See [references/python-sink.md](./references/python-sink.md) for complete guide.

### Rust

```rust
use substreams_stream::{BlockResponse, SubstreamsStream};

let stream = SubstreamsStream::new(
    endpoint,
    cursor,
    package,
    modules.clone(),
    "map_events".to_string(),
    start_block,
    stop_block,
);

while let Some(response) = stream.next().await {
    match response? {
        BlockResponse::New(data) => {
            // Process block data
            persist_cursor(&data.cursor);
        }
        BlockResponse::Undo(signal) => {
            // Handle reorg
            rewind_to_block(signal.last_valid_block);
            persist_cursor(&signal.last_valid_cursor);
        }
    }
}
```

See [references/rust-sink.md](./references/rust-sink.md) for complete guide.

## Critical Concepts

### Cursor Management

**The cursor is the most critical piece of sink development.**

```
RULE #1: Persist cursor AFTER successful processing, never before.
RULE #2: On restart, load persisted cursor and resume from there.
RULE #3: Blank/empty cursor means start from the beginning.
```

The cursor is an opaque string that encodes:
- Block number and hash
- Module execution state
- Position within the block

**Cursor persistence patterns:**

| Storage | Use Case | Example |
|---------|----------|---------|
| File | Development, single instance | `cursor.txt` |
| Database | Production, multi-instance | Cursors table with module key |
| Redis | High availability | Key-value with TTL |

See [references/cursor-reorg.md](./references/cursor-reorg.md) for detailed patterns.

### Chain Reorganization (Reorg) Handling

When a chain reorganizes, you receive a `BlockUndoSignal`:

```
BlockUndoSignal {
    last_valid_block: BlockRef { num: 17000100, id: "0xabc..." }
    last_valid_cursor: "opaque-cursor-string"
}
```

**Required actions:**
1. Delete/revert all data for blocks > `last_valid_block.num`
2. Persist the `last_valid_cursor`
3. Continue streaming (new blocks will follow automatically)

**Final blocks only mode** (recommended for most sinks):
- Set `final_blocks_only: true` in request
- Only receive blocks that cannot be reorganized
- Eliminates need for undo handling
- Trade-off: ~2-3 minutes delay from chain tip

### Error Handling & Retry

**Fatal errors (do not retry):**
- `Unauthenticated` - Invalid or expired token
- `InvalidArgument` - Bad request parameters
- `Internal` - Server-side bug

**Retryable errors (implement exponential backoff):**
- `Unavailable` - Server temporarily unavailable
- `ResourceExhausted` - Rate limited
- Connection timeouts

**Exponential backoff pattern:**
```
Base delay: 500ms
Max delay: 45-90 seconds
Jitter: Add random 0-100ms
```

### Production Mode vs Development Mode

| Feature | Production Mode | Development Mode |
|---------|-----------------|------------------|
| Parallel execution | Yes | No |
| Output | Single module only | All modules |
| Performance | Optimized | Debug-friendly |
| Use case | Sinks | Testing, debugging |

**Always use production mode for sinks:**
```go
sink.WithProductionMode()  // Go
production_mode=True       // Python
productionMode: true       // JavaScript
```

## Common Endpoints

| Network | Endpoint |
|---------|----------|
| Ethereum Mainnet | `mainnet.eth.streamingfast.io:443` |
| Ethereum Sepolia | `sepolia.eth.streamingfast.io:443` |
| Polygon | `polygon.streamingfast.io:443` |
| Arbitrum One | `arb-one.streamingfast.io:443` |
| Optimism | `optimism.streamingfast.io:443` |
| Base | `base.streamingfast.io:443` |
| BSC | `bsc.streamingfast.io:443` |
| Solana | `mainnet.sol.streamingfast.io:443` |
| Near | `mainnet.near.streamingfast.io:443` |

Full list: [thegraph.market/supported-networks](https://thegraph.market/supported-networks)

## Block Range Syntax

```bash
# Explicit range
--start-block 17000000 --stop-block 17001000

# From manifest initialBlock
--start-block : --stop-block 17001000

# Relative stop (process 1000 blocks)
--start-block 17000000 --stop-block +1000

# To chain head (live streaming)
--start-block 17000000 --stop-block 0
```

## Module Parameters

Pass runtime parameters to modules:

```bash
# Single parameter
-p "map_events=0xa0b86a33e6..."

# Multiple parameters
-p "map_events=0xa0b86a33..." -p "filter_module=type:transfer"

# JSON parameters
-p 'map_events={"contracts":["0x123","0x456"],"min_value":1000}'
```

## Protobuf Code Generation

Generate language bindings from .spkg files:

```bash
# Install buf
go install github.com/bufbuild/buf/cmd/buf@latest

# Generate from local .spkg
buf generate --exclude-path="google" ./my-substreams.spkg#format=bin

# Generate from URL
buf generate "https://spkg.io/streamingfast/substreams-eth-block-meta-v0.4.3.spkg#format=binpb"

# Generate from buf registry
buf generate buf.build/streamingfast/substreams --include-imports
```

## Troubleshooting

### Connection Issues

**"Unauthenticated" error:**
- Verify API key/token is set correctly
- Check token hasn't expired
- Ensure correct environment variable name

**"Connection refused" error:**
- Verify endpoint URL and port
- Check TLS is enabled for https://
- Test network connectivity

### Empty Output

**No data received:**
- Verify `initialBlock` in manifest is before your start block
- Check the output module name is correct
- Ensure the block range contains relevant data
- Try a known-good block range first

### Performance Issues

**Slow processing:**
- Enable production mode
- Use gzip compression
- Increase connection timeout
- Consider final_blocks_only for non-realtime needs

## Resources

* [Official Documentation](https://substreams.streamingfast.io)
* [Go Sink Reference](./references/go-sink.md)
* [JavaScript Sink Reference](./references/javascript-sink.md)
* [Python Sink Reference](./references/python-sink.md)
* [Rust Sink Reference](./references/rust-sink.md)
* [Cursor & Reorg Handling](./references/cursor-reorg.md)

## Getting Help

* [Discord Community](https://discord.gg/streamingfast)
* [GitHub Issues](https://github.com/streamingfast/substreams/issues)
* [Example Repository](https://github.com/streamingfast/substreams-sink-examples)
