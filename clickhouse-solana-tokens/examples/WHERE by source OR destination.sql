WITH source_dates AS (
    SELECT toDate(timestamp)
    FROM transfers
    WHERE source = 'GwBvj8V36fNqQBnUF2RZ4WWAA7XpoHGMz9jxoHLWnsGi'
    GROUP BY toDate(timestamp)
), destination_dates AS (
    SELECT toDate(timestamp)
    FROM transfers
    WHERE destination = 'GwBvj8V36fNqQBnUF2RZ4WWAA7XpoHGMz9jxoHLWnsGi'
    GROUP BY toDate(timestamp)
), dates AS (
    SELECT *
    FROM source_dates
    INTERSECT
    SELECT *
    FROM destination_dates
)
SELECT *
FROM transfers
WHERE toDate(timestamp) IN dates AND (
    source = 'GwBvj8V36fNqQBnUF2RZ4WWAA7XpoHGMz9jxoHLWnsGi' OR
    destination = 'GwBvj8V36fNqQBnUF2RZ4WWAA7XpoHGMz9jxoHLWnsGi'
)
LIMIT 100
SETTINGS max_execution_time = 10;


EXPLAIN indexes =1, projections = 1
SELECT toDate(timestamp)
FROM transfers
WHERE destination = '2KE2UNJKB6RGgb78DxJbi2HXSfCs1EocHj4FDMZPr4HA'
GROUP BY toDate(timestamp)

WITH dates AS (
    SELECT date FROM accounts_by_date
    WHERE account = 'GwBvj8V36fNqQBnUF2RZ4WWAA7XpoHGMz9jxoHLWnsGi'
)
SELECT *
FROM transfers
WHERE toDate(timestamp) IN dates AND (
    source = 'GwBvj8V36fNqQBnUF2RZ4WWAA7XpoHGMz9jxoHLWnsGi' OR
    destination = 'GwBvj8V36fNqQBnUF2RZ4WWAA7XpoHGMz9jxoHLWnsGi'
)
LIMIT 100