CREATE TABLE IF NOT EXISTS account_extensions (
  version               UInt64,
  account               String,
  is_owner_immutable    Boolean
)
ENGINE = ReplacingMergeTree(version)
ORDER BY account;

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_initialize_immutable_owner
TO account_extensions AS
SELECT
  to_version(block_num, transaction_index, instruction_index) AS version,
  account,
  true AS is_owner_immutable
FROM initialize_immutable_owner;
