-- Blocks
CREATE TABLE IF NOT EXISTS blocks (
    block_num            INTEGER NOT NULL,
    block_hash           TEXT NOT NULL,
    timestamp            TIMESTAMP NOT NULL,

    PRIMARY KEY (block_num)
);

CREATE INDEX IF NOT EXISTS idx_blocks_block_hash ON blocks (block_hash);
CREATE INDEX IF NOT EXISTS idx_blocks_timestamp ON blocks (timestamp);

-- SPL Token Transfers
CREATE TABLE IF NOT EXISTS spl_transfer (
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

    -- transfer --
    source                      TEXT NOT NULL,
    destination                 TEXT NOT NULL,
    amount                      BIGINT NOT NULL,
    mint                        TEXT NOT NULL,
    decimals_raw                TEXT NOT NULL DEFAULT '',
    authority                   TEXT NOT NULL,
    multisig_authority_raw      TEXT NOT NULL DEFAULT '',

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_spl_transfer_block_num ON spl_transfer (block_num);
CREATE INDEX IF NOT EXISTS idx_spl_transfer_timestamp ON spl_transfer (timestamp);
CREATE INDEX IF NOT EXISTS idx_spl_transfer_signature ON spl_transfer (signature);
CREATE INDEX IF NOT EXISTS idx_spl_transfer_source ON spl_transfer (source);
CREATE INDEX IF NOT EXISTS idx_spl_transfer_destination ON spl_transfer (destination);
CREATE INDEX IF NOT EXISTS idx_spl_transfer_mint ON spl_transfer (mint);

-- System Transfer (Native SOL)
CREATE TABLE IF NOT EXISTS system_transfer (
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

    -- transfer --
    source                      TEXT NOT NULL,
    destination                 TEXT NOT NULL,
    lamports                    BIGINT NOT NULL,

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_system_transfer_block_num ON system_transfer (block_num);
CREATE INDEX IF NOT EXISTS idx_system_transfer_timestamp ON system_transfer (timestamp);
CREATE INDEX IF NOT EXISTS idx_system_transfer_signature ON system_transfer (signature);
CREATE INDEX IF NOT EXISTS idx_system_transfer_source ON system_transfer (source);
CREATE INDEX IF NOT EXISTS idx_system_transfer_destination ON system_transfer (destination);

-- System Transfer With Seed
CREATE TABLE IF NOT EXISTS system_transfer_with_seed (
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

    -- transfer --
    source                      TEXT NOT NULL,
    destination                 TEXT NOT NULL,
    lamports                    BIGINT NOT NULL,
    source_base                 TEXT NOT NULL,
    source_owner                TEXT NOT NULL,
    source_seed                 TEXT NOT NULL,

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_system_transfer_with_seed_block_num ON system_transfer_with_seed (block_num);
CREATE INDEX IF NOT EXISTS idx_system_transfer_with_seed_timestamp ON system_transfer_with_seed (timestamp);
CREATE INDEX IF NOT EXISTS idx_system_transfer_with_seed_signature ON system_transfer_with_seed (signature);
CREATE INDEX IF NOT EXISTS idx_system_transfer_with_seed_source ON system_transfer_with_seed (source);
CREATE INDEX IF NOT EXISTS idx_system_transfer_with_seed_destination ON system_transfer_with_seed (destination);

-- System Withdraw Nonce Account
CREATE TABLE IF NOT EXISTS system_withdraw_nonce_account (
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

    -- withdraw --
    destination                 TEXT NOT NULL,
    lamports                    BIGINT NOT NULL,
    nonce_account               TEXT NOT NULL,
    nonce_authority             TEXT NOT NULL,

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_system_withdraw_nonce_account_block_num ON system_withdraw_nonce_account (block_num);
CREATE INDEX IF NOT EXISTS idx_system_withdraw_nonce_account_timestamp ON system_withdraw_nonce_account (timestamp);
CREATE INDEX IF NOT EXISTS idx_system_withdraw_nonce_account_signature ON system_withdraw_nonce_account (signature);
CREATE INDEX IF NOT EXISTS idx_system_withdraw_nonce_account_destination ON system_withdraw_nonce_account (destination);
CREATE INDEX IF NOT EXISTS idx_system_withdraw_nonce_account_nonce_account ON system_withdraw_nonce_account (nonce_account);
