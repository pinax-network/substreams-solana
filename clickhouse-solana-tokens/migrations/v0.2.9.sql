-- Accounts interactions by Date --
CREATE TABLE IF NOT EXISTS accounts_by_date ON CLUSTER 'tokenapis-b' (
    account                 String,
    date                    Date COMMENT 'toDate(timestamp)',
    hour                    UInt32 COMMENT 'toRelativeHourNum(timestamp)',

    INDEX idx_date          (date) TYPE minmax GRANULARITY 1,
    INDEX idx_hour          (hour) TYPE minmax GRANULARITY 1
)
ENGINE = ReplicatedReplacingMergeTree
ORDER BY (account, date, hour)
COMMENT 'Accounts interactions by date/hour';

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_accounts_by_date ON CLUSTER 'tokenapis-b'
TO accounts_by_date
AS
SELECT
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
    ) AS account,
    toDate(timestamp) AS date,
    toRelativeHourNum(timestamp) AS hour
FROM transfers
GROUP BY account, date, hour;

-- Set Projections for Transfers --
ALTER TABLE transfers ON CLUSTER 'tokenapis-b'
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
ALTER TABLE transfers ON CLUSTER 'tokenapis-b' MATERIALIZE PROJECTION prj_signature;
ALTER TABLE transfers ON CLUSTER 'tokenapis-b' MATERIALIZE PROJECTION prj_fee_payer;
ALTER TABLE transfers ON CLUSTER 'tokenapis-b' MATERIALIZE PROJECTION prj_signer;

-- Transfers --
ALTER TABLE transfers ON CLUSTER 'tokenapis-b' MATERIALIZE PROJECTION prj_authority;
ALTER TABLE transfers ON CLUSTER 'tokenapis-b' MATERIALIZE PROJECTION prj_source;
ALTER TABLE transfers ON CLUSTER 'tokenapis-b' MATERIALIZE PROJECTION prj_destination;
ALTER TABLE transfers ON CLUSTER 'tokenapis-b' MATERIALIZE PROJECTION prj_mint;

-- Backfill historical data directly into the target table
INSERT INTO accounts_by_date (account, date, hour)
SELECT
    arrayJoin(
        arrayDistinct(
            arrayFilter(s -> isNotNull(s) AND s != '',
                [source, destination, authority, signer, fee_payer, CAST(mint AS String)]
            )
        )
    )                                   AS account,
    toDate(timestamp)                   AS date,
    toRelativeHourNum(timestamp)       AS hour
FROM transfers
WHERE YEAR(timestamp) IN (2023) AND MONTH(timestamp) IN (1,2,3,4,5,6)
GROUP BY account, date, hour;

INSERT INTO accounts_by_date (account, date, hour)
SELECT
    arrayJoin(
        arrayDistinct(
            arrayFilter(s -> isNotNull(s) AND s != '',
                [source, destination, authority, signer, fee_payer, CAST(mint AS String)]
            )
        )
    )                                   AS account,
    toDate(timestamp)                   AS date,
    toRelativeHourNum(timestamp)       AS hour
FROM transfers
WHERE YEAR(timestamp) IN (2023) AND MONTH(timestamp) IN (7,8,9,10,11,12)
GROUP BY account, date, hour;