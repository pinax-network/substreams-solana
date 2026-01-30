# Python Substreams Sink Reference

Complete guide for consuming Substreams data in Python applications.

## Important Note

Python support is a **reference implementation**. Unlike Go and JavaScript which have official SDKs, Python requires manual implementation of:

- Cursor persistence
- Reconnection with exponential backoff
- Chain reorganization (undo) handling

This guide provides production-ready patterns for all these requirements.

## Why Python?

Python is useful for:

- Data analysis and prototyping
- Integration with data science tools (pandas, numpy)
- Quick scripts and exploration
- Jupyter notebook workflows

## Installation

```bash
# Using poetry (recommended)
poetry add grpcio protobuf requests

# Or using pip
pip install grpcio protobuf requests
```

## Dependencies

```toml
# pyproject.toml
[tool.poetry.dependencies]
python = "^3.11"
grpcio = "^1.67.0"
protobuf = "^5.28.2"
requests = "^2.26.0"
```

## Protobuf Generation

Generate Python bindings from Substreams packages:

```bash
# Install buf
# See: https://buf.build/docs/installation

# Generate core Substreams bindings
buf generate buf.build/streamingfast/substreams --include-imports --exclude-path=sf/substreams/intern

# Generate from .spkg file (local)
buf generate ./my-substreams.spkg#format=bin

# Generate from URL
buf generate "https://github.com/streamingfast/substreams-solana-spl-token/raw/refs/heads/master/tokens/solana-spl-token-v0.1.0.spkg#format=binpb"
```

**buf.gen.yaml example:**

```yaml
version: v1
plugins:
  - plugin: buf.build/protocolbuffers/python
    out: .
```

## Basic Sink Structure

```python
import grpc
import os
import sys
import time
import requests
from typing import Optional

import sf.substreams.v1.package_pb2 as package_pb2
import sf.substreams.rpc.v2.service_pb2 as service_pb2
import sf.substreams.rpc.v2.service_pb2_grpc as service_pb2_grpc

# Import your output type
import your_module_pb2


def get_auth_token() -> str:
    """Get authentication token from API key or use direct token."""
    # First check for direct token
    token = os.getenv("SUBSTREAMS_API_TOKEN") or os.getenv("SF_API_TOKEN")
    if token:
        return token

    # Otherwise, exchange API key for token
    api_key = os.getenv("SUBSTREAMS_API_KEY")
    if not api_key:
        print("Error: Neither SUBSTREAMS_API_TOKEN nor SUBSTREAMS_API_KEY is set")
        sys.exit(1)

    response = requests.post(
        "https://auth.streamingfast.io/v1/auth/issue",
        json={"api_key": api_key}
    )
    response.raise_for_status()
    return response.json()["token"]


def main():
    token = get_auth_token()

    endpoint = "mainnet.eth.streamingfast.io:443"
    spkg_url = "https://spkg.io/streamingfast/substreams-eth-block-meta-v0.4.3.spkg"
    module = "db_out"

    # Load package
    package = load_package(spkg_url)

    # Run with reconnection
    run_with_reconnection(
        endpoint=endpoint,
        token=token,
        package=package,
        module=module,
        start_block=17000000,
        stop_block=17001000,
    )


if __name__ == "__main__":
    main()
```

## Package Loading

```python
def load_package(source: str) -> package_pb2.Package:
    """Load a Substreams package from file or URL."""
    package = package_pb2.Package()

    if source.startswith("http"):
        response = requests.get(source)
        response.raise_for_status()
        package.ParseFromString(response.content)
    else:
        with open(source, "rb") as f:
            package.ParseFromString(f.read())

    return package
```

## Cursor Management

```python
import os
from pathlib import Path

CURSOR_FILE = Path("cursor.txt")


def load_cursor() -> Optional[str]:
    """Load cursor from persistent storage."""
    try:
        if CURSOR_FILE.exists():
            return CURSOR_FILE.read_text().strip()
    except Exception as e:
        print(f"Warning: Failed to load cursor: {e}")
    return None


def persist_cursor(cursor: str) -> None:
    """Persist cursor to storage. MUST be called AFTER successful processing."""
    try:
        CURSOR_FILE.write_text(cursor)
    except Exception as e:
        # Cursor persistence failure is fatal
        raise RuntimeError(f"Failed to persist cursor: {e}")
```

**Database cursor (production):**

