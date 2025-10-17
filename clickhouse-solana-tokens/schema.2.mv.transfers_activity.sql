-- Transfers activity by Account & Date --
CREATE TABLE IF NOT EXISTS transfers_activity (
    account                 String,
    date                    Date,

    INDEX idx_date          (date) TYPE minmax GRANULARITY 1
)
ENGINE = ReplacingMergeTree
ORDER BY (account, date)
COMMENT 'Transfers activity by account & date';

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_transfers_activity
TO transfers_activity
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
FROM transfers
GROUP BY date, account;