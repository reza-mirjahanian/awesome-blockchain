### [Instruction Context](#instruction-context)

Instruction handlers are functions that define the logic executed when an instruction is invoked. The first parameter of each handler is a `Context<T>` type, where `T` is a struct implementing the [`Accounts`](https://github.com/coral-xyz/anchor/blob/0e5285aecdf410fa0779b7cd09a47f235882c156/lang/src/lib.rs#L108) trait and specifies the accounts the instruction requires.

The [`Context`](https://github.com/coral-xyz/anchor/blob/0e5285aecdf410fa0779b7cd09a47f235882c156/lang/src/context.rs#L24) type provides the instruction with access to the following non-argument inputs:

```rust
pub struct Context<'a, 'b, 'c, 'info, T: Bumps> {
    /// Currently executing program id.
    pub program_id: &'a Pubkey,
    /// Deserialized accounts.
    pub accounts: &'b mut T,
    /// Remaining accounts given but not deserialized or validated.
    /// Be very careful when using this directly.
    pub remaining_accounts: &'c [AccountInfo<'info>],
    /// Bump seeds found during constraint validation. This is provided as a
    /// convenience so that handlers don't have to recalculate bump seeds or
    /// pass them in as arguments.
    /// Type is the bumps struct generated by #[derive(Accounts)]
    pub bumps: T::Bumps,
}
```

The `Context` fields can be accessed in an instruction using dot notation:

* `ctx.accounts`: The accounts required for the instruction
* `ctx.program_id`: The program's public key (address)
* `ctx.remaining_accounts`: Additional accounts not specified in the `Accounts` struct.
* `ctx.bumps`: Bump seeds for any Program Derived Address (PDA) accounts specified in the `Accounts` struct

Additional parameters are optional and can be included to specify arguments that must be provided when the instruction is invoked.

lib.rs

```rust


pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
    ctx.accounts.new_account.data = data;
    msg!("Changed data to: {}!", data);
    Ok(())
}
```

In this example, the `Initialize` struct implements the `Accounts` trait where each field in the struct represents an account required by the `initialize` instruction.

lib.rs

```rust


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
```

