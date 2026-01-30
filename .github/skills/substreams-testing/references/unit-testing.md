# Unit Testing Guide

Comprehensive guide for unit testing Substreams modules and functions.

## Testing Philosophy

Unit tests should be:
- **Fast** - Run in milliseconds, not seconds
- **Isolated** - Test one unit of functionality
- **Deterministic** - Same input always produces same output
- **Self-contained** - No external dependencies
- **Comprehensive** - Cover edge cases and error conditions

## Setting Up Unit Tests

### Rust Test Configuration

```toml
# Cargo.toml
[dev-dependencies]
# Core testing
assert_matches = "1.5"
proptest = "1.0"
quickcheck = "1.0"
quickcheck_macros = "1.0"

# Mocking and fixtures
mockall = "0.11"
hex = "0.4"

# Performance testing
criterion = "0.4"

[[bench]]
name = "module_benchmarks"
harness = false

[features]
default = []
test-utils = []  # Enable test utilities
```

### Test Module Structure

```rust
// src/lib.rs
#[cfg(test)]
mod tests {
    use super::*;
    
    mod unit {
        use super::*;
        mod parsing_tests;
        mod calculation_tests;
        mod validation_tests;
    }
    
    mod integration {
        use super::*;
        mod module_tests;
        mod store_tests;
    }
    
    mod fixtures;
    mod test_utils;
}
```

## Testing Core Functions

### Data Parsing Tests

```rust
// src/tests/unit/parsing_tests.rs
use crate::*;
use hex;

#[test]
fn test_parse_erc20_transfer_valid() {
    // Arrange - Valid ERC20 Transfer event
    let log = create_transfer_log(
        "0xA0b86a33E6Fe17d67C8c086c6c4c0E3C8E3B7EC2", // USDC contract
        "0x742d35Cc6B8B4d1e8d37a1E5B0b4F8e8B7F4D2a1", // From
        "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed", // To
        "1000000000"  // 1000 USDC (6 decimals)
    );
    
    // Act
    let result = parse_erc20_transfer(&log);
    
    // Assert
    assert!(result.is_ok(), "Should parse valid transfer log");
    let transfer = result.unwrap();
    
    assert_eq!(transfer.contract_address, "0xA0b86a33E6Fe17d67C8c086c6c4c0E3C8E3B7EC2");
    assert_eq!(transfer.from_address, "0x742d35Cc6B8B4d1e8d37a1E5B0b4F8e8B7F4D2a1");
    assert_eq!(transfer.to_address, "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed");
    assert_eq!(transfer.amount, BigInt::from_str("1000000000").unwrap());
}

#[test]
fn test_parse_erc20_transfer_zero_amount() {
    let log = create_transfer_log(
        "0xA0b86a33E6Fe17d67C8c086c6c4c0E3C8E3B7EC2",
        "0x742d35Cc6B8B4d1e8d37a1E5B0b4F8e8B7F4D2a1",
        "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed",
        "0"  // Zero amount
    );
    
    let result = parse_erc20_transfer(&log);
    
    assert!(result.is_ok(), "Zero amount transfers are valid");
    assert_eq!(result.unwrap().amount, BigInt::zero());
}

#[test]
fn test_parse_erc20_transfer_max_amount() {
    let max_uint256 = "115792089237316195423570985008687907853269984665640564039457584007913129639935";
    
    let log = create_transfer_log(
        "0xA0b86a33E6Fe17d67C8c086c6c4c0E3C8E3B7EC2",
        "0x742d35Cc6B8B4d1e8d37a1E5B0b4F8e8B7F4D2a1",
        "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed",
        max_uint256
    );
    
    let result = parse_erc20_transfer(&log);
    
    assert!(result.is_ok(), "Should handle maximum uint256 values");
    assert_eq!(result.unwrap().amount, BigInt::from_str(max_uint256).unwrap());
}

#[test]
fn test_parse_erc20_transfer_insufficient_topics() {
    let mut log = create_valid_log();
    log.topics = vec![
        hex::decode("ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef").unwrap()
        // Missing 'from' and 'to' topics
    ];
    
    let result = parse_erc20_transfer(&log);
    
    assert!(result.is_err(), "Should fail with insufficient topics");
    
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("insufficient topics") || error_msg.contains("topic"));
}

#[test]
fn test_parse_erc20_transfer_invalid_signature() {
    let mut log = create_transfer_log(
        "0xA0b86a33E6Fe17d67C8c086c6c4c0E3C8E3B7EC2",
        "0x742d35Cc6B8B4d1e8d37a1E5B0b4F8e8B7F4D2a1",
        "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed",
        "1000000000"
    );
    
    // Corrupt the event signature (first topic)
    log.topics[0] = hex::decode("1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef").unwrap();
    
    let result = parse_erc20_transfer(&log);
    
    assert!(result.is_err(), "Should fail with invalid event signature");
}

#[test]
fn test_parse_erc20_transfer_malformed_data() {
    let mut log = create_transfer_log(
        "0xA0b86a33E6Fe17d67C8c086c6c4c0E3C8E3B7EC2",
        "0x742d35Cc6B8B4d1e8d37a1E5B0b4F8e8B7F4D2a1",
        "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed",
        "1000000000"
    );
    
    // Corrupt the data field (amount)
    log.data = vec![0x12, 0x34]; // Too short
    
    let result = parse_erc20_transfer(&log);
    
    assert!(result.is_err(), "Should fail with malformed data");
}

// Test utility functions
fn create_transfer_log(contract: &str, from: &str, to: &str, amount: &str) -> Log {
    Log {
        address: hex::decode(&contract[2..]).unwrap(),
        topics: vec![
            // Transfer(address,address,uint256) signature  
            hex::decode("ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef").unwrap(),
            // From address (padded to 32 bytes)
            hex::decode(&format!("000000000000000000000000{}", &from[2..])).unwrap(),
            // To address (padded to 32 bytes) 
            hex::decode(&format!("000000000000000000000000{}", &to[2..])).unwrap(),
        ],
        // Amount as 32-byte big-endian integer
        data: {
            let amount_big = BigInt::from_str(amount).unwrap();
            let mut bytes = vec![0u8; 32];
            let amount_bytes = amount_big.to_bytes_be().1;
            let start = 32 - amount_bytes.len();
            bytes[start..].copy_from_slice(&amount_bytes);
            bytes
        },
        ..Default::default()
    }
}

fn create_valid_log() -> Log {
    create_transfer_log(
        "0xA0b86a33E6Fe17d67C8c086c6c4c0E3C8E3B7EC2",
        "0x742d35Cc6B8B4d1e8d37a1E5B0b4F8e8B7F4D2a1",
        "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed",
        "1000000000"
    )
}
```

