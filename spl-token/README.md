# SPL Token

> SPL Token & SPL Token 2022 program instructions and Pre-Post Token Balances.

## Includes

### Transfers

- [x] `Transfer`
- [x] `TransferChecked`
- [x] `MintTo`
- [x] `MintToChecked`
- [x] `Burn`
- [x] `BurnChecked`

### Balances

- [x] `PreTokenBalance`
- [x] `PostTokenBalance`

### Permissions

- [x] `Approve` — delegate transfer rights
- [x] `Revoke` — revoke delegate rights
- [x] `FreezeAccount` — disable account
- [x] `ThawAccount` — re-enable account

### Mints

- [x] `InitializeMint/2`
- [ ] ~~`InitializeMintCloseAuthority`~~ (not implemented yet)

### Accounts

- [x] `InitializeAccount/2/3`
- [x] `InitializeImmutableOwner`
- [x] `CloseAccount` (sets balance to zero)
- [x] `SetAuthority` (for `AccountOwner`, `CloseAccount` authority)
- [ ] ~~`Reallocate`~~ (doesn't seem to emit any events)

### Metadata

- [x] `InitializeTokenMetadata` (SPL-2022)
- [x] `UpdateTokenMetadataAuthority` (SPL-2022)
- [x] `UpdateTokenMetadataField` (SPL-2022)
- [x] `RemoveTokenMetadataField` (SPL-2022)

### Extensions

- [ ] ~~`MemoTransferExtension`~~ (doesn't seem to emit any events)
- [ ] ~~`TransferCheckedWithFee` (SPL-2022)~~ (not implemented yet)
