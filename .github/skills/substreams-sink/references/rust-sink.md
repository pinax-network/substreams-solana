# Rust Substreams Sink Reference

Complete guide for consuming Substreams data in Rust applications.

## Important Note

Rust support is a **reference implementation**. Unlike Go and JavaScript which have official SDKs, Rust requires more manual implementation. The example provides a solid foundation with auto-reconnection but requires you to implement:

- Cursor persistence
- Chain reorganization (undo) handling

## Why Rust?

Rust is useful for:

- High-performance sinks
- Systems with strict memory requirements
- Integration with other Rust-based infrastructure
- When you need fine-grained control

## Dependencies

```toml
# Cargo.toml
[package]
name = "substreams-sink"
version = "0.1.0"
edition = "2021"

[dependencies]
# Async runtime
tokio = { version = "1.27", features = ["time", "sync", "macros", "rt-multi-thread"] }

# gRPC
tonic = { version = "0.12", features = ["gzip", "tls-roots"] }
prost = "0.13"
prost-types = "0.13"

# Retry & backoff
tokio-retry = "0.3"

# Async streams
futures03 = "0.3.1"
async-stream = "0.3"

# HTTP for package fetching
# Note: Default uses native-tls which requires OpenSSL.
# For environments without OpenSSL, use rustls instead:
# reqwest = { version = "0.11", default-features = false, features = ["rustls-tls"] }
reqwest = "0.11"

# Error handling
anyhow = "1"

# Utilities
chrono = "0.4"
lazy_static = "1.4"
regex = "1"
semver = "1"
```

## Project Structure

```
my-sink/
├── Cargo.toml
├── src/
│   ├── main.rs           # Entry point
│   ├── substreams.rs     # Endpoint configuration
│   ├── substreams_stream.rs  # Stream wrapper with reconnection
│   └── pb/
│       └── mod.rs        # Generated protobuf code
└── buf.gen.yaml
```

## Protobuf Generation

```yaml
# buf.gen.yaml
version: v1
plugins:
  - plugin: buf.build/community/neoeinstein-prost
    out: src/pb
    opt:
      - compile_well_known_types
  - plugin: buf.build/community/neoeinstein-tonic
    out: src/pb
```

```bash
# Generate from Substreams registry
buf generate buf.build/streamingfast/substreams

# Generate from your output module's .spkg
buf generate --exclude-path="google" ./your-substreams.spkg#format=bin
```

## Core Components

### SubstreamsEndpoint

Handles gRPC connection setup with TLS and authentication:

```rust
// src/substreams.rs
use anyhow::Error;
use tonic::transport::{Channel, ClientTlsConfig, Uri};
use tonic::{metadata::MetadataValue, Request};

use crate::pb::sf::substreams::rpc::v3::stream_client::StreamClient;

pub struct SubstreamsEndpoint {
    pub uri: String,
    pub token: Option<String>,
    channel: Channel,
}

impl SubstreamsEndpoint {
    pub async fn new(url: &str, token: Option<String>) -> Result<Self, Error> {
        let uri = url.parse::<Uri>()?;

        let channel = match uri.scheme_str() {
            Some("http") => Channel::builder(uri).connect().await?,
            _ => {
                // HTTPS by default
                let tls_config = ClientTlsConfig::new()
                    .with_native_roots();
                Channel::builder(uri)
                    .tls_config(tls_config)?
                    .connect()
                    .await?
            }
        };

        Ok(Self {
            uri: url.to_string(),
            token,
            channel,
        })
    }

    pub async fn substreams(
        self: Arc<Self>,
        request: crate::pb::sf::substreams::rpc::v3::Request,
    ) -> Result<tonic::Streaming<crate::pb::sf::substreams::rpc::v2::Response>, Error> {
        let mut client = StreamClient::new(self.channel.clone())
            .accept_compressed(tonic::codec::CompressionEncoding::Gzip)
            .send_compressed(tonic::codec::CompressionEncoding::Gzip);

        let mut req = Request::new(request);

        // Add authentication header
        if let Some(token) = &self.token {
            let bearer = format!("Bearer {}", token);
            req.metadata_mut().insert(
                "authorization",
                MetadataValue::try_from(&bearer)?,
            );
        }

        let response = client.blocks(req).await?;
        Ok(response.into_inner())
    }
}

impl std::fmt::Display for SubstreamsEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.uri)
    }
}
```

