-- SPL-2022 Token & Classic transfers --
CREATE TABLE IF NOT EXISTS transfers (
    -- block --
    block_num                   UInt32,
    block_hash                  FixedString(44),
    timestamp                   DateTime(0, 'UTC'),
    timestamp_since_genesis     DateTime(0, 'UTC')
        MATERIALIZED if (
            timestamp = 0,
            toDateTime(1584332940 + intDiv(block_num * 2, 5), 'UTC'),
            timestamp
        ),

    -- ordering --
    transaction_index           UInt32,
    instruction_index           UInt32,

    -- transaction --
    signature                   FixedString(88),
    fee_payer                   FixedString(44),
    signers_raw                 String,
    signers                     Array(FixedString(44)) MATERIALIZED arrayMap(x -> toFixedString(x, 44), splitByChar(',', signers_raw)),
    signer                      FixedString(44) MATERIALIZED if(length(signers) > 0, signers[1], ''),
    fee                         UInt64 DEFAULT 0,
    compute_units_consumed      UInt64 DEFAULT 0,

    -- instruction --
    program_id                  LowCardinality(FixedString(44)),
    stack_height                UInt32,
    is_root                     Bool COMMENT 'Indicates if the instruction is a root instruction or an inner instruction',
    discriminator               FixedString(16) COMMENT 'Discriminator for the instruction, used to identify the type of instruction',

    -- accounts --
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
    INDEX idx_timestamp          (timestamp)          TYPE minmax GRANULARITY 4,
    INDEX idx_block_num          (block_num)          TYPE minmax GRANULARITY 4,
    INDEX idx_signature          (signature)          TYPE bloom_filter GRANULARITY 4,  -- always unique
    INDEX idx_program_id         (program_id)         TYPE set(2) GRANULARITY 1,

    -- indexes (event) --
    INDEX idx_authority          (authority)          TYPE set(2048) GRANULARITY 1,
    INDEX idx_source             (source)             TYPE set(4096) GRANULARITY 1,
    INDEX idx_destination        (destination)        TYPE set(4096) GRANULARITY 1,
    INDEX idx_mint               (mint)               TYPE set(2048) GRANULARITY 1,
    INDEX idx_amount             (amount)             TYPE minmax GRANULARITY 1,
    INDEX idx_decimals           (decimals)           TYPE set(8) GRANULARITY 1,

    -- projections --
    PROJECTION prj_timestamp ( SELECT timestamp, block_num, _part_offset ORDER BY (timestamp, block_num) )
)
ENGINE = MergeTree
ORDER BY (
    program_id, mint, source, destination,
    block_hash, transaction_index, instruction_index
);
