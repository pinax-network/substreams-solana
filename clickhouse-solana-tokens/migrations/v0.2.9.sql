-- Accounts interactions by Date --
CREATE TABLE IF NOT EXISTS accounts_by_date ON CLUSTER 'dev1' (
    account                 String,
    date                    Date COMMENT 'toDate(timestamp)',

    INDEX idx_date          (date) TYPE minmax GRANULARITY 1
)
ENGINE = ReplacingMergeTree
ORDER BY (account, date, hour)
COMMENT 'Accounts interactions by date';

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_accounts_by_date ON CLUSTER 'dev1'
TO accounts_by_date
AS
SELECT
    toDate(timestamp) AS date,
    arrayJoin(
        arrayDistinct(
            arrayFilter(s -> isNotNull(s) AND s != '',
                [
                    source,
                    destination,
                    authority,
                    signer,
                    fee_payer,
                    CAST(mint AS String)
                ]
            )
        )
    ) AS account
FROM transfers;

-- Set Projections for Transfers --
ALTER TABLE transfers ON CLUSTER 'dev1'
    -- Base Events --
    ADD PROJECTION IF NOT EXISTS prj_signature (SELECT signature, timestamp, _part_offset ORDER BY (signature, timestamp)),
    ADD PROJECTION IF NOT EXISTS prj_fee_payer (SELECT fee_payer, timestamp, _part_offset ORDER BY (fee_payer, timestamp)),
    ADD PROJECTION IF NOT EXISTS prj_signer (SELECT signer, timestamp, _part_offset ORDER BY (signer, timestamp)),
    -- Transfers --
    ADD PROJECTION IF NOT EXISTS prj_authority (SELECT authority, timestamp, _part_offset ORDER BY (authority, timestamp)),
    ADD PROJECTION IF NOT EXISTS prj_source (SELECT source, timestamp, _part_offset ORDER BY (source, timestamp)),
    ADD PROJECTION IF NOT EXISTS prj_destination (SELECT destination, timestamp, _part_offset ORDER BY (destination, timestamp)),
    ADD PROJECTION IF NOT EXISTS prj_mint (SELECT mint, timestamp, _part_offset ORDER BY (mint, timestamp));

-- Base Events --
ALTER TABLE transfers ON CLUSTER 'dev1' MATERIALIZE PROJECTION prj_signature;
ALTER TABLE transfers ON CLUSTER 'dev1' MATERIALIZE PROJECTION prj_fee_payer;
ALTER TABLE transfers ON CLUSTER 'dev1' MATERIALIZE PROJECTION prj_signer;

-- Transfers --
ALTER TABLE transfers ON CLUSTER 'dev1' MATERIALIZE PROJECTION prj_authority;
ALTER TABLE transfers ON CLUSTER 'dev1' MATERIALIZE PROJECTION prj_source;
ALTER TABLE transfers ON CLUSTER 'dev1' MATERIALIZE PROJECTION prj_destination;
ALTER TABLE transfers ON CLUSTER 'dev1' MATERIALIZE PROJECTION prj_mint;
