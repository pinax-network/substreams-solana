# Integration Testing Guide

Comprehensive guide for integration testing of Substreams modules with real blockchain data and end-to-end workflows.

## Integration Testing Philosophy

Integration tests should:
- **Use real blockchain data** - Test with actual Ethereum/other chain blocks
- **Test module interactions** - How modules work together in pipelines
- **Verify data flow** - Inputs and outputs between modules
- **Test with production-like data volumes** - Not just single blocks
- **Validate against known ground truth** - Use blocks with known expected outcomes

## Setting Up Integration Tests

### Test Environment Configuration

```rust
// tests/integration_tests.rs
// Note: There is no substreams::test_utils module. Use standard Rust test infrastructure.
use your_substreams::*;
use std::collections::HashMap;

// Test configuration
const TEST_NETWORK: &str = "mainnet";
const TEST_START_BLOCK: u64 = 17000000;
const TEST_BLOCK_RANGE: u64 = 100;

#[tokio::test]
async fn test_integration_environment_setup() {
    // Verify test data availability
    assert!(test_data_available(TEST_START_BLOCK, TEST_START_BLOCK + TEST_BLOCK_RANGE));
    
    // Verify network connectivity
    assert!(can_connect_to_network(TEST_NETWORK).await);
    
    println!("✅ Integration test environment ready");
}

async fn can_connect_to_network(network: &str) -> bool {
    // This would typically check if you can fetch blocks from your data source
    match std::env::var("SUBSTREAMS_API_KEY") {
        Ok(_) => true,
        Err(_) => {
            println!("⚠️  SUBSTREAMS_API_KEY not set, some integration tests may fail");
            false
        }
    }
}

fn test_data_available(start: u64, end: u64) -> bool {
    // Check if test fixture files exist
    for block_num in start..=end {
        let fixture_path = format!("fixtures/blocks/block_{}.json", block_num);
        if !std::path::Path::new(&fixture_path).exists() {
            println!("⚠️  Missing test fixture: {}", fixture_path);
            return false;
        }
    }
    true
}
```

### Test Data Management

```rust
// tests/test_data.rs
use serde::{Deserialize, Serialize};
use std::fs;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct TestBlockData {
    pub block_number: u64,
    pub block_hash: String,
    pub timestamp: u64,
    pub expected_transfers: Vec<ExpectedTransfer>,
    pub expected_swaps: Vec<ExpectedSwap>,
    pub expected_balance_changes: HashMap<String, String>, // address -> balance change
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExpectedTransfer {
    pub contract_address: String,
    pub from_address: String,
    pub to_address: String,
    pub amount: String,
    pub tx_hash: String,
    pub log_index: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExpectedSwap {
    pub pool_address: String,
    pub user_address: String,
    pub token_in: String,
    pub token_out: String,
    pub amount_in: String,
    pub amount_out: String,
}

pub struct TestDataManager {
    test_cases: HashMap<u64, TestBlockData>,
}

impl TestDataManager {
    pub fn new() -> Self {
        let mut manager = Self {
            test_cases: HashMap::new(),
        };
        manager.load_test_cases();
        manager
    }
    
    fn load_test_cases(&mut self) {
        // Load known test cases with expected outcomes
        self.load_test_case(17000000, "fixtures/expected/block_17000000.json");
        self.load_test_case(17000001, "fixtures/expected/block_17000001.json");
        self.load_test_case(17500000, "fixtures/expected/block_17500000.json"); // High activity block
        self.load_test_case(18000000, "fixtures/expected/block_18000000.json"); // Recent block
    }
    
    fn load_test_case(&mut self, block_number: u64, file_path: &str) {
        if let Ok(data) = fs::read_to_string(file_path) {
            if let Ok(test_case) = serde_json::from_str::<TestBlockData>(&data) {
                self.test_cases.insert(block_number, test_case);
            } else {
                println!("⚠️  Failed to parse test case: {}", file_path);
            }
        }
    }
    
    pub fn get_test_case(&self, block_number: u64) -> Option<&TestBlockData> {
        self.test_cases.get(&block_number)
    }
    
    pub fn get_all_test_blocks(&self) -> Vec<u64> {
        self.test_cases.keys().copied().collect()
    }
}
```

## Module Integration Tests

### Single Module Integration

