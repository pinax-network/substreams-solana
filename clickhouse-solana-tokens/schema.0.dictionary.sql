CREATE TABLE IF NOT EXISTS DICTIONARY_SOLANA_TOKENS_SOURCE
(
    program_id String,
    type String,
    name String
)
ENGINE = MergeTree
ORDER BY program_id;

CREATE DICTIONARY IF NOT EXISTS DICTIONARY_SOLANA_TOKENS
(
    program_id String,
    type String,
    name String
)
PRIMARY KEY program_id
SOURCE(CLICKHOUSE(TABLE 'DICTIONARY_SOLANA_TOKENS_SOURCE'))
LIFETIME(MIN 0 MAX 1000)
LAYOUT(HASHED());

INSERT INTO DICTIONARY_SOLANA_TOKENS_SOURCE (program_id, type, name) VALUES
('Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB', 'USD', 'Tether USD'),
('EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v', 'USD', 'Circle: USDC Token'),
('2FPyTwcZLUg1MDrwsyoP4D6s1tM7hAkHYRjkNb5w6Pxk', 'ETH', 'Wrapped ETH "Sollet"'),
('7vfCXTUXx5WJV5JADk17DUJ4ksgau7utNKj4b963voxs', 'ETH', 'Wrapped ETH "Wormhole"'),
('So11111111111111111111111111111111111111111', 'SOL', 'Solana'),
('So11111111111111111111111111111111111111112', 'SOL', 'Wrapped SOL'),
('3NZ9JMVBmGAqocybic2c7LQCJScmgsAZ6vQqTDzcqmJh', 'BTC', 'Wrapped BTC "Wormhole"'),
('9n4nbM75f5Ui33ZbPYXn59EwSgE8CGsHtAeTH5YFeJ9E', 'BTC', 'Wrapped BTC "Sollet"'),
('cbbtcf3aa214zXHbiAZQwf4122FBYbraNdFqgw4iMij', 'BTC', 'cbBTC (Coinbase Wrapped BTC)');
