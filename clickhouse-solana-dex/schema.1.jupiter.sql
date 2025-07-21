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

    -- indexes --
    INDEX idx_signature         (signature)         TYPE bloom_filter   GRANULARITY 8,  -- always unique
    INDEX idx_fee_payer         (fee_payer)         TYPE set(4096)      GRANULARITY 1,
    INDEX idx_signer            (signer)            TYPE set(4096)      GRANULARITY 1
)
ENGINE = MergeTree
ORDER BY (
    timestamp, block_num,
    block_hash, transaction_index, instruction_index
);