```rust
// tests/module_integration_tests.rs
use your_substreams::*;
use test_data::*;

#[test]
fn test_map_transfers_integration() {
    let test_manager = TestDataManager::new();
    
    // Test against multiple known blocks
    for block_number in test_manager.get_all_test_blocks() {
        let test_case = test_manager.get_test_case(block_number).unwrap();
        
        // Load actual block data
        let block = load_block_from_fixture(block_number);
        
        // Execute the module
        let result = map_transfers(block).expect(&format!("Failed to process block {}", block_number));
        
        // Validate against expected results
        validate_transfer_results(&result, &test_case.expected_transfers, block_number);
    }
}

fn validate_transfer_results(actual: &TransferEvents, expected: &[ExpectedTransfer], block_number: u64) {
    println!("Validating {} transfers for block {}", actual.transfers.len(), block_number);
    
    // Check transfer count
    assert_eq!(
        actual.transfers.len(), 
        expected.len(),
        "Block {}: Expected {} transfers, got {}", 
        block_number, expected.len(), actual.transfers.len()
    );
    
    // Create lookup map for efficient comparison
    let actual_map: HashMap<String, &Transfer> = actual.transfers
        .iter()
        .map(|t| (format!("{}:{}", t.transaction_hash, t.log_index), t))
        .collect();
    
    // Validate each expected transfer
    for expected_transfer in expected {
        let key = format!("{}:{}", expected_transfer.tx_hash, expected_transfer.log_index);
        
        let actual_transfer = actual_map.get(&key)
            .expect(&format!("Block {}: Expected transfer not found: {}", block_number, key));
        
        assert_eq!(actual_transfer.contract_address, expected_transfer.contract_address);
        assert_eq!(actual_transfer.from_address, expected_transfer.from_address);
        assert_eq!(actual_transfer.to_address, expected_transfer.to_address);
        assert_eq!(actual_transfer.amount.to_string(), expected_transfer.amount);
    }
    
    println!("✅ Block {} transfer validation passed", block_number);
}

#[test]
fn test_map_transfers_with_edge_cases() {
    // Test with blocks known to have edge cases
    let edge_case_blocks = vec![
        17000000, // Block with zero-amount transfers
        17500000, // Block with maximum uint256 transfers  
        18000000, // Block with many self-transfers
    ];
    
    for block_number in edge_case_blocks {
        let block = load_block_from_fixture(block_number);
        
        let result = map_transfers(block);
        
        // Should not panic or error on edge cases
        assert!(result.is_ok(), "Block {} should process without error", block_number);
        
        let transfers = result.unwrap();
        
        // Validate edge case handling
        for transfer in &transfers.transfers {
            // All transfers should have valid addresses
            assert!(is_valid_ethereum_address(&transfer.contract_address));
            assert!(is_valid_ethereum_address(&transfer.from_address));
            assert!(is_valid_ethereum_address(&transfer.to_address));
            
            // Amounts should be non-negative (or handle negative if your logic allows)
            assert!(transfer.amount >= num_bigint::BigInt::from(0));
            
            // Transaction hashes should be valid
            assert_eq!(transfer.transaction_hash.len(), 66); // 0x + 64 hex chars
            assert!(transfer.transaction_hash.starts_with("0x"));
        }
        
        println!("✅ Block {} edge case validation passed", block_number);
    }
}
```

### Multi-Module Pipeline Integration

