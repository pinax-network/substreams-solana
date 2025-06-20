-- SPL Token-2022 & Classic Mints --
CREATE TABLE IF NOT EXISTS mints (
    block_num                   UInt32,
    mint                        FixedString(44),
    program_id                  LowCardinality(FixedString(44)),
    decimals                    UInt8
)
ENGINE = ReplacingMergeTree(block_num)
ORDER BY (mint, program_id);