### Calculation Tests

```rust
// src/tests/unit/calculation_tests.rs
use crate::*;
use num_bigint::BigInt;
use std::str::FromStr;

#[test]
fn test_calculate_balance_single_transfer() {
    let transfers = vec![
        create_transfer("alice", "bob", "1000000000") // 1000 USDC
    ];
    
    let balances = calculate_balances(&transfers);
    
    // Alice should have -1000, Bob should have +1000
    assert_eq!(balances.get("alice"), Some(&BigInt::from_str("-1000000000").unwrap()));
    assert_eq!(balances.get("bob"), Some(&BigInt::from_str("1000000000").unwrap()));
}

#[test]
fn test_calculate_balance_multiple_transfers() {
    let transfers = vec![
        create_transfer("alice", "bob", "1000000000"),   // Alice -1000, Bob +1000
        create_transfer("bob", "charlie", "500000000"),   // Bob -500, Charlie +500
        create_transfer("charlie", "alice", "200000000"), // Charlie -200, Alice +200
    ];
    
    let balances = calculate_balances(&transfers);
    
    // Net balances: Alice = -1000 + 200 = -800, Bob = 1000 - 500 = 500, Charlie = 500 - 200 = 300
    assert_eq!(balances.get("alice"), Some(&BigInt::from_str("-800000000").unwrap()));
    assert_eq!(balances.get("bob"), Some(&BigInt::from_str("500000000").unwrap()));
    assert_eq!(balances.get("charlie"), Some(&BigInt::from_str("300000000").unwrap()));
}

#[test]
fn test_calculate_balance_zero_sum() {
    let transfers = vec![
        create_transfer("alice", "bob", "1000000000"),
        create_transfer("bob", "alice", "1000000000"), // Exact reversal
    ];
    
    let balances = calculate_balances(&transfers);
    
    // Both should have zero net balance
    assert_eq!(balances.get("alice"), Some(&BigInt::zero()));
    assert_eq!(balances.get("bob"), Some(&BigInt::zero()));
}

#[test]
fn test_calculate_price_impact() {
    // Test Uniswap price impact calculation
    let reserve0 = BigInt::from_str("1000000000000").unwrap(); // 1M USDC
    let reserve1 = BigInt::from_str("500000000000000000000").unwrap(); // 500 ETH
    let amount_in = BigInt::from_str("10000000000").unwrap(); // 10K USDC
    
    let price_impact = calculate_price_impact(&reserve0, &reserve1, &amount_in, true);
    
    // Price impact should be positive (price goes up when buying ETH with USDC)
    assert!(price_impact > 0.0);
    assert!(price_impact < 0.1); // Should be less than 10% for reasonable trade sizes
    
    println!("Price impact: {:.4}%", price_impact * 100.0);
}

#[test]
fn test_calculate_price_impact_zero_reserves() {
    let reserve0 = BigInt::zero();
    let reserve1 = BigInt::from_str("1000000000000000000000").unwrap();
    let amount_in = BigInt::from_str("1000000000").unwrap();
    
    let price_impact = calculate_price_impact(&reserve0, &reserve1, &amount_in, true);
    
    // Should handle zero reserves gracefully (return infinite or error)
    assert!(price_impact.is_infinite() || price_impact.is_nan());
}

#[test]
fn test_format_token_amount() {
    struct TestCase {
        amount: &'static str,
        decimals: u8,
        expected: &'static str,
    }
    
    let test_cases = vec![
        TestCase { amount: "1000000000000000000", decimals: 18, expected: "1.0" },
        TestCase { amount: "1500000000000000000", decimals: 18, expected: "1.5" },
        TestCase { amount: "1000000", decimals: 6, expected: "1.0" },
        TestCase { amount: "1500000", decimals: 6, expected: "1.5" },
        TestCase { amount: "0", decimals: 18, expected: "0.0" },
        TestCase { amount: "1", decimals: 18, expected: "0.000000000000000001" },
        TestCase { amount: "999999999999999999", decimals: 18, expected: "0.999999999999999999" },
    ];
    
    for case in test_cases {
        let amount = BigInt::from_str(case.amount).unwrap();
        let formatted = format_token_amount(&amount, case.decimals);
        
        assert_eq!(formatted, case.expected, 
                  "Failed for amount {} with {} decimals", case.amount, case.decimals);
    }
}

#[test]
fn test_validate_ethereum_address() {
    let valid_addresses = vec![
        "0xA0b86a33E6Fe17d67C8c086c6c4c0E3C8E3B7EC2",
        "0x742d35Cc6B8B4d1e8d37a1E5B0b4F8e8B7F4D2a1",
        "0x0000000000000000000000000000000000000000", // Zero address
        "0xFFfFfFffFFfffFFfFFfFFFFFffFFFffffFfFFFfF", // Max address
    ];
    
    for address in valid_addresses {
        assert!(is_valid_ethereum_address(address), 
               "Address {} should be valid", address);
    }
    
    let invalid_addresses = vec![
        "0x123", // Too short
        "0xA0b86a33E6Fe17d67C8c086c6c4c0E3C8E3B7EC", // Too short by 1 char
        "0xA0b86a33E6Fe17d67C8c086c6c4c0E3C8E3B7EC22", // Too long by 1 char
        "A0b86a33E6Fe17d67C8c086c6c4c0E3C8E3B7EC2", // Missing 0x prefix
        "0xA0b86a33E6Fe17d67C8c086c6c4c0E3C8E3B7EGZ", // Invalid hex chars
        "", // Empty string
        "0x", // Only prefix
    ];
    
    for address in invalid_addresses {
        assert!(!is_valid_ethereum_address(address), 
               "Address {} should be invalid", address);
    }
}

// Test helper functions
fn create_transfer(from: &str, to: &str, amount: &str) -> Transfer {
    Transfer {
        contract_address: "0xA0b86a33E6Fe17d67C8c086c6c4c0E3C8E3B7EC2".to_string(),
        from_address: from.to_string(),
        to_address: to.to_string(),
        amount: BigInt::from_str(amount).unwrap(),
        block_number: 17000000,
        log_index: 0,
        transaction_hash: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string(),
    }
}
```

