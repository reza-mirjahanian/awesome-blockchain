The `Error` type in Anchor programs can be one of two variants:

1.  [`ProgramErrorWithOrigin`](https://github.com/coral-xyz/anchor/blob/0e5285aecdf410fa0779b7cd09a47f235882c156/lang/src/error.rs#L389-L394): Custom type that wraps a standard Solana [`ProgramError`](https://github.com/anza-xyz/agave/blob/v1.18.26/sdk/program/src/program_error.rs#L12-L66) type. These errors come from the `solana_program` crate.

```rust
#[derive(Debug)]
pub struct ProgramErrorWithOrigin {
    pub program_error: ProgramError,
    pub error_origin: Option<ErrorOrigin>,
    pub compared_values: Option<ComparedValues>,
}
```

2.  [`AnchorError`](https://github.com/coral-xyz/anchor/blob/0e5285aecdf410fa0779b7cd09a47f235882c156/lang/src/error.rs#L490-L497): Errors defined by the Anchor framework.

```rust
#[derive(Debug)]
pub struct AnchorError {
    pub error_name: String,
    pub error_code_number: u32,
    pub error_msg: String,
    pub error_origin: Option<ErrorOrigin>,
    pub compared_values: Option<ComparedValues>,
}
```

An `AnchorError` can be thought of as having two categories:

1.  Internal Anchor Errors - These are built-in errors included with the Anchor framework. They are defined in the [`ErrorCode`](https://github.com/coral-xyz/anchor/blob/0e5285aecdf410fa0779b7cd09a47f235882c156/lang/src/error.rs#L10-L275) enum.

2.  Custom Program Errors - These are program specific errors that developers define to handle custom error cases.

The `error_code_number` from an `AnchorError` has the following numbering scheme:

| Error Code | Description |
| --- |  --- |
| \>= 100 | Instruction error codes |
| \>= 1000 | IDL error codes |
| \>= 2000 | Constraint error codes |
| \>= 3000 | Account error codes |
| \>= 4100 | Misc error codes |
| \= 5000 | Deprecated error code |
| \>= 6000 | Starting point for custom user errors |

[Usage](https://www.anchor-lang.com/docs/features/errors#usage)
---------------------------------------------------------------


Anchor provides a convenient way to define custom errors through the `error_code` attribute. The implementation details can be found [here](https://github.com/coral-xyz/anchor/blob/master/lang/syn/src/codegen/error.rs).

When you define an enum with the `error_code` attribute, Anchor automatically:

-   Assigns an error code starting from 6000
-   Generates the necessary boilerplate for error handling
-   Enables the use of custom error messages via the `msg` attribute

```
#[error_code]
pub enum MyError {
    #[msg("My custom error message")]
    MyCustomError,
    #[msg("My second custom error message")]
    MySecondCustomError,
}
```

### [err!](https://www.anchor-lang.com/docs/features/errors#err)

To throw an error, use the [`err!`](https://github.com/coral-xyz/anchor/blob/0e5285aecdf410fa0779b7cd09a47f235882c156/lang/src/lib.rs#L735-L743) macro. The `err!` macro provides a convenient way to return custom errors from your program. Under the hood, `err!` uses the `error!` macro to construct `AnchorError`. The implementation can be found [here](https://github.com/coral-xyz/anchor/blob/0e5285aecdf410fa0779b7cd09a47f235882c156/lang/attribute/error/src/lib.rs#L84-L116).

```rust
#[program]
mod hello_anchor {
    use super::*;
    pub fn set_data(ctx: Context<SetData>, data: MyAccount) - Result<()> {
        if data.data = 100 {


            return err!(MyError::DataTooLarge);
        }
        ctx.accounts.my_account.set_inner(data);
        Ok(())
    }
}
 
 
#[error_code]
pub enum MyError {
    #[msg("MyAccount may only hold data below 100")]
    DataTooLarge
}
```

### [require!](https://www.anchor-lang.com/docs/features/errors#require)

The [`require!`](https://github.com/coral-xyz/anchor/blob/0e5285aecdf410fa0779b7cd09a47f235882c156/lang/src/lib.rs#L525-L537) macro provides a more concise way to handle error conditions. It combines a condition check with returning an error if the condition is false. Here's how we can rewrite the previous example using `require!`:

```rust
use anchor_lang::prelude::*;
 
declare_id!("9oECKMeeyf1fWNPKzyrB2x1AbLjHDFjs139kEyFwBpoV");
 
#[program]
pub mod custom_error {
    use super::*;
 
    pub fn validate_amount(_ctx: Context<ValidateAmount>, amount: u64) - Result<()> {


        require!(amount >= 10, CustomError::AmountTooSmall);
        require!(amount <= 100, CustomError::AmountTooLarge);
 
        msg!("Amount validated successfully: {}", amount);
        Ok(())
    }
}
 
#[derive(Accounts)]
pub struct ValidateAmount {}
 
#[error_code]
pub enum CustomError {
    #[msg("Amount must be greater than or equal to 10")]
    AmountTooSmall,
    #[msg("Amount must be less than or equal to 100")]
    AmountTooLarge,
}
```

Anchor provides several "require" macros for different validation needs. You can find the implementation of these macros [here](https://github.com/coral-xyz/anchor/blob/0e5285aecdf410fa0779b7cd09a47f235882c156/lang/src/lib.rs).

| Macro | Description |
| --- |  --- |
| `require!` | Ensures a condition is true, otherwise returns with the given error. |
| --- |  --- |
| `require_eq!` | Ensures two NON-PUBKEY values are equal. |
| `require_neq!` | Ensures two NON-PUBKEY values are not equal. |
| `require_keys_eq!` | Ensures two pubkeys values are equal. |
| `require_keys_neq!` | Ensures two pubkeys are not equal. |
| `require_gt!` | Ensures the first NON-PUBKEY value is greater than the second NON-PUBKEY value. |
| `require_gte!` | Ensures the first NON-PUBKEY value is greater than or equal to the second NON-PUBKEY value. |

