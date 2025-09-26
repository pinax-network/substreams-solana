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

### Full Support of DEX protocol

> Includes `amm_pool` field
>
> Swaps are not being reported directly from Swap programs
>
> Programs ordered by most transactions

| Program Name | Program ID |
|------------|--------------|
| 675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8 | Raydium Liquidity Pool V4 |
| pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA | Pump.fun AMM |
| 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P | Pump.fun |
| CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK | Raydium Concentrated Liquidity |
| CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C | Raydium CPMM |
| LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo | Meteora DLMM Program |
| Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB | Meteora Pools Program |
| cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG | Meteora DAMM v2 |
| LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj | Raydium Launchpad |

### Partial Support of AMM

> Does **NOT** include `amm_pool` field
>
> Swaps are not being reported via Aggregator programs
>
> AMMs ordered by most transactions

| AMM Name | AMM ID |
|-----|--------------|
| Whirlpools Program | whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc |
| Lifinity Swap V2 | 2wT8Yq49kHgDzXuPxZSaeLaH1qbmGXtEyPy64bL7aD3c |
| SolFi | SoLFiHG9TfgtdUXUjWAxi3LtvYuFyDLVhBWxdMZxyCe |
| Obric V2 | obriQD1zbpyLz95G5n7nJe6a4DPjpFwa5XYPoNm113y |
| stabble Stable Swap | swapNyd8XiQwJ6ianp9snpu4brUqFxadzvHebnAXjJZ |
| Phoenix | PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY |
| ZeroFi | ZERor4xhbUycZ6gb9ntrhqscUcZmAbQDjEAtCf4hbZY |
| Openbook V2 | opnb2LAfJYbRMAHHvqjCwQxanZn7ReEHp1k81EohpZb |
| Invariant Swap | HyaB3W9q6XdA5xwpU4XnSZV94htfmbmqJXZcEbRaJutt |
| stabble Weighted Swap | swapFpHZwjELNnjvThjajtiVmkz3yPQEHjLtka2fwHW |
| GoonFi | goonERTdGsjnkZqWuVjs73BZ3Pb9qoCUdBUL17BnS5j |
| 1Dex Program | DEXYosS6oEGvk8uCDayvwEZz4qEyDJRf9nFgYCaqPMTm |
| Fluxbeam Program | FLUXubRmkEi2q6K3Y9kBPg9248ggaZVsoSFhtJHSrm1X |
| Mercurial Stable Swap | MERLuDFBMmsHnsBPZw2sDQZHvXFMwp8EdjudcU2HKky |
| Saber Stable Swap | SSwpkEEcbUqx4vtoEByFjSkhKdCT862DNVb52nZg1UZ |
| Orca Token Swap V2 | 9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP |
| Cropper Whirlpool | H8W3ctz92svYg6mkn1UtGfu2aQr2fnUFHM1RhScEtQDt |
| Lifinity Swap | EewxydAPCCVuNEyrVN68PuSYdQ7wKn27V9Gjeoi8dy3S |
| Meteora Dynamic Bonding Curve Program | dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN |
| Moonit | MoonCVVNZFSYkqNXP6bxHLPL6QQJiMagDL3qcqUQTrG |
| BonkSwap | BSwp6bEBihVLdqJRKGgzjcGLHkcTuzmSo1TQkHepzH8p |
| Saber Decimal Wrapper | DecZY86MU5Gj7kppfUCEmd4LbXXuyZH1yHaP2NTqdiZB |
| Saros AMM | SSwapUtytfBdBn1b9NUGG6foMVPtcWgpRU32HToDUZr |
| Jupiter Perpetuals | PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu |
| Sanctum Router Program | stkitrT1Uoy18Dk1fTrgPw8W6MVzoCfYoAFT4MLsmhq |
| Numeraire | NUMERUNsFCP3kuNmWZuXtm1AaQCPj9uw6Guv2Ekoi5P |
| Crema Finance Program | CLMM9tUoggJu2wagPkkqs9eFG4BWhVBZWkP1qv3Sp7tR |
| Sanctum Program | 5ocnV1qiCgaQR8Jb8xWnVbApfaygJ8tNoZfgPwsgx9kx |
| GooseFX: GAMMA | GAMMA7meSFWaBXF25oSUgmGRwaW6sCMFLmBNiMSdbHVT |
| Orca Token Swap | DjVE6JNiYqPL2QXyCUUh8rNjHrbz9hXHNYt99MQ59qw1 |
| OpenBook | srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX |
| Swap Program | SwaPpA9LAaLfeLi3a68M4DjnLqgtticKg6CnyNwgAC8 |
| Guac Swap | Gswppe6ERWKpUTXvRPfXdzHhiCyJvLadVvXGfdpBqcE1 |
| Dexlab Swap | DSwpgjMvXhtGn6BsbqmacdBZyfLj6jSWf3HJpdJtmg6N |
| GooseFX V2 | GFXsSL5sSaDfNFQUYsHekbWBW1TsFdjDYzACh62tEHxn |
| Aldrin AMM V2 | CURVGoZn8zycx6FXwwevgBTB2gVvdbGTEpvMJDbgs2t4 |
| Helium Treasury Management | treaf4wWBBty3fHdyBpo35Mz84M8k3heKXmjmi9vFt5 |
| Aldrin AMM | AMM55ShdkoGRB5jVYPjWziwk8m5MpwyDgsMWHaMSQWH6 |
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
| Serum DEX V3 | 9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin |
