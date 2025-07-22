-- ──────────────────────────────────────────────────────────────────────────
-- Raydium AMM V4 Swaps
-- ──────────────────────────────────────────────────────────────────────────
--- SwapBaseIn --
CREATE TABLE IF NOT EXISTS raydium_amm_v4_swap_base_in (
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
    token_program               FixedString(44) COMMENT 'Token program (usually SPL Token)',
    amm                         FixedString(44) COMMENT 'AMM pool account (Raydium V4 liquidity-state)',
    amm_authority               FixedString(44) COMMENT 'AMM authority PDA',
    amm_open_orders             FixedString(44) COMMENT 'AMM open-orders',
    amm_target_orders           FixedString(44) COMMENT 'AMM target-orders',
    amm_coin_vault              FixedString(44) COMMENT 'AMM coin vault (base-token vault)',
    amm_pc_vault                FixedString(44) COMMENT 'AMM pc vault (quote-token vault)',
    market_program              FixedString(44) COMMENT 'OpenBook (or Serum) DEX program',
    market                      FixedString(44) COMMENT 'OpenBook (or Serum) DEX market account',
    market_bids                 FixedString(44) COMMENT 'Market account',
    market_asks                 FixedString(44) COMMENT 'Market bids slab',
    market_event_queue          FixedString(44) COMMENT 'Market asks slab',
    market_coin_vault           FixedString(44) COMMENT 'Market event queue',
    market_pc_vault             FixedString(44) COMMENT 'Market pc vault (quote)',
    market_vault_signer         FixedString(44) COMMENT 'Market vault-signer PDA',
    user_token_source           FixedString(44) COMMENT 'User source ATA (base token)',
    user_token_destination      FixedString(44) COMMENT 'User destination ATA (quote token)',
    user_source_owner           FixedString(44) COMMENT 'User wallet (authority & fee-payer)',

    -- data --
    amount_in                   UInt64,
    minimum_amount_out          UInt64,

    -- log --
    amount_out                  UInt64,
    direction                   Enum8('PC2Coin' = 1, 'Coin2PC' = 2),
    user_source                 UInt64,
    pool_coin                   UInt64,
    pool_pc                     UInt64,

    -- projections (parts) --
    -- https://clickhouse.com/docs/sql-reference/statements/alter/projection#normal-projection-with-part-offset-field
    PROJECTION prj_part_signature       (SELECT signature,      _part_offset ORDER BY signature),
    PROJECTION prj_part_fee_payer       (SELECT fee_payer,      _part_offset ORDER BY fee_payer),
    PROJECTION prj_part_signer          (SELECT signer,         _part_offset ORDER BY signer)
)
ENGINE = MergeTree
ORDER BY (
    timestamp, block_num,
    block_hash, transaction_index, instruction_index
)
COMMENT 'Raydium AMM V4 Swap Base In';

--- SwapBaseOut --
CREATE TABLE IF NOT EXISTS raydium_amm_v4_swap_base_out AS raydium_amm_v4_swap_base_in
COMMENT 'Raydium AMM V4 Swap Base Out';
ALTER TABLE raydium_amm_v4_swap_base_out RENAME COLUMN IF EXISTS minimum_amount_out TO max_amount_in;
