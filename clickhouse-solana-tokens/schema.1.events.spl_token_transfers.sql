-- SPL-2022 Token & Classic transfers --
CREATE TABLE IF NOT EXISTS transfers (
    -- block --
    block_num                   UInt32,
    block_hash                  FixedString(44),
    timestamp                   DateTime(0, 'UTC'),
    timestamp_since_genesis     DateTime(0, 'UTC')
        MATERIALIZED if (
            timestamp = 0,
            toDateTime(1584332940 + intDiv(block_num * 2, 5), 'UTC'),
            timestamp
        ),

    -- ordering --
    execution_index             UInt32, -- relative index
    instruction_index           UInt32,
    inner_instruction_index     UInt32,
    stack_height                UInt32,
    global_sequence             UInt64, -- latest global sequence (block_num << 32 + execution_index)

    -- transaction --
    tx_hash                     FixedString(88),

    -- instruction --
    program_id                  LowCardinality(FixedString(44)),
    instruction                 LowCardinality(String),

    -- authority --
    authority                   FixedString(44),
    multisig_authority_raw      String, -- comma-separated list of multisig authorities
    multisig_authority          Array(FixedString(44)) MATERIALIZED splitByChar(',', multisig_authority_raw),

    -- event --
    source                      FixedString(44),
    destination                 FixedString(44),
    amount                      UInt64,

    -- event (Optional) --
    mint_raw                    String, -- can be empty
    mint                        Nullable(FixedString(44)) MATERIALIZED accurateCastOrNull(nullIf(mint_raw, ''), 'FixedString(44)'),
    decimals_raw                String, -- can be empty
    decimals                    Nullable(UInt8) MATERIALIZED toUInt8OrNull(nullIf(decimals_raw, '')),

    -- indexes --
    INDEX idx_block_num          (block_num)          TYPE minmax GRANULARITY 4,
    INDEX idx_tx_hash            (tx_hash)            TYPE bloom_filter GRANULARITY 4,
    INDEX idx_program_id         (program_id)         TYPE set(2) GRANULARITY 1,
    INDEX idx_authority          (authority)          TYPE bloom_filter GRANULARITY 4,
    INDEX idx_timestamp_since_genesis    (timestamp_since_genesis)  TYPE minmax GRANULARITY 4

    -- indexes (event) --
    INDEX idx_source             (source)             TYPE bloom_filter GRANULARITY 4,
    INDEX idx_destination        (destination)        TYPE bloom_filter GRANULARITY 4,
    INDEX idx_mint               (mint)               TYPE set(128) GRANULARITY 4,
    INDEX idx_amount             (amount)             TYPE minmax GRANULARITY 4,
    INDEX idx_decimals           (decimals)           TYPE set(8) GRANULARITY 4
)
ENGINE = ReplacingMergeTree
ORDER BY (timestamp, block_hash, execution_index);

-- SPL Token-2022 & Classic approves --
CREATE TABLE IF NOT EXISTS approves (
    -- block --
    block_num                   UInt32,
    block_hash                  FixedString(44),
    timestamp                   DateTime(0, 'UTC'),
    timestamp_since_genesis     DateTime(0, 'UTC')
        MATERIALIZED if (
            timestamp = 0,
            toDateTime(1584332940 + intDiv(block_num * 2, 5), 'UTC'),
            timestamp
        ),

    -- ordering --
    execution_index             UInt32, -- relative index
    instruction_index           UInt32,
    inner_instruction_index     UInt32,
    stack_height                UInt32,
    global_sequence             UInt64, -- latest global sequence (block_num << 32 + execution_index)

    -- transaction --
    tx_hash                     FixedString(88),

    -- instruction --
    program_id                  LowCardinality(FixedString(44)),
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

    -- event (Optional) --
    mint_raw                    String, -- can be empty
    mint                        Nullable(FixedString(44)) MATERIALIZED accurateCastOrNull(nullIf(mint_raw, ''), 'FixedString(44)'),
    decimals_raw                String, -- can be empty
    decimals                    Nullable(UInt8) MATERIALIZED toUInt8OrNull(nullIf(decimals_raw, '')),

    -- indexes --
    INDEX idx_block_num          (block_num)          TYPE minmax GRANULARITY 4,
    INDEX idx_tx_hash            (tx_hash)            TYPE bloom_filter GRANULARITY 4,
    INDEX idx_program_id         (program_id)         TYPE set(2) GRANULARITY 1,
    INDEX idx_authority          (authority)          TYPE bloom_filter GRANULARITY 4,

    -- indexes (event) --
    INDEX idx_source             (source)             TYPE bloom_filter GRANULARITY 4,
    INDEX idx_delegate           (delegate)           TYPE bloom_filter GRANULARITY 4,
    INDEX idx_owner              (owner)              TYPE bloom_filter GRANULARITY 4,
    INDEX idx_mint               (mint)               TYPE set(128) GRANULARITY 4,
    INDEX idx_amount             (amount)             TYPE minmax GRANULARITY 4,
    INDEX idx_decimals           (decimals)           TYPE set(8) GRANULARITY 4
)
ENGINE = ReplacingMergeTree
ORDER BY (timestamp, block_hash, execution_index);

