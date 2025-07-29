CREATE FUNCTION IF NOT EXISTS token_types AS ( program_id ) -> CASE program_id
    WHEN CAST ('Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB' AS FixedString(44)) THEN 'USD'
    WHEN CAST ('EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v' AS FixedString(44)) THEN 'USD'
    WHEN CAST ('2FPyTwcZLUg1MDrwsyoP4D6s1tM7hAkHYRjkNb5w6Pxk' AS FixedString(44)) THEN 'ETH'
    WHEN CAST ('7vfCXTUXx5WJV5JADk17DUJ4ksgau7utNKj4b963voxs' AS FixedString(44)) THEN 'ETH'
    WHEN CAST ('So11111111111111111111111111111111111111111' AS FixedString(44)) THEN 'SOL'
    WHEN CAST ('So11111111111111111111111111111111111111112' AS FixedString(44)) THEN 'SOL'
    WHEN CAST ('3NZ9JMVBmGAqocybic2c7LQCJScmgsAZ6vQqTDzcqmJh' AS FixedString(44)) THEN 'BTC'
    WHEN CAST ('9n4nbM75f5Ui33ZbPYXn59EwSgE8CGsHtAeTH5YFeJ9E' AS FixedString(44)) THEN 'BTC'
    WHEN CAST ('cbbtcf3aa214zXHbiAZQwf4122FBYbraNdFqgw4iMij' AS FixedString(44)) THEN 'BTC'
    ELSE 'Unknown'
END;

CREATE FUNCTION IF NOT EXISTS token_names AS ( program_id ) -> CASE program_id
    WHEN CAST ('Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB' AS FixedString(44)) THEN 'Tether USD'
    WHEN CAST ('EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v' AS FixedString(44)) THEN 'Circle: USDC Token'
    WHEN CAST ('2FPyTwcZLUg1MDrwsyoP4D6s1tM7hAkHYRjkNb5w6Pxk' AS FixedString(44)) THEN 'Wrapped ETH "Sollet"'
    WHEN CAST ('7vfCXTUXx5WJV5JADk17DUJ4ksgau7utNKj4b963voxs' AS FixedString(44)) THEN 'Wrapped ETH "Wormhole"'
    WHEN CAST ('So11111111111111111111111111111111111111111' AS FixedString(44)) THEN 'Solana'
    WHEN CAST ('So11111111111111111111111111111111111111112' AS FixedString(44)) THEN 'Wrapped SOL'
    WHEN CAST ('3NZ9JMVBmGAqocybic2c7LQCJScmgsAZ6vQqTDzcqmJh' AS FixedString(44)) THEN 'Wrapped BTC "Wormhole"'
    WHEN CAST ('9n4nbM75f5Ui33ZbPYXn59EwSgE8CGsHtAeTH5YFeJ9E' AS FixedString(44)) THEN 'Wrapped BTC "Sollet"'
    WHEN CAST ('cbbtcf3aa214zXHbiAZQwf4122FBYbraNdFqgw4iMij' AS FixedString(44)) THEN 'cbBTC (Coinbase Wrapped BTC)'
    ELSE 'Unknown'
END;
