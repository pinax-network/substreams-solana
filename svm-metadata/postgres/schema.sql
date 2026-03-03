-- Blocks
CREATE TABLE IF NOT EXISTS blocks (
    block_num            INTEGER NOT NULL,
    block_hash           TEXT NOT NULL,
    timestamp            TIMESTAMP NOT NULL,

    PRIMARY KEY (block_num)
);

CREATE INDEX IF NOT EXISTS idx_blocks_block_hash ON blocks (block_hash);
CREATE INDEX IF NOT EXISTS idx_blocks_timestamp ON blocks (timestamp);

-- Initialize Token Metadata
CREATE TABLE IF NOT EXISTS initialize_token_metadata (
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
    metadata                    TEXT NOT NULL,
    update_authority            TEXT NOT NULL,
    mint                        TEXT NOT NULL,
    mint_authority              TEXT NOT NULL,
    name                        TEXT NOT NULL,
    symbol                      TEXT NOT NULL,
    uri                         TEXT NOT NULL,

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_initialize_token_metadata_block_num ON initialize_token_metadata (block_num);
CREATE INDEX IF NOT EXISTS idx_initialize_token_metadata_timestamp ON initialize_token_metadata (timestamp);
CREATE INDEX IF NOT EXISTS idx_initialize_token_metadata_signature ON initialize_token_metadata (signature);
CREATE INDEX IF NOT EXISTS idx_initialize_token_metadata_mint ON initialize_token_metadata (mint);
CREATE INDEX IF NOT EXISTS idx_initialize_token_metadata_name ON initialize_token_metadata (name);
CREATE INDEX IF NOT EXISTS idx_initialize_token_metadata_symbol ON initialize_token_metadata (symbol);

-- Update Token Metadata Field
CREATE TABLE IF NOT EXISTS update_token_metadata_field (
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
    metadata                    TEXT NOT NULL,
    update_authority            TEXT NOT NULL,
    field                       TEXT NOT NULL,
    value                       TEXT NOT NULL,

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_update_token_metadata_field_block_num ON update_token_metadata_field (block_num);
CREATE INDEX IF NOT EXISTS idx_update_token_metadata_field_timestamp ON update_token_metadata_field (timestamp);
CREATE INDEX IF NOT EXISTS idx_update_token_metadata_field_signature ON update_token_metadata_field (signature);
CREATE INDEX IF NOT EXISTS idx_update_token_metadata_field_metadata ON update_token_metadata_field (metadata);

-- Update Token Metadata Authority
CREATE TABLE IF NOT EXISTS update_token_metadata_authority (
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
    metadata                    TEXT NOT NULL,
    update_authority            TEXT NOT NULL,
    new_authority               TEXT NOT NULL,

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_update_token_metadata_authority_block_num ON update_token_metadata_authority (block_num);
CREATE INDEX IF NOT EXISTS idx_update_token_metadata_authority_timestamp ON update_token_metadata_authority (timestamp);
CREATE INDEX IF NOT EXISTS idx_update_token_metadata_authority_signature ON update_token_metadata_authority (signature);
CREATE INDEX IF NOT EXISTS idx_update_token_metadata_authority_metadata ON update_token_metadata_authority (metadata);

-- Remove Token Metadata Field
CREATE TABLE IF NOT EXISTS remove_token_metadata_field (
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
    metadata                    TEXT NOT NULL,
    update_authority            TEXT NOT NULL,
    key                         TEXT NOT NULL,
    idempotent                  BOOLEAN NOT NULL,

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_remove_token_metadata_field_block_num ON remove_token_metadata_field (block_num);
CREATE INDEX IF NOT EXISTS idx_remove_token_metadata_field_timestamp ON remove_token_metadata_field (timestamp);
CREATE INDEX IF NOT EXISTS idx_remove_token_metadata_field_signature ON remove_token_metadata_field (signature);
CREATE INDEX IF NOT EXISTS idx_remove_token_metadata_field_metadata ON remove_token_metadata_field (metadata);

-- Metaplex Create Metadata Account
CREATE TABLE IF NOT EXISTS metaplex_create_metadata_account (
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
    metadata                    TEXT NOT NULL,
    mint                        TEXT NOT NULL,
    update_authority            TEXT NOT NULL,
    payer                       TEXT NOT NULL,
    name                        TEXT NOT NULL,
    symbol                      TEXT NOT NULL,
    uri                         TEXT NOT NULL,

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_metaplex_create_metadata_account_block_num ON metaplex_create_metadata_account (block_num);
CREATE INDEX IF NOT EXISTS idx_metaplex_create_metadata_account_timestamp ON metaplex_create_metadata_account (timestamp);
CREATE INDEX IF NOT EXISTS idx_metaplex_create_metadata_account_signature ON metaplex_create_metadata_account (signature);
CREATE INDEX IF NOT EXISTS idx_metaplex_create_metadata_account_mint ON metaplex_create_metadata_account (mint);
CREATE INDEX IF NOT EXISTS idx_metaplex_create_metadata_account_name ON metaplex_create_metadata_account (name);
CREATE INDEX IF NOT EXISTS idx_metaplex_create_metadata_account_symbol ON metaplex_create_metadata_account (symbol);

-- Metaplex Update Metadata Account
CREATE TABLE IF NOT EXISTS metaplex_update_metadata_account (
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
    metadata                    TEXT NOT NULL,
    update_authority            TEXT NOT NULL,
    name                        TEXT NOT NULL,
    symbol                      TEXT NOT NULL,
    uri                         TEXT NOT NULL,

    PRIMARY KEY (block_hash, transaction_index, instruction_index)
);

CREATE INDEX IF NOT EXISTS idx_metaplex_update_metadata_account_block_num ON metaplex_update_metadata_account (block_num);
CREATE INDEX IF NOT EXISTS idx_metaplex_update_metadata_account_timestamp ON metaplex_update_metadata_account (timestamp);
CREATE INDEX IF NOT EXISTS idx_metaplex_update_metadata_account_signature ON metaplex_update_metadata_account (signature);
CREATE INDEX IF NOT EXISTS idx_metaplex_update_metadata_account_metadata ON metaplex_update_metadata_account (metadata);
