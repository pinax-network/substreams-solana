CREATE TABLE IF NOT EXISTS mints (
    block_num           UInt32,
    mint                FixedString(44),
    mint_authority      FixedString(44),
    decimals            UInt8
) ENGINE = ReplacingMergeTree(block_num)
ORDER BY (mint, mint_authority)
COMMENT 'Solana Mints, used by SPL Tokens';

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_initialize_mint
TO mints AS
SELECT
    block_num,
    mint,
    mint_authority,
    decimals
FROM initialize_mint;