### SubstreamsStream

Wrapper that handles auto-reconnection with exponential backoff:

```rust
// src/substreams_stream.rs
use anyhow::{anyhow, Error};
use async_stream::try_stream;
use futures03::{Stream, StreamExt};
use std::{
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    time::Duration,
};
use tokio::time::sleep;
use tokio_retry::strategy::ExponentialBackoff;

use crate::pb::sf::substreams::rpc::v2::{
    response::Message, BlockScopedData, BlockUndoSignal, Response,
};
use crate::pb::sf::substreams::rpc::v3::Request;
use crate::pb::sf::substreams::v1::Package;
use crate::substreams::SubstreamsEndpoint;

pub enum BlockResponse {
    New(BlockScopedData),
    Undo(BlockUndoSignal),
}

pub struct SubstreamsStream {
    stream: Pin<Box<dyn Stream<Item = Result<BlockResponse, Error>> + Send>>,
}

impl SubstreamsStream {
    pub fn new(
        endpoint: Arc<SubstreamsEndpoint>,
        cursor: Option<String>,
        package: Option<Package>,
        output_module_name: String,
        start_block: i64,
        stop_block: u64,
    ) -> Self {
        SubstreamsStream {
            stream: Box::pin(stream_blocks(
                endpoint,
                cursor,
                package,
                output_module_name,
                start_block,
                stop_block,
            )),
        }
    }
}

fn stream_blocks(
    endpoint: Arc<SubstreamsEndpoint>,
    cursor: Option<String>,
    package: Option<Package>,
    output_module_name: String,
    start_block_num: i64,
    stop_block_num: u64,
) -> impl Stream<Item = Result<BlockResponse, Error>> {
    let mut latest_cursor = cursor.unwrap_or_default();
    let mut backoff = ExponentialBackoff::from_millis(500)
        .max_delay(Duration::from_secs(45));

    try_stream! {
        loop {
            println!("Connecting to {} (cursor: {})", &endpoint, &latest_cursor);

            let result = endpoint.clone().substreams(Request {
                start_block_num,
                start_cursor: latest_cursor.clone(),
                stop_block_num,
                final_blocks_only: false,
                package: package.clone(),
                output_module: output_module_name.clone(),
                production_mode: true,
                ..Default::default()
            }).await;

            match result {
                Ok(stream) => {
                    println!("Connected");
                    let mut encountered_error = false;

                    for await response in stream {
                        match process_response(response) {
                            ProcessResult::BlockScopedData(data) => {
                                // Reset backoff on success
                                backoff = ExponentialBackoff::from_millis(500)
                                    .max_delay(Duration::from_secs(45));

                                latest_cursor = data.cursor.clone();
                                yield BlockResponse::New(data);
                            }
                            ProcessResult::BlockUndoSignal(signal) => {
                                backoff = ExponentialBackoff::from_millis(500)
                                    .max_delay(Duration::from_secs(45));

                                latest_cursor = signal.last_valid_cursor.clone();
                                yield BlockResponse::Undo(signal);
                            }
                            ProcessResult::Skip => {}
                            ProcessResult::Error(status) => {
                                if status.code() == tonic::Code::Unauthenticated {
                                    Err(anyhow::Error::new(status))?;
                                }
                                encountered_error = true;
                                break;
                            }
                        }
                    }

                    if !encountered_error {
                        println!("Stream completed");
                        return;
                    }
                }
                Err(e) => {
                    println!("Connection failed: {:#}", e);
                }
            }

            // Wait before retry
            if let Some(duration) = backoff.next() {
                sleep(duration).await;
            } else {
                Err(anyhow!("Max retries exceeded"))?;
            }
        }
    }
}

enum ProcessResult {
    Skip,
    BlockScopedData(BlockScopedData),
    BlockUndoSignal(BlockUndoSignal),
    Error(tonic::Status),
}

fn process_response(result: Result<Response, tonic::Status>) -> ProcessResult {
    let response = match result {
        Ok(v) => v,
        Err(e) => return ProcessResult::Error(e),
    };

    match response.message {
        Some(Message::BlockScopedData(data)) => ProcessResult::BlockScopedData(data),
        Some(Message::BlockUndoSignal(signal)) => ProcessResult::BlockUndoSignal(signal),
        Some(Message::Progress(_)) => ProcessResult::Skip,
        Some(Message::Session(_)) => ProcessResult::Skip,
        _ => ProcessResult::Skip,
    }
}

impl Stream for SubstreamsStream {
    type Item = Result<BlockResponse, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.stream.poll_next_unpin(cx)
    }
}
```

