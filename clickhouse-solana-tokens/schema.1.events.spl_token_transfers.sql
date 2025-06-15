-- SPL-2022 Token & Classic transfers --
CREATE TABLE IF NOT EXISTS transfers (
    -- block --
    block_num                   UInt32,
    block_hash                  FixedString(44),
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
    multisig_authority_raw      String, -- comma-separated list of multisig authorities
    multisig_authority          Array(FixedString(44)) MATERIALIZED splitByChar(',', multisig_authority_raw),

    -- event --
    source                      FixedString(44),
    destination                 FixedString(44),
    amount                      UInt64,

    -- event (SPL Token-2022) --
    mint_raw                    String, -- can be empty for Classic SPL Token
    mint                        Optional(FixedString(44)) MATERIALIZED
        CASE
            WHEN mint_raw = '' THEN NULL
            ELSE toFixedString(mint_raw, 44)
        END,
    decimals_raw                String, -- can be empty for Classic SPL Token
    decimals                    Optional(UInt8) MATERIALIZED
        CASE
            WHEN decimals_raw = '' THEN NULL
            ELSE toUInt8(decimals_raw)
        END,

    -- classification --
    token_standard              Enum8('Classic SPL Token' = 1, 'SPL Token-2022' = 2, 'Native' = 3),

    -- indexes --
    INDEX idx_block_num          (block_num)          TYPE minmax GRANULARITY 4,
    INDEX idx_tx_hash            (tx_hash)            TYPE bloom_filter GRANULARITY 4,
    INDEX idx_program_id         (program_id)         TYPE set(128) GRANULARITY 4,
    INDEX idx_authority          (authority)          TYPE bloom_filter GRANULARITY 4,

    -- indexes (event) --
    INDEX idx_source             (source)             TYPE bloom_filter GRANULARITY 4,
    INDEX idx_destination        (destination)        TYPE bloom_filter GRANULARITY 4,
    INDEX idx_mint               (mint)               TYPE set(128) GRANULARITY 4,
    INDEX idx_amount             (amount)             TYPE minmax GRANULARITY 4,
    INDEX idx_decimals           (decimals)           TYPE set(8) GRANULARITY 4
    INDEX idx_token_standard     (token_standard)     TYPE set(2) GRANULARITY 1
)
ENGINE = ReplacingMergeTree
ORDER BY (timestamp, block_hash, execution_index);

-- SPL Token-2022 & Classic approves --
CREATE TABLE IF NOT EXISTS approves (
    -- block --
    block_num                   UInt32,
    block_hash                  FixedString(44),
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
    multisig_authority_raw      String, -- comma-separated list of multisig authorities
    multisig_authority          Array(FixedString(44)) MATERIALIZED splitByChar(',', multisig_authority_raw),

    -- event --
    source                      FixedString(44),
    delegate                    FixedString(44),
    owner                       FixedString(44),
    amount                      UInt64,

    -- event (SPL Token-2022) --
    mint_raw                    String, -- can be empty for Classic SPL Token
    mint                        Optional(FixedString(44)) MATERIALIZED
        CASE
            WHEN mint_raw = '' THEN NULL
            ELSE toFixedString(mint_raw, 44)
        END,
    decimals_raw                String, -- can be empty for Classic SPL Token
    decimals                    Optional(UInt8) MATERIALIZED
        CASE
            WHEN decimals_raw = '' THEN NULL
            ELSE toUInt8(decimals_raw)
        END,

    -- classification --
    token_standard              Enum8('Classic SPL Token' = 1, 'SPL Token-2022' = 2),

    -- indexes --
    INDEX idx_block_num          (block_num)          TYPE minmax GRANULARITY 4,
    INDEX idx_tx_hash            (tx_hash)            TYPE bloom_filter GRANULARITY 4,
    INDEX idx_program_id         (program_id)         TYPE set(128) GRANULARITY 4,
    INDEX idx_authority          (authority)          TYPE bloom_filter GRANULARITY 4,

    -- indexes (event) --
    INDEX idx_source             (source)             TYPE bloom_filter GRANULARITY 4,
    INDEX idx_delegate           (delegate)           TYPE bloom_filter GRANULARITY 4,
    INDEX idx_owner              (owner)              TYPE bloom_filter GRANULARITY 4,
    INDEX idx_mint               (mint)               TYPE set(128) GRANULARITY 4,
    INDEX idx_amount             (amount)             TYPE minmax GRANULARITY 4,
    INDEX idx_decimals           (decimals)           TYPE set(8) GRANULARITY 4
    INDEX idx_token_standard     (token_standard)     TYPE set(2) GRANULARITY 1
)
ENGINE = ReplacingMergeTree
ORDER BY (timestamp, block_hash, execution_index);

-- SPL Token-2022 & Classic revokes --
CREATE TABLE IF NOT EXISTS revokes (
    -- block --
    block_num                   UInt32,
    block_hash                  FixedString(44),
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
    multisig_authority_raw      String, -- comma-separated list of multisig authorities
    multisig_authority          Array(FixedString(44)) MATERIALIZED splitByChar(',', multisig_authority_raw),

    -- event --
    source                      FixedString(44),
    owner                       FixedString(44),

    -- indexes --
    INDEX idx_block_num          (block_num)          TYPE minmax GRANULARITY 4,
    INDEX idx_tx_hash            (tx_hash)            TYPE bloom_filter GRANULARITY 4,
    INDEX idx_program_id         (program_id)         TYPE set(128) GRANULARITY 4,
    INDEX idx_authority          (authority)          TYPE bloom_filter GRANULARITY 4,

    -- indexes (event) --
    INDEX idx_source             (source)             TYPE bloom_filter GRANULARITY 4,
    INDEX idx_owner              (owner)              TYPE bloom_filter GRANULARITY 4
)
ENGINE = ReplacingMergeTree
ORDER BY (timestamp, block_hash, execution_index);
