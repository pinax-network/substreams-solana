-- EVM Contracts Table
CREATE TABLE IF NOT EXISTS contracts (
    address                     FixedString(42),         -- The address of the contract
    block_num                   UInt64,                  -- The block number of the transaction that deployed the contract
    block_hash                  FixedString(66),         -- The hash of the block that deployed the contract
    timestamp                   DateTime(0, 'UTC'),      -- The timestamp of the block that deployed the contract
    tx_hash                     FixedString(66),         -- The hash of the transaction that deployed the contract
    tx_index                    UInt32,                  -- The index of the transaction in the block
    creator                     FixedString(42),         -- The address of the account that sent the creation transaction
    factory                     FixedString(42),         -- The address of the factory contract that deployed the contract
    code                        String,                  -- The code of the contract
    code_hash                   FixedString(66),         -- The hash of the code of the contract
    input                       String,                  -- The input data of the transaction that deployed the contract

    INDEX idx_block_num          (block_num)             TYPE minmax GRANULARITY 4,
    INDEX idx_tx_hash            (tx_hash)               TYPE bloom_filter GRANULARITY 4,
    INDEX idx_factory            (factory)               TYPE bloom_filter GRANULARITY 4,
    INDEX idx_creator            (creator)               TYPE bloom_filter GRANULARITY 4,
) ENGINE = ReplacingMergeTree
ORDER BY address;