```python
import psycopg2
from contextlib import contextmanager


@contextmanager
def get_db_connection():
    conn = psycopg2.connect(os.getenv("DATABASE_URL"))
    try:
        yield conn
    finally:
        conn.close()


def load_cursor_from_db(sink_id: str) -> Optional[str]:
    with get_db_connection() as conn:
        with conn.cursor() as cur:
            cur.execute(
                "SELECT cursor FROM cursors WHERE sink_id = %s",
                (sink_id,)
            )
            row = cur.fetchone()
            return row[0] if row else None


def persist_cursor_to_db(sink_id: str, cursor: str, block_num: int) -> None:
    with get_db_connection() as conn:
        with conn.cursor() as cur:
            cur.execute("""
                INSERT INTO cursors (sink_id, cursor, block_num, updated_at)
                VALUES (%s, %s, %s, NOW())
                ON CONFLICT (sink_id)
                DO UPDATE SET cursor = %s, block_num = %s, updated_at = NOW()
            """, (sink_id, cursor, block_num, cursor, block_num))
            conn.commit()
```

## Reconnection with Exponential Backoff

```python
import random
import time
from enum import Enum


class ErrorType(Enum):
    FATAL = "fatal"
    RETRYABLE = "retryable"


# Fatal gRPC status codes
FATAL_CODES = {
    grpc.StatusCode.UNAUTHENTICATED,
    grpc.StatusCode.INVALID_ARGUMENT,
    grpc.StatusCode.INTERNAL,
}


def classify_error(error: Exception) -> ErrorType:
    """Classify error as fatal or retryable."""
    if isinstance(error, grpc.RpcError):
        if error.code() in FATAL_CODES:
            return ErrorType.FATAL
    return ErrorType.RETRYABLE


def exponential_backoff(attempt: int, base_ms: int = 500, max_ms: int = 45000) -> float:
    """Calculate backoff delay with jitter."""
    delay_ms = min(base_ms * (2 ** attempt), max_ms)
    jitter_ms = random.uniform(0, 100)
    return (delay_ms + jitter_ms) / 1000


def run_with_reconnection(
    endpoint: str,
    token: str,
    package: package_pb2.Package,
    module: str,
    start_block: int,
    stop_block: int,
) -> None:
    """Run sink with automatic reconnection."""
    attempt = 0
    max_retries = -1  # -1 for infinite retries

    while True:
        try:
            cursor = load_cursor()
            stream_blocks(
                endpoint=endpoint,
                token=token,
                package=package,
                module=module,
                start_block=start_block,
                stop_block=stop_block,
                cursor=cursor,
            )
            print("Stream completed successfully")
            break

        except Exception as e:
            error_type = classify_error(e)

            if error_type == ErrorType.FATAL:
                print(f"Fatal error: {e}")
                raise

            if max_retries >= 0 and attempt >= max_retries:
                print(f"Max retries ({max_retries}) exceeded")
                raise

            delay = exponential_backoff(attempt)
            print(f"Retryable error ({e}), reconnecting in {delay:.1f}s...")
            time.sleep(delay)
            attempt += 1
```

## Stream Processing

```python
def stream_blocks(
    endpoint: str,
    token: str,
    package: package_pb2.Package,
    module: str,
    start_block: int,
    stop_block: int,
    cursor: Optional[str] = None,
) -> None:
    """Stream blocks from Substreams endpoint."""

    request = service_pb2.Request(
        start_block_num=start_block,
        stop_block_num=stop_block,
        modules=package.modules,
        output_module=module,
        production_mode=True,
    )

    # Set cursor if resuming
    if cursor:
        request.start_cursor = cursor
        print(f"Resuming from cursor...")

    # Create secure channel
    creds = grpc.ssl_channel_credentials()

    with grpc.secure_channel(endpoint, creds) as channel:
        stub = service_pb2_grpc.StreamStub(channel)
        metadata = [("authorization", f"Bearer {token}")]

        print(f"Starting stream from block {start_block}...")
        stream = stub.Blocks(request, metadata=metadata)

        for response in stream:
            handle_response(response, module)
```

## Response Handling