```rust
// tests/pipeline_integration_tests.rs
use your_substreams::*;

#[test] 
fn test_full_pipeline_integration() {
    let test_blocks = vec![17000000, 17000001, 17000002];
    let mut store = create_test_store();
    
    for block_number in test_blocks {
        let block = load_block_from_fixture(block_number);
        
        // Step 1: Extract transfers
        let transfers = map_transfers(block.clone())
            .expect(&format!("Failed to extract transfers from block {}", block_number));
        
        // Step 2: Update balances 
        store_balances(transfers.clone(), &store);
        
        // Step 3: Extract events (depends on transfers)
        let events = map_events(transfers.clone())
            .expect(&format!("Failed to extract events from block {}", block_number));
        
        // Step 4: Update aggregated stats
        store_hourly_stats(events, &store);
        
        println!("✅ Processed block {} through full pipeline", block_number);
    }
    
    // Validate final state
    validate_pipeline_final_state(&store);
}

fn validate_pipeline_final_state(store: &TestStore) {
    // Check that balances are consistent
    let alice = "0x742d35Cc6B8B4d1e8d37a1E5B0b4F8e8B7F4D2a1";
    let bob = "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed";
    let usdc = "0xA0b86a33E6Fe17d67C8c086c6c4c0E3C8E3B7EC2";
    
    // Get final balances
    let alice_balance = store.get_last(&format!("{}:{}", usdc, alice)).unwrap_or(0);
    let bob_balance = store.get_last(&format!("{}:{}", usdc, bob)).unwrap_or(0);
    
    println!("Final balances - Alice: {}, Bob: {}", alice_balance, bob_balance);
    
    // Validate that balances make sense (implement your specific logic)
    assert!(alice_balance >= 0, "Alice balance should not be negative");
    assert!(bob_balance >= 0, "Bob balance should not be negative");
    
    // Check hourly stats
    let hour_key = "2023-04-01T10:00:00";
    let hourly_stats = store.get_proto::<HourlyStats>(&hour_key);
    assert!(hourly_stats.is_some(), "Should have hourly stats");
    
    let stats = hourly_stats.unwrap();
    assert!(stats.transfer_count > 0, "Should have recorded transfers");
    assert!(stats.total_volume > 0, "Should have recorded volume");
}

#[test]
fn test_module_dependency_chain() {
    // Test that modules correctly depend on each other
    let block = load_block_from_fixture(17000000);
    
    // Module A: Extract raw events
    let raw_events = extract_raw_events(block.clone())
        .expect("Failed to extract raw events");
    
    assert!(!raw_events.logs.is_empty(), "Should extract some logs");
    
    // Module B: Parse events (depends on A)
    let parsed_events = parse_events(raw_events.clone())
        .expect("Failed to parse events");
    
    assert!(!parsed_events.transfers.is_empty(), "Should parse some transfers");
    
    // Module C: Enrich events (depends on B)  
    let enriched_events = enrich_events(parsed_events.clone())
        .expect("Failed to enrich events");
    
    assert!(!enriched_events.transfers.is_empty(), "Should enrich transfers");
    
    // Validate enrichment worked
    for transfer in &enriched_events.transfers {
        assert!(transfer.token_symbol.is_some(), "Should have token symbol");
        assert!(transfer.token_decimals.is_some(), "Should have token decimals");
        assert!(transfer.usd_value.is_some(), "Should have USD value");
    }
    
    println!("✅ Module dependency chain validation passed");
}
```

## Store Integration Tests

### Testing Store Behaviors