### Main Entry Point

```rust
// src/main.rs
use anyhow::Error;
use futures03::StreamExt;
use std::{env, sync::Arc};
use prost::Message;

use crate::pb::sf::substreams::rpc::v2::{BlockScopedData, BlockUndoSignal};
use crate::pb::sf::substreams::v1::Package;
use substreams::SubstreamsEndpoint;
use substreams_stream::{BlockResponse, SubstreamsStream};

mod pb;
mod substreams;
mod substreams_stream;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let endpoint_url = env::args().nth(1).expect("missing endpoint");
    let spkg_path = env::args().nth(2).expect("missing spkg");
    let module_name = env::args().nth(3).expect("missing module");

    let token = env::var("SUBSTREAMS_API_TOKEN").ok();
    let package = load_package(&spkg_path).await?;
    let endpoint = Arc::new(SubstreamsEndpoint::new(&endpoint_url, token).await?);

    // Load cursor from storage
    let cursor = load_cursor()?;

    let mut stream = SubstreamsStream::new(
        endpoint,
        cursor,
        Some(package),
        module_name,
        0,  // start_block
        0,  // stop_block (0 = live)
    );

    while let Some(result) = stream.next().await {
        match result? {
            BlockResponse::New(data) => {
                process_block(&data)?;
                persist_cursor(&data.cursor)?;
            }
            BlockResponse::Undo(signal) => {
                process_undo(&signal)?;
                persist_cursor(&signal.last_valid_cursor)?;
            }
        }
    }

    Ok(())
}

fn process_block(data: &BlockScopedData) -> Result<(), Error> {
    let output = data.output.as_ref()
        .and_then(|o| o.map_output.as_ref())
        .expect("missing output");

    let clock = data.clock.as_ref().expect("missing clock");

    // Decode your protobuf type
    // let events = YourType::decode(output.value.as_slice())?;

    println!(
        "Block #{}: {} ({} bytes)",
        clock.number,
        output.type_url.replace("type.googleapis.com/", ""),
        output.value.len()
    );

    // Process your data here
    // ...

    Ok(())
}

fn process_undo(signal: &BlockUndoSignal) -> Result<(), Error> {
    let last_valid = signal.last_valid_block.as_ref()
        .expect("missing last_valid_block");

    println!("Reorg: rewind to block #{}", last_valid.number);

    // CRITICAL: Implement your rewind logic
    // DELETE FROM table WHERE block_num > last_valid.number

    Ok(())
}

// CRITICAL: Implement cursor persistence
fn load_cursor() -> Result<Option<String>, Error> {
    // Load from file, database, etc.
    Ok(None)
}

fn persist_cursor(cursor: &str) -> Result<(), Error> {
    // Save to file, database, etc.
    Ok(())
}

async fn load_package(path: &str) -> Result<Package, Error> {
    let bytes = if path.starts_with("http") {
        reqwest::get(path).await?.bytes().await?.to_vec()
    } else {
        std::fs::read(path)?
    };

    Ok(Package::decode(bytes.as_slice())?)
}
```

## Cursor Management

### File-Based Cursor

