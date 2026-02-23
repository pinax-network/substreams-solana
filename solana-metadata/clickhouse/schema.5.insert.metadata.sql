/* ===========================
   ONE-TIME INSERT: Native SOL asset
   =========================== */
INSERT INTO initialize_token_metadata (metadata, update_authority, mint, mint_authority, name, symbol, uri, block_num, block_hash, timestamp)
VALUES ('So11111111111111111111111111111111111111111', '', 'So11111111111111111111111111111111111111111', '', 'Native', 'Native', '', toUInt64(0), '', toDateTime(0, 'UTC'));
