EXPLAIN indexes = 1, projections = 1
WITH dates AS (
    SELECT toRelativeMinuteNum(timestamp) as ts
    FROM transfers
    WHERE destination IN ['zzzzzupPKEyu2dHMVKSWzCYm3rHc5q4WmV68rwM4Ncq']
    GROUP BY ts

    UNION ALL

    SELECT toRelativeMinuteNum(timestamp) as ts
    FROM transfers
    WHERE source IN ['zzzzzupPKEyu2dHMVKSWzCYm3rHc5q4WmV68rwM4Ncq']
    GROUP BY ts
)
SELECT *
FROM transfers
WHERE toRelativeMinuteNum(timestamp) IN dates AND (
    destination IN ['zzzzzupPKEyu2dHMVKSWzCYm3rHc5q4WmV68rwM4Ncq'] OR
    source IN ['zzzzzupPKEyu2dHMVKSWzCYm3rHc5q4WmV68rwM4Ncq']
)
LIMIT 100;