```python
def handle_response(response, module: str) -> None:
    """Handle a stream response message."""
    message_type = response.WhichOneof("message")

    if message_type == "block_scoped_data":
        handle_block_scoped_data(response.block_scoped_data, module)

    elif message_type == "block_undo_signal":
        handle_block_undo_signal(response.block_undo_signal)

    elif message_type == "progress":
        handle_progress(response.progress)

    # Other message types: session_init, debug_snapshot_data, debug_snapshot_complete


def handle_block_scoped_data(data, module: str) -> None:
    """Process new block data."""
    if data.output.name != module:
        return

    # Get block info
    block_num = data.clock.number
    block_id = data.clock.id
    cursor = data.cursor

    # Unpack output
    output = data.output.map_output

    # Create instance of your output type
    result = your_module_pb2.YourOutputType()
    if not output.Unpack(result):
        raise ValueError(f"Failed to unpack {output.TypeName()}")

    # Process your data
    print(f"Block #{block_num}: {len(result.items)} items")

    # Your processing logic here
    process_data(result, block_num)

    # CRITICAL: Persist cursor AFTER successful processing
    persist_cursor(cursor)


def handle_block_undo_signal(signal) -> None:
    """Handle chain reorganization."""
    last_valid_block = signal.last_valid_block.number
    last_valid_block_id = signal.last_valid_block.id
    last_valid_cursor = signal.last_valid_cursor

    print(f"Reorg detected: rewinding to block #{last_valid_block}")

    # CRITICAL: Delete/revert data for blocks > last_valid_block
    rewind_data(last_valid_block)

    # CRITICAL: Persist the valid cursor
    persist_cursor(last_valid_cursor)


def handle_progress(progress) -> None:
    """Handle progress messages (optional)."""
    # Progress contains module execution stages
    for stage in progress.running_jobs:
        print(f"Progress: stage {stage.stage}, {stage.processed_blocks} blocks")


def rewind_data(to_block: int) -> None:
    """Revert data to handle chain reorg."""
    # Implement your rewind logic
    # Example: DELETE FROM events WHERE block_num > to_block
    print(f"Rewinding data to block #{to_block}")


def process_data(result, block_num: int) -> None:
    """Process the unpacked data."""
    # Implement your processing logic
    pass
```

## Complete Production Example