### Validation Tests

```rust
// src/tests/unit/validation_tests.rs
use crate::*;
use assert_matches::assert_matches;

#[test]
fn test_validate_transfer_valid() {
    let transfer = Transfer {
        contract_address: "0xA0b86a33E6Fe17d67C8c086c6c4c0E3C8E3B7EC2".to_string(),
        from_address: "0x742d35Cc6B8B4d1e8d37a1E5B0b4F8e8B7F4D2a1".to_string(),
        to_address: "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed".to_string(),
        amount: BigInt::from_str("1000000000").unwrap(),
        block_number: 17000000,
        log_index: 0,
        transaction_hash: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string(),
    };
    
    let result = validate_transfer(&transfer);
    assert!(result.is_ok(), "Valid transfer should pass validation");
}

#[test]
fn test_validate_transfer_self_transfer() {
    let mut transfer = create_valid_transfer();
    transfer.to_address = transfer.from_address.clone(); // Self-transfer
    
    let result = validate_transfer(&transfer);
    
    // Depending on your business logic, self-transfers might be valid or invalid
    // This test documents your decision
    assert!(result.is_ok(), "Self-transfers are allowed in ERC20");
    // OR: assert!(result.is_err(), "Self-transfers should be rejected");
}

#[test]
fn test_validate_transfer_negative_amount() {
    let mut transfer = create_valid_transfer();
    transfer.amount = BigInt::from_str("-1000000000").unwrap();
    
    let result = validate_transfer(&transfer);
    assert!(result.is_err(), "Negative amounts should be invalid");
    
    let error = result.unwrap_err();
    assert!(error.to_string().contains("negative") || error.to_string().contains("amount"));
}

#[test]
fn test_validate_transfer_invalid_addresses() {
    let invalid_addresses = vec![
        "",
        "0x123",
        "not_an_address",
        "0xGHIJKL", // Invalid hex
    ];
    
    for invalid_addr in invalid_addresses {
        let mut transfer = create_valid_transfer();
        transfer.from_address = invalid_addr.to_string();
        
        let result = validate_transfer(&transfer);
        assert!(result.is_err(), "Invalid from_address '{}' should fail validation", invalid_addr);
        
        transfer = create_valid_transfer();
        transfer.to_address = invalid_addr.to_string();
        
        let result = validate_transfer(&transfer);
        assert!(result.is_err(), "Invalid to_address '{}' should fail validation", invalid_addr);
    }
}

#[test]
fn test_validate_block_range() {
    struct TestCase {
        start: u64,
        end: u64,
        expected_valid: bool,
    }
    
    let test_cases = vec![
        TestCase { start: 17000000, end: 17000100, expected_valid: true },
        TestCase { start: 17000100, end: 17000000, expected_valid: false }, // End before start
        TestCase { start: 17000000, end: 17000000, expected_valid: false }, // Same block
        TestCase { start: 0, end: 1, expected_valid: true },
        TestCase { start: 17000000, end: 18000000, expected_valid: true }, // Large range (might warn but valid)
    ];
    
    for case in test_cases {
        let result = validate_block_range(case.start, case.end);
        
        if case.expected_valid {
            assert!(result.is_ok(), "Block range {}..{} should be valid", case.start, case.end);
        } else {
            assert!(result.is_err(), "Block range {}..{} should be invalid", case.start, case.end);
        }
    }
}

#[test]
fn test_sanitize_input_data() {
    struct TestCase {
        input: &'static str,
        expected: &'static str,
    }
    
    let test_cases = vec![
        TestCase { input: "0xA0b86a33E6Fe17d67C8c086c6c4c0E3C8E3B7EC2", expected: "0xa0b86a33e6fe17d67c8c086c6c4c0e3c8e3b7ec2" },
        TestCase { input: "A0b86a33E6Fe17d67C8c086c6c4c0E3C8E3B7EC2", expected: "0xa0b86a33e6fe17d67c8c086c6c4c0e3c8e3b7ec2" },
        TestCase { input: "  0xA0b86a33E6Fe17d67C8c086c6c4c0E3C8E3B7EC2  ", expected: "0xa0b86a33e6fe17d67c8c086c6c4c0e3c8e3b7ec2" },
        TestCase { input: "", expected: "" },
    ];
    
    for case in test_cases {
        let result = sanitize_ethereum_address(case.input);
        assert_eq!(result, case.expected, 
                  "Input '{}' should sanitize to '{}'", case.input, case.expected);
    }
}

fn create_valid_transfer() -> Transfer {
    Transfer {
        contract_address: "0xA0b86a33E6Fe17d67C8c086c6c4c0E3C8E3B7EC2".to_string(),
        from_address: "0x742d35Cc6B8B4d1e8d37a1E5B0b4F8e8B7F4D2a1".to_string(),
        to_address: "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed".to_string(),
        amount: BigInt::from_str("1000000000").unwrap(),
        block_number: 17000000,
        log_index: 0,
        transaction_hash: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string(),
    }
}
```

