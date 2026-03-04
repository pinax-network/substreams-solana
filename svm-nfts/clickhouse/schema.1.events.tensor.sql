-- Tensor List --
CREATE TABLE IF NOT EXISTS tensor_list (
    -- block --
    block_num                   UInt32,
    block_hash                  FixedString(44),
    timestamp                   DateTime(0, 'UTC'),

    -- ordering --
    transaction_index           UInt32,
    instruction_index           UInt32,

    -- transaction --
    signature                   FixedString(88),
    fee_payer                   FixedString(44),
    signers_raw                 String,
    fee                         UInt64 DEFAULT 0,
    compute_units_consumed      UInt64 DEFAULT 0,

    -- instruction --
    program_id                  LowCardinality(FixedString(44)),
    stack_height                UInt32,

    -- data --
    amount                      UInt64 COMMENT 'List price in lamports',
    expire_in_sec               UInt64 COMMENT 'Expiry in seconds (0 = no expiry)',
    currency                    FixedString(44) COMMENT 'Currency mint (empty = SOL)',
    nft_standard                LowCardinality(String) COMMENT 'NFT standard (cNFT, pNFT, T22, WNS, Core)'

) ENGINE = ReplacingMergeTree
ORDER BY (timestamp, block_num, block_hash, transaction_index, instruction_index)
COMMENT 'Tensor NFT listings';

-- Tensor Buy --
CREATE TABLE IF NOT EXISTS tensor_buy (
    -- block --
    block_num                   UInt32,
    block_hash                  FixedString(44),
    timestamp                   DateTime(0, 'UTC'),

    -- ordering --
    transaction_index           UInt32,
    instruction_index           UInt32,

    -- transaction --
    signature                   FixedString(88),
    fee_payer                   FixedString(44),
    signers_raw                 String,
    fee                         UInt64 DEFAULT 0,
    compute_units_consumed      UInt64 DEFAULT 0,

    -- instruction --
    program_id                  LowCardinality(FixedString(44)),
    stack_height                UInt32,

    -- data --
    max_amount                  UInt64 COMMENT 'Maximum price willing to pay in lamports',
    nft_standard                LowCardinality(String) COMMENT 'NFT standard (cNFT, pNFT, T22, WNS, Core)'

) ENGINE = ReplacingMergeTree
ORDER BY (timestamp, block_num, block_hash, transaction_index, instruction_index)
COMMENT 'Tensor NFT buys';

-- Tensor Bid --
CREATE TABLE IF NOT EXISTS tensor_bid (
    -- block --
    block_num                   UInt32,
    block_hash                  FixedString(44),
    timestamp                   DateTime(0, 'UTC'),

    -- ordering --
    transaction_index           UInt32,
    instruction_index           UInt32,

    -- transaction --
    signature                   FixedString(88),
    fee_payer                   FixedString(44),
    signers_raw                 String,
    fee                         UInt64 DEFAULT 0,
    compute_units_consumed      UInt64 DEFAULT 0,

    -- instruction --
    program_id                  LowCardinality(FixedString(44)),
    stack_height                UInt32,

    -- data --
    bid_id                      FixedString(44) COMMENT 'Bid account ID',
    target                      UInt32 COMMENT 'Target type (0=AssetId, 1=Whitelist)',
    target_id                   FixedString(44) COMMENT 'Target ID (asset or whitelist)',
    amount                      UInt64 COMMENT 'Bid amount in lamports',
    quantity                    UInt32 COMMENT 'Number of NFTs to buy',
    expire_in_sec               UInt64 COMMENT 'Expiry in seconds (0 = no expiry)',
    currency                    FixedString(44) COMMENT 'Currency mint (empty = SOL)'

) ENGINE = ReplacingMergeTree
ORDER BY (timestamp, block_num, block_hash, transaction_index, instruction_index)
COMMENT 'Tensor NFT bids';

-- Tensor Take (sale event) --
CREATE TABLE IF NOT EXISTS tensor_take (
    -- block --
    block_num                   UInt32,
    block_hash                  FixedString(44),
    timestamp                   DateTime(0, 'UTC'),

    -- ordering --
    transaction_index           UInt32,
    instruction_index           UInt32,

    -- transaction --
    signature                   FixedString(88),
    fee_payer                   FixedString(44),
    signers_raw                 String,
    fee                         UInt64 DEFAULT 0,
    compute_units_consumed      UInt64 DEFAULT 0,

    -- log --
    program_id                  LowCardinality(FixedString(44)),

    -- data --
    taker                       FixedString(44) COMMENT 'Taker account',
    bid_id                      FixedString(44) COMMENT 'Bid ID (empty for listings)',
    target                      UInt32 COMMENT 'Target type',
    target_id                   FixedString(44) COMMENT 'Target ID',
    amount                      UInt64 COMMENT 'Sale price in lamports',
    quantity                    UInt32 COMMENT 'Number of NFTs traded',
    tcomp_fee                   UInt64 COMMENT 'Protocol fee',
    taker_broker_fee            UInt64 COMMENT 'Taker broker fee',
    maker_broker_fee            UInt64 COMMENT 'Maker broker fee',
    creator_fee                 UInt64 COMMENT 'Creator royalty fee',
    currency                    FixedString(44) COMMENT 'Currency mint (empty = SOL)',
    asset_id                    FixedString(44) COMMENT 'NFT asset ID'

) ENGINE = ReplacingMergeTree
ORDER BY (timestamp, block_num, block_hash, transaction_index, instruction_index)
COMMENT 'Tensor NFT take events (sales)';
