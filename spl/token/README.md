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

### Memo

- [ ] `Memo` (SPL Memo V1 & V2)

### Metaplex Token Metadata Program ID

- [x] `metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s`

### Memo Program IDs

- [x] `Memo1UhkJRfHyvLMcVucJwxXeuD728EqVDDwQDxFMNo`
- [x] `MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr`

### SPL Token Programs

- [x] `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA` (Token)
- [x] `TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb` (Token-2022)
- [x] `ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL` (Associated Token)
- [x] `cTokenmWW8bLPjZEBAUgYy3zKxQZW6VKi7bqNFEVv3m` (Confidential Token)

### Wrapped SOL Mint

- [x] `So11111111111111111111111111111111111111112` (commonly seen)
