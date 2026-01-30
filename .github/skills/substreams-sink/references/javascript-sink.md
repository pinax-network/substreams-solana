# JavaScript Substreams Sink Reference

Complete guide for consuming Substreams data in JavaScript applications (Node.js and browser).

## Why JavaScript?

JavaScript is a good choice for:

- Web applications with real-time blockchain data
- Node.js backend services
- Rapid prototyping
- Integration with existing JavaScript ecosystems

## Installation

```bash
# Node.js
npm install @substreams/core @connectrpc/connect-node @connectrpc/connect

# Browser
npm install @substreams/core @connectrpc/connect-web @connectrpc/connect
```

## Key Dependencies

**Node.js:**
```json
{
  "dependencies": {
    "@substreams/core": "0.16.0",
    "@connectrpc/connect-node": "1.3.0",
    "@connectrpc/connect": "1.3.0"
  },
  "type": "module"
}
```

**Browser:**
```json
{
  "dependencies": {
    "@substreams/core": "0.16.0",
    "@connectrpc/connect-web": "1.3.0",
    "@connectrpc/connect": "1.3.0"
  },
  "type": "module"
}
```

## Node.js Sink Structure

### Main Entry Point

```javascript
import {
    createRequest,
    streamBlocks,
    createAuthInterceptor,
    createRegistry,
    applyParams,
    fetchSubstream,
    authIssue
} from '@substreams/core';
import { createConnectTransport } from "@connectrpc/connect-node";

// Configuration
const API_KEY = process.env.SUBSTREAMS_API_KEY;
const ENDPOINT = "https://mainnet.eth.streamingfast.io:443";
const SPKG = "https://spkg.io/streamingfast/substreams-eth-block-meta-v0.4.3.spkg";
const MODULE = "db_out";
const START_BLOCK = '17000000';
const STOP_BLOCK = '+1000';

const main = async () => {
    // Get authentication token
    const { token } = await authIssue(API_KEY);

    // Fetch and parse the Substreams package
    const pkg = await fetchSubstream(SPKG);

    // Create type registry for protobuf decoding
    const registry = createRegistry(pkg);

    // Create gRPC transport
    const transport = createConnectTransport({
        baseUrl: ENDPOINT,
        interceptors: [createAuthInterceptor(token)],
        useBinaryFormat: true,
        jsonOptions: {
            typeRegistry: registry,
        },
    });

    // Infinite loop handles disconnections
    while (true) {
        try {
            await stream(pkg, registry, transport);
            break; // Stream finished successfully
        } catch (e) {
            if (!isErrorRetryable(e)) {
                console.error(`Fatal error: ${e}`);
                throw e;
            }
            console.log(`Retryable error (${e}), reconnecting after backoff...`);
            await sleep(exponentialBackoff());
        }
    }
};

const stream = async (pkg, registry, transport) => {
    const cursor = await getCursor();

    const request = createRequest({
        substreamPackage: pkg,
        outputModule: MODULE,
        productionMode: true,
        startBlockNum: START_BLOCK,
        stopBlockNum: STOP_BLOCK,
        startCursor: cursor ?? undefined,
    });

    for await (const response of streamBlocks(transport, request)) {
        await handleResponse(response.message, registry);
    }
};

main();
```

### Response Handlers

```javascript
export const handleResponse = async (message, registry) => {
    switch (message.case) {
        case "blockScopedData":
            await handleBlockScopedData(message.value, registry);
            break;

        case "blockUndoSignal":
            await handleBlockUndoSignal(message.value);
            break;

        case "progress":
            handleProgress(message.value);
            break;
    }
};

const handleBlockScopedData = async (data, registry) => {
    const output = data.output?.mapOutput;
    const cursor = data.cursor;

    if (output !== undefined) {
        // Unpack the protobuf message
        const message = output.unpack(registry);
        if (message === undefined) {
            throw new Error(`Failed to unpack: ${output.typeUrl}`);
        }

        // Get block info
        const blockNum = data.clock.number;
        const blockId = data.clock.id;
        const timestamp = data.clock.timestamp;

        // Process your data here
        console.log(`Block #${blockNum}: ${JSON.stringify(message)}`);

        // CRITICAL: Persist cursor AFTER successful processing
        await writeCursor(cursor);
    }
};

const handleBlockUndoSignal = async (signal) => {
    const lastValidBlock = signal.lastValidBlock;
    const lastValidCursor = signal.lastValidCursor;

    console.log(`Reorg: rewinding to block #${lastValidBlock.num}`);

    // 1. Revert data for blocks > lastValidBlock.num
    await rewindData(lastValidBlock.num);

    // 2. Persist the valid cursor
    await writeCursor(lastValidCursor);
};

const handleProgress = (progress) => {
    console.log(`Progress: ${JSON.stringify(progress)}`);
};
```

### Cursor Management

**File-based (Node.js):**

```javascript
import fs from "fs";

const CURSOR_FILE = "./cursor";

export const getCursor = async () => {
    try {
        return await fs.promises.readFile(CURSOR_FILE, 'utf8');
    } catch (e) {
        return undefined; // First run, no cursor
    }
};

