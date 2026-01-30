# FireCore Tools Guide

Comprehensive guide for using FireCore tools in Substreams testing, including Firehose data access, StreamingFast services, and testing utilities.

## Overview of FireCore Tools

FireCore provides the foundational infrastructure for Substreams:

- **Firehose**: High-performance blockchain data streaming
- **Substreams Engine**: Parallel processing runtime
- **StreamingFast API**: Cloud-based data access
- **Testing Utilities**: Tools for development and testing

## Firehose Integration

### Setting Up FireCore for Testing

The `firecore` CLI is the main tool for interacting with Firehose. Install it from the [firehose-core releases](https://github.com/streamingfast/firehose-core/releases).

```bash
# Configure authentication (required for hosted endpoints)
substreams auth

# Verify firecore is available
firecore --version

# Test connection by fetching the latest block
firecore tools firehose-client mainnet -o text -- -1
```

### Fetching Test Data

Use `firecore tools firehose-client` to fetch blocks for test fixtures:

```bash
#!/bin/bash
# scripts/fetch_test_data.sh

set -e

NETWORK="mainnet"  # Network ID or endpoint
START_BLOCK=17000000
BLOCK_COUNT=100
OUTPUT_DIR="test_data/firehose"

echo "Fetching test data from Firehose"

mkdir -p "$OUTPUT_DIR"

# Fetch blocks as JSON (one block per line)
firecore tools firehose-client "$NETWORK" -o json -- $START_BLOCK +$BLOCK_COUNT \
    > "$OUTPUT_DIR/blocks_${START_BLOCK}_$((START_BLOCK + BLOCK_COUNT)).jsonl"

echo "Test data fetching complete"
echo "  Data stored in: $OUTPUT_DIR"
echo "  Total blocks: $BLOCK_COUNT"
```

### Using Firehose Data in Tests

```rust
// src/testing/firehose_data.rs
use serde_json;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use crate::Block; // Your block type

pub struct FirehoseTestData {
    data_dir: String,
}

impl FirehoseTestData {
    pub fn new(data_dir: &str) -> Self {
        Self {
            data_dir: data_dir.to_string(),
        }
    }
    
    pub fn load_block(&self, block_number: u64) -> Result<Block, Box<dyn std::error::Error>> {
        // Find the file containing this block
        let file_path = self.find_block_file(block_number)?;
        
        // Read the JSONL file and find the specific block
        let file = File::open(&file_path)?;
        let reader = BufReader::new(file);
        
        for line in reader.lines() {
            let line = line?;
            let block_data: serde_json::Value = serde_json::from_str(&line)?;
            
            if let Some(number) = block_data.get("number") {
                if number.as_u64() == Some(block_number) {
                    // Convert JSON to your block type
                    return self.json_to_block(&block_data);
                }
            }
        }
        
        Err(format!("Block {} not found in test data", block_number).into())
    }
    
    pub fn load_block_range(&self, start: u64, end: u64) -> Result<Vec<Block>, Box<dyn std::error::Error>> {
        let mut blocks = Vec::new();
        
        for block_number in start..=end {
            match self.load_block(block_number) {
                Ok(block) => blocks.push(block),
                Err(e) => {
                    eprintln!("Warning: Failed to load block {}: {}", block_number, e);
                    // Continue loading other blocks
                }
            }
        }
        
        if blocks.is_empty() {
            return Err("No blocks loaded from range".into());
        }
        
        Ok(blocks)
    }
    
    fn find_block_file(&self, block_number: u64) -> Result<String, Box<dyn std::error::Error>> {
        // Look for files that might contain this block number
        let data_path = Path::new(&self.data_dir);
        
        for entry in std::fs::read_dir(data_path)? {
            let entry = entry?;
            let filename = entry.file_name();
            let filename_str = filename.to_string_lossy();
            
            if filename_str.starts_with("blocks_") && filename_str.ends_with(".jsonl") {
                // Parse range from filename: blocks_17000000_17000100.jsonl
                let parts: Vec<&str> = filename_str
                    .strip_prefix("blocks_")
                    .unwrap()
                    .strip_suffix(".jsonl")
                    .unwrap()
                    .split('_')
                    .collect();
                
                if parts.len() >= 2 {
                    if let (Ok(start), Ok(end)) = (parts[0].parse::<u64>(), parts[1].parse::<u64>()) {
                        if block_number >= start && block_number <= end {
                            return Ok(entry.path().to_string_lossy().to_string());
                        }
                    }
                }
            }
        }
        
        Err(format!("No data file found containing block {}", block_number).into())
    }
    
    fn json_to_block(&self, json: &serde_json::Value) -> Result<Block, Box<dyn std::error::Error>> {
        // Convert Firehose JSON format to your Block struct
        // This implementation depends on your specific block structure
        
        let number = json.get("number")
            .and_then(|n| n.as_u64())
            .ok_or("Missing block number")?;
            
        let hash = json.get("hash")
            .and_then(|h| h.as_str())
            .ok_or("Missing block hash")?;
            
        let timestamp = json.get("header")
            .and_then(|h| h.get("timestamp"))
            .and_then(|t| t.as_u64())
            .ok_or("Missing timestamp")?;
        
        // Parse transactions
        let mut transactions = Vec::new();
        if let Some(tx_array) = json.get("transactions").and_then(|t| t.as_array()) {
            for tx_json in tx_array {
                transactions.push(self.json_to_transaction(tx_json)?);
            }
        }
        
        Ok(Block {
            number,
            hash: hex::decode(&hash[2..])?, // Remove 0x prefix
            timestamp_seconds: timestamp,
            transaction_traces: transactions,
            ..Default::default()
        })
    }
    
    fn json_to_transaction(&self, json: &serde_json::Value) -> Result<TransactionTrace, Box<dyn std::error::Error>> {
        // Convert transaction JSON to TransactionTrace
        let hash = json.get("hash")
            .and_then(|h| h.as_str())
            .ok_or("Missing transaction hash")?;
        
        let from = json.get("from")
            .and_then(|f| f.as_str())
            .ok_or("Missing from address")?;
        
        let to = json.get("to")
            .and_then(|t| t.as_str());
        
        let value = json.get("value")
            .and_then(|v| v.as_str())
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);
        
        // Parse receipt and logs
        let mut logs = Vec::new();
        if let Some(receipt) = json.get("receipt") {
            if let Some(log_array) = receipt.get("logs").and_then(|l| l.as_array()) {
                for log_json in log_array {
                    logs.push(self.json_to_log(log_json)?);
                }
            }
        }
        
        Ok(TransactionTrace {
            hash: hex::decode(&hash[2..])?,
            from: hex::decode(&from[2..])?,
            to: to.map(|t| hex::decode(&t[2..]).unwrap_or_default()).unwrap_or_default(),
            value: vec![], // Convert value appropriately
            receipt: Some(TransactionReceipt {
                logs,
                ..Default::default()
            }),
            ..Default::default()
        })
    }
    
    fn json_to_log(&self, json: &serde_json::Value) -> Result<Log, Box<dyn std::error::Error>> {
        let address = json.get("address")
            .and_then(|a| a.as_str())
            .ok_or("Missing log address")?;
        
        let data = json.get("data")
            .and_then(|d| d.as_str())
            .unwrap_or("");
        
        let mut topics = Vec::new();
        if let Some(topic_array) = json.get("topics").and_then(|t| t.as_array()) {
            for topic in topic_array {
                if let Some(topic_str) = topic.as_str() {
                    topics.push(hex::decode(&topic_str[2..]).unwrap_or_default());
                }
            }
        }
        
        Ok(Log {
            address: hex::decode(&address[2..])?,
            topics,
            data: hex::decode(&data[2..]).unwrap_or_default(),
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_load_firehose_block() {
        let firehose_data = FirehoseTestData::new("test_data/firehose");
        
        // Test loading a specific block
        let block = firehose_data.load_block(17000000).unwrap();
        
        assert_eq!(block.number, 17000000);
        assert!(!block.transaction_traces.is_empty());
        
        // Validate block structure
        assert!(block.hash.len() == 32);
        assert!(block.timestamp_seconds > 0);
        
        println!("✅ Loaded block {} with {} transactions", 
                block.number, block.transaction_traces.len());
    }
    
    #[test]
    fn test_load_block_range() {
        let firehose_data = FirehoseTestData::new("test_data/firehose");
        
        let blocks = firehose_data.load_block_range(17000000, 17000010).unwrap();
        
        assert_eq!(blocks.len(), 11); // Inclusive range
        
        // Verify blocks are in order
        for i in 1..blocks.len() {
            assert_eq!(blocks[i].number, blocks[i-1].number + 1);
        }
        
        println!("✅ Loaded {} blocks from range", blocks.len());
    }
}
```

## StreamingFast API Integration

### API-Based Testing

```rust
// src/testing/streamingfast_api.rs
use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct ApiResponse {
    data: serde_json::Value,
    cursor: Option<String>,
}

#[derive(Debug, Serialize)]
struct SubstreamsRequest {
    start_block_num: u64,
    stop_block_num: u64, 
    output_module: String,
    production_mode: bool,
}

pub struct StreamingFastClient {
    api_key: String,
    base_url: String,
    client: reqwest::Client,
}

impl StreamingFastClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://api.streamingfast.io".to_string(),
            client: reqwest::Client::new(),
        }
    }
    
    pub async fn run_substreams(
        &self,
        network: &str,
        module: &str,
        start_block: u64,
        block_count: u64,
        production_mode: bool,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let url = format!("{}/v1/substreams/{}/run", self.base_url, network);
        
        let request = SubstreamsRequest {
            start_block_num: start_block,
            stop_block_num: start_block + block_count,
            output_module: module.to_string(),
            production_mode,
        };
        
        let response = self.client
            .post(&url)
            .header("Authorization", &format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("API request failed: {}", error_text).into());
        }
        
        let api_response: ApiResponse = response.json().await?;
        
        // Parse the streaming response
        self.parse_streaming_response(&api_response.data)
    }
    
    fn parse_streaming_response(
        &self,
        data: &serde_json::Value,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let mut results = Vec::new();
        
        if let Some(array) = data.as_array() {
            for item in array {
                if let Some(output) = item.get("output") {
                    results.push(output.clone());
                }
            }
        }
        
        Ok(results)
    }
    
    pub async fn get_network_info(&self, network: &str) -> Result<NetworkInfo, Box<dyn std::error::Error>> {
        let url = format!("{}/v1/networks/{}", self.base_url, network);
        
        let response = self.client
            .get(&url)
            .header("Authorization", &format!("Bearer {}", self.api_key))
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err("Failed to fetch network info".into());
        }
        
        let network_info: NetworkInfo = response.json().await?;
        Ok(network_info)
    }
}

#[derive(Debug, Deserialize)]
pub struct NetworkInfo {
    pub name: String,
    pub chain_id: u64,
    pub head_block: u64,
    pub lib_block: u64, // Last irreversible block
}

// Integration test using StreamingFast API
#[cfg(test)]
mod api_tests {
    use super::*;
    
    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_api_integration() {
        let api_key = std::env::var("STREAMINGFAST_API_KEY")
            .expect("STREAMINGFAST_API_KEY environment variable required");
        
        let client = StreamingFastClient::new(api_key);
        
        // Test network info
        let network_info = client.get_network_info("ethereum").await.unwrap();
        println!("Network info: {:?}", network_info);
        
        assert_eq!(network_info.name, "ethereum");
        assert!(network_info.head_block > 17000000);
        
        // Test small substreams run
        let results = client.run_substreams(
            "ethereum",
            "map_transfers", 
            17000000,
            10, // 10 blocks
            false // dev mode
        ).await.unwrap();
        
        assert!(!results.is_empty());
        println!("Got {} results from API", results.len());
        
        // Validate result structure
        for result in &results {
            assert!(result.get("block_number").is_some());
            // Add more validation based on your module output
        }
    }
    
    #[tokio::test]
    #[ignore]
    async fn test_api_performance_comparison() {
        let api_key = std::env::var("STREAMINGFAST_API_KEY").unwrap();
        let client = StreamingFastClient::new(api_key);
        
        let start_block = 17000000;
        let block_count = 1000;
        
        // Test development mode
        let start = std::time::Instant::now();
        let dev_results = client.run_substreams(
            "ethereum",
            "map_transfers",
            start_block,
            block_count,
            false
        ).await.unwrap();
        let dev_duration = start.elapsed();
        
        // Test production mode
        let start = std::time::Instant::now();
        let prod_results = client.run_substreams(
            "ethereum", 
            "map_transfers",
            start_block,
            block_count,
            true
        ).await.unwrap();
        let prod_duration = start.elapsed();
        
        println!("API Performance Comparison:");
        println!("  Dev mode: {:?} ({} results)", dev_duration, dev_results.len());
        println!("  Prod mode: {:?} ({} results)", prod_duration, prod_results.len());
        
        let speedup = dev_duration.as_secs_f64() / prod_duration.as_secs_f64();
        println!("  Speedup: {:.2}x", speedup);
        
        // Production mode should be faster
        assert!(speedup > 1.0);
        
        // Results should be consistent
        assert_eq!(dev_results.len(), prod_results.len());
    }
}
```

## Testing with Local Firehose

### Running Local Firehose for Testing

```yaml
# docker-compose.firehose.yml
version: '3.8'

services:
  firehose:
    image: streamingfast/firehose-ethereum:latest
    ports:
      - "9030:9030" # gRPC
      - "9031:9031" # HTTP
    environment:
      - FIREHOSE_CONFIG_FILE=/app/firehose.yaml
    volumes:
      - ./firehose-config.yaml:/app/firehose.yaml
      - ./test-data:/data
    command: ["start", "--config-file=/app/firehose.yaml"]

  substreams:
    image: streamingfast/substreams:latest
    ports:
      - "9032:9032"
    depends_on:
      - firehose
    environment:
      - SUBSTREAMS_FIREHOSE_ENDPOINT=firehose:9030
    volumes:
      - .:/workspace
    working_dir: /workspace
```

```yaml
# firehose-config.yaml
start:
  args:
    - firehose
  flags:
    common-first-streamable-block: 17000000
    common-forked-blocks-file-path: /data/forked_blocks.txt
    firehose-grpc-listen-addr: ":9030"
    firehose-http-listen-addr: ":9031"
    
firehose:
  args: []
  flags:
    merger-time-between-store-pruning: 10m
    merger-prune-forked-blocks-after: 50000
```

```bash
#!/bin/bash
# scripts/setup_local_firehose.sh

set -e

echo "🔥 Setting up local Firehose for testing"

# Create data directories
mkdir -p test-data/blocks
mkdir -p test-data/forked-blocks

# Start Firehose
docker-compose -f docker-compose.firehose.yml up -d

# Wait for services to be ready
echo "⏳ Waiting for Firehose to start..."
sleep 30

# Health check
curl -f http://localhost:9031/health || {
    echo "❌ Firehose health check failed"
    docker-compose -f docker-compose.firehose.yml logs
    exit 1
}

echo "✅ Local Firehose ready for testing"
echo "   gRPC endpoint: localhost:9030"
echo "   HTTP endpoint: localhost:9031"

# Test connection
substreams run \
    --endpoint localhost:9030 \
    --plaintext \
    -s 17000000 \
    -t +10 \
    map_transfers

echo "🎉 Local Firehose test successful"
```

## Advanced Testing Patterns with FireCore

### Cursor and State Management Testing

```rust
// tests/cursor_state_tests.rs
use your_substreams::*;

#[test]
fn test_cursor_based_resumption() {
    let test_range = 17000000..17001000;
    let checkpoint_interval = 100; // Save cursor every 100 blocks
    
    let mut cursors = Vec::new();
    let mut expected_results = Vec::new();
    
    // First run: process all blocks and collect cursors
    for (i, block_num) in test_range.clone().enumerate() {
        let block = load_test_block(block_num);
        let result = map_transfers(block).unwrap();
        
        expected_results.push(result);
        
        // Save cursor at checkpoints
        if i % checkpoint_interval == 0 {
            cursors.push((block_num, get_current_cursor()));
        }
    }
    
    // Second run: resume from each checkpoint and verify consistency
    for (checkpoint_block, cursor) in cursors {
        println!("🔄 Testing resumption from block {}", checkpoint_block);
        
        set_cursor(cursor.clone());
        
        let remaining_range = checkpoint_block..test_range.end;
        let mut resumed_results = Vec::new();
        
        for block_num in remaining_range {
            let block = load_test_block(block_num);
            let result = map_transfers(block).unwrap();
            resumed_results.push(result);
        }
        
        // Compare with expected results
        let expected_idx = (checkpoint_block - test_range.start) as usize;
        let expected_remaining = &expected_results[expected_idx..];
        
        assert_eq!(resumed_results.len(), expected_remaining.len());
        
        for (resumed, expected) in resumed_results.iter().zip(expected_remaining.iter()) {
            assert_eq!(resumed.transfers.len(), expected.transfers.len());
            // Add more detailed comparison as needed
        }
        
        println!("✅ Resumption from block {} successful", checkpoint_block);
    }
}

fn get_current_cursor() -> String {
    // In real implementation, this would get the cursor from Substreams
    format!("cursor_at_{}", chrono::Utc::now().timestamp())
}

fn set_cursor(_cursor: String) {
    // In real implementation, this would set the cursor for resumption
}
```

### Reorg Testing with FireCore

```rust
// tests/reorg_tests.rs
use your_substreams::*;

#[test]
fn test_blockchain_reorganization() {
    // Simulate a blockchain reorganization scenario
    
    // Original chain
    let original_chain = vec![
        create_test_block(17000000, "0xabc123", "0x000000"),
        create_test_block(17000001, "0xdef456", "0xabc123"),
        create_test_block(17000002, "0x789xyz", "0xdef456"),
        create_test_block(17000003, "0x111aaa", "0x789xyz"),
    ];
    
    // Alternative chain (reorg at block 17000002)
    let reorg_chain = vec![
        create_test_block(17000000, "0xabc123", "0x000000"), // Same
        create_test_block(17000001, "0xdef456", "0xabc123"), // Same
        create_test_block(17000002, "0x999bbb", "0xdef456"), // Different!
        create_test_block(17000003, "0x222ccc", "0x999bbb"), // Different!
        create_test_block(17000004, "0x333ddd", "0x222ccc"), // New!
    ];
    
    let mut store = create_test_store();
    
    // Process original chain
    println!("📦 Processing original chain");
    for block in &original_chain {
        let transfers = map_transfers(block.clone()).unwrap();
        store_balances(transfers, &store);
    }
    
    let balance_after_original = store.get_last("USDC:alice").unwrap_or(0);
    println!("Balance after original chain: {}", balance_after_original);
    
    // Simulate reorg detection and rollback
    println!("🔄 Simulating blockchain reorganization");
    
    // In a real scenario, Substreams would handle the rollback
    // Here we simulate by resetting state to the common ancestor
    rollback_to_block(&store, 17000001);
    
    // Process reorg chain from the fork point
    for block in &reorg_chain[2..] { // Skip common blocks
        let transfers = map_transfers(block.clone()).unwrap();
        store_balances(transfers, &store);
    }
    
    let balance_after_reorg = store.get_last("USDC:alice").unwrap_or(0);
    println!("Balance after reorg: {}", balance_after_reorg);
    
    // Verify the system handled the reorg correctly
    // The specific assertion depends on your test scenario
    assert_ne!(balance_after_original, balance_after_reorg);
    
    // Verify final state is consistent
    validate_store_consistency(&store);
    
    println!("✅ Reorganization handling test passed");
}

fn create_test_block(number: u64, hash: &str, parent_hash: &str) -> Block {
    let mut block = Block {
        number,
        hash: hex::decode(&hash[2..]).unwrap(),
        parent_hash: hex::decode(&parent_hash[2..]).unwrap(),
        timestamp_seconds: 1680000000 + (number * 12),
        ..Default::default()
    };
    
    // Add some test transactions with transfers
    block.transaction_traces.push(create_test_transaction_with_transfer(
        &format!("0x{:064x}", number), // tx hash
        "0xA0b86a33E6Fe17d67C8c086c6c4c0E3C8E3B7EC2", // USDC
        "alice", 
        "bob",
        &(1000000000 + number).to_string() // amount varies by block
    ));
    
    block
}

fn rollback_to_block(store: &TestStore, block_number: u64) {
    // In real implementation, this would use Substreams' built-in rollback
    // Here we simulate by clearing state after the rollback point
    store.rollback_to_block(block_number);
}

fn validate_store_consistency(store: &TestStore) {
    // Verify that all balances sum to zero (conservation)
    let all_keys = store.get_all_keys();
    let mut total_balance = 0i64;
    
    for key in all_keys {
        if let Some(balance) = store.get_last(&key) {
            total_balance += balance;
        }
    }
    
    assert_eq!(total_balance, 0, "Balance conservation violated");
}
```

### Performance Testing with FireCore Metrics

```rust
// tests/firecore_performance_tests.rs
use your_substreams::*;
use std::time::Instant;
use std::collections::HashMap;

#[test]
fn test_firecore_performance_metrics() {
    let mut metrics = FireCoreMetrics::new();
    
    let test_blocks = load_test_blocks(17000000..17001000);
    
    let start_time = Instant::now();
    
    for (i, block) in test_blocks.iter().enumerate() {
        let block_start = Instant::now();
        
        // Process block
        let result = map_transfers(block.clone()).unwrap();
        
        let block_duration = block_start.elapsed();
        
        // Record metrics
        metrics.record_block_processed(
            block.number,
            block_duration,
            result.transfers.len(),
            estimate_block_size(block)
        );
        
        if i % 100 == 0 {
            println!("Processed {} blocks", i + 1);
        }
    }
    
    let total_duration = start_time.elapsed();
    
    // Analyze performance
    let analysis = metrics.analyze();
    
    println!("📊 FireCore Performance Analysis:");
    println!("  Total blocks: {}", analysis.total_blocks);
    println!("  Total duration: {:?}", total_duration);
    println!("  Avg block time: {:?}", analysis.avg_block_time);
    println!("  Blocks/second: {:.2}", analysis.blocks_per_second);
    println!("  Total transfers: {}", analysis.total_transfers);
    println!("  Transfers/second: {:.2}", analysis.transfers_per_second);
    println!("  Throughput (MB/s): {:.2}", analysis.throughput_mbps);
    
    // Performance assertions
    assert!(analysis.blocks_per_second > 50.0, "Processing too slow");
    assert!(analysis.avg_block_time.as_millis() < 100, "Block processing too slow");
    
    // Check for performance degradation over time
    let degradation = metrics.check_performance_degradation();
    assert!(degradation < 0.2, "Performance degraded by {}%", degradation * 100.0);
}

struct FireCoreMetrics {
    block_times: Vec<(u64, std::time::Duration, usize, usize)>, // (block_num, duration, transfers, size)
}

impl FireCoreMetrics {
    fn new() -> Self {
        Self {
            block_times: Vec::new(),
        }
    }
    
    fn record_block_processed(
        &mut self, 
        block_number: u64, 
        duration: std::time::Duration,
        transfer_count: usize,
        block_size: usize
    ) {
        self.block_times.push((block_number, duration, transfer_count, block_size));
    }
    
    fn analyze(&self) -> PerformanceAnalysis {
        let total_blocks = self.block_times.len();
        let total_duration: std::time::Duration = self.block_times.iter().map(|(_, d, _, _)| *d).sum();
        let total_transfers: usize = self.block_times.iter().map(|(_, _, t, _)| *t).sum();
        let total_size: usize = self.block_times.iter().map(|(_, _, _, s)| *s).sum();
        
        let avg_block_time = total_duration / total_blocks as u32;
        let blocks_per_second = total_blocks as f64 / total_duration.as_secs_f64();
        let transfers_per_second = total_transfers as f64 / total_duration.as_secs_f64();
        let throughput_mbps = (total_size as f64 / (1024.0 * 1024.0)) / total_duration.as_secs_f64();
        
        PerformanceAnalysis {
            total_blocks,
            total_duration,
            avg_block_time,
            blocks_per_second,
            total_transfers,
            transfers_per_second,
            throughput_mbps,
        }
    }
    
    fn check_performance_degradation(&self) -> f64 {
        if self.block_times.len() < 100 {
            return 0.0; // Not enough data
        }
        
        let chunk_size = self.block_times.len() / 10;
        let first_chunk = &self.block_times[..chunk_size];
        let last_chunk = &self.block_times[self.block_times.len() - chunk_size..];
        
        let first_avg: std::time::Duration = first_chunk.iter().map(|(_, d, _, _)| *d).sum::<std::time::Duration>() / chunk_size as u32;
        let last_avg: std::time::Duration = last_chunk.iter().map(|(_, d, _, _)| *d).sum::<std::time::Duration>() / chunk_size as u32;
        
        let degradation = (last_avg.as_secs_f64() - first_avg.as_secs_f64()) / first_avg.as_secs_f64();
        degradation.max(0.0) // Only report degradation, not improvements
    }
}

struct PerformanceAnalysis {
    total_blocks: usize,
    total_duration: std::time::Duration,
    avg_block_time: std::time::Duration,
    blocks_per_second: f64,
    total_transfers: usize,
    transfers_per_second: f64,
    throughput_mbps: f64,
}

fn estimate_block_size(block: &Block) -> usize {
    // Rough estimate of block size in bytes
    let base_size = 100; // Block header estimate
    let tx_size: usize = block.transaction_traces.iter()
        .map(|tx| {
            let logs_size: usize = tx.receipt.as_ref()
                .map(|r| r.logs.len() * 100) // Rough log size estimate
                .unwrap_or(0);
            200 + logs_size // Base tx size + logs
        })
        .sum();
    
    base_size + tx_size
}

fn load_test_blocks(range: std::ops::Range<u64>) -> Vec<Block> {
    // In real implementation, this would load from Firehose test data
    range.map(|block_num| {
        create_test_block(block_num, &format!("0x{:064x}", block_num), &format!("0x{:064x}", block_num - 1))
    }).collect()
}
```

## Debugging with FireCore Tools

### Debug Logging and Tracing

```rust
// src/testing/debug_helpers.rs
use tracing::{info, debug, warn, error, span, Level};
use tracing_subscriber;

pub fn setup_debug_logging() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();
}

pub fn trace_substreams_execution<F, R>(operation: &str, f: F) -> R 
where 
    F: FnOnce() -> R,
{
    let span = span!(Level::INFO, "substreams_op", op = operation);
    let _enter = span.enter();
    
    info!("Starting operation: {}", operation);
    let start = std::time::Instant::now();
    
    let result = f();
    
    let duration = start.elapsed();
    info!("Completed operation: {} in {:?}", operation, duration);
    
    result
}

#[macro_export]
macro_rules! debug_block_processing {
    ($block:expr, $operation:expr) => {
        debug!(
            block_number = $block.number,
            tx_count = $block.transaction_traces.len(),
            operation = $operation,
            "Processing block"
        );
    };
}

// Usage in tests
#[cfg(test)]
mod debug_tests {
    use super::*;
    
    #[test]
    fn test_with_debug_tracing() {
        setup_debug_logging();
        
        let result = trace_substreams_execution("map_transfers_test", || {
            let block = load_test_block(17000000);
            debug_block_processing!(block, "map_transfers");
            
            map_transfers(block)
        });
        
        assert!(result.is_ok());
        info!("Test completed successfully");
    }
}
```

### FireCore Integration Checklist

When testing with FireCore tools:

**✅ Data Access**
- [ ] Firehose connection configured correctly
- [ ] API authentication working
- [ ] Test data downloaded and accessible
- [ ] Block range validation working

**✅ Performance**
- [ ] Development vs production mode comparison
- [ ] Parallel execution scaling tested
- [ ] Memory usage profiled
- [ ] Throughput benchmarks established

**✅ Reliability**
- [ ] Cursor-based resumption tested
- [ ] Reorg handling validated
- [ ] Error recovery mechanisms verified
- [ ] Long-running stability tested

**✅ Integration**
- [ ] Local Firehose setup working
- [ ] StreamingFast API integration tested
- [ ] End-to-end workflows validated
- [ ] Monitoring and alerting configured

## Best Practices for FireCore Testing

### DO

✅ **Use real blockchain data** - Firehose provides authentic test data  
✅ **Test with both local and cloud** - Local for development, cloud for integration  
✅ **Validate cursor management** - Test resumption and rollback scenarios  
✅ **Monitor resource usage** - Track memory, CPU, and network utilization  
✅ **Test reorg scenarios** - Blockchain reorganizations are common  
✅ **Automate data fetching** - Script test data downloads  
✅ **Profile with metrics** - Use FireCore's built-in performance monitoring  
✅ **Test at scale** - Large block ranges reveal different issues  

### DON'T

❌ **Skip authentication testing** - API keys and permissions matter  
❌ **Only test happy path** - Network failures and timeouts occur  
❌ **Ignore cursor state** - State management is critical for correctness  
❌ **Test only recent blocks** - Historical blocks have different characteristics  
❌ **Skip performance regression testing** - FireCore optimizations can regress  
❌ **Hardcode block numbers** - Use configurable test ranges  
❌ **Ignore reorg depth** - Test various reorganization scenarios  
❌ **Skip integration with monitoring** - Production debugging requires observability  

FireCore tools provide powerful capabilities for comprehensive Substreams testing. Use them to ensure your applications work correctly with real blockchain data at production scale.