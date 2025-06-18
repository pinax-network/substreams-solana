-- Raydium Swaps --
CREATE TABLE IF NOT EXISTS raydium_amm_swaps (
    -- block --
    block_num                   UInt32,
    block_hash                  FixedString(44),
    timestamp                   DateTime(0, 'UTC'),

    -- ordering --
    execution_index             UInt32,
    transaction_index           UInt32,
    instruction_index           UInt32,
    global_sequence             UInt64, -- latest global sequence (block_num << 32 + execution_index)

    -- transaction --
    tx_hash                     FixedString(88),

    -- instruction --
    program_id                  LowCardinality(FixedString(44)),

    -- -- authority --
    -- authority                   FixedString(44),
    -- multisig_authority_raw      String, -- comma-separated list of multisig authorities
    -- multisig_authority          Array(FixedString(44)) MATERIALIZED splitByChar(',', multisig_authority_raw),

    -- event --
    amm                         FixedString(44),
    user                        FixedString(44),
    mint_in                     FixedString(44),
    mint_out                    FixedString(44),
    amount_in                   UInt64,
    amount_out                  UInt64,
    direction                   LowCardinality(String), -- 'in' or 'out'
    pool_pc_amount              UInt64,
    pool_coin_amount            UInt64,
    user_pre_balance_in         UInt64,
    user_pre_balance_out        UInt64,

    -- -- event (Optional) --
    -- parent_instruction_index Int64 DEFAULT -1,
    -- top_instruction_index Int64 DEFAULT -1,
    -- parent_instruction_program_id LowCardinality(String) DEFAULT '' CODEC(LZ4),
    -- top_instruction_program_id LowCardinality(String) DEFAULT '' CODEC(LZ4),

    -- indexes --
    INDEX idx_block_num          (block_num)          TYPE minmax GRANULARITY 4,
    INDEX idx_tx_hash            (tx_hash)            TYPE bloom_filter GRANULARITY 4,
    INDEX idx_program_id         (program_id)         TYPE set(2) GRANULARITY 1,

    -- projections --
    PROJECTION projection_amm (SELECT * ORDER BY amm, block_num, transaction_index, instruction_index), -- RECOMMENDED
    PROJECTION projection_user (SELECT * ORDER BY user, block_num, transaction_index, instruction_index), -- RECOMMENDED
    PROJECTION projection_mint_in (SELECT * ORDER BY mint_in, block_num, transaction_index, instruction_index), -- RECOMMENDED
    PROJECTION projection_mint_out (SELECT * ORDER BY mint_out, block_num, transaction_index, instruction_index) -- RECOMMENDED

    -- -- indexes (event) --
    -- INDEX idx_source             (source)             TYPE bloom_filter GRANULARITY 4,
    -- INDEX idx_destination        (destination)        TYPE bloom_filter GRANULARITY 4,
    -- INDEX idx_mint               (mint)               TYPE set(128) GRANULARITY 4,
    -- INDEX idx_amount             (amount)             TYPE minmax GRANULARITY 4,
    -- INDEX idx_decimals           (decimals)           TYPE set(8) GRANULARITY 4
)
ENGINE = MergeTree
ORDER BY (timestamp, block_num, block_hash, execution_index);
