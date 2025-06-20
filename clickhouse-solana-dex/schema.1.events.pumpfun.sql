-- ──────────────────────────────────────────────────────────────────────────
-- Pump.fun Create
-- ──────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS pumpfun_create (
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
    tx_hash                     FixedString(88),
    program_id                  LowCardinality(FixedString(44)),

    -- event ---------------------------------------------------------------
    user                        FixedString(44),
    name                        LowCardinality(String),
    symbol                      LowCardinality(String),
    uri                         String,
    mint                        FixedString(44),
    bonding_curve               FixedString(44),
    associated_bonding_curve    FixedString(44),
    metadata                    String,

    -- indexes -------------------------------------------------------------
    INDEX idx_block_num         (block_num)          TYPE minmax           GRANULARITY 4,
    INDEX idx_tx_hash           (tx_hash)            TYPE bloom_filter     GRANULARITY 4,
    INDEX idx_program_id        (program_id)         TYPE set(2)           GRANULARITY 1,

    INDEX idx_user              (user)               TYPE set(128)         GRANULARITY 4,
    INDEX idx_mint              (mint)               TYPE set(128)         GRANULARITY 4,
    INDEX idx_bonding_curve     (bonding_curve)      TYPE set(128)         GRANULARITY 4,

    -- projections ---------------------------------------------------------
    PROJECTION projection_user  (SELECT * ORDER BY user, timestamp, block_num, execution_index, block_hash),
    PROJECTION projection_mint  (SELECT * ORDER BY mint, timestamp, block_num, execution_index, block_hash),
    PROJECTION projection_bonding_curve (SELECT * ORDER BY bonding_curve, timestamp, block_num, execution_index, block_hash)
)
ENGINE = MergeTree
ORDER BY (timestamp, block_num, execution_index, block_hash);

-- ──────────────────────────────────────────────────────────────────────────
-- Pump.fun Initialize
-- ──────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS pumpfun_initialize (
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
    tx_hash                     FixedString(88),
    program_id                  LowCardinality(FixedString(44)),

    -- event ---------------------------------------------------------------
    user                        FixedString(44),

    -- indexes -------------------------------------------------------------
    INDEX idx_block_num         (block_num)          TYPE minmax           GRANULARITY 4,
    INDEX idx_tx_hash           (tx_hash)            TYPE bloom_filter     GRANULARITY 4,
    INDEX idx_program_id        (program_id)         TYPE set(2)           GRANULARITY 1,
    INDEX idx_user              (user)               TYPE set(128)         GRANULARITY 4,

    -- projections ---------------------------------------------------------
    PROJECTION projection_user  (SELECT * ORDER BY user, timestamp, block_num, execution_index, block_hash)
)
ENGINE = MergeTree
ORDER BY (timestamp, block_num, execution_index, block_hash);

-- ──────────────────────────────────────────────────────────────────────────
-- Pump.fun Set-Params
-- ──────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS pumpfun_set_params (
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
    tx_hash                     FixedString(88),
    program_id                  LowCardinality(FixedString(44)),

    -- event ---------------------------------------------------------------
    user                                FixedString(44),
    fee_recipient                       FixedString(44),
    initial_virtual_token_reserves      UInt64,
    initial_virtual_sol_reserves        UInt64,
    initial_real_token_reserves         UInt64,
    token_total_supply                  UInt64,
    fee_basis_points                    UInt64,

    -- indexes -------------------------------------------------------------
    INDEX idx_block_num         (block_num)          TYPE minmax           GRANULARITY 4,
    INDEX idx_tx_hash           (tx_hash)            TYPE bloom_filter     GRANULARITY 4,
    INDEX idx_program_id        (program_id)         TYPE set(2)           GRANULARITY 1,

    INDEX idx_user              (user)               TYPE set(128)         GRANULARITY 4,
    INDEX idx_fee_recipient     (fee_recipient)      TYPE set(128)         GRANULARITY 4,
    INDEX idx_amounts           (initial_virtual_token_reserves,
                                 initial_virtual_sol_reserves,
                                 initial_real_token_reserves,
                                 token_total_supply,
                                 fee_basis_points)   TYPE minmax           GRANULARITY 4,

    -- projections ---------------------------------------------------------
    PROJECTION projection_user  (SELECT * ORDER BY user, timestamp, block_num, execution_index, block_hash),
    PROJECTION projection_fee_recipient (SELECT * ORDER BY fee_recipient, timestamp, block_num, execution_index, block_hash)
)
ENGINE = MergeTree
ORDER BY (timestamp, block_num, execution_index, block_hash);

