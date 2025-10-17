WITH source_dates AS (
    SELECT toDate(timestamp)
    FROM transfers
    WHERE source = '2KE2UNJKB6RGgb78DxJbi2HXSfCs1EocHj4FDMZPr4HA'
    GROUP BY toDate(timestamp)
), destination_dates AS (
    SELECT toDate(timestamp)
    FROM transfers
    WHERE destination = '2KE2UNJKB6RGgb78DxJbi2HXSfCs1EocHj4FDMZPr4HA'
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
    source = '2KE2UNJKB6RGgb78DxJbi2HXSfCs1EocHj4FDMZPr4HA' OR
    destination = '2KE2UNJKB6RGgb78DxJbi2HXSfCs1EocHj4FDMZPr4HA'
)
LIMIT 100
SETTINGS max_execution_time = 10;


EXPLAIN indexes =1, projections = 1
SELECT toDate(timestamp)
FROM transfers
WHERE destination = '2KE2UNJKB6RGgb78DxJbi2HXSfCs1EocHj4FDMZPr4HA'
GROUP BY toDate(timestamp)