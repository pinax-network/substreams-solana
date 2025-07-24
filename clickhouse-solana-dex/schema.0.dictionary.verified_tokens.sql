CREATE TABLE IF NOT EXISTS verified_tokens  (
    network              LowCardinality(String),
    contract             FixedString(44),
    description          String,
    type                 Enum8('USD' = 1, 'ETH' = 2, 'BTC' = 3, 'SOL' = 4)
)
ENGINE = ReplacingMergeTree
ORDER BY (contract, network);

-- Insert initial Stable Tokens (1:1 USD) verified tokens
INSERT INTO verified_tokens (network, contract, description, type) VALUES
    ('solana', 'Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB', 'USDT (Tether USD)', 'USD'),
    ('solana', 'EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v', 'USDC (Circle: USDC Token)', 'USD');

-- Insert initial Wrapped/Native Tokens (1:1 Native) verified tokens
INSERT INTO verified_tokens (network, contract, description, type) VALUES
    ('solana', '2FPyTwcZLUg1MDrwsyoP4D6s1tM7hAkHYRjkNb5w6Pxk', 'ETH (Wrapped ETH "Sollet")', 'ETH'),
    ('solana', '7vfCXTUXx5WJV5JADk17DUJ4ksgau7utNKj4b963voxs', 'ETH (Wrapped ETH "Wormhole")', 'ETH'),
    ('solana', 'So11111111111111111111111111111111111111111', 'SOL', 'SOL'),
    ('solana', 'So11111111111111111111111111111111111111112', 'SOL (Wrapped SOL)', 'SOL'),
    ('solana', '3NZ9JMVBmGAqocybic2c7LQCJScmgsAZ6vQqTDzcqmJh', 'WBTC (Wrapped BTC "Wormhole")', 'BTC'),
    ('solana', '9n4nbM75f5Ui33ZbPYXn59EwSgE8CGsHtAeTH5YFeJ9E', 'WBTC (Wrapped BTC "Sollet")', 'BTC'),
    ('solana', 'cbbtcf3aa214zXHbiAZQwf4122FBYbraNdFqgw4iMij', 'cbBTC (Coinbase Wrapped BTC)', 'BTC');
