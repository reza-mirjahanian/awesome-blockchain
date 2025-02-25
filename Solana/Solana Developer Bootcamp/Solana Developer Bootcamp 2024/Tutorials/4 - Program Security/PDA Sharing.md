[Summary]
-------------------------------------------------------------------------------------

-   Using the same PDA for multiple authority domains opens your program up to the possibility of users accessing data and funds that don't belong to them
-   Prevent the same PDA from being used for multiple accounts by using seeds that are user and/or domain-specific
-   Use Anchor's `seeds` and `bump` constraints to validate that a PDA is derived using the expected seeds and bump

-------------


[Lesson]
-----------------------------------------------------------------------------------

PDA sharing refers to using the same PDA as a signer across multiple users or domains. Especially when using PDAs for signing, it may seem appropriate to use a global PDA to represent the program. However, this opens up the possibility of account validation passing but a user being able to access funds, transfers, or data not belonging to them.

------------
### [Insecure Global PDA]

In the example below, the `authority` of the `vault` account is a PDA derived using the `mint` address stored on the `pool` account. This PDA is passed into the instruction handler as the `authority` account to sign for the transfer of tokens from the `vault` to the `withdraw_destination`.

Using the `mint` address as a seed to derive the PDA to sign for the `vault` is insecure because multiple `pool` accounts could be created for the same `vault` token account, but with different `withdraw_destination` accounts. By using the `mint` as a `seed` to derive the PDA for signing token transfers, any `pool` account could sign for the transfer of tokens from a `vault` token account to an arbitrary `withdraw_destination`.

```Rust
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
 
declare_id!("ABQaKhtpYQUUgZ9m2sAY7ZHxWv6KyNdhUJW8Dh8NQbkf");
 
#[program]
pub mod pda_sharing_insecure {
    use super::*;
 
    pub fn withdraw_tokens(ctx: Context<WithdrawTokens>) -> Result<()> {
        let amount = ctx.accounts.vault.amount;
        let seeds = &[ctx.accounts.pool.mint.as_ref(), &[ctx.accounts.pool.bump]];
        token::transfer(get_transfer_ctx(&ctx.accounts).with_signer(&[seeds]), amount)
    }
}
 
#[derive(Accounts)]
pub struct WithdrawTokens<'info> {
    #[account(has_one = vault, has_one = withdraw_destination)]
    pool: Account<'info, TokenPool>,
    vault: Account<'info, TokenAccount>,
    withdraw_destination: Account<'info, TokenAccount>,
    /// CHECK: This is the PDA that signs for the transfer
    authority: UncheckedAccount<'info>,
    token_program: Program<'info, Token>,
}
 
pub fn get_transfer_ctx<'accounts, 'remaining, 'cpi_code, 'info>(
    accounts: &'accounts WithdrawTokens<'info>,
) -> CpiContext<'accounts, 'remaining, 'cpi_code, 'info, token::Transfer<'info>> {
    CpiContext::new(
        accounts.token_program.to_account_info(),
        token::Transfer {
            from: accounts.vault.to_account_info(),
            to: accounts.withdraw_destination.to_account_info(),
            authority: accounts.authority.to_account_info(),
        },
    )
}
 
#[account]
#[derive(InitSpace)]
pub struct TokenPool {
    pub vault: Pubkey,
    pub mint: Pubkey,
    pub withdraw_destination: Pubkey,
    pub bump: u8,
}
```


## Secure account specific PDA

One approach to create an account specific PDA is to use the `withdraw_destination` as a seed to derive the PDA used as the authority of the `vault` token account. This ensures the PDA signing for the CPI in the `withdraw_tokens` instruction handler is derived using the intended `withdraw_destination` token account. In other words, tokens from a `vault` token account can only be withdrawn to the `withdraw_destination` that was originally initialized with the `pool` account.

