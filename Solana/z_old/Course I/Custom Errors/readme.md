
**The significant difference between how Ethereum and Solana stops transactions with invalid parameters is that Ethereum triggers a revert and Solana returns an error.**

```rust
use anchor_lang::prelude::*;

declare_id!("8o3ehd3XnyDocd9hG1uz5trbmSRB7gaLaE9BCXDpEnMY");

#[program]
pub mod day4 {
    use super::*;

    pub fn limit_range(ctx: Context<LimitRange>, a: u64) -> Result<()> {
        if a < 10 {
            return err!(MyError::AisTooSmall);
        }
        if a > 100 {
            return err!(MyError::AisTooBig);
        }
        msg!("Result = {}", a);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LimitRange {}

#[error_code]
pub enum MyError {
    #[msg("a is too big")]
    AisTooBig,
    #[msg("a is too small")]
    AisTooSmall,
}


```

Using require statements
------------------------

There is a `require!` macro, which is conceptually the same as `require` from Solidity, which we can use to consolidate our code. Switching from `if` checks (which take three lines) to `require!` calls, our earlier code translates to the following:

```rust
pub fn limit_range(ctx: Context<LimitRange>, a: u64) -> Result<()> {
    require!(a >= 10, Day4Error::AisTooSmall);
    require!(a <= 100, Day4Error::AisTooBig);

    msg!("Result = {}", a);
    Ok(())
}

```

In Ethereum, we know nothing gets logged if a function reverts, even if the revert happens after the log. For example, a call to `tryToLog` in the contract below would not log anything, because the function reverts:

```
contract DoesNotLog {
    event SomeEvent(uint256);

    function tryToLog() public {
        emit SomeEvent(100);
        require(false);
    }
}

```

**Exercise**: What happens if you put a `msg!` macro before the return error statements in a Solana program function? What happens if you replace `return err!` with `Ok(())`? Below we have a function that logs something with `msg!` then returns an error. See if the contents of the `msg!` macro get logged.

```rust
pub fn func(ctx: Context<ReturnError>) -> Result<()> {
    msg!("Will this print?");
    return err!(Day4Error::AlwaysErrors);
}

#[derive(Accounts)]
pub struct ReturnError {}

#[error_code]
pub enum Day4Error {
    #[msg("AlwaysErrors")]
    AlwaysErrors,
}

```

**Under the hood, the `require!` macro is no different from returning an error, it's just syntactic sugar.**

The expected result is that "`Will this print?`" will print when you return `Ok(())` and not print when you return an error.


Differences between Solana and Solidity with regards to errors
--------------------------------------------------------------

In Solidity, the require statement halts the execution with the revert op code. Solana does not halt execution but simply returns a different value. This is analogous to how linux returns 0 or 1 on success. If a 0 is returned (equivalent of returning `Ok(())`), everything went smoothly.

Therefore, Solana programs should always return something --- either an `Ok(())` or an `Error`.

In Anchor, errors are an enum with the `#[error_code]` attribute.

Note how all the functions in Solana have a return type of `Result<()>`. A [result](https://doc.rust-lang.org/std/result/) is a type that could either be an `Ok(())` or an error.