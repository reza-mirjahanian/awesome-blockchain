  *  You can find a full list of the constraints [here](https://github.com/coral-xyz/anchor/blob/0e5285aecdf410fa0779b7cd09a47f235882c156/lang/syn/src/parser/accounts/constraints.rs) and implementation [here](https://github.com/coral-xyz/anchor/blob/0e5285aecdf410fa0779b7cd09a47f235882c156/lang/syn/src/codegen/accounts/constraints.rs).

  * You can find the implementation of the account types [here](https://github.com/coral-xyz/anchor/tree/0e5285aecdf410fa0779b7cd09a47f235882c156/lang/src/accounts).

Accounts are validated in Anchor programs in two ways that are generally used together:

* [Account Constraints](/docs/references/account-constraints): Constraints define additional conditions that an account must satisfy to be considered valid for the instruction. Constraints are applied using the `#[account(..)]` attribute, which is placed above a field in a struct that implements the `Accounts` trait.



  ```rust
  #[derive(Accounts)]
  pub struct Initialize<'info> {

      #[account(init, payer = signer, space = 8 + 8)]
      pub new_account: Account<'info, NewAccount>,

      #[account(mut)]
      pub signer: Signer<'info>,
      pub system_program: Program<'info, System>,
  }
  ```

* [Account Types](/docs/references/account-types): Anchor provides various account types to help ensure that the account provided by the client matches what the program expects.



  ```rust
  #[derive(Accounts)]
  pub struct Initialize<'info> {
      #[account(init, payer = signer, space = 8 + 8)]

      pub new_account: Account<'info, NewAccount>,
      #[account(mut)]

      pub signer: Signer<'info>,

      pub system_program: Program<'info, System>,
  }
  ```

When an instruction in an Anchor program is invoked, the program first validates the accounts provided before executing the instruction's logic. After validation, these accounts can be accessed within the instruction using the `ctx.accounts` syntax.

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

