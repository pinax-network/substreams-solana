-- SPL Token-2022 & Classic Initialize Mints --
CREATE TABLE IF NOT EXISTS initialize_mints (
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

    -- event --
    mint                        FixedString(44),
    mint_authority              FixedString(44),
    freeze_authority            FixedString(44),
    decimals                    UInt8
)
ENGINE = MergeTree
ORDER BY (
    program_id, mint,
    block_hash, transaction_index, instruction_index
);