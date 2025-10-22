-- Set Projections for Transfers --
ALTER TABLE transfers ON CLUSTER 'tokenapis-b'
    -- Base Events --
    ADD PROJECTION IF NOT EXISTS prj_signature (SELECT signature, timestamp, _part_offset ORDER BY (signature, timestamp)),
    ADD PROJECTION IF NOT EXISTS prj_fee_payer (SELECT fee_payer, timestamp, _part_offset ORDER BY (fee_payer, timestamp)),
    ADD PROJECTION IF NOT EXISTS prj_signer (SELECT signer, timestamp, _part_offset ORDER BY (signer, timestamp)),
    -- Transfers --
    ADD PROJECTION IF NOT EXISTS prj_authority (SELECT authority, timestamp, _part_offset ORDER BY (authority, timestamp)),
    ADD PROJECTION IF NOT EXISTS prj_source (SELECT source, timestamp, _part_offset ORDER BY (source, timestamp)),
    ADD PROJECTION IF NOT EXISTS prj_destination (SELECT destination, timestamp, _part_offset ORDER BY (destination, timestamp)),
    ADD PROJECTION IF NOT EXISTS prj_mint (SELECT mint, timestamp, _part_offset ORDER BY (mint, timestamp));

-- Base Events --
ALTER TABLE transfers ON CLUSTER 'tokenapis-b' MATERIALIZE PROJECTION prj_signature;
ALTER TABLE transfers ON CLUSTER 'tokenapis-b' MATERIALIZE PROJECTION prj_fee_payer;
ALTER TABLE transfers ON CLUSTER 'tokenapis-b' MATERIALIZE PROJECTION prj_signer;

-- Transfers --
ALTER TABLE transfers ON CLUSTER 'tokenapis-b' MATERIALIZE PROJECTION prj_authority;
ALTER TABLE transfers ON CLUSTER 'tokenapis-b' MATERIALIZE PROJECTION prj_source;
ALTER TABLE transfers ON CLUSTER 'tokenapis-b' MATERIALIZE PROJECTION prj_destination;
ALTER TABLE transfers ON CLUSTER 'tokenapis-b' MATERIALIZE PROJECTION prj_mint;
