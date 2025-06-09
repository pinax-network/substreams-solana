-- Search Balances by Owner --
SELECT
    b.contract as address,
    b.new_balance as amount,
    b.timestamp as timestamp,
    c.name as name,
    c.symbol as symbol,
    c.decimals as decimals
FROM balances b FINAL
LEFT JOIN contracts c
    ON c.address = b.contract
WHERE b.owner = '0x61b62c5d56ccd158a38367ef2f539668a06356ab' AND b.new_balance != 0