## [derive(AccountS)] macro

The [`#[derive(Accounts)]`](https://github.com/coral-xyz/anchor/blob/0e5285aecdf410fa0779b7cd09a47f235882c156/lang/derive/accounts/src/lib.rs#L631) macro is applied to a struct to specify the accounts that must be provided when an instruction is invoked. This macro implements the [`Accounts`](https://github.com/coral-xyz/anchor/blob/0e5285aecdf410fa0779b7cd09a47f235882c156/lang/src/lib.rs#L108) trait, which simplifies account validation and serialization and deserialization of account data.

You can find the implementation of the code generated by the `#[derive(Accounts)]` macro [here](https://github.com/coral-xyz/anchor/tree/0e5285aecdf410fa0779b7cd09a47f235882c156/lang/syn/src/codegen/accounts).

```

#[derive(Accounts)]

pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 8)]
    pub new_account: Account<'info, NewAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

Each field in the struct represents an account required by an instruction. The naming of each field is arbitrary, but it is recommended to use a descriptive name that indicates the purpose of the account.

```
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 8)]

    pub new_account: Account<'info, NewAccount>,
    #[account(mut)]

    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
```