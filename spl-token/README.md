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

- [x] `InitializeMint*`

### Accounts

- [x] `InitializeAccount*`
- [x] `InitializeImmutableOwner`
- [x] `CloseAccount` (sets balance to zero)
- [x] `SetAuthority` (for `AccountOwner`, `CloseAccount` authority)
- [ ] ~~`Reallocate` (if it reshapes the account layout)~~
- [ ] ~~`AuthorizeAccount` (SPL-2022)~~