## Property-Based Testing

### Using QuickCheck

```rust
// src/tests/unit/property_tests.rs
use quickcheck::*;
use quickcheck_macros::quickcheck;

#[derive(Clone, Debug)]
struct ArbitraryEthereumAddress(String);

impl Arbitrary for ArbitraryEthereumAddress {
    fn arbitrary(g: &mut Gen) -> Self {
        // Generate valid Ethereum address
        let bytes: [u8; 20] = [0; 20].map(|_| u8::arbitrary(g));
        let addr = format!("0x{}", hex::encode(bytes));
        ArbitraryEthereumAddress(addr)
    }
}

#[derive(Clone, Debug)]  
struct ArbitraryTransfer {
    contract: ArbitraryEthereumAddress,
    from: ArbitraryEthereumAddress,
    to: ArbitraryEthereumAddress,
    amount: u64, // Use u64 to avoid overflow in tests
}

impl Arbitrary for ArbitraryTransfer {
    fn arbitrary(g: &mut Gen) -> Self {
        ArbitraryTransfer {
            contract: ArbitraryEthereumAddress::arbitrary(g),
            from: ArbitraryEthereumAddress::arbitrary(g),
            to: ArbitraryEthereumAddress::arbitrary(g),
            amount: u64::arbitrary(g),
        }
    }
}

#[quickcheck]
fn prop_parse_transfer_preserves_data(transfer_data: ArbitraryTransfer) -> bool {
    let log = create_transfer_log(
        &transfer_data.contract.0,
        &transfer_data.from.0,
        &transfer_data.to.0,
        &transfer_data.amount.to_string()
    );
    
    match parse_erc20_transfer(&log) {
        Ok(parsed) => {
            parsed.contract_address == transfer_data.contract.0 &&
            parsed.from_address == transfer_data.from.0 &&
            parsed.to_address == transfer_data.to.0 &&
            parsed.amount == BigInt::from(transfer_data.amount)
        }
        Err(_) => {
            // Some edge cases might fail parsing, that's acceptable
            transfer_data.from.0 == transfer_data.to.0 || // Self-transfer edge case
            transfer_data.amount == 0 // Zero amount edge case
        }
    }
}

#[quickcheck]
fn prop_balance_calculation_conservation(transfers: Vec<ArbitraryTransfer>) -> bool {
    // Property: Total balance change should be zero (conservation)
    let transfer_objects: Vec<Transfer> = transfers
        .iter()
        .filter(|t| t.from.0 != t.to.0) // Filter out self-transfers
        .enumerate()
        .map(|(i, t)| Transfer {
            contract_address: t.contract.0.clone(),
            from_address: t.from.0.clone(),
            to_address: t.to.0.clone(),
            amount: BigInt::from(t.amount),
            block_number: 17000000,
            log_index: i as u32,
            transaction_hash: format!("0x{:064x}", i),
        })
        .collect();
    
    let balances = calculate_balances(&transfer_objects);
    
    // Sum of all balance changes should be zero
    let total_balance_change: BigInt = balances.values().sum();
    total_balance_change == BigInt::zero()
}

#[quickcheck]
fn prop_address_validation_consistency(addr: String) -> bool {
    // Property: Address validation should be consistent
    let is_valid_result1 = is_valid_ethereum_address(&addr);
    let is_valid_result2 = is_valid_ethereum_address(&addr);
    
    // Same input should always give same result
    is_valid_result1 == is_valid_result2
}

#[quickcheck]  
fn prop_sanitize_address_idempotent(addr: String) -> bool {
    // Property: Sanitizing twice should give same result as sanitizing once
    let sanitized_once = sanitize_ethereum_address(&addr);
    let sanitized_twice = sanitize_ethereum_address(&sanitized_once);
    
    sanitized_once == sanitized_twice
}

#[quickcheck]
fn prop_format_amount_reversible(amount: u64, decimals: u8) -> bool {
    // Limit decimals to reasonable range
    let decimals = decimals % 19; // 0-18 decimals
    
    let original_amount = BigInt::from(amount);
    let formatted = format_token_amount(&original_amount, decimals);
    
    // Parse back (this tests the property if you have a parse function)
    match parse_token_amount(&formatted, decimals) {
        Ok(parsed_amount) => parsed_amount == original_amount,
        Err(_) => {
            // Some edge cases might not be reversible (e.g., precision loss)
            // This is acceptable for very small amounts
            amount < 1000
        }
    }
}
```

