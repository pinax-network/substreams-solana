-- Blocks
CREATE TABLE IF NOT EXISTS blocks (
    block_num            INTEGER NOT NULL,
    block_hash           TEXT NOT NULL,
    timestamp            TIMESTAMP NOT NULL,

    PRIMARY KEY (block_num)
);

CREATE INDEX IF NOT EXISTS idx_blocks_block_hash ON blocks (block_hash);
CREATE INDEX IF NOT EXISTS idx_blocks_timestamp ON blocks (timestamp);

-- Jupiter V4 & V6 Swaps
CREATE TABLE IF NOT EXISTS jupiter_swap (
    -- block --
    block_num                   INTEGER NOT NULL,
    block_hash                  TEXT NOT NULL,
    timestamp                   TIMESTAMP NOT NULL,

    -- ordering --
    transaction_index           INTEGER NOT NULL,
    instruction_index           INTEGER NOT NULL,

    -- transaction --
    signature                   TEXT NOT NULL,
    fee_payer                   TEXT NOT NULL,
    signers_raw                 TEXT NOT NULL DEFAULT '',
    fee                         BIGINT NOT NULL DEFAULT 0,
    compute_units_consumed      BIGINT NOT NULL DEFAULT 0,

    -- instruction --
    program_id                  TEXT NOT NULL,
    stack_height                INTEGER NOT NULL,

    -- event --
    amm                         TEXT NOT NULL,
    input_mint                  TEXT NOT NULL,
    input_amount                BIGINT NOT NULL,
    output_mint                 TEXT NOT NULL,
    output_amount               BIGINT NOT NULL,

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_jupiter_swap_block_num ON jupiter_swap (block_num);
CREATE INDEX IF NOT EXISTS idx_jupiter_swap_timestamp ON jupiter_swap (timestamp);
CREATE INDEX IF NOT EXISTS idx_jupiter_swap_signature ON jupiter_swap (signature);
CREATE INDEX IF NOT EXISTS idx_jupiter_swap_fee_payer ON jupiter_swap (fee_payer);
CREATE INDEX IF NOT EXISTS idx_jupiter_swap_input_mint ON jupiter_swap (input_mint);
CREATE INDEX IF NOT EXISTS idx_jupiter_swap_output_mint ON jupiter_swap (output_mint);

-- Pump.fun Buy
CREATE TABLE IF NOT EXISTS pumpfun_buy (
    -- block --
    block_num                   INTEGER NOT NULL,
    block_hash                  TEXT NOT NULL,
    timestamp                   TIMESTAMP NOT NULL,

    -- ordering --
    transaction_index           INTEGER NOT NULL,
    instruction_index           INTEGER NOT NULL,

    -- transaction --
    signature                   TEXT NOT NULL,
    fee_payer                   TEXT NOT NULL,
    signers_raw                 TEXT NOT NULL DEFAULT '',
    fee                         BIGINT NOT NULL DEFAULT 0,
    compute_units_consumed      BIGINT NOT NULL DEFAULT 0,

    -- instruction --
    program_id                  TEXT NOT NULL,
    stack_height                INTEGER NOT NULL,

    -- data --
    amount                      BIGINT NOT NULL,
    max_sol_cost                BIGINT NOT NULL,

    -- accounts --
    global                      TEXT NOT NULL,
    fee_recipient               TEXT NOT NULL,
    mint                        TEXT NOT NULL,
    bonding_curve               TEXT NOT NULL,
    associated_bonding_curve    TEXT NOT NULL,
    associated_user             TEXT NOT NULL,
    "user"                      TEXT NOT NULL,
    creator_vault               TEXT NOT NULL,

    -- event --
    sol_amount                  BIGINT NOT NULL,
    token_amount                BIGINT NOT NULL,
    is_buy                      BOOLEAN NOT NULL,
    virtual_sol_reserves        BIGINT NOT NULL,
    virtual_token_reserves      BIGINT NOT NULL,
    real_sol_reserves           BIGINT NOT NULL DEFAULT 0,
    real_token_reserves         BIGINT NOT NULL DEFAULT 0,
    protocol_fee_recipient      TEXT NOT NULL DEFAULT '',
    protocol_fee_basis_points   BIGINT NOT NULL DEFAULT 0,
    protocol_fee                BIGINT NOT NULL DEFAULT 0,
    creator                     TEXT NOT NULL DEFAULT '',
    creator_fee_basis_points    BIGINT NOT NULL DEFAULT 0,
    creator_fee                 BIGINT NOT NULL DEFAULT 0,

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_pumpfun_buy_block_num ON pumpfun_buy (block_num);
CREATE INDEX IF NOT EXISTS idx_pumpfun_buy_timestamp ON pumpfun_buy (timestamp);
CREATE INDEX IF NOT EXISTS idx_pumpfun_buy_signature ON pumpfun_buy (signature);
CREATE INDEX IF NOT EXISTS idx_pumpfun_buy_mint ON pumpfun_buy (mint);
CREATE INDEX IF NOT EXISTS idx_pumpfun_buy_user ON pumpfun_buy ("user");

-- Pump.fun Sell
CREATE TABLE IF NOT EXISTS pumpfun_sell (
    -- block --
    block_num                   INTEGER NOT NULL,
    block_hash                  TEXT NOT NULL,
    timestamp                   TIMESTAMP NOT NULL,

    -- ordering --
    transaction_index           INTEGER NOT NULL,
    instruction_index           INTEGER NOT NULL,

    -- transaction --
    signature                   TEXT NOT NULL,
    fee_payer                   TEXT NOT NULL,
    signers_raw                 TEXT NOT NULL DEFAULT '',
    fee                         BIGINT NOT NULL DEFAULT 0,
    compute_units_consumed      BIGINT NOT NULL DEFAULT 0,

    -- instruction --
    program_id                  TEXT NOT NULL,
    stack_height                INTEGER NOT NULL,

    -- data --
    amount                      BIGINT NOT NULL,
    min_sol_output              BIGINT NOT NULL,

    -- accounts --
    global                      TEXT NOT NULL,
    fee_recipient               TEXT NOT NULL,
    mint                        TEXT NOT NULL,
    bonding_curve               TEXT NOT NULL,
    associated_bonding_curve    TEXT NOT NULL,
    associated_user             TEXT NOT NULL,
    "user"                      TEXT NOT NULL,
    creator_vault               TEXT NOT NULL,

    -- event --
    sol_amount                  BIGINT NOT NULL,
    token_amount                BIGINT NOT NULL,
    is_buy                      BOOLEAN NOT NULL,
    virtual_sol_reserves        BIGINT NOT NULL,
    virtual_token_reserves      BIGINT NOT NULL,
    real_sol_reserves           BIGINT NOT NULL DEFAULT 0,
    real_token_reserves         BIGINT NOT NULL DEFAULT 0,
    protocol_fee_recipient      TEXT NOT NULL DEFAULT '',
    protocol_fee_basis_points   BIGINT NOT NULL DEFAULT 0,
    protocol_fee                BIGINT NOT NULL DEFAULT 0,
    creator                     TEXT NOT NULL DEFAULT '',
    creator_fee_basis_points    BIGINT NOT NULL DEFAULT 0,
    creator_fee                 BIGINT NOT NULL DEFAULT 0,

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_pumpfun_sell_block_num ON pumpfun_sell (block_num);
CREATE INDEX IF NOT EXISTS idx_pumpfun_sell_timestamp ON pumpfun_sell (timestamp);
CREATE INDEX IF NOT EXISTS idx_pumpfun_sell_signature ON pumpfun_sell (signature);
CREATE INDEX IF NOT EXISTS idx_pumpfun_sell_mint ON pumpfun_sell (mint);
CREATE INDEX IF NOT EXISTS idx_pumpfun_sell_user ON pumpfun_sell ("user");

-- Pump.fun AMM Buy
CREATE TABLE IF NOT EXISTS pumpfun_amm_buy (
    -- block --
    block_num                   INTEGER NOT NULL,
    block_hash                  TEXT NOT NULL,
    timestamp                   TIMESTAMP NOT NULL,

    -- ordering --
    transaction_index           INTEGER NOT NULL,
    instruction_index           INTEGER NOT NULL,

    -- transaction --
    signature                   TEXT NOT NULL,
    fee_payer                   TEXT NOT NULL,
    signers_raw                 TEXT NOT NULL DEFAULT '',
    fee                         BIGINT NOT NULL DEFAULT 0,
    compute_units_consumed      BIGINT NOT NULL DEFAULT 0,

    -- instruction --
    program_id                  TEXT NOT NULL,
    stack_height                INTEGER NOT NULL,

    -- data --
    base_amount_out             BIGINT NOT NULL,
    max_quote_amount_in         BIGINT NOT NULL,

    -- event --
    quote_amount_in             BIGINT NOT NULL,
    quote_amount_in_with_lp_fee BIGINT NOT NULL,
    user_quote_amount_in        BIGINT NOT NULL,

    -- accounts --
    pool                        TEXT NOT NULL,
    "user"                      TEXT NOT NULL,
    global_config               TEXT NOT NULL,
    base_mint                   TEXT NOT NULL,
    quote_mint                  TEXT NOT NULL,
    user_base_token_account     TEXT NOT NULL,
    user_quote_token_account    TEXT NOT NULL,
    pool_base_token_account     TEXT NOT NULL,
    pool_quote_token_account    TEXT NOT NULL,
    protocol_fee_recipient      TEXT NOT NULL,
    protocol_fee_recipient_token_account TEXT NOT NULL,
    coin_creator_vault_ata      TEXT NOT NULL DEFAULT '',
    coin_creator_vault_authority TEXT NOT NULL DEFAULT '',

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_pumpfun_amm_buy_block_num ON pumpfun_amm_buy (block_num);
CREATE INDEX IF NOT EXISTS idx_pumpfun_amm_buy_timestamp ON pumpfun_amm_buy (timestamp);
CREATE INDEX IF NOT EXISTS idx_pumpfun_amm_buy_signature ON pumpfun_amm_buy (signature);
CREATE INDEX IF NOT EXISTS idx_pumpfun_amm_buy_user ON pumpfun_amm_buy ("user");
CREATE INDEX IF NOT EXISTS idx_pumpfun_amm_buy_base_mint ON pumpfun_amm_buy (base_mint);

-- Pump.fun AMM Sell
CREATE TABLE IF NOT EXISTS pumpfun_amm_sell (
    -- block --
    block_num                   INTEGER NOT NULL,
    block_hash                  TEXT NOT NULL,
    timestamp                   TIMESTAMP NOT NULL,

    -- ordering --
    transaction_index           INTEGER NOT NULL,
    instruction_index           INTEGER NOT NULL,

    -- transaction --
    signature                   TEXT NOT NULL,
    fee_payer                   TEXT NOT NULL,
    signers_raw                 TEXT NOT NULL DEFAULT '',
    fee                         BIGINT NOT NULL DEFAULT 0,
    compute_units_consumed      BIGINT NOT NULL DEFAULT 0,

    -- instruction --
    program_id                  TEXT NOT NULL,
    stack_height                INTEGER NOT NULL,

    -- data --
    base_amount_in              BIGINT NOT NULL,
    min_quote_amount_out        BIGINT NOT NULL,

    -- event --
    quote_amount_out            BIGINT NOT NULL,
    quote_amount_out_without_lp_fee BIGINT NOT NULL,
    user_quote_amount_out       BIGINT NOT NULL,

    -- accounts --
    pool                        TEXT NOT NULL,
    "user"                      TEXT NOT NULL,
    global_config               TEXT NOT NULL,
    base_mint                   TEXT NOT NULL,
    quote_mint                  TEXT NOT NULL,
    user_base_token_account     TEXT NOT NULL,
    user_quote_token_account    TEXT NOT NULL,
    pool_base_token_account     TEXT NOT NULL,
    pool_quote_token_account    TEXT NOT NULL,
    protocol_fee_recipient      TEXT NOT NULL,
    protocol_fee_recipient_token_account TEXT NOT NULL,
    coin_creator_vault_ata      TEXT NOT NULL DEFAULT '',
    coin_creator_vault_authority TEXT NOT NULL DEFAULT '',

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_pumpfun_amm_sell_block_num ON pumpfun_amm_sell (block_num);
CREATE INDEX IF NOT EXISTS idx_pumpfun_amm_sell_timestamp ON pumpfun_amm_sell (timestamp);
CREATE INDEX IF NOT EXISTS idx_pumpfun_amm_sell_signature ON pumpfun_amm_sell (signature);
CREATE INDEX IF NOT EXISTS idx_pumpfun_amm_sell_user ON pumpfun_amm_sell ("user");
CREATE INDEX IF NOT EXISTS idx_pumpfun_amm_sell_base_mint ON pumpfun_amm_sell (base_mint);

-- Raydium AMM V4 Swap Base In
CREATE TABLE IF NOT EXISTS raydium_amm_v4_swap_base_in (
    -- block --
    block_num                   INTEGER NOT NULL,
    block_hash                  TEXT NOT NULL,
    timestamp                   TIMESTAMP NOT NULL,

    -- ordering --
    transaction_index           INTEGER NOT NULL,
    instruction_index           INTEGER NOT NULL,

    -- transaction --
    signature                   TEXT NOT NULL,
    fee_payer                   TEXT NOT NULL,
    signers_raw                 TEXT NOT NULL DEFAULT '',
    fee                         BIGINT NOT NULL DEFAULT 0,
    compute_units_consumed      BIGINT NOT NULL DEFAULT 0,

    -- instruction --
    program_id                  TEXT NOT NULL,
    stack_height                INTEGER NOT NULL,

    -- data --
    amount_in                   BIGINT NOT NULL,
    amount_out                  BIGINT NOT NULL,
    minimum_amount_out          BIGINT NOT NULL,

    -- log --
    direction                   TEXT NOT NULL,
    user_source                 BIGINT NOT NULL,
    pool_coin                   BIGINT NOT NULL,
    pool_pc                     BIGINT NOT NULL,

    -- accounts --
    token_program               TEXT NOT NULL,
    amm                         TEXT NOT NULL,
    amm_authority               TEXT NOT NULL,
    amm_open_orders             TEXT NOT NULL,
    amm_coin_vault              TEXT NOT NULL,
    amm_pc_vault                TEXT NOT NULL,
    market_program              TEXT NOT NULL,
    market                      TEXT NOT NULL,
    market_bids                 TEXT NOT NULL,
    market_asks                 TEXT NOT NULL,
    market_event_queue          TEXT NOT NULL,
    market_coin_vault           TEXT NOT NULL,
    market_pc_vault             TEXT NOT NULL,
    market_vault_signer         TEXT NOT NULL,
    user_token_source           TEXT NOT NULL,
    user_token_destination      TEXT NOT NULL,
    user_source_owner           TEXT NOT NULL,
    amm_target_orders           TEXT NOT NULL DEFAULT '',

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_raydium_amm_v4_swap_base_in_block_num ON raydium_amm_v4_swap_base_in (block_num);
CREATE INDEX IF NOT EXISTS idx_raydium_amm_v4_swap_base_in_timestamp ON raydium_amm_v4_swap_base_in (timestamp);
CREATE INDEX IF NOT EXISTS idx_raydium_amm_v4_swap_base_in_signature ON raydium_amm_v4_swap_base_in (signature);
CREATE INDEX IF NOT EXISTS idx_raydium_amm_v4_swap_base_in_amm ON raydium_amm_v4_swap_base_in (amm);

-- Raydium AMM V4 Swap Base Out
CREATE TABLE IF NOT EXISTS raydium_amm_v4_swap_base_out (
    -- block --
    block_num                   INTEGER NOT NULL,
    block_hash                  TEXT NOT NULL,
    timestamp                   TIMESTAMP NOT NULL,

    -- ordering --
    transaction_index           INTEGER NOT NULL,
    instruction_index           INTEGER NOT NULL,

    -- transaction --
    signature                   TEXT NOT NULL,
    fee_payer                   TEXT NOT NULL,
    signers_raw                 TEXT NOT NULL DEFAULT '',
    fee                         BIGINT NOT NULL DEFAULT 0,
    compute_units_consumed      BIGINT NOT NULL DEFAULT 0,

    -- instruction --
    program_id                  TEXT NOT NULL,
    stack_height                INTEGER NOT NULL,

    -- data --
    amount_in                   BIGINT NOT NULL,
    amount_out                  BIGINT NOT NULL,
    max_amount_in               BIGINT NOT NULL,

    -- log --
    direction                   TEXT NOT NULL,
    user_source                 BIGINT NOT NULL,
    pool_coin                   BIGINT NOT NULL,
    pool_pc                     BIGINT NOT NULL,

    -- accounts --
    token_program               TEXT NOT NULL,
    amm                         TEXT NOT NULL,
    amm_authority               TEXT NOT NULL,
    amm_open_orders             TEXT NOT NULL,
    amm_coin_vault              TEXT NOT NULL,
    amm_pc_vault                TEXT NOT NULL,
    market_program              TEXT NOT NULL,
    market                      TEXT NOT NULL,
    market_bids                 TEXT NOT NULL,
    market_asks                 TEXT NOT NULL,
    market_event_queue          TEXT NOT NULL,
    market_coin_vault           TEXT NOT NULL,
    market_pc_vault             TEXT NOT NULL,
    market_vault_signer         TEXT NOT NULL,
    user_token_source           TEXT NOT NULL,
    user_token_destination      TEXT NOT NULL,
    user_source_owner           TEXT NOT NULL,
    amm_target_orders           TEXT NOT NULL DEFAULT '',

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_raydium_amm_v4_swap_base_out_block_num ON raydium_amm_v4_swap_base_out (block_num);
CREATE INDEX IF NOT EXISTS idx_raydium_amm_v4_swap_base_out_timestamp ON raydium_amm_v4_swap_base_out (timestamp);
CREATE INDEX IF NOT EXISTS idx_raydium_amm_v4_swap_base_out_signature ON raydium_amm_v4_swap_base_out (signature);
CREATE INDEX IF NOT EXISTS idx_raydium_amm_v4_swap_base_out_amm ON raydium_amm_v4_swap_base_out (amm);

-- Raydium CLMM Swap
CREATE TABLE IF NOT EXISTS raydium_clmm_swap (
    -- block --
    block_num                   INTEGER NOT NULL,
    block_hash                  TEXT NOT NULL,
    timestamp                   TIMESTAMP NOT NULL,

    -- ordering --
    transaction_index           INTEGER NOT NULL,
    instruction_index           INTEGER NOT NULL,

    -- transaction --
    signature                   TEXT NOT NULL,
    fee_payer                   TEXT NOT NULL,
    signers_raw                 TEXT NOT NULL DEFAULT '',
    fee                         BIGINT NOT NULL DEFAULT 0,
    compute_units_consumed      BIGINT NOT NULL DEFAULT 0,

    -- instruction --
    program_id                  TEXT NOT NULL,
    stack_height                INTEGER NOT NULL,

    -- event --
    payer                       TEXT NOT NULL,
    pool_state                  TEXT NOT NULL,
    input_mint                  TEXT NOT NULL,
    output_mint                 TEXT NOT NULL,
    amount_in                   BIGINT NOT NULL,
    amount_out                  BIGINT NOT NULL,

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_raydium_clmm_swap_block_num ON raydium_clmm_swap (block_num);
CREATE INDEX IF NOT EXISTS idx_raydium_clmm_swap_timestamp ON raydium_clmm_swap (timestamp);
CREATE INDEX IF NOT EXISTS idx_raydium_clmm_swap_signature ON raydium_clmm_swap (signature);
CREATE INDEX IF NOT EXISTS idx_raydium_clmm_swap_pool_state ON raydium_clmm_swap (pool_state);
CREATE INDEX IF NOT EXISTS idx_raydium_clmm_swap_input_mint ON raydium_clmm_swap (input_mint);
CREATE INDEX IF NOT EXISTS idx_raydium_clmm_swap_output_mint ON raydium_clmm_swap (output_mint);

-- Raydium CPMM Swap Base In
CREATE TABLE IF NOT EXISTS raydium_cpmm_swap_base_in (
    -- block --
    block_num                   INTEGER NOT NULL,
    block_hash                  TEXT NOT NULL,
    timestamp                   TIMESTAMP NOT NULL,

    -- ordering --
    transaction_index           INTEGER NOT NULL,
    instruction_index           INTEGER NOT NULL,

    -- transaction --
    signature                   TEXT NOT NULL,
    fee_payer                   TEXT NOT NULL,
    signers_raw                 TEXT NOT NULL DEFAULT '',
    fee                         BIGINT NOT NULL DEFAULT 0,
    compute_units_consumed      BIGINT NOT NULL DEFAULT 0,

    -- instruction --
    program_id                  TEXT NOT NULL,
    stack_height                INTEGER NOT NULL,

    -- event --
    payer                       TEXT NOT NULL,
    pool_state                  TEXT NOT NULL,
    input_token_mint            TEXT NOT NULL,
    output_token_mint           TEXT NOT NULL,
    amount_in                   BIGINT NOT NULL,
    amount_out                  BIGINT NOT NULL,

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_raydium_cpmm_swap_base_in_block_num ON raydium_cpmm_swap_base_in (block_num);
CREATE INDEX IF NOT EXISTS idx_raydium_cpmm_swap_base_in_timestamp ON raydium_cpmm_swap_base_in (timestamp);
CREATE INDEX IF NOT EXISTS idx_raydium_cpmm_swap_base_in_signature ON raydium_cpmm_swap_base_in (signature);
CREATE INDEX IF NOT EXISTS idx_raydium_cpmm_swap_base_in_pool_state ON raydium_cpmm_swap_base_in (pool_state);

-- Raydium CPMM Swap Base Out
CREATE TABLE IF NOT EXISTS raydium_cpmm_swap_base_out (
    -- block --
    block_num                   INTEGER NOT NULL,
    block_hash                  TEXT NOT NULL,
    timestamp                   TIMESTAMP NOT NULL,

    -- ordering --
    transaction_index           INTEGER NOT NULL,
    instruction_index           INTEGER NOT NULL,

    -- transaction --
    signature                   TEXT NOT NULL,
    fee_payer                   TEXT NOT NULL,
    signers_raw                 TEXT NOT NULL DEFAULT '',
    fee                         BIGINT NOT NULL DEFAULT 0,
    compute_units_consumed      BIGINT NOT NULL DEFAULT 0,

    -- instruction --
    program_id                  TEXT NOT NULL,
    stack_height                INTEGER NOT NULL,

    -- event --
    payer                       TEXT NOT NULL,
    pool_state                  TEXT NOT NULL,
    input_token_mint            TEXT NOT NULL,
    output_token_mint           TEXT NOT NULL,
    amount_in                   BIGINT NOT NULL,
    amount_out                  BIGINT NOT NULL,

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_raydium_cpmm_swap_base_out_block_num ON raydium_cpmm_swap_base_out (block_num);
CREATE INDEX IF NOT EXISTS idx_raydium_cpmm_swap_base_out_timestamp ON raydium_cpmm_swap_base_out (timestamp);
CREATE INDEX IF NOT EXISTS idx_raydium_cpmm_swap_base_out_signature ON raydium_cpmm_swap_base_out (signature);
CREATE INDEX IF NOT EXISTS idx_raydium_cpmm_swap_base_out_pool_state ON raydium_cpmm_swap_base_out (pool_state);

-- Raydium Launchpad Buy
CREATE TABLE IF NOT EXISTS raydium_launchpad_buy (
    -- block --
    block_num                   INTEGER NOT NULL,
    block_hash                  TEXT NOT NULL,
    timestamp                   TIMESTAMP NOT NULL,

    -- ordering --
    transaction_index           INTEGER NOT NULL,
    instruction_index           INTEGER NOT NULL,

    -- transaction --
    signature                   TEXT NOT NULL,
    fee_payer                   TEXT NOT NULL,
    signers_raw                 TEXT NOT NULL DEFAULT '',
    fee                         BIGINT NOT NULL DEFAULT 0,
    compute_units_consumed      BIGINT NOT NULL DEFAULT 0,

    -- instruction --
    program_id                  TEXT NOT NULL,
    stack_height                INTEGER NOT NULL,

    -- event --
    payer                       TEXT NOT NULL,
    pool_state                  TEXT NOT NULL,
    base_token_mint             TEXT NOT NULL,
    quote_token_mint            TEXT NOT NULL,
    amount_in                   BIGINT NOT NULL,
    amount_out                  BIGINT NOT NULL,
    exact_in                    BOOLEAN NOT NULL,

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_raydium_launchpad_buy_block_num ON raydium_launchpad_buy (block_num);
CREATE INDEX IF NOT EXISTS idx_raydium_launchpad_buy_timestamp ON raydium_launchpad_buy (timestamp);
CREATE INDEX IF NOT EXISTS idx_raydium_launchpad_buy_signature ON raydium_launchpad_buy (signature);
CREATE INDEX IF NOT EXISTS idx_raydium_launchpad_buy_pool_state ON raydium_launchpad_buy (pool_state);

-- Raydium Launchpad Sell
CREATE TABLE IF NOT EXISTS raydium_launchpad_sell (
    -- block --
    block_num                   INTEGER NOT NULL,
    block_hash                  TEXT NOT NULL,
    timestamp                   TIMESTAMP NOT NULL,

    -- ordering --
    transaction_index           INTEGER NOT NULL,
    instruction_index           INTEGER NOT NULL,

    -- transaction --
    signature                   TEXT NOT NULL,
    fee_payer                   TEXT NOT NULL,
    signers_raw                 TEXT NOT NULL DEFAULT '',
    fee                         BIGINT NOT NULL DEFAULT 0,
    compute_units_consumed      BIGINT NOT NULL DEFAULT 0,

    -- instruction --
    program_id                  TEXT NOT NULL,
    stack_height                INTEGER NOT NULL,

    -- event --
    payer                       TEXT NOT NULL,
    pool_state                  TEXT NOT NULL,
    base_token_mint             TEXT NOT NULL,
    quote_token_mint            TEXT NOT NULL,
    amount_in                   BIGINT NOT NULL,
    amount_out                  BIGINT NOT NULL,
    exact_in                    BOOLEAN NOT NULL,

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_raydium_launchpad_sell_block_num ON raydium_launchpad_sell (block_num);
CREATE INDEX IF NOT EXISTS idx_raydium_launchpad_sell_timestamp ON raydium_launchpad_sell (timestamp);
CREATE INDEX IF NOT EXISTS idx_raydium_launchpad_sell_signature ON raydium_launchpad_sell (signature);
CREATE INDEX IF NOT EXISTS idx_raydium_launchpad_sell_pool_state ON raydium_launchpad_sell (pool_state);

-- Meteora DLLM Swap
CREATE TABLE IF NOT EXISTS meteora_dllm_swap (
    -- block --
    block_num                   INTEGER NOT NULL,
    block_hash                  TEXT NOT NULL,
    timestamp                   TIMESTAMP NOT NULL,

    -- ordering --
    transaction_index           INTEGER NOT NULL,
    instruction_index           INTEGER NOT NULL,

    -- transaction --
    signature                   TEXT NOT NULL,
    fee_payer                   TEXT NOT NULL,
    signers_raw                 TEXT NOT NULL DEFAULT '',
    fee                         BIGINT NOT NULL DEFAULT 0,
    compute_units_consumed      BIGINT NOT NULL DEFAULT 0,

    -- instruction --
    program_id                  TEXT NOT NULL,
    stack_height                INTEGER NOT NULL,

    -- event --
    "user"                      TEXT NOT NULL,
    lb_pair                     TEXT NOT NULL,
    input_mint                  TEXT NOT NULL,
    output_mint                 TEXT NOT NULL,
    amount_in                   BIGINT NOT NULL,
    amount_out                  BIGINT NOT NULL,

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_meteora_dllm_swap_block_num ON meteora_dllm_swap (block_num);
CREATE INDEX IF NOT EXISTS idx_meteora_dllm_swap_timestamp ON meteora_dllm_swap (timestamp);
CREATE INDEX IF NOT EXISTS idx_meteora_dllm_swap_signature ON meteora_dllm_swap (signature);
CREATE INDEX IF NOT EXISTS idx_meteora_dllm_swap_lb_pair ON meteora_dllm_swap (lb_pair);
CREATE INDEX IF NOT EXISTS idx_meteora_dllm_swap_input_mint ON meteora_dllm_swap (input_mint);
CREATE INDEX IF NOT EXISTS idx_meteora_dllm_swap_output_mint ON meteora_dllm_swap (output_mint);

-- Meteora DAAM Swap
CREATE TABLE IF NOT EXISTS meteora_daam_swap (
    -- block --
    block_num                   INTEGER NOT NULL,
    block_hash                  TEXT NOT NULL,
    timestamp                   TIMESTAMP NOT NULL,

    -- ordering --
    transaction_index           INTEGER NOT NULL,
    instruction_index           INTEGER NOT NULL,

    -- transaction --
    signature                   TEXT NOT NULL,
    fee_payer                   TEXT NOT NULL,
    signers_raw                 TEXT NOT NULL DEFAULT '',
    fee                         BIGINT NOT NULL DEFAULT 0,
    compute_units_consumed      BIGINT NOT NULL DEFAULT 0,

    -- instruction --
    program_id                  TEXT NOT NULL,
    stack_height                INTEGER NOT NULL,

    -- event --
    payer                       TEXT NOT NULL,
    pool                        TEXT NOT NULL,
    input_mint                  TEXT NOT NULL,
    output_mint                 TEXT NOT NULL,
    amount_in                   BIGINT NOT NULL,
    amount_out                  BIGINT NOT NULL,

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_meteora_daam_swap_block_num ON meteora_daam_swap (block_num);
CREATE INDEX IF NOT EXISTS idx_meteora_daam_swap_timestamp ON meteora_daam_swap (timestamp);
CREATE INDEX IF NOT EXISTS idx_meteora_daam_swap_signature ON meteora_daam_swap (signature);
CREATE INDEX IF NOT EXISTS idx_meteora_daam_swap_pool ON meteora_daam_swap (pool);
CREATE INDEX IF NOT EXISTS idx_meteora_daam_swap_input_mint ON meteora_daam_swap (input_mint);
CREATE INDEX IF NOT EXISTS idx_meteora_daam_swap_output_mint ON meteora_daam_swap (output_mint);

-- Meteora AMM Swap
CREATE TABLE IF NOT EXISTS meteora_amm_swap (
    -- block --
    block_num                   INTEGER NOT NULL,
    block_hash                  TEXT NOT NULL,
    timestamp                   TIMESTAMP NOT NULL,

    -- ordering --
    transaction_index           INTEGER NOT NULL,
    instruction_index           INTEGER NOT NULL,

    -- transaction --
    signature                   TEXT NOT NULL,
    fee_payer                   TEXT NOT NULL,
    signers_raw                 TEXT NOT NULL DEFAULT '',
    fee                         BIGINT NOT NULL DEFAULT 0,
    compute_units_consumed      BIGINT NOT NULL DEFAULT 0,

    -- instruction --
    program_id                  TEXT NOT NULL,
    stack_height                INTEGER NOT NULL,

    -- event --
    "user"                      TEXT NOT NULL,
    pool                        TEXT NOT NULL,
    input_mint                  TEXT NOT NULL,
    output_mint                 TEXT NOT NULL,
    amount_in                   BIGINT NOT NULL,
    amount_out                  BIGINT NOT NULL,

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_meteora_amm_swap_block_num ON meteora_amm_swap (block_num);
CREATE INDEX IF NOT EXISTS idx_meteora_amm_swap_timestamp ON meteora_amm_swap (timestamp);
CREATE INDEX IF NOT EXISTS idx_meteora_amm_swap_signature ON meteora_amm_swap (signature);
CREATE INDEX IF NOT EXISTS idx_meteora_amm_swap_pool ON meteora_amm_swap (pool);
CREATE INDEX IF NOT EXISTS idx_meteora_amm_swap_input_mint ON meteora_amm_swap (input_mint);
CREATE INDEX IF NOT EXISTS idx_meteora_amm_swap_output_mint ON meteora_amm_swap (output_mint);

-- Orca Swap
CREATE TABLE IF NOT EXISTS orca_swap (
    -- block --
    block_num                   INTEGER NOT NULL,
    block_hash                  TEXT NOT NULL,
    timestamp                   TIMESTAMP NOT NULL,

    -- ordering --
    transaction_index           INTEGER NOT NULL,
    instruction_index           INTEGER NOT NULL,

    -- transaction --
    signature                   TEXT NOT NULL,
    fee_payer                   TEXT NOT NULL,
    signers_raw                 TEXT NOT NULL DEFAULT '',
    fee                         BIGINT NOT NULL DEFAULT 0,
    compute_units_consumed      BIGINT NOT NULL DEFAULT 0,

    -- instruction --
    program_id                  TEXT NOT NULL,
    stack_height                INTEGER NOT NULL,

    -- event --
    "user"                      TEXT NOT NULL,
    whirlpool                   TEXT NOT NULL,
    input_mint                  TEXT NOT NULL,
    output_mint                 TEXT NOT NULL,
    amount_in                   BIGINT NOT NULL,
    amount_out                  BIGINT NOT NULL,

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_orca_swap_block_num ON orca_swap (block_num);
CREATE INDEX IF NOT EXISTS idx_orca_swap_timestamp ON orca_swap (timestamp);
CREATE INDEX IF NOT EXISTS idx_orca_swap_signature ON orca_swap (signature);
CREATE INDEX IF NOT EXISTS idx_orca_swap_whirlpool ON orca_swap (whirlpool);
CREATE INDEX IF NOT EXISTS idx_orca_swap_input_mint ON orca_swap (input_mint);
CREATE INDEX IF NOT EXISTS idx_orca_swap_output_mint ON orca_swap (output_mint);

-- Phoenix Swap
CREATE TABLE IF NOT EXISTS phoenix_swap (
    -- block --
    block_num                   INTEGER NOT NULL,
    block_hash                  TEXT NOT NULL,
    timestamp                   TIMESTAMP NOT NULL,

    -- ordering --
    transaction_index           INTEGER NOT NULL,
    instruction_index           INTEGER NOT NULL,

    -- transaction --
    signature                   TEXT NOT NULL,
    fee_payer                   TEXT NOT NULL,
    signers_raw                 TEXT NOT NULL DEFAULT '',
    fee                         BIGINT NOT NULL DEFAULT 0,
    compute_units_consumed      BIGINT NOT NULL DEFAULT 0,

    -- instruction --
    program_id                  TEXT NOT NULL,
    stack_height                INTEGER NOT NULL,

    -- event --
    trader                      TEXT NOT NULL,
    market                      TEXT NOT NULL,
    base_account                TEXT NOT NULL,
    quote_account               TEXT NOT NULL,

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_phoenix_swap_block_num ON phoenix_swap (block_num);
CREATE INDEX IF NOT EXISTS idx_phoenix_swap_timestamp ON phoenix_swap (timestamp);
CREATE INDEX IF NOT EXISTS idx_phoenix_swap_signature ON phoenix_swap (signature);
CREATE INDEX IF NOT EXISTS idx_phoenix_swap_market ON phoenix_swap (market);
CREATE INDEX IF NOT EXISTS idx_phoenix_swap_trader ON phoenix_swap (trader);

-- OpenBook Fill
CREATE TABLE IF NOT EXISTS openbook_fill (
    -- block --
    block_num                   INTEGER NOT NULL,
    block_hash                  TEXT NOT NULL,
    timestamp                   TIMESTAMP NOT NULL,

    -- ordering --
    transaction_index           INTEGER NOT NULL,
    instruction_index           INTEGER NOT NULL,

    -- transaction --
    signature                   TEXT NOT NULL,
    fee_payer                   TEXT NOT NULL,
    signers_raw                 TEXT NOT NULL DEFAULT '',
    fee                         BIGINT NOT NULL DEFAULT 0,
    compute_units_consumed      BIGINT NOT NULL DEFAULT 0,

    -- instruction --
    program_id                  TEXT NOT NULL,
    stack_height                INTEGER NOT NULL,

    -- event --
    market                      TEXT NOT NULL,
    maker                       TEXT NOT NULL,
    taker                       TEXT NOT NULL,
    price                       BIGINT NOT NULL,
    quantity                    BIGINT NOT NULL,
    taker_side                  INTEGER NOT NULL,
    seq_num                     BIGINT NOT NULL,

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_openbook_fill_block_num ON openbook_fill (block_num);
CREATE INDEX IF NOT EXISTS idx_openbook_fill_timestamp ON openbook_fill (timestamp);
CREATE INDEX IF NOT EXISTS idx_openbook_fill_signature ON openbook_fill (signature);
CREATE INDEX IF NOT EXISTS idx_openbook_fill_market ON openbook_fill (market);

-- OpenBook Total Order Fill
CREATE TABLE IF NOT EXISTS openbook_total_order_fill (
    -- block --
    block_num                   INTEGER NOT NULL,
    block_hash                  TEXT NOT NULL,
    timestamp                   TIMESTAMP NOT NULL,

    -- ordering --
    transaction_index           INTEGER NOT NULL,
    instruction_index           INTEGER NOT NULL,

    -- transaction --
    signature                   TEXT NOT NULL,
    fee_payer                   TEXT NOT NULL,
    signers_raw                 TEXT NOT NULL DEFAULT '',
    fee                         BIGINT NOT NULL DEFAULT 0,
    compute_units_consumed      BIGINT NOT NULL DEFAULT 0,

    -- instruction --
    program_id                  TEXT NOT NULL,
    stack_height                INTEGER NOT NULL,

    -- event --
    taker                       TEXT NOT NULL,
    side                        INTEGER NOT NULL,
    total_quantity_paid         BIGINT NOT NULL,
    total_quantity_received     BIGINT NOT NULL,
    fees                        BIGINT NOT NULL,

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_openbook_total_order_fill_block_num ON openbook_total_order_fill (block_num);
CREATE INDEX IF NOT EXISTS idx_openbook_total_order_fill_timestamp ON openbook_total_order_fill (timestamp);
CREATE INDEX IF NOT EXISTS idx_openbook_total_order_fill_signature ON openbook_total_order_fill (signature);
CREATE INDEX IF NOT EXISTS idx_openbook_total_order_fill_taker ON openbook_total_order_fill (taker);

-- PumpSwap Buy
CREATE TABLE IF NOT EXISTS pumpswap_buy (
    -- block --
    block_num                   INTEGER NOT NULL,
    block_hash                  TEXT NOT NULL,
    timestamp                   TIMESTAMP NOT NULL,

    -- ordering --
    transaction_index           INTEGER NOT NULL,
    instruction_index           INTEGER NOT NULL,

    -- transaction --
    signature                   TEXT NOT NULL,
    fee_payer                   TEXT NOT NULL,
    signers_raw                 TEXT NOT NULL DEFAULT '',
    fee                         BIGINT NOT NULL DEFAULT 0,
    compute_units_consumed      BIGINT NOT NULL DEFAULT 0,

    -- instruction --
    program_id                  TEXT NOT NULL,
    stack_height                INTEGER NOT NULL,

    -- event --
    pool                        TEXT NOT NULL,
    "user"                      TEXT NOT NULL,
    base_amount_out             BIGINT NOT NULL,
    quote_amount_in             BIGINT NOT NULL,
    lp_fee                      BIGINT NOT NULL,
    protocol_fee                BIGINT NOT NULL,
    coin_creator_fee            BIGINT NOT NULL,
    pool_base_token_reserves    BIGINT NOT NULL,
    pool_quote_token_reserves   BIGINT NOT NULL,

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_pumpswap_buy_block_num ON pumpswap_buy (block_num);
CREATE INDEX IF NOT EXISTS idx_pumpswap_buy_timestamp ON pumpswap_buy (timestamp);
CREATE INDEX IF NOT EXISTS idx_pumpswap_buy_signature ON pumpswap_buy (signature);
CREATE INDEX IF NOT EXISTS idx_pumpswap_buy_pool ON pumpswap_buy (pool);

-- PumpSwap Sell
CREATE TABLE IF NOT EXISTS pumpswap_sell (
    -- block --
    block_num                   INTEGER NOT NULL,
    block_hash                  TEXT NOT NULL,
    timestamp                   TIMESTAMP NOT NULL,

    -- ordering --
    transaction_index           INTEGER NOT NULL,
    instruction_index           INTEGER NOT NULL,

    -- transaction --
    signature                   TEXT NOT NULL,
    fee_payer                   TEXT NOT NULL,
    signers_raw                 TEXT NOT NULL DEFAULT '',
    fee                         BIGINT NOT NULL DEFAULT 0,
    compute_units_consumed      BIGINT NOT NULL DEFAULT 0,

    -- instruction --
    program_id                  TEXT NOT NULL,
    stack_height                INTEGER NOT NULL,

    -- event --
    pool                        TEXT NOT NULL,
    "user"                      TEXT NOT NULL,
    base_amount_in              BIGINT NOT NULL,
    quote_amount_out            BIGINT NOT NULL,
    lp_fee                      BIGINT NOT NULL,
    protocol_fee                BIGINT NOT NULL,
    coin_creator_fee            BIGINT NOT NULL,
    pool_base_token_reserves    BIGINT NOT NULL,
    pool_quote_token_reserves   BIGINT NOT NULL,

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_pumpswap_sell_block_num ON pumpswap_sell (block_num);
CREATE INDEX IF NOT EXISTS idx_pumpswap_sell_timestamp ON pumpswap_sell (timestamp);
CREATE INDEX IF NOT EXISTS idx_pumpswap_sell_signature ON pumpswap_sell (signature);
CREATE INDEX IF NOT EXISTS idx_pumpswap_sell_pool ON pumpswap_sell (pool);

-- Darklake Swap
CREATE TABLE IF NOT EXISTS darklake_swap (
    -- block --
    block_num                   INTEGER NOT NULL,
    block_hash                  TEXT NOT NULL,
    timestamp                   TIMESTAMP NOT NULL,

    -- ordering --
    transaction_index           INTEGER NOT NULL,
    instruction_index           INTEGER NOT NULL,

    -- transaction --
    signature                   TEXT NOT NULL,
    fee_payer                   TEXT NOT NULL,
    signers_raw                 TEXT NOT NULL DEFAULT '',
    fee                         BIGINT NOT NULL DEFAULT 0,
    compute_units_consumed      BIGINT NOT NULL DEFAULT 0,

    -- instruction --
    program_id                  TEXT NOT NULL,
    stack_height                INTEGER NOT NULL,

    -- event --
    trader                      TEXT NOT NULL,
    amount_in                   BIGINT NOT NULL,
    amount_out                  BIGINT NOT NULL,
    token_mint_x                TEXT NOT NULL,
    token_mint_y                TEXT NOT NULL,
    direction                   INTEGER NOT NULL,
    trade_fee                   BIGINT NOT NULL,
    protocol_fee                BIGINT NOT NULL,

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_darklake_swap_block_num ON darklake_swap (block_num);
CREATE INDEX IF NOT EXISTS idx_darklake_swap_timestamp ON darklake_swap (timestamp);
CREATE INDEX IF NOT EXISTS idx_darklake_swap_signature ON darklake_swap (signature);
CREATE INDEX IF NOT EXISTS idx_darklake_swap_trader ON darklake_swap (trader);

-- Lifinity Swap
CREATE TABLE IF NOT EXISTS lifinity_swap (
    -- block --
    block_num                   INTEGER NOT NULL,
    block_hash                  TEXT NOT NULL,
    timestamp                   TIMESTAMP NOT NULL,

    -- ordering --
    transaction_index           INTEGER NOT NULL,
    instruction_index           INTEGER NOT NULL,

    -- transaction --
    signature                   TEXT NOT NULL,
    fee_payer                   TEXT NOT NULL,
    signers_raw                 TEXT NOT NULL DEFAULT '',
    fee                         BIGINT NOT NULL DEFAULT 0,
    compute_units_consumed      BIGINT NOT NULL DEFAULT 0,

    -- instruction --
    program_id                  TEXT NOT NULL,
    stack_height                INTEGER NOT NULL,

    -- event --
    "user"                      TEXT NOT NULL,
    amm                         TEXT NOT NULL,
    swap_source                 TEXT NOT NULL,
    swap_destination            TEXT NOT NULL,
    amount_in                   BIGINT NOT NULL,
    minimum_amount_out          BIGINT NOT NULL,

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_lifinity_swap_block_num ON lifinity_swap (block_num);
CREATE INDEX IF NOT EXISTS idx_lifinity_swap_timestamp ON lifinity_swap (timestamp);
CREATE INDEX IF NOT EXISTS idx_lifinity_swap_signature ON lifinity_swap (signature);
CREATE INDEX IF NOT EXISTS idx_lifinity_swap_amm ON lifinity_swap (amm);

-- Moonshot Buy
CREATE TABLE IF NOT EXISTS moonshot_buy (
    -- block --
    block_num                   INTEGER NOT NULL,
    block_hash                  TEXT NOT NULL,
    timestamp                   TIMESTAMP NOT NULL,

    -- ordering --
    transaction_index           INTEGER NOT NULL,
    instruction_index           INTEGER NOT NULL,

    -- transaction --
    signature                   TEXT NOT NULL,
    fee_payer                   TEXT NOT NULL,
    signers_raw                 TEXT NOT NULL DEFAULT '',
    fee                         BIGINT NOT NULL DEFAULT 0,
    compute_units_consumed      BIGINT NOT NULL DEFAULT 0,

    -- instruction --
    program_id                  TEXT NOT NULL,
    stack_height                INTEGER NOT NULL,

    -- event --
    amount                      BIGINT NOT NULL,
    collateral_amount           BIGINT NOT NULL,
    dex_fee                     BIGINT NOT NULL,
    helio_fee                   BIGINT NOT NULL,
    sender                      TEXT NOT NULL,
    trade_type                  INTEGER NOT NULL,
    cost_token                  TEXT NOT NULL,
    curve                       TEXT NOT NULL,

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_moonshot_buy_block_num ON moonshot_buy (block_num);
CREATE INDEX IF NOT EXISTS idx_moonshot_buy_timestamp ON moonshot_buy (timestamp);
CREATE INDEX IF NOT EXISTS idx_moonshot_buy_signature ON moonshot_buy (signature);
CREATE INDEX IF NOT EXISTS idx_moonshot_buy_sender ON moonshot_buy (sender);

-- Moonshot Sell
CREATE TABLE IF NOT EXISTS moonshot_sell (
    -- block --
    block_num                   INTEGER NOT NULL,
    block_hash                  TEXT NOT NULL,
    timestamp                   TIMESTAMP NOT NULL,

    -- ordering --
    transaction_index           INTEGER NOT NULL,
    instruction_index           INTEGER NOT NULL,

    -- transaction --
    signature                   TEXT NOT NULL,
    fee_payer                   TEXT NOT NULL,
    signers_raw                 TEXT NOT NULL DEFAULT '',
    fee                         BIGINT NOT NULL DEFAULT 0,
    compute_units_consumed      BIGINT NOT NULL DEFAULT 0,

    -- instruction --
    program_id                  TEXT NOT NULL,
    stack_height                INTEGER NOT NULL,

    -- event --
    amount                      BIGINT NOT NULL,
    collateral_amount           BIGINT NOT NULL,
    dex_fee                     BIGINT NOT NULL,
    helio_fee                   BIGINT NOT NULL,
    sender                      TEXT NOT NULL,
    trade_type                  INTEGER NOT NULL,
    cost_token                  TEXT NOT NULL,
    curve                       TEXT NOT NULL,

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_moonshot_sell_block_num ON moonshot_sell (block_num);
CREATE INDEX IF NOT EXISTS idx_moonshot_sell_timestamp ON moonshot_sell (timestamp);
CREATE INDEX IF NOT EXISTS idx_moonshot_sell_signature ON moonshot_sell (signature);
CREATE INDEX IF NOT EXISTS idx_moonshot_sell_sender ON moonshot_sell (sender);

-- PancakeSwap Swap
CREATE TABLE IF NOT EXISTS pancakeswap_swap (
    -- block --
    block_num                   INTEGER NOT NULL,
    block_hash                  TEXT NOT NULL,
    timestamp                   TIMESTAMP NOT NULL,

    -- ordering --
    transaction_index           INTEGER NOT NULL,
    instruction_index           INTEGER NOT NULL,

    -- transaction --
    signature                   TEXT NOT NULL,
    fee_payer                   TEXT NOT NULL,
    signers_raw                 TEXT NOT NULL DEFAULT '',
    fee                         BIGINT NOT NULL DEFAULT 0,
    compute_units_consumed      BIGINT NOT NULL DEFAULT 0,

    -- instruction --
    program_id                  TEXT NOT NULL,
    stack_height                INTEGER NOT NULL,

    -- event --
    pool_state                  TEXT NOT NULL,
    sender                      TEXT NOT NULL,
    amount_0                    BIGINT NOT NULL,
    amount_1                    BIGINT NOT NULL,
    zero_for_one                BOOLEAN NOT NULL,
    tick                        INTEGER NOT NULL,
    sqrt_price_x64              TEXT NOT NULL,
    liquidity                   TEXT NOT NULL,

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_pancakeswap_swap_block_num ON pancakeswap_swap (block_num);
CREATE INDEX IF NOT EXISTS idx_pancakeswap_swap_timestamp ON pancakeswap_swap (timestamp);
CREATE INDEX IF NOT EXISTS idx_pancakeswap_swap_signature ON pancakeswap_swap (signature);
CREATE INDEX IF NOT EXISTS idx_pancakeswap_swap_pool_state ON pancakeswap_swap (pool_state);

-- Stabble Swap
CREATE TABLE IF NOT EXISTS stabble_swap (
    -- block --
    block_num                   INTEGER NOT NULL,
    block_hash                  TEXT NOT NULL,
    timestamp                   TIMESTAMP NOT NULL,

    -- ordering --
    transaction_index           INTEGER NOT NULL,
    instruction_index           INTEGER NOT NULL,

    -- transaction --
    signature                   TEXT NOT NULL,
    fee_payer                   TEXT NOT NULL,
    signers_raw                 TEXT NOT NULL DEFAULT '',
    fee                         BIGINT NOT NULL DEFAULT 0,
    compute_units_consumed      BIGINT NOT NULL DEFAULT 0,

    -- instruction --
    program_id                  TEXT NOT NULL,
    stack_height                INTEGER NOT NULL,

    -- event --
    "user"                      TEXT NOT NULL,
    pool                        TEXT NOT NULL,
    amount_in                   BIGINT NOT NULL,
    minimum_amount_out          BIGINT NOT NULL,

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_stabble_swap_block_num ON stabble_swap (block_num);
CREATE INDEX IF NOT EXISTS idx_stabble_swap_timestamp ON stabble_swap (timestamp);
CREATE INDEX IF NOT EXISTS idx_stabble_swap_signature ON stabble_swap (signature);
CREATE INDEX IF NOT EXISTS idx_stabble_swap_pool ON stabble_swap (pool);
