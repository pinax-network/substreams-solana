CREATE TABLE IF NOT EXISTS base_events (
    -- block --
    block_num                   UInt32,
    block_hash                  FixedString(44),
    timestamp                   DateTime(0, 'UTC'),

    -- ordering --
    transaction_index           UInt32,
    instruction_index           UInt32,

    -- transaction --
    signature                   FixedString(88),
    signature_hash              UInt64  MATERIALIZED cityHash64(signature),
    fee_payer                   FixedString(44),
    signers_raw                 String,
    signers                     Array(FixedString(44)) MATERIALIZED arrayMap(x -> toFixedString(x, 44), splitByChar(',', signers_raw)),
    signer                      FixedString(44) MATERIALIZED if(length(signers) > 0, signers[1], ''),
    fee                         UInt64 DEFAULT 0,
    compute_units_consumed      UInt64 DEFAULT 0,

    -- instruction --
    program_id                  LowCardinality(FixedString(44)),
    stack_height                UInt32,

    -- indexes -
    INDEX idx_program_id        (program_id)        TYPE set(8)                 GRANULARITY 1,
    INDEX idx_fee_payer         (fee_payer)         TYPE bloom_filter(0.005)    GRANULARITY 1,
    INDEX idx_signature         (signature)         TYPE bloom_filter(0.005)    GRANULARITY 1,
    INDEX idx_signer            (signer)            TYPE bloom_filter(0.005)    GRANULARITY 1
)
ENGINE = ReplacingMergeTree
PARTITION BY toYYYYMM(timestamp)
ORDER BY (
    timestamp, block_num,
    block_hash, transaction_index, instruction_index
);

-- 'drop': fastest merges; if a deduplicating merge touches a projection, that projection is removed only for the affected parts.
-- Queries still run (the optimizer just won’t pick that projection for those parts until you rebuild).
-- Good default for high‑ingest systems.

-- 'rebuild': keeps projections available but makes deduplicating merges heavier.
-- Use if you depend heavily on the projection for latency and can afford the CPU/IO.
ALTER TABLE base_events
  MODIFY SETTING deduplicate_merge_projection_mode = 'drop';  -- or 'rebuild'

-- PROJECTIONS (Part) --
-- https://clickhouse.com/docs/sql-reference/statements/alter/projection#normal-projection-with-part-offset-field
ALTER TABLE base_events ADD PROJECTION IF NOT EXISTS prj_part_signature_hash (SELECT signature_hash, _part_offset ORDER BY signature_hash);
ALTER TABLE base_events ADD PROJECTION IF NOT EXISTS prj_part_program_id (SELECT program_id, timestamp, _part_offset ORDER BY program_id, timestamp);
ALTER TABLE base_events ADD PROJECTION IF NOT EXISTS prj_part_fee_payer (SELECT fee_payer, timestamp, _part_offset ORDER BY fee_payer, timestamp);
ALTER TABLE base_events ADD PROJECTION IF NOT EXISTS prj_part_signer (SELECT signer, timestamp, _part_offset ORDER BY signer, timestamp);
