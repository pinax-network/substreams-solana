-- SPL Token-2022 & Classic approves --
CREATE TABLE IF NOT EXISTS approves (
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
    delegate                    FixedString(44),
    owner                       FixedString(44),
    amount                      UInt64,

    -- event (Optional) --
    mint_raw                    String, -- can be empty
    mint                        Nullable(FixedString(44)) MATERIALIZED accurateCastOrNull(nullIf(mint_raw, ''), 'FixedString(44)'),
    decimals_raw                String, -- can be empty
    decimals                    Nullable(UInt8) MATERIALIZED toUInt8OrNull(nullIf(decimals_raw, '')),

    -- projections (parts) --
    -- https://clickhouse.com/docs/sql-reference/statements/alter/projection#normal-projection-with-part-offset-field
    PROJECTION prj_part_signature       (SELECT signature,      _part_offset ORDER BY signature),
    PROJECTION prj_part_fee_payer       (SELECT fee_payer,      _part_offset ORDER BY fee_payer),
    PROJECTION prj_part_signer          (SELECT signer,         _part_offset ORDER BY signer)
)
ENGINE = MergeTree
ORDER BY (
    timestamp, block_num,
    block_hash, transaction_index, instruction_index
);

-- SPL Token-2022 & Classic revokes --
CREATE TABLE IF NOT EXISTS revokes (
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
    owner                       FixedString(44),

    -- projections (parts) --
    -- https://clickhouse.com/docs/sql-reference/statements/alter/projection#normal-projection-with-part-offset-field
    PROJECTION prj_part_signature       (SELECT signature,      _part_offset ORDER BY signature),
    PROJECTION prj_part_fee_payer       (SELECT fee_payer,      _part_offset ORDER BY fee_payer),
    PROJECTION prj_part_signer          (SELECT signer,         _part_offset ORDER BY signer)
)
ENGINE = MergeTree
ORDER BY (
    timestamp, block_num,
    block_hash, transaction_index, instruction_index
);