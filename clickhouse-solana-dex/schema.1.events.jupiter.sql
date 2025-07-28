-- ──────────────────────────────────────────────────────────────────────────
-- Jupiter V4 & V6 Swaps
-- ──────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS jupiter_swap (
    -- block --
    block_num                   UInt32,
    block_hash                  FixedString(44),
    timestamp                   DateTime(0, 'UTC'),

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

    -- log --
    amm                         FixedString(44) COMMENT 'AMM pool account (Raydium V4)',
    input_mint                  FixedString(44) COMMENT 'Input token mint address',
    input_amount                UInt64 COMMENT 'Amount of input tokens swapped',
    output_mint                 FixedString(44) COMMENT 'Output token mint address',
    output_amount               UInt64 COMMENT 'Amount of output tokens received',

    -- Convert Keys into CityHash64 for faster lookups --
    -- -83% reduction disk space for FixedString(88) vs. UInt64 --
    -- https://clickhouse.com/docs/sql-reference/functions/hash-functions#cityhash64 --
    signature_hash              UInt64  MATERIALIZED cityHash64(signature),

    -- indexes -
    INDEX idx_program_id        (program_id)        TYPE set(8)                 GRANULARITY 1,
    INDEX idx_fee_payer         (fee_payer)         TYPE bloom_filter(0.005)    GRANULARITY 1,
    INDEX idx_signature         (signature)         TYPE bloom_filter(0.005)    GRANULARITY 1,
    INDEX idx_signer            (signer)            TYPE bloom_filter(0.005)    GRANULARITY 1
)
ENGINE = MergeTree
PARTITION BY toDate(timestamp)
ORDER BY (
    timestamp, block_num,
    block_hash, transaction_index, instruction_index
)
COMMENT 'Jupiter V4 & V6 Swaps';

-- PROJECTIONS (Part) --
-- https://clickhouse.com/docs/sql-reference/statements/alter/projection#normal-projection-with-part-offset-field
ALTER TABLE jupiter_swap ADD PROJECTION IF NOT EXISTS prj_part_signature_hash (SELECT signature_hash, _part_offset ORDER BY signature_hash);
ALTER TABLE jupiter_swap ADD PROJECTION IF NOT EXISTS prj_part_program_id (SELECT program_id, timestamp, _part_offset ORDER BY program_id, timestamp);
ALTER TABLE jupiter_swap ADD PROJECTION IF NOT EXISTS prj_part_fee_payer (SELECT fee_payer, timestamp, _part_offset ORDER BY fee_payer, timestamp);
ALTER TABLE jupiter_swap ADD PROJECTION IF NOT EXISTS prj_part_signer (SELECT signer, timestamp, _part_offset ORDER BY signer, timestamp);