-- SPL Token-2022 & Classic revokes --
CREATE TABLE IF NOT EXISTS revokes (
    -- block --
    block_num                   UInt32,
    block_hash                  FixedString(44),
    timestamp                   DateTime(0, 'UTC'),
    timestamp_since_genesis     DateTime(0, 'UTC')
        MATERIALIZED if (
            timestamp = 0,
            toDateTime(1584332940 + intDiv(block_num * 2, 5), 'UTC'),
            timestamp
        ),

    -- ordering --
    execution_index             UInt32, -- relative index
    instruction_index           UInt32,
    inner_instruction_index     UInt32,
    stack_height                UInt32,
    global_sequence             UInt64, -- latest global sequence (block_num << 32 + execution_index)

    -- transaction --
    tx_hash                     FixedString(88),

    -- instruction --
    program_id                  LowCardinality(FixedString(44)),
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
    INDEX idx_program_id         (program_id)         TYPE set(2) GRANULARITY 1,
    INDEX idx_authority          (authority)          TYPE bloom_filter GRANULARITY 4,

    -- indexes (event) --
    INDEX idx_source             (source)             TYPE bloom_filter GRANULARITY 4,
    INDEX idx_owner              (owner)              TYPE bloom_filter GRANULARITY 4
)
ENGINE = ReplacingMergeTree
ORDER BY (timestamp, block_hash, execution_index);

-- SPL Token-2022 & Classic Initialize Accounts --
CREATE TABLE IF NOT EXISTS initialize_accounts (
    -- block --
    block_num                   UInt32,
    block_hash                  FixedString(44),
    timestamp                   DateTime(0, 'UTC'),
    timestamp_since_genesis     DateTime(0, 'UTC')
        MATERIALIZED if (
            timestamp = 0,
            toDateTime(1584332940 + intDiv(block_num * 2, 5), 'UTC'),
            timestamp
        ),

    -- ordering --
    execution_index             UInt32, -- relative index
    instruction_index           UInt32,
    inner_instruction_index     UInt32,
    stack_height                UInt32,
    global_sequence             UInt64, -- latest global sequence (block_num << 32 + execution_index)

    -- transaction --
    tx_hash                     FixedString(88),

    -- instruction --
    program_id                  LowCardinality(FixedString(44)),
    instruction                 LowCardinality(String),

    -- event --
    account                     FixedString(44),
    mint                        FixedString(44),
    owner                       FixedString(44),

    -- indexes --
    INDEX idx_block_num          (block_num)          TYPE minmax GRANULARITY 4,
    INDEX idx_tx_hash            (tx_hash)            TYPE bloom_filter GRANULARITY 4,
    INDEX idx_program_id         (program_id)         TYPE set(2) GRANULARITY 1,

    -- indexes (event) --
    INDEX idx_mint               (mint)               TYPE set(128) GRANULARITY 4,
    INDEX idx_owner              (owner)              TYPE bloom_filter GRANULARITY 4
)
ENGINE = ReplacingMergeTree(block_num)
ORDER BY (account, mint, program_id);

-- SPL Token-2022 & Classic Initialize Mints --
CREATE TABLE IF NOT EXISTS initialize_mints (
    -- block --
    block_num                   UInt32,
    block_hash                  FixedString(44),
    timestamp                   DateTime(0, 'UTC'),
    timestamp_since_genesis     DateTime(0, 'UTC')
        MATERIALIZED if (
            timestamp = 0,
            toDateTime(1584332940 + intDiv(block_num * 2, 5), 'UTC'),
            timestamp
        ),

    -- ordering --
    execution_index             UInt32, -- relative index
    instruction_index           UInt32,
    inner_instruction_index     UInt32,
    stack_height                UInt32,
    global_sequence             UInt64, -- latest global sequence (block_num << 32 + execution_index)

    -- transaction --
    tx_hash                     FixedString(88),

    -- instruction --
    program_id                  LowCardinality(FixedString(44)),
    instruction                 LowCardinality(String),

    -- event --
    mint                        FixedString(44),
    mint_authority              FixedString(44),
    freeze_authority            FixedString(44),
    decimals                    UInt8,

    -- indexes --
    INDEX idx_block_num          (block_num)          TYPE minmax GRANULARITY 4,
    INDEX idx_tx_hash            (tx_hash)            TYPE bloom_filter GRANULARITY 4,
    INDEX idx_program_id         (program_id)         TYPE set(2) GRANULARITY 1,

    -- indexes (event) --
    INDEX idx_mint               (mint)               TYPE set(128) GRANULARITY 4,
    INDEX idx_mint_authority     (mint_authority)     TYPE set(128) GRANULARITY 4,
    INDEX idx_freeze_authority   (freeze_authority)   TYPE set(128) GRANULARITY 4,
    INDEX idx_decimals           (decimals)           TYPE minmax GRANULARITY 4
)
ENGINE = ReplacingMergeTree(block_num)
ORDER BY (mint, program_id);