```rust
use std::fs;
use std::path::Path;

const CURSOR_FILE: &str = "cursor.txt";

fn load_cursor() -> Result<Option<String>, Error> {
    let path = Path::new(CURSOR_FILE);
    if path.exists() {
        let cursor = fs::read_to_string(path)?;
        let cursor = cursor.trim();
        if cursor.is_empty() {
            Ok(None)
        } else {
            Ok(Some(cursor.to_string()))
        }
    } else {
        Ok(None)
    }
}

fn persist_cursor(cursor: &str) -> Result<(), Error> {
    fs::write(CURSOR_FILE, cursor)?;
    Ok(())
}
```

### Database Cursor (Production)

```rust
use sqlx::PgPool;

async fn load_cursor_from_db(pool: &PgPool, sink_id: &str) -> Result<Option<String>, Error> {
    let row: Option<(String,)> = sqlx::query_as(
        "SELECT cursor FROM cursors WHERE sink_id = $1"
    )
    .bind(sink_id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|(cursor,)| cursor))
}

async fn persist_cursor_to_db(
    pool: &PgPool,
    sink_id: &str,
    cursor: &str,
    block_num: u64,
) -> Result<(), Error> {
    sqlx::query(r#"
        INSERT INTO cursors (sink_id, cursor, block_num, updated_at)
        VALUES ($1, $2, $3, NOW())
        ON CONFLICT (sink_id)
        DO UPDATE SET cursor = $2, block_num = $3, updated_at = NOW()
    "#)
    .bind(sink_id)
    .bind(cursor)
    .bind(block_num as i64)
    .execute(pool)
    .await?;

    Ok(())
}
```

## Decoding Output Data

```rust
use prost::Message;
use crate::pb::your_module::YourOutputType;

fn process_block(data: &BlockScopedData) -> Result<(), Error> {
    let output = data.output.as_ref()
        .and_then(|o| o.map_output.as_ref())
        .ok_or_else(|| anyhow!("missing output"))?;

    // Decode to your generated type
    let events = YourOutputType::decode(output.value.as_slice())?;

    for event in events.items {
        // Process each event
        println!("Event: {:?}", event);
    }

    Ok(())
}
```

## Usage Examples

```bash
# Basic usage
SUBSTREAMS_API_TOKEN="your-token" cargo run -- \
    mainnet.eth.streamingfast.io:443 \
    https://spkg.io/streamingfast/substreams-eth-block-meta-v0.4.3.spkg \
    db_out

# With block range
cargo run -- endpoint spkg module 17000000:17001000

# Live streaming (no stop block)
cargo run -- endpoint spkg module 17000000:

# From chain head
cargo run -- endpoint spkg module -1:

# With module parameters
cargo run -- endpoint spkg module --params="map_events:(type:transfer)"
```

## Final Blocks Only Mode

To avoid handling reorgs, set `final_blocks_only: true`:

```rust
let result = endpoint.clone().substreams(Request {
    // ... other fields ...
    final_blocks_only: true,  // Only receive finalized blocks
    // ...
}).await;
```

This delays data by ~2-3 minutes but eliminates the need for undo handling.

## Best Practices

1. **Always persist cursor after processing** - Never before, never skip
2. **Handle undo signals or use final_blocks_only** - Chain reorgs are inevitable
3. **Use production mode** - Set `production_mode: true`
4. **Enable compression** - Both send and receive gzip
5. **Reset backoff on success** - When data is received
6. **Log progress periodically** - For observability
7. **Use Arc for endpoint** - Enables efficient sharing across tasks

## Troubleshooting

**"Unauthenticated" error:**
- Check `SUBSTREAMS_API_TOKEN` environment variable
- Verify token hasn't expired
- Ensure Bearer prefix is added correctly

**Connection drops:**
- This is normal for long-running streams
- The SubstreamsStream wrapper handles reconnection automatically
- Check backoff configuration if reconnects are too aggressive

**"missing output" panic:**
- Verify module name is correct
- Check the package contains the expected module
- Try running with the Substreams CLI first

**Slow performance:**
- Ensure `production_mode: true` is set
- Enable gzip compression
- Check network latency to endpoint
