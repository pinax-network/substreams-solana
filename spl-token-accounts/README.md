# SPL Token - Accounts

> Tracks lifecycle and identity of token accounts.
> These operations define when a token account becomes active, who owns it, and when it's destroyed.

## Includes

- [x] `InitializeAccount*`
- [x] `InitializeImmutableOwner`
- [x] `CloseAccount` (sets balance to zero)
- [x] `SetAuthority` (for `AccountOwner`, `CloseAccount` authority)
- [ ] ~~`Reallocate` (if it reshapes the account layout)~~
