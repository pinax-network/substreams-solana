SELECT
    partition,
    `table`,
    countIf(active) AS active_parts,
    sumIf(rows, active) AS rows,
    round(sumIf(bytes_on_disk, active) / 1048576) AS mb
FROM system.parts
WHERE (database = currentDatabase()) AND (partition = '202204')
GROUP BY
    `table`,
    partition
ORDER BY partition DESC
LIMIT 30