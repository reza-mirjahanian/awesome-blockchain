In Solidity, we have the following commonly used block variables:

-   block.timestamp
-   block.number
-   blockhash()

And the lesser known ones:

-   block.coinbase
-   block.basefee
-   block.chainid
-   block.difficulty / block.prevrandao

block.timestamp in Solana
-------------------------

By utilizing the `unix_timestamp` field within the [Clock sysvar](https://docs.solanalabs.com/runtime/sysvars), we can access the block timestamp Solana.

```rust
pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let clock: Clock = Clock::get()?;
    msg!(
        "Block timestamp: {}",
        // Get block.timestamp
        clock.unix_timestamp,
    );
    Ok(())
}

```

Anchor's prelude module contains the Clock struct, which is automatically imported by default:

```
use anchor_lang::prelude::*;
```

Somewhat confusingly, the type returned by `unix_timestamp` is an `i64`, not a `u64` meaning it supports negative numbers even though time cannot be negative. Time deltas however can be negative.