```python
#!/usr/bin/env python3
"""
Production-ready Substreams sink for Python.
"""

import grpc
import os
import sys
import time
import random
import requests
from pathlib import Path
from typing import Optional
from contextlib import contextmanager
from enum import Enum

import sf.substreams.v1.package_pb2 as package_pb2
import sf.substreams.rpc.v2.service_pb2 as service_pb2
import sf.substreams.rpc.v2.service_pb2_grpc as service_pb2_grpc

# Import your output protobuf type
# import your_output_pb2


# Configuration
ENDPOINT = os.getenv("SUBSTREAMS_ENDPOINT", "mainnet.eth.streamingfast.io:443")
SPKG = os.getenv("SUBSTREAMS_PACKAGE", "https://spkg.io/streamingfast/substreams-eth-block-meta-v0.4.3.spkg")
MODULE = os.getenv("SUBSTREAMS_MODULE", "db_out")
START_BLOCK = int(os.getenv("START_BLOCK", "17000000"))
STOP_BLOCK = int(os.getenv("STOP_BLOCK", "17001000"))
CURSOR_FILE = Path(os.getenv("CURSOR_FILE", "cursor.txt"))


class ErrorType(Enum):
    FATAL = "fatal"
    RETRYABLE = "retryable"


FATAL_CODES = {
    grpc.StatusCode.UNAUTHENTICATED,
    grpc.StatusCode.INVALID_ARGUMENT,
    grpc.StatusCode.INTERNAL,
}


def load_package(source: str) -> package_pb2.Package:
    """Load Substreams package from file or URL."""
    package = package_pb2.Package()
    if source.startswith("http"):
        response = requests.get(source)
        response.raise_for_status()
        package.ParseFromString(response.content)
    else:
        with open(source, "rb") as f:
            package.ParseFromString(f.read())
    return package


def load_cursor() -> Optional[str]:
    """Load cursor from file."""
    try:
        if CURSOR_FILE.exists():
            return CURSOR_FILE.read_text().strip()
    except Exception as e:
        print(f"Warning: Failed to load cursor: {e}")
    return None


def persist_cursor(cursor: str) -> None:
    """Persist cursor to file."""
    CURSOR_FILE.write_text(cursor)


def classify_error(error: Exception) -> ErrorType:
    """Classify error as fatal or retryable."""
    if isinstance(error, grpc.RpcError):
        if error.code() in FATAL_CODES:
            return ErrorType.FATAL
    return ErrorType.RETRYABLE


def exponential_backoff(attempt: int) -> float:
    """Calculate backoff with jitter."""
    delay_ms = min(500 * (2 ** attempt), 45000)
    jitter_ms = random.uniform(0, 100)
    return (delay_ms + jitter_ms) / 1000


def handle_block_scoped_data(data, module: str) -> None:
    """Process block data."""
    if data.output.name != module:
        return

    block_num = data.clock.number
    cursor = data.cursor

    # Unpack your data type
    output = data.output.map_output
    # result = your_output_pb2.YourType()
    # output.Unpack(result)

    print(f"Block #{block_num}: processing...")

    # Process data here
    # ...

    # CRITICAL: Persist cursor after processing
    persist_cursor(cursor)


def handle_block_undo_signal(signal) -> None:
    """Handle chain reorg."""
    last_valid = signal.last_valid_block.number
    cursor = signal.last_valid_cursor

    print(f"Reorg: rewinding to block #{last_valid}")

    # Implement your rewind logic
    # DELETE FROM table WHERE block_num > last_valid

    persist_cursor(cursor)


def stream_blocks(
    endpoint: str,
    token: str,
    package: package_pb2.Package,
    module: str,
    cursor: Optional[str],
) -> None:
    """Stream and process blocks."""
    request = service_pb2.Request(
        start_block_num=START_BLOCK,
        stop_block_num=STOP_BLOCK,
        modules=package.modules,
        output_module=module,
        production_mode=True,
    )

    if cursor:
        request.start_cursor = cursor

    creds = grpc.ssl_channel_credentials()

    with grpc.secure_channel(endpoint, creds) as channel:
        stub = service_pb2_grpc.StreamStub(channel)
        metadata = [("authorization", f"Bearer {token}")]

        print(f"Streaming from block {START_BLOCK}...")
        stream = stub.Blocks(request, metadata=metadata)

        for response in stream:
            msg_type = response.WhichOneof("message")

            if msg_type == "block_scoped_data":
                handle_block_scoped_data(response.block_scoped_data, module)
            elif msg_type == "block_undo_signal":
                handle_block_undo_signal(response.block_undo_signal)
            elif msg_type == "progress":
                pass  # Optional: log progress


def get_auth_token() -> str:
    """Get authentication token from API key or use direct token."""
    token = os.getenv("SUBSTREAMS_API_TOKEN") or os.getenv("SF_API_TOKEN")
    if token:
        return token

    api_key = os.getenv("SUBSTREAMS_API_KEY")
    if not api_key:
        print("Error: Neither SUBSTREAMS_API_TOKEN nor SUBSTREAMS_API_KEY is set")
        sys.exit(1)

    response = requests.post(
        "https://auth.streamingfast.io/v1/auth/issue",
        json={"api_key": api_key}
    )
    response.raise_for_status()
    return response.json()["token"]


def main():
    token = get_auth_token()

    package = load_package(SPKG)
    attempt = 0

    while True:
        try:
            cursor = load_cursor()
            stream_blocks(ENDPOINT, token, package, MODULE, cursor)
            print("Stream completed")
            break

        except Exception as e:
            if classify_error(e) == ErrorType.FATAL:
                print(f"Fatal error: {e}")
                sys.exit(1)

            delay = exponential_backoff(attempt)
            print(f"Retryable error ({e}), reconnecting in {delay:.1f}s...")
            time.sleep(delay)
            attempt += 1


if __name__ == "__main__":
    main()
```

## Best Practices

1. **Always persist cursor after processing** - Never before, never skip
2. **Implement exponential backoff** - With jitter and max delay
3. **Handle undo signals** - Or use `final_blocks_only` in your request
4. **Classify errors correctly** - Don't retry authentication failures
5. **Use type registry** - Generate protobuf bindings for your output types
6. **Use production mode** - Set `production_mode=True` in requests
7. **Log progress** - For long-running sinks, log block numbers periodically

## Troubleshooting

**"UNAUTHENTICATED" error:**
- Check `SF_API_TOKEN` or `SUBSTREAMS_API_TOKEN` is set
- Verify token hasn't expired
- Ensure Bearer prefix is included

**"Failed to unpack" error:**
- Regenerate protobuf bindings
- Verify output type matches module output
- Check `.spkg` URL is correct

**Empty output:**
- Verify block range contains data
- Check module name is correct
- Try the Substreams CLI first: `substreams run -e endpoint module -s start -t stop`

**Frequent disconnections:**
- This is normal for long-running streams
- Ensure reconnection loop is implemented
- Check network stability
