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

EXPLAIN indexes = 1, projections = 1
WITH dates AS (
    SELECT date FROM accounts_by_date
    WHERE account = '2VdhEhg7VfB1KMNBym5iJP4hqJi62Dqpv4HgeBpceRsn'
    GROUP BY date
)
SELECT *
FROM transfers
WHERE toDate(timestamp) IN dates AND (
    source = '2VdhEhg7VfB1KMNBym5iJP4hqJi62Dqpv4HgeBpceRsn' OR
    destination = '2VdhEhg7VfB1KMNBym5iJP4hqJi62Dqpv4HgeBpceRsn'
)
LIMIT 100


EXPLAIN indexes = 1, projections = 1
WITH ('57jkMQGDbBWsjUR7ZRAYfqiW9wMc1HGrZ2vFDiQ1e6UB', 'HKqPaH9XMouYf2NnmSSSLtU1Kx1GmJ1AMLWHDekuXdV4') AS accounts,
dates AS (
    SELECT date FROM accounts_by_date_only
    WHERE account IN accounts
    GROUP BY date
)
SELECT *
FROM transfers
WHERE toDate(timestamp) IN dates AND (
    source IN accounts OR
    destination IN accounts
)
LIMIT 10


EXPLAIN indexes = 1, projections = 1
WITH ('Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB') AS accounts,
dates AS (
    SELECT DISTINCT date FROM accounts_by_date
    WHERE account IN accounts
    GROUP BY date
    ORDER BY date
)
SELECT *
FROM transfers
WHERE toDate(timestamp) IN dates AND (source IN accounts OR destination IN accounts)
LIMIT 10


EXPLAIN indexes =1, projections = 1
SELECT *
FROM transfers
WHERE _part_starting_offset + _part_offset IN (
    SELECT _part_starting_offset + _part_offset
    FROM transfers
    WHERE source = 'Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB'
);