```rust
// tests/store_integration_tests.rs
use your_substreams::*;

#[test]
fn test_balance_store_consistency() {
    let mut store = create_test_store();
    let alice = "0x742d35Cc6B8B4d1e8d37a1E5B0b4F8e8B7F4D2a1";
    let bob = "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed";
    let usdc = "0xA0b86a33E6Fe17d67C8c086c6c4c0E3C8E3B7EC2";
    
    // Simulate a series of transfers
    let transfers = vec![
        create_test_transfer(usdc, alice, bob, "1000000000", 17000000, 0), // Alice -> Bob: 1000 USDC
        create_test_transfer(usdc, bob, alice, "250000000", 17000001, 0),  // Bob -> Alice: 250 USDC
        create_test_transfer(usdc, alice, bob, "500000000", 17000002, 0),  // Alice -> Bob: 500 USDC
    ];
    
    // Process transfers in order
    for transfer in transfers {
        let transfer_event = TransferEvents {
            transfers: vec![transfer],
        };
        store_balances(transfer_event, &store);
    }
    
    // Check final balances
    let alice_key = format!("{}:{}", usdc, alice);
    let bob_key = format!("{}:{}", usdc, bob);
    
    let alice_balance = store.get_last(&alice_key).expect("Alice should have balance");
    let bob_balance = store.get_last(&bob_key).expect("Bob should have balance");
    
    // Alice: -1000 + 250 - 500 = -1250
    // Bob: +1000 - 250 + 500 = +1250
    assert_eq!(alice_balance, -1250000000i64);
    assert_eq!(bob_balance, 1250000000i64);
    
    // Verify balance conservation
    assert_eq!(alice_balance + bob_balance, 0);
    
    println!("✅ Balance store consistency test passed");
}

#[test]
fn test_store_with_multiple_tokens() {
    let mut store = create_test_store();
    let alice = "0x742d35Cc6B8B4d1e8d37a1E5B0b4F8e8B7F4D2a1";
    let usdc = "0xA0b86a33E6Fe17d67C8c086c6c4c0E3C8E3B7EC2";
    let usdt = "0xdAC17F958D2ee523a2206206994597C13D831ec7";
    let wbtc = "0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599";
    
    // Transfers across multiple tokens
    let transfers = vec![
        create_test_transfer(usdc, alice, "0xBob", "1000000000", 17000000, 0), // 1000 USDC
        create_test_transfer(usdt, alice, "0xBob", "2000000000", 17000000, 1), // 2000 USDT
        create_test_transfer(wbtc, alice, "0xBob", "100000000", 17000000, 2),  // 1 WBTC (8 decimals)
    ];
    
    let transfer_event = TransferEvents { transfers };
    store_balances(transfer_event, &store);
    
    // Verify separate token balances
    assert_eq!(
        store.get_last(&format!("{}:{}", usdc, alice)).unwrap(),
        -1000000000i64
    );
    assert_eq!(
        store.get_last(&format!("{}:{}", usdt, alice)).unwrap(), 
        -2000000000i64
    );
    assert_eq!(
        store.get_last(&format!("{}:{}", wbtc, alice)).unwrap(),
        -100000000i64
    );
    
    println!("✅ Multi-token store test passed");
}

#[test]
fn test_store_aggregation_patterns() {
    let mut store = create_test_store();
    
    // Simulate multiple blocks of transfers
    for block_num in 17000000..17000010 {
        let block = load_block_from_fixture(block_num);
        let transfers = map_transfers(block).unwrap();
        
        // Update individual balances
        store_balances(transfers.clone(), &store);
        
        // Update aggregate statistics
        store_hourly_volume(transfers.clone(), &store);
        store_daily_stats(transfers, &store);
    }
    
    // Verify aggregations
    let hour_key = "USDC:2023-04-01:10";
    let hourly_volume = store.get_last(hour_key).unwrap_or(0);
    assert!(hourly_volume > 0, "Should have recorded hourly volume");
    
    let daily_key = "USDC:2023-04-01";
    let daily_stats = store.get_proto::<DailyStats>(daily_key);
    assert!(daily_stats.is_some(), "Should have daily stats");
    
    let stats = daily_stats.unwrap();
    assert!(stats.total_volume > 0, "Daily volume should be positive");
    assert!(stats.unique_users > 0, "Should have unique users");
    assert!(stats.transfer_count > 0, "Should have transfer count");
    
    println!("✅ Store aggregation test passed");
}

fn create_test_transfer(
    contract: &str,
    from: &str, 
    to: &str,
    amount: &str,
    block_number: u64,
    log_index: u32
) -> Transfer {
    Transfer {
        contract_address: contract.to_string(),
        from_address: from.to_string(),
        to_address: to.to_string(), 
        amount: num_bigint::BigInt::from_str(amount).unwrap(),
        block_number,
        log_index,
        transaction_hash: format!("0x{:064x}", log_index),
    }
}
```

## Real Data Integration Tests  

### Testing with Live Blockchain Data

