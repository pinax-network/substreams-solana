-- Split a comma-separated string into Array(String)
-- - Trims whitespace around tokens
-- - Drops empty tokens
CREATE FUNCTION IF NOT EXISTS string_to_array AS raw ->
    arrayFilter(x -> x != '',
        arrayMap(x -> trim(x),
            splitByChar(',', ifNull(raw, ''))
        )
    );

-- String to UInt8 conversion
-- Returns NULL if the input is empty or NULL
CREATE FUNCTION IF NOT EXISTS string_to_uint8 AS raw ->
    toUInt8OrNull(nullIf(raw, ''));

-- Accurate cast or NULL
-- Returns NULL if the input is empty or NULL
CREATE FUNCTION IF NOT EXISTS fixed_string_or_null AS raw ->
    accurateCastOrNull(nullIf(raw, ''), 'FixedString(44)');

