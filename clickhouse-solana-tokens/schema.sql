-- This file is generated. Do not edit.

CREATE TABLE IF NOT EXISTS blocks (
    block_num   UInt32,
    block_hash  FixedString(44),
    timestamp   DateTime(0, 'UTC'),

    -- indexes --
    INDEX idx_block_hash   (block_hash)      TYPE bloom_filter GRANULARITY 4,
    INDEX idx_timestamp    (timestamp)       TYPE minmax GRANULARITY 4

) ENGINE = ReplacingMergeTree(timestamp)
ORDER BY block_num;


-- SPL Token-2022 & Classic balance changes by mint/owner --
-- Only keep last balance change per account per block, use `execution_index` to resolve conflicts
CREATE TABLE IF NOT EXISTS balance_changes  (
    -- block --
    block_num           UInt32,
    block_hash          FixedString(44),
    timestamp           DateTime(0, 'UTC'),

    -- transaction --
    tx_hash             FixedString(88),

    -- ordering --
    execution_index     UInt32, -- relative index

    -- account --
    program_id          LowCardinality(FixedString(44)),

    -- event --
    owner               FixedString(44),
    mint                FixedString(44),
    amount              UInt64,
    decimals            UInt8,

    -- indexes --
    INDEX idx_block_num          (block_num)           TYPE minmax GRANULARITY 4,
    INDEX idx_timestamp          (timestamp)           TYPE minmax GRANULARITY 4,

    -- indexes (event) --
    INDEX idx_program_id         (program_id)          TYPE set(8) GRANULARITY 4,
    INDEX idx_mint               (mint)                TYPE set(128) GRANULARITY 4,
    INDEX idx_owner              (owner)               TYPE bloom_filter GRANULARITY 4,
    INDEX idx_amount             (amount)              TYPE minmax GRANULARITY 4,
    INDEX idx_decimals           (decimals)            TYPE minmax GRANULARITY 4
)
ENGINE = ReplacingMergeTree(execution_index)
ORDER BY (mint, owner, block_hash);


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

-- latest balances by owner/mint --
CREATE TABLE IF NOT EXISTS balances AS balance_changes
ENGINE = ReplacingMergeTree(block_num)
ORDER BY (owner, mint);

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_balances
TO balances AS
SELECT * FROM balance_changes;

-- latest balances by mint/owner --
CREATE TABLE IF NOT EXISTS balances_by_mint AS balance_changes
ENGINE = ReplacingMergeTree(block_num)
ORDER BY (mint, owner);

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_balances_by_mint
TO balances AS
SELECT * FROM balance_changes;

