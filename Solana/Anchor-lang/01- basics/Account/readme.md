## [account] attribute




The [`#[account]`](https://github.com/coral-xyz/anchor/blob/0e5285aecdf410fa0779b7cd09a47f235882c156/lang/attribute/account/src/lib.rs#L97) attribute is applied to structs that define the structure of the data stored in custom accounts created by your program.

```rust

#[account]
pub struct NewAccount {
    data: u64,
}
```

This macro implements various traits [detailed here](https://docs.rs/anchor-lang/latest/anchor_lang/attr.account.html). The key functionalities of the `#[account]` macro include:

* [Assign Program Owner](https://github.com/coral-xyz/anchor/blob/0e5285aecdf410fa0779b7cd09a47f235882c156/lang/attribute/account/src/lib.rs#L130-L143): When creating an account, the program owner of the account is automatically set to the program specified in `declare_id`.
* [Set Discriminator](https://github.com/coral-xyz/anchor/blob/0e5285aecdf410fa0779b7cd09a47f235882c156/lang/attribute/account/src/lib.rs#L111-L128): A unique 8 byte discriminator, specific to the account type, is added as the first 8 bytes of account data during its initialization. This helps in differentiating account types and is used for account validation.
* [Data Serialization and Deserialization](https://github.com/coral-xyz/anchor/blob/0e5285aecdf410fa0779b7cd09a47f235882c156/lang/attribute/account/src/lib.rs#L224-L270): Account data is automatically serialized and deserialized as the account type.

lib.rs

```rust

use anchor_lang::prelude::*;
 
declare_id!("11111111111111111111111111111111");
 
#[program]
mod hello_anchor {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {

        ctx.accounts.new_account.data = data;
        msg!("Changed data to: {}!", data);
        Ok(())
    }
}
 
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 8)]
    pub new_account: Account<'info, NewAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
 


#[account]
pub struct NewAccount {
    data: u64,
}
```
