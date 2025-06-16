# Solana Clock

> Injects missing timestamp to clock.

> Solana never stored a wall-clock timestamp inside the block/slot itself.
> Solana targets 400 ms per slot, but historical averages drift (network slowdowns, skipped slots, etc.).
> Measuring over a wide window gives you the era-specific rate.

## Reason

- Timestamps are only ever attached to vote transactions, not to the blocks themselves.
- Once votes with timestamps started landing (mid-2020), every later slot can be dated precisely; earlier ones can be dated by counting PoH slots backwards using the observed slot-rate.
- The method avoids hard-coding a 0.4 s figure and instead learns the real historical rate from the chain itself, giving better accuracy when the network was running slower than nominal.

<https://solana.stackexchange.com/questions/9393/older-solana-blocks-have-no-timestamp>