export const writeCursor = async (cursor) => {
    try {
        await fs.promises.writeFile(CURSOR_FILE, cursor);
    } catch (e) {
        throw new Error("COULD_NOT_COMMIT_CURSOR");
    }
};
```

**LocalStorage (Browser):**

```javascript
export const getCursor = async () => {
    const cursor = localStorage.getItem("substreams_cursor");
    return cursor ?? undefined;
};

export const writeCursor = async (cursor) => {
    localStorage.setItem("substreams_cursor", cursor);
};
```

**Database (Production):**

```javascript
import { Pool } from 'pg';

const pool = new Pool();

export const getCursor = async (sinkId) => {
    const result = await pool.query(
        'SELECT cursor FROM cursors WHERE sink_id = $1',
        [sinkId]
    );
    return result.rows[0]?.cursor;
};

export const writeCursor = async (sinkId, cursor) => {
    await pool.query(`
        INSERT INTO cursors (sink_id, cursor, updated_at)
        VALUES ($1, $2, NOW())
        ON CONFLICT (sink_id)
        DO UPDATE SET cursor = $2, updated_at = NOW()
    `, [sinkId, cursor]);
};
```

### Error Handling

```javascript
import { Code } from '@connectrpc/connect';

// Fatal errors that should NOT be retried
const FATAL_ERRORS = [
    Code.Unauthenticated,  // Invalid/expired token
    Code.InvalidArgument,  // Bad request parameters
    Code.Internal,         // Server bug
];

// Application-level errors
const isAppErrorRetryable = (e) => {
    // Cursor errors are fatal (data integrity)
    if (e.message === "COULD_NOT_READ_CURSOR" ||
        e.message === "COULD_NOT_COMMIT_CURSOR") {
        return false;
    }
    return true;
};

// Transport-level errors
const isTransportErrorRetryable = (e) => {
    return !FATAL_ERRORS.includes(e.code);
};

export const isErrorRetryable = (e) => {
    if (e.constructor.name === 'ConnectError') {
        return isTransportErrorRetryable(e);
    }
    if (e instanceof Error) {
        return isAppErrorRetryable(e);
    }
    return false;
};
```

### Exponential Backoff

```javascript
let backoffAttempt = 0;
const MAX_BACKOFF_MS = 45000;
const BASE_BACKOFF_MS = 500;

const exponentialBackoff = () => {
    const delay = Math.min(
        BASE_BACKOFF_MS * Math.pow(2, backoffAttempt),
        MAX_BACKOFF_MS
    );
    backoffAttempt++;
    return delay + Math.random() * 100; // Add jitter
};

const resetBackoff = () => {
    backoffAttempt = 0;
};

const sleep = (ms) => new Promise(resolve => setTimeout(resolve, ms));
```

## Browser Sink Structure

```javascript
import {
    createRequest,
    streamBlocks,
    createAuthInterceptor,
    createRegistry,
    fetchSubstream,
} from '@substreams/core';
import { createConnectTransport } from "@connectrpc/connect-web";

const ENDPOINT = "https://mainnet.eth.streamingfast.io:443";
const TOKEN = "your-jwt-token"; // Get from authIssue or your backend

const main = async () => {
    const pkg = await fetchSubstream(SPKG);
    const registry = createRegistry(pkg);

    // Browser transport (different from Node.js)
    const transport = createConnectTransport({
        baseUrl: ENDPOINT,
        interceptors: [createAuthInterceptor(TOKEN)],
        useBinaryFormat: true,
        jsonOptions: {
            typeRegistry: registry,
        },
    });

    // Same streaming logic as Node.js
    while (true) {
        try {
            await stream(pkg, registry, transport);
            break;
        } catch (e) {
            if (!isErrorRetryable(e)) throw e;
            await sleep(exponentialBackoff());
        }
    }
};

// Cursor storage uses localStorage
const getCursor = () => localStorage.getItem("cursor");
const writeCursor = (cursor) => localStorage.setItem("cursor", cursor);
```

## Module Parameters

```javascript
import { applyParams } from '@substreams/core';

// Apply parameters before creating request
applyParams([
    "map_events=0xa0b86a33e6...",
    "filter_module=type:transfer"
], pkg.modules?.modules);

// Or for JSON parameters
applyParams([
    'map_events={"contracts":["0x123","0x456"],"min_value":1000}'
], pkg.modules?.modules);
```

## BigInt Serialization

JavaScript BigInt doesn't serialize to JSON by default:

```javascript
// Add BigInt serialization support
BigInt.prototype.toJSON = function() {
    return this.toString();
};

// Or use a custom replacer
const stringify = (obj) => {
    return JSON.stringify(obj, (key, value) =>
        typeof value === 'bigint' ? value.toString() : value
    );
};
```

## Complete Node.js Example

```javascript
// index.js
import {
    createRequest,
    streamBlocks,
    createAuthInterceptor,
    createRegistry,
    fetchSubstream,
    authIssue,
} from '@substreams/core';
import { createConnectTransport } from "@connectrpc/connect-node";
import { Code } from '@connectrpc/connect';
import fs from 'fs';