-- ──────────────────────────────────────────────────────────────────────────
-- Pump.fun Swap
-- ──────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS pumpfun_swap (
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
    tx_hash                     FixedString(88),
    program_id                  LowCardinality(FixedString(44)),

    -- event ---------------------------------------------------------------
    user                        FixedString(44),
    mint                        FixedString(44),
    bonding_curve               FixedString(44),
    sol_amount                  UInt64              DEFAULT 0,
    token_amount                UInt64,
    direction                   LowCardinality(String),   -- 'buy' | 'sell'
    virtual_sol_reserves        UInt64              DEFAULT 0,
    virtual_token_reserves      UInt64              DEFAULT 0,
    real_sol_reserves           UInt64              DEFAULT 0,
    real_token_reserves         UInt64              DEFAULT 0,
    user_token_pre_balance      UInt64              DEFAULT 0,

    -- indexes -------------------------------------------------------------
    INDEX idx_block_num         (block_num)          TYPE minmax           GRANULARITY 4,
    INDEX idx_tx_hash           (tx_hash)            TYPE bloom_filter     GRANULARITY 4,
    INDEX idx_program_id        (program_id)         TYPE set(2)           GRANULARITY 1,

    INDEX idx_user              (user)               TYPE set(128)         GRANULARITY 4,
    INDEX idx_mint              (mint)               TYPE set(128)         GRANULARITY 4,
    INDEX idx_bonding_curve     (bonding_curve)      TYPE set(128)         GRANULARITY 4,
    INDEX idx_amounts           (sol_amount,
                                 token_amount)       TYPE minmax           GRANULARITY 4,
    INDEX idx_direction         (direction)          TYPE set(1)           GRANULARITY 1,

    -- projections ---------------------------------------------------------
    PROJECTION projection_user  (SELECT * ORDER BY user, timestamp, block_num, execution_index, block_hash),
    PROJECTION projection_mint  (SELECT * ORDER BY mint, timestamp, block_num, execution_index, block_hash),
    PROJECTION projection_bonding_curve (SELECT * ORDER BY bonding_curve, timestamp, block_num, execution_index, block_hash)
)
ENGINE = MergeTree
ORDER BY (timestamp, block_num, execution_index, block_hash);

-- ──────────────────────────────────────────────────────────────────────────
-- Pump.fun Withdraw
-- ──────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS pumpfun_withdraw (
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
    tx_hash                     FixedString(88),
    program_id                  LowCardinality(FixedString(44)),

    -- event ---------------------------------------------------------------
    mint                        FixedString(44),

    -- indexes -------------------------------------------------------------
    INDEX idx_block_num         (block_num)          TYPE minmax           GRANULARITY 4,
    INDEX idx_tx_hash           (tx_hash)            TYPE bloom_filter     GRANULARITY 4,
    INDEX idx_program_id        (program_id)         TYPE set(2)           GRANULARITY 1,
    INDEX idx_mint              (mint)               TYPE set(128)         GRANULARITY 4,

    -- projections ---------------------------------------------------------
    PROJECTION projection_mint  (SELECT * ORDER BY mint, timestamp, block_num, execution_index, block_hash)
)
ENGINE = MergeTree
ORDER BY (timestamp, block_num, execution_index, block_hash);
