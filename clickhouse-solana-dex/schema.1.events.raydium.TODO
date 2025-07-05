-- ──────────────────────────────────────────────────────────────────────────
-- Raydium AMM Swaps  (updated to latest protobuf)
-- ──────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS raydium_amm_v4_swap (
    -- block --
    block_num                   UInt32,
    block_hash                  FixedString(44),
    timestamp                   DateTime(0, 'UTC'),

    -- ordering --
    execution_index             UInt32,
    transaction_index           UInt32,
    instruction_index           UInt32,
    global_sequence             UInt64,

    -- transaction --
    signature                     FixedString(88),
    program_id                  LowCardinality(FixedString(44)),

    -- event --
    amm                         FixedString(44),
    user                        FixedString(44),
    mint_in                     FixedString(44),
    mint_out                    FixedString(44),
    amount_in                   UInt64,
    amount_out                  UInt64,
    direction                   LowCardinality(String),   -- 'in' | 'out'
    pool_pc_amount              UInt64              DEFAULT 0,
    pool_coin_amount            UInt64              DEFAULT 0,
    pc_mint                     FixedString(44),
    coin_mint                   FixedString(44),
    user_pre_balance_in         UInt64              DEFAULT 0,
    user_pre_balance_out        UInt64              DEFAULT 0,

    -- indexes --
    INDEX idx_block_num         (block_num)          TYPE minmax           GRANULARITY 4,
    INDEX idx_signature         (signature)          TYPE bloom_filter     GRANULARITY 4,
    INDEX idx_program_id        (program_id)         TYPE set(2)           GRANULARITY 1,

    INDEX idx_amm               (amm)                TYPE set(128)         GRANULARITY 4,
    INDEX idx_user              (user)               TYPE bloom_filter         GRANULARITY 4,
    INDEX idx_mint_in           (mint_in)            TYPE set(128)         GRANULARITY 4,
    INDEX idx_mint_out          (mint_out)           TYPE set(128)         GRANULARITY 4,
    INDEX idx_pc_mint           (pc_mint)            TYPE set(128)         GRANULARITY 4,
    INDEX idx_coin_mint         (coin_mint)          TYPE set(128)         GRANULARITY 4,
    INDEX idx_amount_in         (amount_in)          TYPE minmax           GRANULARITY 4,
    INDEX idx_amount_out        (amount_out)         TYPE minmax           GRANULARITY 4,
    INDEX idx_amounts           (amount_in, amount_out)     TYPE minmax    GRANULARITY 4,
    INDEX idx_direction         (direction)          TYPE set(1)           GRANULARITY 1,
)
ENGINE = ReplacingMergeTree
ORDER BY (timestamp, block_num, execution_index, block_hash);

-- ──────────────────────────────────────────────────────────────────────────
-- Raydium AMM Initialize
-- ──────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS raydium_amm_v4_initialize (
    -- block ---------------------------------------------------------------
    block_num                   UInt32,
    block_hash                  FixedString(44),
    timestamp                   DateTime(0, 'UTC'),

    -- ordering ------------------------------------------------------------
    execution_index             UInt32,
    transaction_index           UInt32,
    instruction_index           UInt32,
    global_sequence             UInt64,           -- (block_num << 32) | execution_index

    -- transaction ---------------------------------------------------------
    signature                   FixedString(88),
    program_id                  LowCardinality(FixedString(44)),

    -- event ---------------------------------------------------------------
    amm                         FixedString(44),
    user                        FixedString(44),
    pc_init_amount              UInt64,
    coin_init_amount            UInt64,
    lp_init_amount              UInt64,
    pc_mint                     FixedString(44),
    coin_mint                   FixedString(44),
    lp_mint                     FixedString(44),
    nonce                       UInt32,
    market                      FixedString(44)      DEFAULT '',
    user_pc_pre_balance         UInt64               DEFAULT 0,
    user_coin_pre_balance       UInt64               DEFAULT 0,

    -- indexes -------------------------------------------------------------
    INDEX idx_block_num         (block_num)          TYPE minmax           GRANULARITY 4,
    INDEX idx_signature           (signature)            TYPE bloom_filter     GRANULARITY 4,
    INDEX idx_program_id        (program_id)         TYPE set(2)           GRANULARITY 1,

    INDEX idx_amm               (amm)                TYPE set(128)         GRANULARITY 4,
    INDEX idx_user              (user)               TYPE set(128)         GRANULARITY 4,
    INDEX idx_pc_mint           (pc_mint)            TYPE set(128)         GRANULARITY 4,
    INDEX idx_coin_mint         (coin_mint)          TYPE set(128)         GRANULARITY 4,
    INDEX idx_amounts           (pc_init_amount,
                                 coin_init_amount,
                                 lp_init_amount)     TYPE minmax           GRANULARITY 4
)
ENGINE = ReplacingMergeTree
ORDER BY (timestamp, block_num, execution_index, block_hash);


