-- SPL Token-2022 & Classic Mints --
CREATE TABLE IF NOT EXISTS mints (
    -- block --
    block_num                   UInt32,
    block_hash                  FixedString(44),
    timestamp                   DateTime(0, 'UTC'),

    -- mint --
    mint                        FixedString(44),
    program_id                  LowCardinality(FixedString(44)),
    decimals                    UInt8
)
ENGINE = ReplacingMergeTree(block_num)
ORDER BY (mint, program_id);