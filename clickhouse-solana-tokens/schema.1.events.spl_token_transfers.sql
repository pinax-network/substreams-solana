-- SPL Token transfers --
CREATE TABLE IF NOT EXISTS spl_token_transfers  (
    -- block --
    block_num                   UInt32,
    block_hash                  FixedString(66),
    timestamp                   DateTime(0, 'UTC'),

    -- ordering --
    execution_index             UInt32, -- relative index
    instruction_index           UInt32,
    inner_instruction_index     UInt32,
    stack_height                UInt32,
    global_sequence             UInt64, -- latest global sequence (block_num << 32 + execution_index)

    -- transaction --
    tx_hash                     FixedString(88),

    -- instruction --
    program_id                  FixedString(44),
    instruction                 LowCardinality(String),

    -- authority --
    authority                   FixedString(44),
    -- multisig_authority          FixedString(44),

    -- event --
    source                      FixedString(44),
    destination                 FixedString(44),
    amount                      UInt64,
    mint                        FixedString(44),
    decimals                    UInt8,

    -- indexes --
    INDEX idx_tx_hash            (tx_hash)            TYPE bloom_filter GRANULARITY 4,
    INDEX idx_program_id         (program_id)         TYPE set(128) GRANULARITY 4,
    INDEX idx_authority          (authority)          TYPE bloom_filter GRANULARITY 4,

    -- indexes (event) --
    INDEX idx_source             (source)             TYPE bloom_filter GRANULARITY 4,
    INDEX idx_destination        (destination)        TYPE bloom_filter GRANULARITY 4,
    INDEX idx_mint               (mint)               TYPE set(128) GRANULARITY 4,
    INDEX idx_amount             (amount)             TYPE minmax GRANULARITY 4,
    INDEX idx_decimals           (decimals)           TYPE minmax GRANULARITY 4
)
ENGINE = ReplacingMergeTree
ORDER BY (timestamp, block_num, execution_index, block_hash);