## Mock Testing

### Mocking External Dependencies

```rust
// src/tests/unit/mock_tests.rs
use mockall::*;
use crate::*;

// Define trait for external dependencies
#[automock]
trait TokenMetadataProvider {
    fn get_token_decimals(&self, contract_address: &str) -> Result<u8, Error>;
    fn get_token_symbol(&self, contract_address: &str) -> Result<String, Error>;
}

// Test with mocked dependencies
#[test]
fn test_format_transfer_with_metadata() {
    let mut mock_provider = MockTokenMetadataProvider::new();
    
    // Setup mock expectations
    mock_provider
        .expect_get_token_decimals()
        .with(eq("0xA0b86a33E6Fe17d67C8c086c6c4c0E3C8E3B7EC2"))
        .returning(|_| Ok(6)) // USDC has 6 decimals
        .times(1);
        
    mock_provider
        .expect_get_token_symbol()
        .with(eq("0xA0b86a33E6Fe17d67C8c086c6c4c0E3C8E3B7EC2"))
        .returning(|_| Ok("USDC".to_string()))
        .times(1);
    
    let transfer = Transfer {
        contract_address: "0xA0b86a33E6Fe17d67C8c086c6c4c0E3C8E3B7EC2".to_string(),
        from_address: "0x742d35Cc6B8B4d1e8d37a1E5B0b4F8e8B7F4D2a1".to_string(),
        to_address: "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed".to_string(),
        amount: BigInt::from_str("1000000000").unwrap(), // 1000 USDC (6 decimals)
        block_number: 17000000,
        log_index: 0,
        transaction_hash: "0x123...".to_string(),
    };
    
    // Test function that uses the mock
    let formatted = format_transfer_with_metadata(&transfer, &mock_provider).unwrap();
    
    assert_eq!(formatted.amount_formatted, "1000.0");
    assert_eq!(formatted.symbol, "USDC");
    
    // Verify all expectations were met
    // This happens automatically when mock goes out of scope
}

#[test]
fn test_handle_metadata_provider_failure() {
    let mut mock_provider = MockTokenMetadataProvider::new();
    
    // Mock returns error
    mock_provider
        .expect_get_token_decimals()
        .returning(|_| Err(Error::msg("Network error")))
        .times(1);
    
    let transfer = create_test_transfer();
    
    let result = format_transfer_with_metadata(&transfer, &mock_provider);
    
    // Should handle error gracefully
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Network error"));
}
```

