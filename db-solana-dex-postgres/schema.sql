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