```Rust
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
 
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
 
#[program]
pub mod pda_sharing_secure {
    use super::*;
 
    pub fn withdraw_tokens(ctx: Context<WithdrawTokens>) -> Result<()> {
        let amount = ctx.accounts.vault.amount;
        let seeds = &[
            ctx.accounts.pool.withdraw_destination.as_ref(), //Diff is here!
            &[ctx.accounts.pool.bump],
        ];
        token::transfer(get_transfer_ctx(&ctx.accounts).with_signer(&[seeds]), amount)
    }
}
 
#[derive(Accounts)]
pub struct WithdrawTokens<'info> {
    #[account(has_one = vault, has_one = withdraw_destination)]
    pool: Account<'info, TokenPool>,
    vault: Account<'info, TokenAccount>,
    withdraw_destination: Account<'info, TokenAccount>,
    /// CHECK: This is the PDA that signs for the transfer
    authority: UncheckedAccount<'info>,
    token_program: Program<'info, Token>,
}
 
pub fn get_transfer_ctx<'accounts, 'remaining, 'cpi_code, 'info>(
    accounts: &'accounts WithdrawTokens<'info>,
) -> CpiContext<'accounts, 'remaining, 'cpi_code, 'info, token::Transfer<'info>> {
    CpiContext::new(
        accounts.token_program.to_account_info(),
        token::Transfer {
            from: accounts.vault.to_account_info(),
            to: accounts.withdraw_destination.to_account_info(),
            authority: accounts.authority.to_account_info(),
        },
    )
}
 
#[account]
#[derive(InitSpace)]
pub struct TokenPool {
    pub vault: Pubkey,
    pub mint: Pubkey,
    pub withdraw_destination: Pubkey,
    pub bump: u8,
}
```
-----------------

### [Anchor's seeds and bump Constraints]

PDAs can be used as both the address of an account and allow programs to sign for the PDAs they own.

The example below uses a PDA derived using the `withdraw_destination` as both the address of the `pool` account and the owner of the `vault` token account. This means that only the `pool` account associated with the correct `vault` and `withdraw_destination` can be used in the `withdraw_tokens` instruction handler.

You can use Anchor's `seeds` and `bump` constraints with the [`#[account(...)]`](https://www.anchor-lang.com/docs/account-constraints) attribute to validate the `pool` account PDA. Anchor derives a PDA using the `seeds` and `bump` specified and compares it against the account passed into the instruction handler as the `pool` account. The `has_one` constraint is used to further ensure that only the correct accounts stored on the `pool` account are passed into the instruction handler.

```Rust
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
 
declare_id!("ABQaKhtpYQUUgZ9m2sAY7ZHxWv6KyNdhUJW8Dh8NQbkf");
 
#[program]
pub mod pda_sharing_recommended {
    use super::*;
 
    pub fn withdraw_tokens(ctx: Context<WithdrawTokens>) -> Result<()> {
        let amount = ctx.accounts.vault.amount;
        let seeds = &[
            ctx.accounts.pool.withdraw_destination.as_ref(),
            &[ctx.accounts.pool.bump],
        ];
        token::transfer(get_transfer_ctx(&ctx.accounts).with_signer(&[seeds]), amount)
    }
}
 
#[derive(Accounts)]
pub struct WithdrawTokens<'info> {
    #[account(
        seeds = [withdraw_destination.key().as_ref()],
        bump = pool.bump,
        has_one = vault,
        has_one = withdraw_destination,
    )]
    pool: Account<'info, TokenPool>,
    #[account(mut)]
    vault: Account<'info, TokenAccount>,
    #[account(mut)]
    withdraw_destination: Account<'info, TokenAccount>,
    token_program: Program<'info, Token>,
}
 
pub fn get_transfer_ctx<'accounts, 'remaining, 'cpi_code, 'info>(
    accounts: &'accounts WithdrawTokens<'info>,
) -> CpiContext<'accounts, 'remaining, 'cpi_code, 'info, token::Transfer<'info>> {
    CpiContext::new(
        accounts.token_program.to_account_info(),
        token::Transfer {
            from: accounts.vault.to_account_info(),
            to: accounts.withdraw_destination.to_account_info(),
            authority: accounts.pool.to_account_info(),
        },
    )
}
 
#[account]
#[derive(InitSpace)]
pub struct TokenPool {
    pub vault: Pubkey,
    pub mint: Pubkey,
    pub withdraw_destination: Pubkey,
    pub bump: u8,
}
```