## Test Fixtures and Utilities

### Fixture Management

```rust
// src/tests/fixtures.rs
use std::collections::HashMap;
use once_cell::sync::Lazy;

// Lazy static fixtures for expensive-to-compute test data
static TEST_BLOCKS: Lazy<HashMap<u64, Block>> = Lazy::new(|| {
    let mut blocks = HashMap::new();
    
    // Load pre-computed test blocks
    blocks.insert(17000000, load_block_fixture("block_17000000.json"));
    blocks.insert(17000001, load_block_fixture("block_17000001.json"));
    blocks.insert(17000100, load_block_fixture("block_17000100.json"));
    
    blocks
});

static KNOWN_TOKENS: Lazy<HashMap<&'static str, TokenMetadata>> = Lazy::new(|| {
    let mut tokens = HashMap::new();
    
    tokens.insert("0xA0b86a33E6Fe17d67C8c086c6c4c0E3C8E3B7EC2", TokenMetadata {
        symbol: "USDC".to_string(),
        name: "USD Coin".to_string(),
        decimals: 6,
    });
    
    tokens.insert("0xdAC17F958D2ee523a2206206994597C13D831ec7", TokenMetadata {
        symbol: "USDT".to_string(),
        name: "Tether USD".to_string(),
        decimals: 6,
    });
    
    tokens.insert("0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599", TokenMetadata {
        symbol: "WBTC".to_string(),
        name: "Wrapped Bitcoin".to_string(),
        decimals: 8,
    });
    
    tokens
});

pub fn get_test_block(block_number: u64) -> Option<&'static Block> {
    TEST_BLOCKS.get(&block_number)
}

pub fn get_token_metadata(contract_address: &str) -> Option<&'static TokenMetadata> {
    KNOWN_TOKENS.get(contract_address)
}

fn load_block_fixture(filename: &str) -> Block {
    let fixture_path = std::path::Path::new("fixtures").join(filename);
    let data = std::fs::read_to_string(fixture_path)
        .unwrap_or_else(|_| panic!("Failed to load fixture: {}", filename));
    
    serde_json::from_str(&data)
        .unwrap_or_else(|_| panic!("Failed to parse fixture: {}", filename))
}

#[derive(Clone, Debug)]
pub struct TokenMetadata {
    pub symbol: String,
    pub name: String,
    pub decimals: u8,
}

// Builder pattern for test data
pub struct BlockBuilder {
    block: Block,
}

impl BlockBuilder {
    pub fn new(block_number: u64) -> Self {
        Self {
            block: Block {
                number: block_number,
                hash: generate_block_hash(block_number),
                parent_hash: generate_block_hash(block_number.saturating_sub(1)),
                timestamp_seconds: 1680000000 + (block_number * 12), // 12 second blocks
                ..Default::default()
            }
        }
    }
    
    pub fn with_timestamp(mut self, timestamp: u64) -> Self {
        self.block.timestamp_seconds = timestamp;
        self
    }
    
    pub fn add_transaction(mut self, tx: TransactionTrace) -> Self {
        self.block.transaction_traces.push(tx);
        self
    }
    
    pub fn add_erc20_transfer(self, contract: &str, from: &str, to: &str, amount: &str) -> Self {
        let tx = TransactionBuilder::new()
            .add_erc20_transfer_log(contract, from, to, amount)
            .build();
            
        self.add_transaction(tx)
    }
    
    pub fn build(self) -> Block {
        self.block
    }
}

pub struct TransactionBuilder {
    tx: TransactionTrace,
}

impl TransactionBuilder {
    pub fn new() -> Self {
        Self {
            tx: TransactionTrace {
                hash: generate_tx_hash(),
                receipt: Some(TransactionReceipt {
                    logs: vec![],
                    ..Default::default()
                }),
                ..Default::default()
            }
        }
    }
    
    pub fn with_hash(mut self, hash: &str) -> Self {
        self.tx.hash = hex::decode(&hash[2..]).unwrap();
        self
    }
    
    pub fn add_erc20_transfer_log(mut self, contract: &str, from: &str, to: &str, amount: &str) -> Self {
        let log = create_transfer_log(contract, from, to, amount);
        if let Some(ref mut receipt) = self.tx.receipt {
            receipt.logs.push(log);
        }
        self
    }
    
    pub fn build(self) -> TransactionTrace {
        self.tx
    }
}

// Utility functions
fn generate_block_hash(block_number: u64) -> Vec<u8> {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(b"test_block_");
    hasher.update(block_number.to_le_bytes());
    hasher.finalize().to_vec()
}

fn generate_tx_hash() -> Vec<u8> {
    use sha2::{Sha256, Digest};
    use std::sync::atomic::{AtomicU64, Ordering};
    
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let counter = COUNTER.fetch_add(1, Ordering::Relaxed);
    
    let mut hasher = Sha256::new();
    hasher.update(b"test_tx_");
    hasher.update(counter.to_le_bytes());
    hasher.finalize().to_vec()
}
```

