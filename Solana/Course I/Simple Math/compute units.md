Solana calls "gas" a "compute unit." By default, a transaction is limited to 200,000 compute units. If more than 200,000 compute units are consumed, the transaction reverts.


Floating points
---------------

One nice thing about using Rust for smart contracts is that we don't have to import libraries like Solmate or Solady to do math. Rust is a pretty sophisticated language with a lot of operations built in, and if we need some piece of code, we can look outside the Solana ecosystem for a Rust crate (this is what libraries are called in Rust) to do the job.

Let's take the cube root of 50. The cube root function for floats is built into the Rust language with the function `cbrt()`.

----
```rust
// note that we changed `a` to f32 (float 32)
// because `cbrt()` is not available for u64
pub fn initialize(ctx: Context<Initialize>, a: f32) -> Result<()> {
  msg!("You said {:?}", a.cbrt());
  Ok(());
}

```

 floats can be computationally **expensive**? Well, here we see our cube root operation consumed over 5 times as much as simple arithmetic on unsigned integers:

``` bash
Transaction executed in slot unspecified:
  Signature: VfvySG5vvVSAnsYLCsvB9N6PsuGwL39kKd1fMsyvuB7y5DUHURwQVHU9rv3Xkz5NJqGHLSXoWoW92zJb5VKYCEF
  Status: Ok
  Log Messages:
    Program 8o3ehd3XnyDocd9hG1uz5trbmSRB7gaLaE9BCXDpEnMY invoke [1]
    Program log: Instruction: Initialize
    Program log: attempting to begin the function with 50
    Program log: Result = 3.6840315
    Program 8o3ehd3XnyDocd9hG1uz5trbmSRB7gaLaE9BCXDpEnMY consumed 4860 of 200000 compute units
    Program 8o3ehd3XnyDocd9hG1uz5trbmSRB7gaLaE9BCXDpEnMY success


```