```rust
// tests/live_data_integration_tests.rs
use std::process::Command;
use serde_json::Value;

#[test]
#[ignore] // Ignore by default since it requires network access
fn test_with_live_data_small_range() {
    let output = Command::new("substreams")
        .args(&[
            "run",
            "-s", "17000000",
            "-t", "+10", // Small range for quick test
            "map_transfers",
            "--network", "mainnet"
        ])
        .output()
        .expect("Failed to execute substreams command");
    
    assert!(output.status.success(), 
           "Substreams execution failed: {}", 
           String::from_utf8_lossy(&output.stderr));
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Parse and validate output
    let mut block_count = 0;
    let mut total_transfers = 0;
    
    for line in stdout.lines() {
        if let Ok(json) = serde_json::from_str::<Value>(line) {
            if let Some(block_num) = json.get("block_number") {
                block_count += 1;
                
                if let Some(transfers) = json.get("transfers") {
                    if let Some(transfer_array) = transfers.as_array() {
                        total_transfers += transfer_array.len();
                        
                        // Validate each transfer
                        for transfer in transfer_array {
                            validate_transfer_json(transfer, block_num.as_u64().unwrap());
                        }
                    }
                }
            }
        }
    }
    
    assert_eq!(block_count, 10, "Should process exactly 10 blocks");
    assert!(total_transfers > 0, "Should find some transfers in 10 blocks");
    
    println!("✅ Processed {} blocks with {} total transfers", block_count, total_transfers);
}

fn validate_transfer_json(transfer: &Value, block_number: u64) {
    // Validate transfer JSON structure
    assert!(transfer.get("contract_address").is_some(), 
           "Block {}: Transfer missing contract_address", block_number);
    assert!(transfer.get("from_address").is_some(),
           "Block {}: Transfer missing from_address", block_number);
    assert!(transfer.get("to_address").is_some(),
           "Block {}: Transfer missing to_address", block_number);
    assert!(transfer.get("amount").is_some(),
           "Block {}: Transfer missing amount", block_number);
    
    // Validate address formats
    if let Some(contract) = transfer.get("contract_address").and_then(|v| v.as_str()) {
        assert!(is_valid_ethereum_address(contract),
               "Block {}: Invalid contract address: {}", block_number, contract);
    }
    
    // Validate amount is a valid number string
    if let Some(amount_str) = transfer.get("amount").and_then(|v| v.as_str()) {
        assert!(amount_str.parse::<num_bigint::BigInt>().is_ok(),
               "Block {}: Invalid amount: {}", block_number, amount_str);
    }
}

#[test]
#[ignore] // Heavy test, run manually
fn test_performance_with_live_data() {
    use std::time::Instant;
    
    let start = Instant::now();
    
    let output = Command::new("substreams")
        .args(&[
            "run",
            "-s", "17000000", 
            "-t", "+1000", // 1000 blocks
            "map_transfers",
            "--network", "mainnet",
            "--production-mode" // Enable parallel processing
        ])
        .output()
        .expect("Failed to execute substreams");
    
    let duration = start.elapsed();
    
    assert!(output.status.success(),
           "Performance test failed: {}", 
           String::from_utf8_lossy(&output.stderr));
    
    // Performance assertions
    assert!(duration.as_secs() < 120, // Should complete in under 2 minutes
           "Processing took too long: {:?}", duration);
    
    // Validate output quality
    let stdout = String::from_utf8_lossy(&output.stdout);
    let line_count = stdout.lines().count();
    
    assert!(line_count >= 1000, // Should have output for each block
           "Expected at least 1000 lines of output, got {}", line_count);
    
    println!("✅ Performance test: {} blocks in {:?}", 1000, duration);
    println!("   Average: {:.2} ms/block", duration.as_millis() as f64 / 1000.0);
}
```

### Testing Against Historical Ground Truth