## Performance Unit Tests

### Benchmarking Critical Functions

```rust
// benches/unit_benchmarks.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use your_substreams::*;

fn benchmark_parse_transfer(c: &mut Criterion) {
    let log = create_transfer_log(
        "0xA0b86a33E6Fe17d67C8c086c6c4c0E3C8E3B7EC2",
        "0x742d35Cc6B8B4d1e8d37a1E5B0b4F8e8B7F4D2a1", 
        "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed",
        "1000000000000000000"
    );
    
    c.bench_function("parse_erc20_transfer", |b| {
        b.iter(|| parse_erc20_transfer(black_box(&log)))
    });
}

fn benchmark_calculate_balances(c: &mut Criterion) {
    let mut group = c.benchmark_group("calculate_balances");
    
    for size in [10, 100, 1000, 10000].iter() {
        let transfers = create_test_transfers(*size);
        
        group.bench_with_input(
            BenchmarkId::new("transfers", size), 
            size, 
            |b, _| {
                b.iter(|| calculate_balances(black_box(&transfers)))
            }
        );
    }
    
    group.finish();
}

fn benchmark_address_validation(c: &mut Criterion) {
    let valid_address = "0xA0b86a33E6Fe17d67C8c086c6c4c0E3C8E3B7EC2";
    let invalid_address = "not_an_address";
    
    c.bench_function("validate_valid_address", |b| {
        b.iter(|| is_valid_ethereum_address(black_box(valid_address)))
    });
    
    c.bench_function("validate_invalid_address", |b| {
        b.iter(|| is_valid_ethereum_address(black_box(invalid_address)))
    });
}

fn create_test_transfers(count: usize) -> Vec<Transfer> {
    (0..count)
        .map(|i| Transfer {
            contract_address: "0xA0b86a33E6Fe17d67C8c086c6c4c0E3C8E3B7EC2".to_string(),
            from_address: format!("0x{:040x}", i),
            to_address: format!("0x{:040x}", i + 1),
            amount: BigInt::from(1000000000u64 + i as u64),
            block_number: 17000000 + i as u64,
            log_index: i as u32,
            transaction_hash: format!("0x{:064x}", i),
        })
        .collect()
}

criterion_group!(
    benches, 
    benchmark_parse_transfer,
    benchmark_calculate_balances, 
    benchmark_address_validation
);
criterion_main!(benches);
```

