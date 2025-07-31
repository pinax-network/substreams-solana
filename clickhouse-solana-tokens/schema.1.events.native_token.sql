-- SPL Token Transfers --
CREATE TABLE IF NOT EXISTS system_transfer AS base_events
COMMENT 'System token transfer';
ALTER TABLE system_transfer
    ADD COLUMN IF NOT EXISTS source                  String,
    ADD COLUMN IF NOT EXISTS destination             String,
    ADD COLUMN IF NOT EXISTS lamports                UInt64;

-- TransferWithSeed --
CREATE TABLE IF NOT EXISTS system_transfer_with_seed AS base_events
COMMENT 'System token transfer with seed';
ALTER TABLE system_transfer_with_seed
    ADD COLUMN IF NOT EXISTS source                  String,
    ADD COLUMN IF NOT EXISTS destination             String,
    ADD COLUMN IF NOT EXISTS lamports                UInt64,
    ADD COLUMN IF NOT EXISTS source_base             String COMMENT 'Base account address for the seed.',
    ADD COLUMN IF NOT EXISTS source_owner            String COMMENT 'Owner of the source account.',
    ADD COLUMN IF NOT EXISTS source_seed             String COMMENT 'Seed used to derive the source account.';

-- WithdrawNonceAccount --
CREATE TABLE IF NOT EXISTS system_withdraw_nonce_account AS base_events
COMMENT 'System token withdraw nonce account';
ALTER TABLE system_withdraw_nonce_account
    ADD COLUMN IF NOT EXISTS destination             String,
    ADD COLUMN IF NOT EXISTS lamports                UInt64,
    ADD COLUMN IF NOT EXISTS nonce_account           String COMMENT 'Nonce account address.',
    ADD COLUMN IF NOT EXISTS nonce_authority         String COMMENT 'Nonce authority account address.';

-- CreateAccount --
CREATE TABLE IF NOT EXISTS system_create_account AS base_events
COMMENT 'System token create account';
ALTER TABLE system_create_account
    ADD COLUMN IF NOT EXISTS source                  String COMMENT 'Funding account address.',
    ADD COLUMN IF NOT EXISTS new_account             String COMMENT 'New account address.',
    ADD COLUMN IF NOT EXISTS owner                   String COMMENT 'Owner program account address',
    ADD COLUMN IF NOT EXISTS lamports                UInt64 COMMENT 'Initial balance in lamports.',
    ADD COLUMN IF NOT EXISTS space                   UInt64 COMMENT 'Space allocated for the new account.';

-- CreateAccountWithSeed --
CREATE TABLE IF NOT EXISTS system_create_account_with_seed AS base_events
COMMENT 'System token create account with seed';
ALTER TABLE system_create_account_with_seed
    ADD COLUMN IF NOT EXISTS source                  String COMMENT 'Funding account address.',
    ADD COLUMN IF NOT EXISTS new_account             String COMMENT 'New account address.',
    ADD COLUMN IF NOT EXISTS base                    String COMMENT 'Primary base account address used for deriving the seed.',
    ADD COLUMN IF NOT EXISTS base_account_raw        String COMMENT 'Optional secondary account related to the base.',
    ADD COLUMN IF NOT EXISTS base_account            Nullable(String) MATERIALIZED fixed_string_or_null(base_account_raw),
    ADD COLUMN IF NOT EXISTS owner                   String COMMENT 'Owner program account address',
    ADD COLUMN IF NOT EXISTS lamports                UInt64 COMMENT 'Initial balance in lamports.',
    ADD COLUMN IF NOT EXISTS space                   UInt64 COMMENT 'Space allocated for the new account.',
    ADD COLUMN IF NOT EXISTS seed                   String COMMENT 'Seed used to derive the new account.';

-- System Post Balance --
CREATE TABLE IF NOT EXISTS system_post_balances AS base_transactions
COMMENT 'System post balances';
ALTER TABLE system_post_balances
    ADD COLUMN IF NOT EXISTS account                  String COMMENT 'Account address.',
    ADD COLUMN IF NOT EXISTS amount                   UInt64 COMMENT 'Balance amount in lamports.';

-- System Pre Balance --
CREATE TABLE IF NOT EXISTS system_pre_balances AS system_post_balances
COMMENT 'System pre balances';