```rust
// tests/historical_validation_tests.rs
use your_substreams::*;

#[test]
fn test_against_known_historical_events() {
    // Test against well-known historical events
    
    // USDC deployment block
    test_usdc_deployment();
    
    // Large known transfers
    test_known_large_transfers();
    
    // Specific DEX interactions
    test_known_uniswap_swaps();
}

fn test_usdc_deployment() {
    // USDC was deployed at block 6082465
    let deployment_block = 6082465;
    let block = load_block_from_fixture_or_fetch(deployment_block);
    
    let transfers = map_transfers(block).unwrap();
    
    // USDC deployment should have initial mint
    let usdc_transfers: Vec<_> = transfers.transfers
        .iter()
        .filter(|t| t.contract_address.to_lowercase() == "0xa0b86a33e6fe17d67c8c086c6c4c0e3c8e3b7ec2")
        .collect();
    
    assert!(!usdc_transfers.is_empty(), "Should find USDC transfers in deployment block");
    
    // Find the initial mint (from zero address)
    let initial_mint = usdc_transfers
        .iter()
        .find(|t| t.from_address == "0x0000000000000000000000000000000000000000");
    
    assert!(initial_mint.is_some(), "Should find initial USDC mint from zero address");
    
    println!("✅ USDC deployment validation passed");
}

fn test_known_large_transfers() {
    // Test against known large transfers (these would be research beforehand)
    let test_cases = vec![
        // (block_number, tx_hash, expected_transfer_count, expected_large_transfer_amount)
        (17000000, "0x123...", 5, "1000000000000"), // 1M USDC
        (17500000, "0x456...", 3, "5000000000000000000000"), // 5K ETH worth
    ];
    
    for (block_number, tx_hash, expected_count, expected_amount) in test_cases {
        let block = load_block_from_fixture_or_fetch(block_number);
        let transfers = map_transfers(block).unwrap();
        
        // Find transfers in specific transaction
        let tx_transfers: Vec<_> = transfers.transfers
            .iter()
            .filter(|t| t.transaction_hash == tx_hash)
            .collect();
        
        assert_eq!(tx_transfers.len(), expected_count,
                  "Block {} tx {}: Expected {} transfers", 
                  block_number, tx_hash, expected_count);
        
        // Find the large transfer
        let large_transfer = tx_transfers
            .iter()
            .find(|t| t.amount >= num_bigint::BigInt::from_str(expected_amount).unwrap());
        
        assert!(large_transfer.is_some(),
               "Block {} tx {}: Should find large transfer >= {}", 
               block_number, tx_hash, expected_amount);
    }
    
    println!("✅ Known large transfer validation passed");
}

fn test_known_uniswap_swaps() {
    // Test against known Uniswap V2/V3 swaps
    let swap_block = 17000000; // Block with known Uniswap activity
    let block = load_block_from_fixture_or_fetch(swap_block);
    
    let swaps = map_uniswap_swaps(block).unwrap();
    
    assert!(!swaps.swaps.is_empty(), "Should find Uniswap swaps");
    
    for swap in &swaps.swaps {
        // Validate swap structure
        assert!(is_valid_ethereum_address(&swap.pool_address));
        assert!(is_valid_ethereum_address(&swap.user_address));
        assert!(swap.amount_in > num_bigint::BigInt::from(0) || swap.amount_out > num_bigint::BigInt::from(0));
        
        // Validate that it's a real Uniswap pool (would require pool registry)
        // assert!(is_known_uniswap_pool(&swap.pool_address));
    }
    
    println!("✅ Uniswap swap validation passed");
}

fn load_block_from_fixture_or_fetch(block_number: u64) -> Block {
    // Try to load from fixture first, then fetch if not available
    let fixture_path = format!("fixtures/blocks/block_{}.json", block_number);
    
    if std::path::Path::new(&fixture_path).exists() {
        let data = std::fs::read_to_string(fixture_path).unwrap();
        serde_json::from_str(&data).unwrap()
    } else {
        // In a real implementation, you'd fetch from blockchain RPC
        // For tests, you might want to fail instead
        panic!("Block fixture not available: {}", block_number);
    }
}
```

## Performance Integration Tests

### Benchmarking Complete Workflows

```rust
// tests/performance_integration_tests.rs
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use your_substreams::*;

fn benchmark_module_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("module_performance");
    
    // Test with different block types
    let test_blocks = vec![
        (17000000, "normal"), // Normal activity block  
        (17500000, "heavy"),  // Heavy activity block
        (18000000, "light"),  // Light activity block
    ];
    
    for (block_number, block_type) in test_blocks {
        let block = load_block_from_fixture(block_number);
        
        group.bench_with_input(
            BenchmarkId::new("map_transfers", block_type),
            &block,
            |b, block| {
                b.iter(|| map_transfers(block.clone()))
            }
        );
    }
    
    group.finish();
}

fn benchmark_pipeline_performance(c: &mut Criterion) {
    let blocks = vec![
        load_block_from_fixture(17000000),
        load_block_from_fixture(17000001),
        load_block_from_fixture(17000002),
    ];
    
    c.bench_function("full_pipeline", |b| {
        b.iter(|| {
            let mut store = create_test_store();
            
            for block in &blocks {
                let transfers = map_transfers(block.clone()).unwrap();
                store_balances(transfers.clone(), &store);
                let events = map_events(transfers).unwrap();
                store_hourly_stats(events, &store);
            }
            
            store
        })
    });
}

criterion_group!(benches, benchmark_module_performance, benchmark_pipeline_performance);
criterion_main!(benches);
```

## Test Automation and CI/CD

### GitHub Actions Integration Tests

