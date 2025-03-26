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



### Getting the day of the week

Now let's create a program that tells us the current day of the week using the `unix_timestamp` from the Clock `sysvar`.

The [chrono](https://docs.rs/chrono/latest/chrono/) crate provides functionality for operations on dates and times in Rust.

Add the `chrono` crate as a dependency in our Cargo.toml file in the program directory `./sysvar/Cargo.toml`:

```
[dependencies]
chrono = "0.4.31"
```

**Import the `chrono` crate inside the `sysvar` module:**

```
// ...other code

#[program]
pub mod sysvar {
    use super::*;
    use chrono::*;  // new line here

    // ...
}

```

**Now, we add this function below to our program:**

```
pub fn get_day_of_the_week(
    _ctx: Context<Initialize>) -> Result<()> {

    let clock = Clock::get()?;
    let time_stamp = clock.unix_timestamp; // current timestamp

    let date_time = chrono::NaiveDateTime::from_timestamp_opt(time_stamp, 0).unwrap();
    let day_of_the_week = date_time.weekday();

    msg!("Week day is: {}", day_of_the_week);

    Ok(())
}

```

We pass the current unix timestamp we get from the Clock `sysvar` as an argument to the `from_timestamp_opt` function which returns a `NaiveDateTime` struct containing a date and time. Then we call the weekday method to get the current weekday based on the timestamp we passed.

**And update our test:**

```
it("Get day of the week", async () => {
    const tx = await program.methods.getDayOfTheWeek().rpc();
    console.log("Your transaction signature", tx);
});

```

**Run the test again and get the following logs:**

```
Transaction executed in slot 36:
  Signature: 5HVAjmo85Yi3yeQX5t6fNorU1da4H1zvgcJN7BaiPGnRwQhjbKd5YHsVE8bppU9Bg2toF4iVBvhbwkAtMo4NJm7V
  Status: Ok
  Log Messages:
    Program H52ppiSyiZyYVn1Yr9DgeUKeChktUiPwDfuuo932Uqxy invoke [1]
    Program log: Instruction: GetDayOfTheWeek
    Program log: Week day is: Wed
    Program H52ppiSyiZyYVn1Yr9DgeUKeChktUiPwDfuuo932Uqxy consumed 1597 of 200000 compute units

```

Notice the "`Week day is: Wed`" log.