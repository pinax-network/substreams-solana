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

    -- indexes --
    INDEX idx_block_num         (block_num)          TYPE minmax           GRANULARITY 4,
    INDEX idx_timestamp         (timestamp)          TYPE minmax           GRANULARITY 4,
    INDEX idx_signature         (signature)          TYPE bloom_filter     GRANULARITY 4
)
ENGINE = ReplacingMergeTree
ORDER BY (block_hash, program_id, transaction_index, instruction_index);
