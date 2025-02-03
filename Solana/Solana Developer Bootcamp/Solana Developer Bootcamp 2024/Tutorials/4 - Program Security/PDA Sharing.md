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