-- ──────────────────────────────────────────────────────────────────────────
-- Raydium AMM Deposit
-- ──────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS raydium_amm_v4_deposit (
    -- block ---------------------------------------------------------------
    block_num                   UInt32,
    block_hash                  FixedString(44),
    timestamp                   DateTime(0, 'UTC'),

    -- ordering ------------------------------------------------------------
    execution_index             UInt32,
    transaction_index           UInt32,
    instruction_index           UInt32,
    global_sequence             UInt64,

    -- transaction ---------------------------------------------------------
    signature                     FixedString(88),
    program_id                  LowCardinality(FixedString(44)),

    -- event ---------------------------------------------------------------
    amm                         FixedString(44),
    user                        FixedString(44),
    pc_amount                   UInt64,
    coin_amount                 UInt64,
    lp_amount                   UInt64,
    pc_mint                     FixedString(44),
    coin_mint                   FixedString(44),
    lp_mint                     FixedString(44),
    pool_pc_amount              UInt64              DEFAULT 0,
    pool_coin_amount            UInt64              DEFAULT 0,
    pool_lp_amount              UInt64              DEFAULT 0,
    user_pc_pre_balance         UInt64              DEFAULT 0,
    user_coin_pre_balance       UInt64              DEFAULT 0,

    -- indexes -------------------------------------------------------------
    INDEX idx_block_num         (block_num)          TYPE minmax           GRANULARITY 4,
    INDEX idx_signature           (signature)            TYPE bloom_filter     GRANULARITY 4,
    INDEX idx_program_id        (program_id)         TYPE set(2)           GRANULARITY 1,

    INDEX idx_amm               (amm)                TYPE set(128)         GRANULARITY 4,
    INDEX idx_user              (user)               TYPE set(128)         GRANULARITY 4,
    INDEX idx_pc_mint           (pc_mint)            TYPE set(128)         GRANULARITY 4,
    INDEX idx_coin_mint         (coin_mint)          TYPE set(128)         GRANULARITY 4,
    INDEX idx_amounts           (pc_amount,
                                 coin_amount,
                                 lp_amount)          TYPE minmax           GRANULARITY 4
)
ENGINE = ReplacingMergeTree
ORDER BY (timestamp, block_num, execution_index, block_hash);

-- ──────────────────────────────────────────────────────────────────────────
-- Raydium AMM Withdraw
-- ──────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS raydium_amm_v4_withdraw (
    -- block ---------------------------------------------------------------
    block_num                   UInt32,
    block_hash                  FixedString(44),
    timestamp                   DateTime(0, 'UTC'),

    -- ordering ------------------------------------------------------------
    execution_index             UInt32,
    transaction_index           UInt32,
    instruction_index           UInt32,
    global_sequence             UInt64,

    -- transaction ---------------------------------------------------------
    signature                     FixedString(88),
    program_id                  LowCardinality(FixedString(44)),

    -- event ---------------------------------------------------------------
    amm                         FixedString(44),
    user                        FixedString(44),
    pc_amount                   UInt64,
    coin_amount                 UInt64,
    lp_amount                   UInt64,
    pc_mint                     FixedString(44),
    coin_mint                   FixedString(44),
    lp_mint                     FixedString(44),
    pool_pc_amount              UInt64              DEFAULT 0,
    pool_coin_amount            UInt64              DEFAULT 0,
    pool_lp_amount              UInt64              DEFAULT 0,
    user_pc_pre_balance         UInt64              DEFAULT 0,
    user_coin_pre_balance       UInt64              DEFAULT 0,

    -- indexes -------------------------------------------------------------
    INDEX idx_block_num         (block_num)          TYPE minmax           GRANULARITY 4,
    INDEX idx_signature           (signature)            TYPE bloom_filter     GRANULARITY 4,
    INDEX idx_program_id        (program_id)         TYPE set(2)           GRANULARITY 1,

    INDEX idx_amm               (amm)                TYPE set(128)         GRANULARITY 4,
    INDEX idx_user              (user)               TYPE set(128)         GRANULARITY 4,
    INDEX idx_pc_mint           (pc_mint)            TYPE set(128)         GRANULARITY 4,
    INDEX idx_coin_mint         (coin_mint)          TYPE set(128)         GRANULARITY 4,
    INDEX idx_amounts           (pc_amount,
                                 coin_amount,
                                 lp_amount)          TYPE minmax           GRANULARITY 4
)
ENGINE = ReplacingMergeTree
ORDER BY (timestamp, block_num, execution_index, block_hash);

-- ──────────────────────────────────────────────────────────────────────────
-- Raydium AMM Withdraw PnL
-- ──────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS raydium_amm_v4_withdraw_pnl (
    -- block ---------------------------------------------------------------
    block_num                   UInt32,
    block_hash                  FixedString(44),
    timestamp                   DateTime(0, 'UTC'),

    -- ordering ------------------------------------------------------------
    execution_index             UInt32,
    transaction_index           UInt32,
    instruction_index           UInt32,
    global_sequence             UInt64,

    -- transaction ---------------------------------------------------------
    signature                     FixedString(88),
    program_id                  LowCardinality(FixedString(44)),

    -- event ---------------------------------------------------------------
    amm                         FixedString(44),
    user                        FixedString(44),
    pc_amount                   UInt64              DEFAULT 0,
    coin_amount                 UInt64              DEFAULT 0,
    pc_mint                     FixedString(44)     DEFAULT '',
    coin_mint                   FixedString(44)     DEFAULT '',

    -- indexes -------------------------------------------------------------
    INDEX idx_block_num         (block_num)          TYPE minmax           GRANULARITY 4,
    INDEX idx_signature           (signature)            TYPE bloom_filter     GRANULARITY 4,
    INDEX idx_program_id        (program_id)         TYPE set(2)           GRANULARITY 1,

    INDEX idx_amm               (amm)                TYPE set(128)         GRANULARITY 4,
    INDEX idx_user              (user)               TYPE set(128)         GRANULARITY 4,
    INDEX idx_pc_mint           (pc_mint)            TYPE set(128)         GRANULARITY 4,
    INDEX idx_coin_mint         (coin_mint)          TYPE set(128)         GRANULARITY 4,
    INDEX idx_amounts           (pc_amount,
                                 coin_amount)        TYPE minmax           GRANULARITY 4
)
ENGINE = ReplacingMergeTree
ORDER BY (timestamp, block_num, execution_index, block_hash);
