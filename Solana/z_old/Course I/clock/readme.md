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

-------

block.number in Solana
----------------------

Solana has a notion of a "slot number" which is very related to the "block number" but is not the same thing.

block.coinbase
--------------

In Ethereum, the "Block Coinbase" represents the address of the miner who has successfully mined a block in Proof of Work (PoW). On the other hand, Solana uses a leader-based consensus mechanism which is a combination of both Proof of History (PoH) and Proof of Stake (PoS), removing the concept of mining. Instead, a [block or slot leader](https://docs.solana.com/cluster/leader-rotation) is appointed to validate transactions and propose blocks during certain intervals, under a system known as the [leader schedule](https://docs.solana.com/cluster/leader-rotation#leader-schedule-rotation). This schedule determines who will be the block producer at a certain time.

However, presently, there's no specific way to access the address of the block leader in Solana programs.

----

block.gaslimit
--------------

Solana has a per-block compute unit [limit of 48 million](https://github.com/solana-labs/solana/issues/29492). Each transaction is by default limited to 200,000 compute units, though it can be raised to 1.4 million compute units ( [see an example here](https://solanacookbook.com/references/basic-transactions.html#how-to-change-compute-budget-fee-priority-for-a-transaction)).

It is **not possible** to access this limit from a Rust program

--

block.basefee
-------------

In Ethereum, the basefee is dynamic per EIP-1559; it is a function of previous block utilization. In Solana, the base price of a transaction is static, so there is no need for a variable like this.

block.difficulty
----------------

Block difficulty is a concept associated with Proof of Work (PoW) blockchains. Solana, on the other hand, operates on a Proof of History (PoH) combined with Proof of Stake (PoS) consensus mechanism, which doesn't involve the concept of block difficulty.

block.chainid
-------------

Solana does not have a chain id because it is not an EVM compatible blockchain. The block.chainid is how Solidity smart contracts know if they are on a testnet, L2, mainnet or some other EVM compatible chain.

Solana runs separate clusters for [Devnet, Testnet, and Mainnet](https://docs.solana.com/clusters), but programs do not have a mechanism to know which one they are on. However, you can programatically adjust your code at deploy time using the Rust cfg feature to have different features depending on which cluster it is deployed to. Here is an example of [changing the program id depending on the cluster](https://solana.stackexchange.com/questions/848/how-to-have-a-different-program-id-depending-on-the-cluster).