## Best Practices for Unit Testing

### DO

✅ **Test one thing at a time** - Each test should verify a single behavior  
✅ **Use descriptive test names** - `test_parse_erc20_transfer_with_zero_amount`  
✅ **Follow AAA pattern** - Arrange, Act, Assert  
✅ **Test edge cases** - Zero values, maximum values, invalid input  
✅ **Use property-based testing** - Let the computer find edge cases  
✅ **Mock external dependencies** - Keep tests isolated and fast  
✅ **Create test utilities** - Reusable builders and fixtures  
✅ **Benchmark critical paths** - Know where performance bottlenecks are  

### DON'T

❌ **Test implementation details** - Test behavior, not internals  
❌ **Write tests that depend on external state** - Keep tests isolated  
❌ **Skip error case testing** - Error handling is critical  
❌ **Use production data in tests** - Use controlled test fixtures  
❌ **Write tests that are slower than the code** - Unit tests should be fast  
❌ **Copy-paste test code** - Use builders and utilities  
❌ **Ignore failing tests** - Fix or remove broken tests  
❌ **Test everything** - Focus on business logic and edge cases  

### Test Organization

```
src/
  lib.rs
  parsing.rs
  validation.rs
  calculations.rs
  
tests/
  unit/
    parsing_tests.rs
    validation_tests.rs  
    calculation_tests.rs
    property_tests.rs
  fixtures/
    blocks/
      block_17000000.json
      block_17000001.json
    tokens/
      token_metadata.json
  test_utils.rs
  
benches/
  unit_benchmarks.rs
```

This structure keeps tests organized and makes it easy to find relevant tests for any module.