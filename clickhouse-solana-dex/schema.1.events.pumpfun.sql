-- ──────────────────────────────────────────────────────────────────────────
-- Pump.fun Bonding Curve
-- ──────────────────────────────────────────────────────────────────────────
-- Buy --
CREATE TABLE IF NOT EXISTS pumpfun_buy (
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
    signers                     Array(FixedString(44)) MATERIALIZED arrayMap(x -> toFixedString(x, 44), splitByChar(',', signers_raw)),
    signer                      FixedString(44) MATERIALIZED if(length(signers) > 0, signers[1], ''),
    fee                         UInt64 DEFAULT 0,
    compute_units_consumed      UInt64 DEFAULT 0,

    -- instruction --
    program_id                  LowCardinality(FixedString(44)),
    stack_height                UInt32,

    -- accounts --
    global                      FixedString(44),
    fee_recipient               FixedString(44),
    mint                        FixedString(44),
    bonding_curve               FixedString(44),
    associated_bonding_curve    FixedString(44),
    associated_user             FixedString(44),
    user                        FixedString(44),
    creator_vault               FixedString(44),

    -- data --
    amount                      UInt64,
    max_sol_cost                UInt64,

    -- event --
    sol_amount                  UInt64,
    token_amount                UInt64,
    is_buy                      Bool,
    virtual_sol_reserves        UInt64,
    virtual_token_reserves      UInt64,
    real_sol_reserves           UInt64 DEFAULT 0,
    real_token_reserves         UInt64 DEFAULT 0,
    protocol_fee_recipient      FixedString(44) DEFAULT '',
    protocol_fee_basis_points   UInt64 DEFAULT 0, -- basis-points, 1 bp = 0.01 %
    protocol_fee                UInt64 DEFAULT 0, -- lamports
    creator                     FixedString(44) DEFAULT '',
    creator_fee_basis_points    UInt64 DEFAULT 0,
    creator_fee                 UInt64 DEFAULT 0 -- lamports

)
ENGINE = MergeTree
ORDER BY (
    timestamp, block_num,
    block_hash, transaction_index, instruction_index
)
COMMENT 'Pump.fun Bonding Curve Buy';

-- PROJECTIONS (Part) --
-- https://clickhouse.com/docs/sql-reference/statements/alter/projection#normal-projection-with-part-offset-field
ALTER TABLE jupiter_swap ADD PROJECTION IF NOT EXISTS prj_part_signature       (SELECT signature,      _part_offset ORDER BY signature);
ALTER TABLE jupiter_swap ADD PROJECTION IF NOT EXISTS prj_part_fee_payer       (SELECT fee_payer,      _part_offset ORDER BY fee_payer);
ALTER TABLE jupiter_swap ADD PROJECTION IF NOT EXISTS prj_part_signer          (SELECT signer,         _part_offset ORDER BY signer);

-- Sell --
CREATE TABLE IF NOT EXISTS pumpfun_sell AS pumpfun_buy
COMMENT 'Pump.fun Bonding Curve Sell';
ALTER TABLE pumpfun_sell RENAME COLUMN IF EXISTS max_sol_cost TO min_sol_output;