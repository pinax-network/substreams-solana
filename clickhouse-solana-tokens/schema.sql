-- This file is generated. Do not edit.

-- SPL balance by account --
-- There can only be a single SPL balance change per block for a given address / contract pair --
CREATE TABLE IF NOT EXISTS spl_token_balance_changes  (
    -- block --
    block_num           UInt32,
    block_hash          FixedString(66),
    timestamp           DateTime(0, 'UTC'),

    -- transaction --
    tx_hash             FixedString(88),

    -- ordering --
    execution_index     UInt32, -- relative index
    global_sequence     UInt64, -- latest global sequence (block_num << 32 + execution_index)

    -- account --
    program_id          FixedString(32),

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
ENGINE = ReplacingMergeTree
ORDER BY (mint, owner, block_hash, execution_index);


-- SPL-2022 Token transfers --
-- DOES NOT INCLUDE Native nor legacy SPL Token --
CREATE TABLE IF NOT EXISTS transfers (
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
    multisig_authority_raw      String, -- comma-separated list of multisig authorities
    multisig_authority          Array(FixedString(44)) MATERIALIZED splitByChar(',', multisig_authority_raw),

    -- event --
    source                      FixedString(44),
    destination                 FixedString(44),
    amount                      UInt64,
    mint                        FixedString(44), -- can be empty for Classic SPL Token
    decimals_raw                String, -- can be empty for Classic SPL Token
    decimals                    Optional(UInt8) MATERIALIZED
        CASE
            WHEN decimals_raw = '' THEN NULL
            ELSE toUInt8(decimals_raw)
        END,

    -- classification --
    token_standard              Enum8('Native' = 1, 'Classic SPL Token' = 2, 'SPL Token-2022' = 3),

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
    INDEX idx_decimals           (decimals)           TYPE minmax GRANULARITY 4
    INDEX idx_token_standard     (token_standard)     TYPE set(1) GRANULARITY 1
)
ENGINE = ReplacingMergeTree
ORDER BY (timestamp, block_hash, execution_index);


-- latest balances by owner/contract --
CREATE TABLE IF NOT EXISTS balances  (
    -- block --
    block_num            UInt32,
    block_hash           FixedString(66),
    timestamp            DateTime(0, 'UTC'),

    -- ordering --
    execution_index     UInt32, -- relative index
    global_sequence     UInt64, -- latest global sequence (block_num << 32 + execution_index)

    -- account --
    program_id          FixedString(32),

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
ENGINE = ReplacingMergeTree(block_num)
ORDER BY (address, contract);

-- insert SPL Token balance changes --
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_spl_token_balance_changes
TO balances AS
SELECT
    -- block --
    b.block_num AS block_num,
    b.block_hash AS block_hash,
    b.timestamp AS timestamp,

    -- ordering --
    b.execution_index AS execution_index,
    b.global_sequence AS global_sequence,

    -- event --
    b.program_id AS program_id,
    b.owner AS owner,
    b.mint AS mint,
    b.amount AS amount,
    b.decimals AS decimals

FROM spl_token_balance_changes AS b;


