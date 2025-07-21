-- Pool activity (Transactions) --
CREATE TABLE IF NOT EXISTS pool_activity_summary (
    -- Order By --
    program_id                  LowCardinality(FixedString(44)),
    program_name                LowCardinality(String) MATERIALIZED program_names(program_id),
    amm                         LowCardinality(FixedString(44)),
    amm_name                    LowCardinality(String) MATERIALIZED program_names(amm),
    amm_pool                    LowCardinality(FixedString(44)),
    mint0                       LowCardinality(FixedString(44)),
    mint1                       LowCardinality(FixedString(44)),

    -- summing --
    transactions                UInt64,

    -- indexes --
    INDEX idx_program_id        (program_id)                TYPE set(8)             GRANULARITY 1,
    INDEX idx_amm               (amm)                       TYPE set(256)           GRANULARITY 1,
    INDEX idx_amm_pool          (amm_pool)                  TYPE set(1024)          GRANULARITY 1,
    INDEX idx_mint0             (mint0)                     TYPE set(1024)          GRANULARITY 1,
    INDEX idx_mint1             (mint1)                     TYPE set(1024)          GRANULARITY 1,
    INDEX idx_transactions      (transactions)              TYPE minmax             GRANULARITY 1
)
ENGINE = SummingMergeTree
ORDER BY (program_id, amm, amm_pool, mint0, mint1);

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_pool_activity_summary
REFRESH EVERY 1 MINUTE APPEND
TO pool_activity_summary
AS
WITH
    (input_mint <= output_mint) AS dir,
    if (dir, input_mint,  output_mint) AS mint0,
    if (dir, output_mint, input_mint) AS mint1
SELECT
    program_id,
    amm,
    amm_pool,
    mint0,
    mint1,

    -- summing --
    1 as transactions
FROM swaps;
