-- Blocks
CREATE TABLE IF NOT EXISTS blocks (
    block_num            INTEGER NOT NULL,
    block_hash           TEXT NOT NULL,
    timestamp            TIMESTAMP NOT NULL,

    PRIMARY KEY (block_num)
);

CREATE INDEX IF NOT EXISTS idx_blocks_block_hash ON blocks (block_hash);
CREATE INDEX IF NOT EXISTS idx_blocks_timestamp ON blocks (timestamp);

-- Initialize Mint
CREATE TABLE IF NOT EXISTS initialize_mint (
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
    mint                        TEXT NOT NULL,
    mint_authority              TEXT NOT NULL,
    decimals                    INTEGER NOT NULL,
    freeze_authority_raw        TEXT NOT NULL DEFAULT '',

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_initialize_mint_block_num ON initialize_mint (block_num);
CREATE INDEX IF NOT EXISTS idx_initialize_mint_timestamp ON initialize_mint (timestamp);
CREATE INDEX IF NOT EXISTS idx_initialize_mint_signature ON initialize_mint (signature);
CREATE INDEX IF NOT EXISTS idx_initialize_mint_mint ON initialize_mint (mint);

-- Initialize Account
CREATE TABLE IF NOT EXISTS initialize_account (
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
    account                     TEXT NOT NULL,
    mint                        TEXT NOT NULL,
    owner                       TEXT NOT NULL,

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_initialize_account_block_num ON initialize_account (block_num);
CREATE INDEX IF NOT EXISTS idx_initialize_account_timestamp ON initialize_account (timestamp);
CREATE INDEX IF NOT EXISTS idx_initialize_account_signature ON initialize_account (signature);
CREATE INDEX IF NOT EXISTS idx_initialize_account_account ON initialize_account (account);
CREATE INDEX IF NOT EXISTS idx_initialize_account_mint ON initialize_account (mint);
CREATE INDEX IF NOT EXISTS idx_initialize_account_owner ON initialize_account (owner);

-- Initialize Immutable Owner
CREATE TABLE IF NOT EXISTS initialize_immutable_owner (
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
    account                     TEXT NOT NULL,

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_initialize_immutable_owner_block_num ON initialize_immutable_owner (block_num);
CREATE INDEX IF NOT EXISTS idx_initialize_immutable_owner_timestamp ON initialize_immutable_owner (timestamp);
CREATE INDEX IF NOT EXISTS idx_initialize_immutable_owner_signature ON initialize_immutable_owner (signature);
CREATE INDEX IF NOT EXISTS idx_initialize_immutable_owner_account ON initialize_immutable_owner (account);

-- Set Authority
CREATE TABLE IF NOT EXISTS set_authority (
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
    account                     TEXT NOT NULL,
    authority_type              TEXT NOT NULL,
    new_authority_raw           TEXT NOT NULL DEFAULT '',
    authority                   TEXT NOT NULL,
    multisig_authority_raw      TEXT NOT NULL DEFAULT '',

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_set_authority_block_num ON set_authority (block_num);
CREATE INDEX IF NOT EXISTS idx_set_authority_timestamp ON set_authority (timestamp);
CREATE INDEX IF NOT EXISTS idx_set_authority_signature ON set_authority (signature);
CREATE INDEX IF NOT EXISTS idx_set_authority_account ON set_authority (account);

-- Close Account
CREATE TABLE IF NOT EXISTS close_account (
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
    account                     TEXT NOT NULL,
    destination                 TEXT NOT NULL,
    authority                   TEXT NOT NULL,
    multisig_authority_raw      TEXT NOT NULL DEFAULT '',

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_close_account_block_num ON close_account (block_num);
CREATE INDEX IF NOT EXISTS idx_close_account_timestamp ON close_account (timestamp);
CREATE INDEX IF NOT EXISTS idx_close_account_signature ON close_account (signature);
CREATE INDEX IF NOT EXISTS idx_close_account_account ON close_account (account);

-- Approve
CREATE TABLE IF NOT EXISTS approve (
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
    source                      TEXT NOT NULL,
    mint_raw                    TEXT NOT NULL DEFAULT '',
    delegate                    TEXT NOT NULL,
    owner                       TEXT NOT NULL,
    amount                      BIGINT NOT NULL,
    decimals_raw                TEXT NOT NULL DEFAULT '',
    authority                   TEXT NOT NULL,
    multisig_authority_raw      TEXT NOT NULL DEFAULT '',

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_approve_block_num ON approve (block_num);
CREATE INDEX IF NOT EXISTS idx_approve_timestamp ON approve (timestamp);
CREATE INDEX IF NOT EXISTS idx_approve_signature ON approve (signature);
CREATE INDEX IF NOT EXISTS idx_approve_source ON approve (source);
CREATE INDEX IF NOT EXISTS idx_approve_delegate ON approve (delegate);

-- Revoke
CREATE TABLE IF NOT EXISTS revoke (
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
    source                      TEXT NOT NULL,
    owner                       TEXT NOT NULL,
    authority                   TEXT NOT NULL,
    multisig_authority_raw      TEXT NOT NULL DEFAULT '',

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_revoke_block_num ON revoke (block_num);
CREATE INDEX IF NOT EXISTS idx_revoke_timestamp ON revoke (timestamp);
CREATE INDEX IF NOT EXISTS idx_revoke_signature ON revoke (signature);
CREATE INDEX IF NOT EXISTS idx_revoke_source ON revoke (source);

-- Freeze Account
CREATE TABLE IF NOT EXISTS freeze_account (
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
    account                     TEXT NOT NULL,
    mint                        TEXT NOT NULL,
    authority                   TEXT NOT NULL,
    multisig_authority_raw      TEXT NOT NULL DEFAULT '',

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_freeze_account_block_num ON freeze_account (block_num);
CREATE INDEX IF NOT EXISTS idx_freeze_account_timestamp ON freeze_account (timestamp);
CREATE INDEX IF NOT EXISTS idx_freeze_account_signature ON freeze_account (signature);
CREATE INDEX IF NOT EXISTS idx_freeze_account_account ON freeze_account (account);
CREATE INDEX IF NOT EXISTS idx_freeze_account_mint ON freeze_account (mint);

-- Thaw Account
CREATE TABLE IF NOT EXISTS thaw_account (
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
    account                     TEXT NOT NULL,
    mint                        TEXT NOT NULL,
    authority                   TEXT NOT NULL,
    multisig_authority_raw      TEXT NOT NULL DEFAULT '',

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_thaw_account_block_num ON thaw_account (block_num);
CREATE INDEX IF NOT EXISTS idx_thaw_account_timestamp ON thaw_account (timestamp);
CREATE INDEX IF NOT EXISTS idx_thaw_account_signature ON thaw_account (signature);
CREATE INDEX IF NOT EXISTS idx_thaw_account_account ON thaw_account (account);
CREATE INDEX IF NOT EXISTS idx_thaw_account_mint ON thaw_account (mint);

-- System Create Account
CREATE TABLE IF NOT EXISTS system_create_account (
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
    source                      TEXT NOT NULL,
    new_account                 TEXT NOT NULL,
    owner                       TEXT NOT NULL,
    lamports                    BIGINT NOT NULL,
    space                       BIGINT NOT NULL,

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_system_create_account_block_num ON system_create_account (block_num);
CREATE INDEX IF NOT EXISTS idx_system_create_account_timestamp ON system_create_account (timestamp);
CREATE INDEX IF NOT EXISTS idx_system_create_account_signature ON system_create_account (signature);
CREATE INDEX IF NOT EXISTS idx_system_create_account_new_account ON system_create_account (new_account);
CREATE INDEX IF NOT EXISTS idx_system_create_account_owner ON system_create_account (owner);

-- System Create Account With Seed
CREATE TABLE IF NOT EXISTS system_create_account_with_seed (
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
    source                      TEXT NOT NULL,
    new_account                 TEXT NOT NULL,
    base                        TEXT NOT NULL,
    base_account_raw            TEXT NOT NULL DEFAULT '',
    owner                       TEXT NOT NULL,
    lamports                    BIGINT NOT NULL,
    space                       BIGINT NOT NULL,
    seed                        TEXT NOT NULL,

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_system_create_account_with_seed_block_num ON system_create_account_with_seed (block_num);
CREATE INDEX IF NOT EXISTS idx_system_create_account_with_seed_timestamp ON system_create_account_with_seed (timestamp);
CREATE INDEX IF NOT EXISTS idx_system_create_account_with_seed_signature ON system_create_account_with_seed (signature);
CREATE INDEX IF NOT EXISTS idx_system_create_account_with_seed_new_account ON system_create_account_with_seed (new_account);
CREATE INDEX IF NOT EXISTS idx_system_create_account_with_seed_owner ON system_create_account_with_seed (owner);