// Configuration
const API_KEY = process.env.SUBSTREAMS_API_KEY;
const ENDPOINT = process.env.SUBSTREAMS_ENDPOINT || "https://mainnet.eth.streamingfast.io:443";
const SPKG = process.env.SUBSTREAMS_PACKAGE || "https://spkg.io/streamingfast/substreams-eth-block-meta-v0.4.3.spkg";
const MODULE = process.env.SUBSTREAMS_MODULE || "db_out";
const START_BLOCK = process.env.START_BLOCK || '17000000';
const STOP_BLOCK = process.env.STOP_BLOCK || '+1000';
const CURSOR_FILE = './cursor.txt';

// BigInt serialization
BigInt.prototype.toJSON = function() { return this.toString(); };

// Cursor management
const getCursor = async () => {
    try {
        return await fs.promises.readFile(CURSOR_FILE, 'utf8');
    } catch {
        return undefined;
    }
};

const writeCursor = async (cursor) => {
    await fs.promises.writeFile(CURSOR_FILE, cursor);
};

// Error handling
const FATAL_ERRORS = [Code.Unauthenticated, Code.InvalidArgument, Code.Internal];

const isRetryable = (e) => {
    if (e.constructor.name === 'ConnectError') {
        return !FATAL_ERRORS.includes(e.code);
    }
    return e.message !== 'CURSOR_ERROR';
};

// Backoff
let attempt = 0;
const backoff = () => {
    const delay = Math.min(500 * Math.pow(2, attempt++), 45000);
    return delay + Math.random() * 100;
};
const resetBackoff = () => { attempt = 0; };
const sleep = (ms) => new Promise(r => setTimeout(r, ms));

// Main logic
const main = async () => {
    console.log('Starting Substreams sink...');

    const { token } = await authIssue(API_KEY);
    const pkg = await fetchSubstream(SPKG);
    const registry = createRegistry(pkg);

    const transport = createConnectTransport({
        baseUrl: ENDPOINT,
        interceptors: [createAuthInterceptor(token)],
        useBinaryFormat: true,
        jsonOptions: { typeRegistry: registry },
    });

    while (true) {
        try {
            const cursor = await getCursor();
            console.log(`Starting stream${cursor ? ' from cursor' : ' from beginning'}...`);

            const request = createRequest({
                substreamPackage: pkg,
                outputModule: MODULE,
                productionMode: true,
                startBlockNum: START_BLOCK,
                stopBlockNum: STOP_BLOCK,
                startCursor: cursor ?? undefined,
            });

            for await (const response of streamBlocks(transport, request)) {
                const msg = response.message;

                if (msg.case === 'blockScopedData') {
                    const data = msg.value;
                    const output = data.output?.mapOutput;

                    if (output) {
                        const message = output.unpack(registry);
                        const blockNum = data.clock.number;

                        // Process your data
                        console.log(`Block #${blockNum}: ${message ? 'data received' : 'empty'}`);

                        // Persist cursor after processing
                        await writeCursor(data.cursor);
                        resetBackoff();
                    }
                } else if (msg.case === 'blockUndoSignal') {
                    const signal = msg.value;
                    console.log(`Reorg: rewind to block #${signal.lastValidBlock.num}`);

                    // Implement your rewind logic here
                    // ...

                    await writeCursor(signal.lastValidCursor);
                } else if (msg.case === 'progress') {
                    // Optional: log progress
                }
            }

            console.log('Stream completed successfully');
            break;

        } catch (e) {
            if (!isRetryable(e)) {
                console.error('Fatal error:', e);
                process.exit(1);
            }
            const delay = backoff();
            console.log(`Retryable error, reconnecting in ${Math.round(delay)}ms...`);
            await sleep(delay);
        }
    }
};

main().catch(console.error);
```

## Best Practices

1. **Always use production mode** - Set `productionMode: true` for sinks
2. **Handle all message types** - `blockScopedData`, `blockUndoSignal`, `progress`
3. **Persist cursor after processing** - Never before, never skip
4. **Implement exponential backoff** - With max delay and jitter
5. **Classify errors correctly** - Fatal vs retryable
6. **Reset backoff on success** - When data is received successfully
7. **Handle BigInt serialization** - Add toJSON method or custom replacer
8. **Use environment variables** - For configuration (API key, endpoint, etc.)

## Troubleshooting

**"Unauthenticated" error:**
- Verify API key is set: `process.env.SUBSTREAMS_API_KEY`
- Check token hasn't expired
- Try re-issuing token with `authIssue()`

**"Failed to unpack" error:**
- Ensure registry is created from the same package
- Verify output module name is correct

**No data received:**
- Check block range contains data
- Verify module name matches the package
- Try a known-good block range first

**Connection drops frequently:**
- Implement proper retry loop with backoff
- Check network stability
- Consider using `final_blocks_only` for less time-sensitive sinks

**Browser CORS errors:**
- Use a backend proxy for the Substreams endpoint
- Or use endpoints that support CORS
