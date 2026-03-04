# Solana: `Substreams`

Substreams for tracking native, SPL and SPL-2022 tokens for Solana blockchains.

## Supported by Sinks

- [x] [Substreams: File Sink](https://github.com/streamingfast/substreams-sink-files) - Apache Parquet (Protobuf Map modules)
- [x] [Substreams: SQL Sink](https://github.com/streamingfast/substreams-sink-sql) - Clickhouse / ~~PostgreSQL~~

## Substreams Packages

- [x] SPL and SPL-2022 tokens
  - [x] Transfers (Mints, Burns, Approves, Revokes)
  - [x] Balances
- [ ] Native tokens
- [x] Raydium AMM v4
- [x] Pump.fun - Bonding Curve
- [x] Jupiter Aggregator V4
- [x] Jupiter Aggregator V6

## Supported DEXes and AMMs

### Native Support

> Swaps decoded directly from on-chain program instructions
>
> Ordered by most transactions

| Program Name | Program ID |
|------------|--------------|
| Raydium Liquidity Pool V4 | 675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8 |
| Pump.fun AMM | pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA |
| Pump.fun | 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P |
| Raydium Concentrated Liquidity | CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK |
| Raydium CPMM | CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C |
| Meteora DLMM Program | LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo |
| Meteora Pools Program | Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB |
| Meteora DAMM v2 | cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG |
| Raydium Launchpad | LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj |
| Whirlpools Program | whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc |
| Lifinity Swap V2 | 2wT8Yq49kHgDzXuPxZSaeLaH1qbmGXtEyPy64bL7aD3c |
| Phoenix | PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY |
| Openbook V2 | opnb2LAfJYbRMAHHvqjCwQxanZn7ReEHp1k81EohpZb |
| stabble Weighted Swap | swapFpHZwjELNnjvThjajtiVmkz3yPQEHjLtka2fwHW |
| BonkSwap | BSwp6bEBihVLdqJRKGgzjcGLHkcTuzmSo1TQkHepzH8p |
| PancakeSwap | HpNfyc2Saw7RKkQd8nEL4khUcuPhQ7WwY1B2qjx8jxFq |
| Moonshot | MoonCVVNZFSYkqNXP6bxHLPL6QQJiMagDL3qcqUQTrG |
| Darklake | darkr3FB87qAZmgLwKov6Hk9Yiah5UT4rUYu8Zhthw1 |
| Aldrin AMM V2 | CURVGoZn8zycx6FXwwevgBTB2gVvdbGTEpvMJDbgs2t4 |
| Serum DEX V3 | 9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin |
| SolFi | SoLFiHG9TfgtdUXUjWAxi3LtvYuFyDLVhBWxdMZxyCe |
| SolFi V2 | SV2EYYJyRz2YhfXwXnhNAevDEui5Q6yrfyo13WtupPF |
| Obric V2 | coUnmi3oBUtwtd9fU42p6jg75UmdnNMgBQMedGNYEAs |
| Obric V3 | ob2wXVFGiWXkPwYv8CSpPMm5m3NR7DjSrVJAHGSb9Gu |
| GoonFi | goonERTdGsjnkZqWuVjs73BZ3Pb9qoCUdBUL17BnS5j |
| Saros AMM | SSwapUtytfBdBn1b9NUGG6foMVPtcWgpRU32HToDUZr |
| Sanctum | 5ocnV1qiCgaQR8Jb8xWnVbApfaygJ8tNoZfgPwsgx9kx |
| DumpFun | DumpFunGAgW6kPHzWMA3Nnqecyrd6SGnLZvNGp2aHwEa |
| Boop | boop8hVGQGqehUK2iVEMEnMrL5RbjywRzHKBmBE7ry4 |
| Heaven | HEAVENoP2qxoeuF8Dj2oT1GHEnu49U5mJYkdeC8BAX2o |
| Plasma (Gavel) | srAMMzfVHVAtgSJc8iH6CfKzuWuUTzLHVCE81QU1rgi |
| Drift V2 | dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH |
| DFlow V4 | DF1ow4tspfHX9JwWJsAb9epbkA8hmpSEAtxXy1V27QBH |
| OKX DEX V2 | 6m2CDdhRgxpH4WjvdzxAYbGxwdGUz5MziiL5jek2kBma |
| ByReal CLMM | REALQqNEomY6cQGZJUGwywTBD2UmDT32rZcNnfxQ5N2 |

### Aggregator Support

> Swaps reported via **Aggregator** programs (no native decoder yet)
>
> Ordered by most transactions

| AMM Name | AMM ID |
|-----|--------------|
| ZeroFi | ZERor4xhbUycZ6gb9ntrhqscUcZmAbQDjEAtCf4hbZY |
| Invariant Swap | HyaB3W9q6XdA5xwpU4XnSZV94htfmbmqJXZcEbRaJutt |
| 1Dex Program | DEXYosS6oEGvk8uCDayvwEZz4qEyDJRf9nFgYCaqPMTm |
| Fluxbeam Program | FLUXubRmkEi2q6K3Y9kBPg9248ggaZVsoSFhtJHSrm1X |
| Mercurial Stable Swap | MERLuDFBMmsHnsBPZw2sDQZHvXFMwp8EdjudcU2HKky |
| Saber Stable Swap | SSwpkEEcbUqx4vtoEByFjSkhKdCT862DNVb52nZg1UZ |
| Orca Token Swap V2 | 9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP |
| Cropper Whirlpool | H8W3ctz92svYg6mkn1UtGfu2aQr2fnUFHM1RhScEtQDt |
| Lifinity Swap | EewxydAPCCVuNEyrVN68PuSYdQ7wKn27V9Gjeoi8dy3S |
| Meteora Dynamic Bonding Curve Program | dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN |
| stabble Stable Swap | swapNyd8XiQwJ6ianp9snpu4brUqFxadzvHebnAXjJZ |
| Saber Decimal Wrapper | DecZY86MU5Gj7kppfUCEmd4LbXXuyZH1yHaP2NTqdiZB |
| Jupiter Perpetuals | PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu |
| Sanctum Router Program | stkitrT1Uoy18Dk1fTrgPw8W6MVzoCfYoAFT4MLsmhq |
| Numeraire | NUMERUNsFCP3kuNmWZuXtm1AaQCPj9uw6Guv2Ekoi5P |
| Crema Finance Program | CLMM9tUoggJu2wagPkkqs9eFG4BWhVBZWkP1qv3Sp7tR |
| GooseFX: GAMMA | GAMMA7meSFWaBXF25oSUgmGRwaW6sCMFLmBNiMSdbHVT |
| Orca Token Swap | DjVE6JNiYqPL2QXyCUUh8rNjHrbz9hXHNYt99MQ59qw1 |
| OpenBook | srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX |
| Swap Program | SwaPpA9LAaLfeLi3a68M4DjnLqgtticKg6CnyNwgAC8 |
| Guac Swap | Gswppe6ERWKpUTXvRPfXdzHhiCyJvLadVvXGfdpBqcE1 |
| Dexlab Swap | DSwpgjMvXhtGn6BsbqmacdBZyfLj6jSWf3HJpdJtmg6N |
| GooseFX V2 | GFXsSL5sSaDfNFQUYsHekbWBW1TsFdjDYzACh62tEHxn |
| Aldrin AMM | AMM55ShdkoGRB5jVYPjWziwk8m5MpwyDgsMWHaMSQWH6 |
| Helium Treasury Management | treaf4wWBBty3fHdyBpo35Mz84M8k3heKXmjmi9vFt5 |
| Symmetry Engine | 2KehYt3KsEQR53jYcxjbQp2d2kCp4AkuQW68atufRwSr |
| Marinade Finance | MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD |
| Cropper Finance | CTMAxxk34HjKWxQ3QLZK1HpaLXmBveao3ESePXbiyfzh |
| Oasis | 9tKE7Mbmj4mxDjWatikzGAtkoWosiiZX9y6J4Hfm2R8H |
| StepN DOOAR Swap | Dooar9JkhdZ7J3LHN3A7YCuoGRUggXhQaG4kijfLGU2j |
| Penguin Finance | PSwapMdSai8tjrEXcxFeQth87xC4rRsa4VA5mhGhXkP |
| Step Finance Swap Program | SSwpMgqNDsyV7mAgN9ady4bDVu5ySjmmXejXvy2vLt1 |
| WOOFi | WooFif76YGRNjk1pA8wCsN67aQsD9f9iLsz4NcJ1AVb |
| Cykura Swap | cysPXAjehMpVKUapzbMCCnpFxUFFryEWEaLgnb9NrR8 |
| Sentre Swap | D3BBjqUdCYuP18fNvvMbPAZ8DpcRi4io2EsYHQawJDag |
| Dradex Program | dp2waEWSBy5yKmq65ergoU3G6qRLmqa6K7We4rZSKph |
| Gavel | srAMMzfVHVAtgSJc8iH6CfKzuWuUTzLHVCE81QU1rgi |
| Sencha Cpamm | SCHAtsf8mbjyjiv4LkhLKutTf6JnZAbdJKFkXQNMFHZ |
| Clone | C1onEW2kPetmHmwe74YC1ESx3LnFEpVau6g2pg4fHycr |
| 1Sol | 1MooN32fuBBgApc8ujknKJw5sef3BVwPGgz3pto1BAh |
| GooseFX SSL | 7WduLbRfYhTJktjLw5FDEyrqoEv61aTTCuGAetgLjzN5 |
