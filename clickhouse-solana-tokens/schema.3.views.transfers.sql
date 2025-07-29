CREATE TABLE IF NOT EXISTS transfers AS spl_transfer
COMMENT 'SPL 2022 & Native token transfers';

-- SPL Token Transfers --
ALTER TABLE transfers
    -- JOIN `*_owner` with `initialize_accounts` TABLE --
    ADD COLUMN IF NOT EXISTS source_owner            FixedString(44),
    ADD COLUMN IF NOT EXISTS destination_owner       FixedString(44),
    -- require `decimals` to be present for token transfers
    DROP COLUMN IF EXISTS decimals,
    DROP COLUMN IF EXISTS decimals_raw,
    ADD COLUMN decimals UInt8,
    -- require `mint` to be present for token transfers
    DROP COLUMN IF EXISTS mint,
    DROP COLUMN IF EXISTS mint_raw,
    ADD COLUMN mint FixedString(44);

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_spl_transfer
TO transfers AS
SELECT
    -- base fields --
    t.block_num AS block_num,
    t.block_hash AS block_hash,
    t.timestamp AS timestamp,
    t.transaction_index AS transaction_index,
    t.instruction_index AS instruction_index,
    t.signature AS signature,
    t.fee_payer AS fee_payer,
    t.signers_raw AS signers_raw,
    t.fee AS fee,
    t.compute_units_consumed AS compute_units_consumed,
    t.program_id AS program_id,
    t.stack_height AS stack_height,

    -- events fields --
    t.authority AS authority,
    t.multisig_authority_raw AS multisig_authority_raw,
    t.source AS source,
    t.destination AS destination,
    t.amount AS amount,
    t.mint AS mint,
    -- if transfer.decimals is not null, use mint.decimals
    ifNull(t.decimals, m.decimals) AS decimals,

    -- JOIN fields --
    a1.owner AS source_owner,
    a2.owner AS destination_owner
FROM spl_transfer AS t
JOIN accounts AS a1 ON (t.mint = a1.mint AND t.source = a1.account)
JOIN accounts AS a2 ON (t.mint = a2.mint AND t.destination = a2.account)
JOIN mints AS m ON m.mint = t.mint
-- ignore 0 transfers
WHERE t.amount > 0 AND t.mint IS NOT NULL;