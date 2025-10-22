SELECT
    partition,
    `table`,
    countIf(active) AS active_parts,
    sumIf(rows, active) AS rows,
    round(sumIf(bytes_on_disk, active) / 1048576) AS mb
FROM system.parts
WHERE (database = currentDatabase())
GROUP BY
    `table`,
    partition
ORDER BY partition DESC
LIMIT 30;

/* How many active projection parts exist?  */
SELECT name,
    sum(rows) AS total_rows,
    formatReadableSize(sum(data_compressed_bytes)) AS on_disk
FROM system.projection_parts
WHERE database = currentDatabase() AND active = 1 AND table = 'transfers'
GROUP BY name
ORDER BY sum(data_compressed_bytes) DESC;

EXPLAIN indexes =1, projections = 1
SELECT *
FROM system_post_balances
WHERE _part_starting_offset + _part_offset IN (
    SELECT _part_starting_offset + _part_offset
    FROM system_post_balances
    WHERE signature = (SELECT signature FROM system_post_balances ORDER BY rand() LIMIT 1)
);