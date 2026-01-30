# Substreams Manifest Specification

Complete reference for `substreams.yaml` manifest files.

## Basic Structure

```yaml
specVersion: v0.1.0
package:
  name: my-substreams
  version: v1.0.0
  url: https://github.com/myorg/my-substreams
  description: Short description of what this substreams does

protobuf:
  files:
    - events.proto
    - types.proto
  importPaths:
    - ./proto
    - ./third_party

binaries:
  default:
    type: wasm/rust-v1
    file: ./target/wasm32-unknown-unknown/release/my_substreams.wasm

network: mainnet

modules:
  - name: map_events
    kind: map
    initialBlock: 12369621
    inputs:
      - source: sf.ethereum.type.v2.Block
    output:
      type: proto:my.types.Events

  - name: store_totals
    kind: store
    initialBlock: 12369621
    updatePolicy: add
    valueType: int64
    inputs:
      - map: map_events

  - name: index_transfers
    kind: index
    initialBlock: 12369621
    inputs:
      - map: map_events
    output:
      type: proto:sf.substreams.index.v1.Keys
```

## Package Section

### Required Fields

- `name`: Package name (lowercase, alphanumeric, hyphens)
- `version`: Semantic version (e.g., v1.0.0)

### Optional Fields

- `url`: Repository or documentation URL
- `description`: Short description of the package
- `doc`: Multi-line description (deprecated, use a `README.md` file instead)
- `image`: Container image for custom runtime

## Imports Section

Import external `.spkg` packages to use their protobuf definitions and module types:

```yaml
imports:
    imported_pkg: https://example.com/path/to/package-v1.0.0.spkg
```

Imported packages make their protobuf types and modules available for use in your manifest. See sink-specific documentation for required imports.

## Protobuf Section

### Files

List of `.proto` files to compile:

```yaml
protobuf:
  files:
    - events.proto
    - types.proto
  importPaths:
    - ./proto
```

### Exclude Paths

**Always** include `excludePaths` when importing spkgs, even when you have no custom proto files. Without this, the build may generate unnecessary proto code or fail:

```yaml
protobuf:
  excludePaths:
    - sf/substreams
    - google
```

### Import Paths

Directories to search for proto imports:

```yaml
protobuf:
  importPaths:
    - ./proto
    - ./third_party
    - ../shared-protos
```

## Binaries Section

### Rust WASM Binary

```yaml
binaries:
  default:
    type: wasm/rust-v1
    file: ./target/wasm32-unknown-unknown/release/my_substreams.wasm
```

### Multiple Binaries

```yaml
binaries:
  default:
    type: wasm/rust-v1
    file: ./target/wasm32-unknown-unknown/release/my_substreams.wasm
  
  stores:
    type: wasm/rust-v1
    file: ./target/wasm32-unknown-unknown/release/stores.wasm
```

## Network Configuration

Supported networks (see [references/networks.md](./networks.md) for complete list):

- `mainnet` - Ethereum Mainnet
- `optimism` - OP Mainnet
- `arbitrum-one` - Arbitrum One Mainnet
- `base` - Base Chain
- `matic` - Polygon Mainnet
- `bsc` - BNB Smart Chain Mainnet
- `avalanche` - Avalanche C-Chain
- `fantom` - Fantom Opera Mainnet
- `solana-mainnet-beta` - Solana Mainnet
- `near-mainnet` - Near Mainnet
- And 70+ additional networks...

## Module Types

### Map Module

Transforms input data to output data:

```yaml
- name: map_events
  kind: map
  initialBlock: 12369621
  inputs:
    - source: sf.ethereum.type.v2.Block
    - params: string
  output:
    type: proto:my.types.Events
  doc: "Extracts transfer events from blocks"
```

### Store Module

Aggregates data across blocks:

```yaml
- name: store_totals
  kind: store
  initialBlock: 12369621
  updatePolicy: add  # or set, set_if_not_exists, append
  valueType: int64   # or string, bytes, bigint, float64, bigdecimal, proto:my.Type
  inputs:
    - map: map_events
    - params: string
  doc: "Accumulates transfer totals by token"
```

### Index Module

Filters blocks for efficient querying:

```yaml
- name: index_transfers
  kind: index
  initialBlock: 12369621
  inputs:
    - map: map_events
  output:
    type: proto:sf.substreams.index.v1.Keys
  doc: "Indexes blocks containing transfers"
```

## Input Types

### Source Inputs

Direct blockchain data:

```yaml
inputs:
  - source: sf.ethereum.type.v2.Block
```

### Module Inputs

Output from other modules:

```yaml
inputs:
  - map: map_events
  - store: store_totals
  - store: store_totals, mode: get    # Read-only access
  - store: store_totals, mode: deltas # Get deltas only
```

### Parameter Inputs

Runtime parameters:

```yaml
inputs:
  - params: string
```

## Update Policies (Store Modules)

- `set`: Replace existing value
- `set_if_not_exists`: Set only if key doesn't exist
- `add`: Add to existing numeric value
- `append`: Append to existing bytes value
- `max`: Keep maximum value
- `min`: Keep minimum value

## Value Types (Store Modules)

- `string`: UTF-8 string
- `bytes`: Raw bytes
- `int64`: 64-bit signed integer
- `bigint`: Arbitrary precision integer
- `float64`: 64-bit floating point
- `bigdecimal`: Arbitrary precision decimal
- `proto:my.Type`: Custom protobuf type

## Advanced Features

### Parameterized Modules

```yaml
- name: map_token_transfers
  kind: map
  inputs:
    - params: string  # Token contract address
    - source: sf.ethereum.type.v2.Block
  output:
    type: proto:my.types.Transfers
```

Usage:
```bash
substreams run map_token_transfers -p map_token_transfers=0xa0b86a33e6...
```

### Module Dependencies

```yaml
- name: map_events
  kind: map
  inputs:
    - source: sf.ethereum.type.v2.Block

- name: map_enriched
  kind: map
  inputs:
    - map: map_events
    - store: store_metadata, mode: get

- name: store_metadata
  kind: store
  inputs:
    - map: map_events
```

### Binary Selection

```yaml
- name: map_events
  kind: map
  binary: default  # Use specific binary
  inputs:
    - source: sf.ethereum.type.v2.Block

- name: store_totals
  kind: store
  binary: stores   # Use different binary
  inputs:
    - map: map_events
```

## Sink Section

Required when deploying to a sink service. Without this section, the sink will error with `no sink config found in spkg`.

```yaml
sink:
  module: <output_module_name>
  type: <sink.service.protobuf.Type>
  config:
    # Sink-specific configuration
```

- `module`: The map module whose output feeds the sink
- `type`: The sink service protobuf type (from imported spkg)
- `config`: Sink-specific configuration fields

See the SQL skill documentation for SQL sink configuration details.

## Best Practices

1. **Set initialBlock**: Always specify the first relevant block
2. **Use descriptive names**: Module names should be clear and consistent
3. **Document modules**: Add `doc` fields for complex modules
4. **Minimize dependencies**: Avoid deep module dependency chains
5. **Version carefully**: Use semantic versioning for packages
6. **Test manifests**: Use `substreams graph` to visualize dependencies

## Validation

Validate your manifest:

```bash
# Check manifest syntax
substreams info

# Visualize module graph
substreams graph

# Validate against network
substreams run -s 1000 -t +10 module_name
```
