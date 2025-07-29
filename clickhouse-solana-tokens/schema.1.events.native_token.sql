-- SPL Token Transfers --
CREATE TABLE IF NOT EXISTS native_transfer AS base_events
COMMENT 'Native token transfer';
ALTER TABLE native_transfer
    ADD COLUMN IF NOT EXISTS source                  FixedString(44),
    ADD COLUMN IF NOT EXISTS destination             FixedString(44),
    ADD COLUMN IF NOT EXISTS lamports                UInt64;

-- TransferWithSeed --
CREATE TABLE IF NOT EXISTS transfer_with_seed AS base_events
COMMENT 'Native token transfer with seed';
ALTER TABLE transfer_with_seed
    ADD COLUMN IF NOT EXISTS source                  FixedString(44),
    ADD COLUMN IF NOT EXISTS destination             FixedString(44),
    ADD COLUMN IF NOT EXISTS lamports                UInt64,
    ADD COLUMN IF NOT EXISTS source_base             FixedString(44) COMMENT 'Base account address for the seed.',
    ADD COLUMN IF NOT EXISTS source_owner            FixedString(44) COMMENT 'Owner of the source account.',
    ADD COLUMN IF NOT EXISTS source_seed             String COMMENT 'Seed used to derive the source account.';

-- WithdrawNonceAccount --
CREATE TABLE IF NOT EXISTS withdraw_nonce_account AS base_events
COMMENT 'Native token withdraw nonce account';
ALTER TABLE withdraw_nonce_account
    ADD COLUMN IF NOT EXISTS destination             FixedString(44),
    ADD COLUMN IF NOT EXISTS lamports                UInt64,
    ADD COLUMN IF NOT EXISTS nonce_account           FixedString(44) COMMENT 'Nonce account address.',
    ADD COLUMN IF NOT EXISTS nonce_authority         FixedString(44) COMMENT 'Nonce authority account address.';

-- CreateAccount --
CREATE TABLE IF NOT EXISTS create_account AS base_events
COMMENT 'Native token create account';
ALTER TABLE create_account
    ADD COLUMN IF NOT EXISTS source                  FixedString(44) COMMENT 'Funding account address.',
    ADD COLUMN IF NOT EXISTS new_account             FixedString(44) COMMENT 'New account address.',
    ADD COLUMN IF NOT EXISTS owner                   FixedString(44) COMMENT 'Owner program account address',
    ADD COLUMN IF NOT EXISTS lamports                UInt64 COMMENT 'Initial balance in lamports.',
    ADD COLUMN IF NOT EXISTS space                   UInt64 COMMENT 'Space allocated for the new account.';


-- CreateAccountWithSeed --
CREATE TABLE IF NOT EXISTS create_account_with_seed AS base_events
COMMENT 'Native token create account with seed';
ALTER TABLE create_account_with_seed
    ADD COLUMN IF NOT EXISTS source                  FixedString(44) COMMENT 'Funding account address.',
    ADD COLUMN IF NOT EXISTS new_account             FixedString(44) COMMENT 'New account address.',
    ADD COLUMN IF NOT EXISTS base                    FixedString(44) COMMENT 'Primary base account address used for deriving the seed.',
    ADD COLUMN IF NOT EXISTS base_account_raw        String COMMENT 'Optional secondary account related to the base.',
    ADD COLUMN IF NOT EXISTS base_account            Nullable(FixedString(44)) MATERIALIZED fixed_string_or_null(base_account_raw),
    ADD COLUMN IF NOT EXISTS owner                   FixedString(44) COMMENT 'Owner program account address',
    ADD COLUMN IF NOT EXISTS lamports                UInt64 COMMENT 'Initial balance in lamports.',
    ADD COLUMN IF NOT EXISTS space                   UInt64 COMMENT 'Space allocated for the new account.',
    ADD COLUMN IF NOT EXISTS seed                   String COMMENT 'Seed used to derive the new account.';
