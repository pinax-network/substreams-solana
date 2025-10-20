-- Accounts interactions by Date --
CREATE TABLE IF NOT EXISTS accounts_by_date (
    account                 String,
    date                    Date,
    hour                    UInt32,

    INDEX idx_date          (date) TYPE minmax GRANULARITY 1,
    INDEX idx_hour          (hour) TYPE minmax GRANULARITY 1
)
ENGINE = ReplacingMergeTree
ORDER BY (account, date, hour)
COMMENT 'Accounts interactions by date';

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_accounts_by_date
TO accounts_by_date
AS
SELECT
    toDate(timestamp) AS date,
    toRelativeHourNum(timestamp) AS hour,
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
GROUP BY account, date, hour;