-- SPL Token-2022 & Classic balance changes by mint/owner --
-- Only keep last balance change per account per block, use `execution_index` to resolve conflicts
CREATE TABLE IF NOT EXISTS balance_changes  (
    -- block --
    block_num           UInt32,
    block_hash          FixedString(44),
    timestamp           DateTime(0, 'UTC'),
    timestamp_since_genesis     DateTime(0, 'UTC')
        MATERIALIZED if (
            timestamp = 0,
            toDateTime(1584332940 + intDiv(block_num * 2, 5), 'UTC'),
            timestamp
        ),

    -- ordering --
    transaction_index               UInt32,
    post_token_balances_index       UInt32,

    -- transaction --
    signature                   FixedString(88),
    fee_payer                   FixedString(44),
    signers_raw                 String,
    signers                     Array(FixedString(44)) MATERIALIZED arrayMap(x -> toFixedString(x, 44), splitByChar(',', signers_raw)),
    signer                      FixedString(44) MATERIALIZED if(length(signers) > 0, signers[1], ''),
    fee                         UInt64 DEFAULT 0,
    compute_units_consumed      UInt64 DEFAULT 0,

    -- instruction --
    program_id                  LowCardinality(FixedString(44)),

    -- event --
    owner               FixedString(44),
    mint                FixedString(44),
    amount              UInt64,
    decimals            UInt8
)
ENGINE = MergeTree
ORDER BY (
    program_id, mint, owner,
    block_hash, transaction_index, post_token_balances_index
);