```yaml
# .github/workflows/integration-tests.yml
name: Integration Tests

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]
  schedule:
    - cron: '0 6 * * *'  # Daily at 6 AM UTC

jobs:
  integration-tests:
    runs-on: ubuntu-latest
    timeout-minutes: 30
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
        
    - name: Install Substreams CLI
      run: |
        curl -sSL https://github.com/streamingfast/substreams/releases/download/v1.1.0/substreams_linux_x86_64.tar.gz | tar -xz
        sudo mv substreams /usr/local/bin/
        
    - name: Cache test fixtures
      uses: actions/cache@v3
      with:
        path: fixtures/
        key: ${{ runner.os }}-fixtures-${{ hashFiles('fixtures/**') }}
        
    - name: Download test fixtures
      run: |
        if [ ! -d "fixtures/blocks" ]; then
          mkdir -p fixtures
          curl -L https://github.com/your-org/substreams-fixtures/releases/download/v1.0/ethereum-blocks.tar.gz | tar -xz -C fixtures/
        fi
        
    - name: Build Substreams
      run: substreams build
      
    - name: Run integration tests
      run: cargo test --test integration_tests --release
      env:
        RUST_LOG: info
        
    - name: Run live data tests (if API key available)
      run: cargo test --test live_data_integration_tests --release -- --ignored
      env:
        SUBSTREAMS_API_KEY: ${{ secrets.SUBSTREAMS_API_KEY }}
      continue-on-error: true  # Don't fail CI if live tests fail
      
    - name: Performance regression test
      run: |
        cargo test --test performance_integration_tests --release
        # Store performance results for tracking
        
  nightly-full-integration:
    runs-on: ubuntu-latest
    if: github.event_name == 'schedule'
    timeout-minutes: 120
    
    steps:
    - uses: actions/checkout@v3
    # ... setup steps ...
    
    - name: Run comprehensive integration tests
      run: |
        # Run tests with larger block ranges
        cargo test test_performance_with_live_data --release -- --ignored --nocapture
        
    - name: Historical validation
      run: |
        cargo test test_against_known_historical_events --release -- --ignored --nocapture
```

### Test Data Management Scripts

```bash
#!/bin/bash
# scripts/update-test-fixtures.sh

set -e

echo "Updating test fixtures..."

# Blocks with known interesting activity
BLOCKS=(
    17000000  # Normal activity
    17500000  # High activity  
    18000000  # Recent block
    6082465   # USDC deployment
    12369621  # Uniswap V3 deployment
)

for block in "${BLOCKS[@]}"; do
    echo "Fetching block $block..."
    
    # Fetch block data (this would use your preferred method)
    substreams run -s $block -t +1 extract_block_data --network mainnet \
        > "fixtures/blocks/block_${block}.json"
    
    # Generate expected outputs for validation
    substreams run -s $block -t +1 map_transfers --network mainnet \
        > "fixtures/expected/transfers_${block}.json"
done

echo "✅ Test fixtures updated"
```

## Best Practices for Integration Testing

### DO

✅ **Use real blockchain data** - Synthetic data misses edge cases  
✅ **Test with multiple block ranges** - Different activity levels  
✅ **Validate against ground truth** - Known historical events  
✅ **Test module interactions** - Not just individual modules  
✅ **Include performance tests** - Measure real-world performance  
✅ **Automate with CI/CD** - Catch regressions early  
✅ **Cache test data** - Speed up test runs  
✅ **Test error scenarios** - Network failures, malformed data  
✅ **Monitor resource usage** - Memory, CPU, disk  
✅ **Test with production configuration** - Same settings as deployment  

### DON'T

❌ **Only test happy path** - Edge cases are everywhere in blockchain data  
❌ **Skip performance validation** - Integration tests catch performance regressions  
❌ **Test only single blocks** - Multi-block scenarios reveal state issues  
❌ **Ignore network dependencies** - Test what happens when APIs fail  
❌ **Use only recent data** - Historical edge cases matter  
❌ **Hardcode expected results** - Use data-driven test cases  
❌ **Skip resource monitoring** - Memory leaks show up in integration tests  
❌ **Test only mainnet** - Different networks have different characteristics  

### Test Strategy

1. **Fixture-based tests** - Fast, reliable, repeatable
2. **Live data tests** - Catch issues with current blockchain state  
3. **Historical validation** - Test against known events
4. **Performance benchmarks** - Ensure scalability
5. **Error scenario tests** - Network issues, malformed data
6. **Multi-chain tests** - If supporting multiple blockchains

This comprehensive integration testing approach ensures your Substreams work correctly with real blockchain data and can handle production workloads reliably.