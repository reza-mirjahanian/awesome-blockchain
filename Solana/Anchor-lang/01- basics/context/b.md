

-----

## Core Concepts of `Context`

The `Context` object is a generic struct, `Context<'info, T>`, where `T` is a user-defined struct that implements the `Accounts` trait. This `Accounts` struct defines the specific accounts an instruction expects.

The `#[derive(Accounts)]` macro is the magic behind the scenes. It generates code to:

1.  **Deserialize** incoming account data from the transaction's `account_infos` slice.
2.  **Validate** accounts against the constraints you define (e.g., checking ownership, signers, mutability, PDAs).
3.  **Organize** the validated accounts into a clean, type-safe struct.

If any validation fails, the transaction is immediately rejected, preventing the instruction logic from ever running with invalid accounts. This makes Anchor programs significantly more secure and easier to reason about.

```rust
// lib.rs
use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod hello_anchor {
    use super::*;
    // The first argument to every instruction is the Context.
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Access accounts via ctx.accounts
        ctx.accounts.my_account.data = 0;
        msg!("Initialized new account with data: 0");
        Ok(())
    }
}

// The 'T' in Context<T> is this struct.
#[derive(Accounts)]
pub struct Initialize<'info> {
    // This defines the accounts the 'initialize' instruction needs.
    #[account(init, payer = user, space = 8 + 8)]
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct MyAccount {
    pub data: u64,
}
```

### Accessing Data from `ctx`

The `Context` object provides three primary fields:

  * `ctx.accounts`: A struct containing all the deserialized and validated accounts defined in your `#[derive(Accounts)]` struct. This is what you'll use 99% of the time.
  * `ctx.program_id`: A `&Pubkey` of the currently executing program. Useful for PDAs and CPI.
  * `ctx.remaining_accounts`: A `&[AccountInfo]` slice containing any accounts passed to the instruction that were *not* defined in the `Accounts` struct. This is for advanced use cases like dynamic instruction composition.

<!-- end list -->

```rust
pub fn do_something(ctx: Context<DoSomething>) -> Result<()> {
    // 1. Accessing defined accounts
    let my_data = &mut ctx.accounts.my_account;
    my_data.data += 1;

    // 2. Accessing the program ID
    msg!("Executing program: {}", ctx.program_id);

    // 3. Accessing remaining accounts (advanced)
    for acc_info in ctx.remaining_accounts {
        msg!("Found a remaining account: {}", acc_info.key());
    }

    Ok(())
}

#[derive(Accounts)]
pub struct DoSomething<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
}
```

-----

## Account Types in Context

When defining your `Accounts` struct, you use specific wrapper types around `AccountInfo` to tell Anchor what to expect and what checks to perform.

| Type | Purpose | Common Constraints |
| :--- | :--- | :--- |
| `Account<'info, T>` | An account owned by the current program, whose data is deserialized into type `T`. | `init`, `mut`, `seeds`, `has_one`, `close` |
| `AccountLoader<'info, T>` | For large accounts that should not be fully deserialized on every access. It uses zero-copy deserialization. | `init`, `mut`, `seeds` |
| `Signer<'info>` | An account that must have signed the transaction. It's a wrapper around `AccountInfo`. | `mut` |
| `UncheckedAccount<'info>` | A raw `AccountInfo`. **DANGEROUS**. Anchor performs no checks (ownership, signer, etc.). Use only when absolutely necessary. | `mut` |
| `Program<'info, T>` | An executable account representing another on-chain program (e.g., SPL Token Program). | `executable` (checked by default) |
| `SystemAccount<'info>` | An `AccountInfo` specifically for an account that should be owned by the System Program. It's syntactic sugar. | `mut` |
| `Sysvar<'info, T>` | Provides access to Solana's `Sysvar` accounts like `Clock`, `Rent`, `SlotHashes`. | None |

### Code Examples for Account Types

```rust
#[derive(Accounts)]
pub struct ComprehensiveExample<'info> {
    // 1. Program-owned account, deserialized.
    #[account(mut, has_one = authority)]
    pub state: Account<'info, State>,

    // 2. A Signer account.
    pub authority: Signer<'info>,

    // 3. The SPL Token program for CPI.
    pub token_program: Program<'info, Token>,
    
    // 4. A Sysvar.
    pub clock: Sysvar<'info, Clock>,
    
    // 5. Raw account info, no checks performed. (Use with caution!)
    /// CHECK: This account is used for a legacy integration and is manually verified.
    pub legacy_account: UncheckedAccount<'info>,

    // 6. Large account using zero-copy.
    #[account(mut)]
    pub large_account: AccountLoader<'info, LargeState>,
}

#[account]
pub struct State {
    // ... fields
    pub authority: Pubkey,
}

#[account(zero_copy)]
pub struct LargeState {
    // ... lots of fields
}
```

**Accessing `AccountLoader` data:**

```rust
// In your instruction
let large_account = &mut ctx.accounts.large_account.load_mut()?;
large_account.some_field = 100;
// The changes are written back when `large_account` is dropped (goes out of scope).
```

-----

## Account Constraints (`#[account(...)]`)

Constraints are the core of Anchor's security model. They are applied within the `#[account(...)]` attribute.

### Key Constraints

| Constraint | Description |
| :--- | :--- |
| `init` | Creates and initializes the account. Requires `payer` and `space`. The account must not exist. |
| `mut` | Marks the account as mutable. The transaction will fail if the account is not passed as writable. |
| `seeds = [...]` | Enforces that the account is a Program Derived Address (PDA) derived from the given seeds. Anchor verifies the address. |
| `bump = ...` | Provides the canonical bump seed for PDA validation, often read from another account. |
| `has_one = <field> @ ErrorCode` | Checks that `account.key() == other_account.<field>`. Used for ownership/relationship checks. |
| `constraint = <expression> @ ErrorCode` | A catch-all for custom validation logic. The expression must evaluate to `true`. |
| `realloc = ...` | Reallocates the space for an account. Requires `payer` and `realloc::zero = <bool>`. |
| `close = <destination>` | Marks an account to be closed, sending its lamports to the specified destination account. |
| `token::mint = ...` | SPL Token: asserts the mint of a token account. |
| `token::authority = ...` | SPL Token: asserts the authority of a token account. |

### Constraint Example

This example demonstrates creating a user profile PDA tied to the user's authority key.

```rust
#[program]
pub mod user_profile {
    use super::*;
    pub fn create_profile(ctx: Context<CreateProfile>, username: String) -> Result<()> {
        ctx.accounts.profile.authority = ctx.accounts.authority.key();
        ctx.accounts.profile.username = username;
        ctx.accounts.profile.bump = ctx.bumps.profile;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateProfile<'info> {
    // Initialize a new PDA account.
    // 'seeds' defines how the PDA is derived.
    // 'bump' is the canonical bump seed found by Anchor.
    // 'payer' is who pays for the rent.
    // 'space' is the required bytes for the account.
    #[account(
        init,
        seeds = [b"profile", authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + 32 + 4 + 50 + 1 // Discriminator + pubkey + string_prefix + 50 chars + bump
    )]
    pub profile: Account<'info, UserProfile>,
    
    // The authority must be mutable to pay for the account creation.
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UserProfile {
    pub authority: Pubkey,
    pub username: String,
    pub bump: u8,
}
```

-----

## CPI with `Context`

Cross-Program Invocations (CPIs) are essential for interacting with other programs. Anchor simplifies this by allowing you to build a `CpiContext` directly from your instruction's `Context`.

The pattern is `CpiContext::new(program, accounts)`.

```rust
// Example: Transferring SPL tokens using a CPI
use anchor_spl::token::{self, Token, Transfer};

pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
    // 1. Define the CPI accounts struct
    let cpi_accounts = Transfer {
        from: ctx.accounts.from.to_account_info(),
        to: ctx.accounts.to.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };

    // 2. Get the target program
    let cpi_program = ctx.accounts.token_program.to_account_info();

    // 3. Create the CpiContext
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    // 4. Call the token program's transfer instruction
    token::transfer(cpi_ctx, amount)?;

    Ok(())
}

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut)]
    pub from: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, token::TokenAccount>,
    pub authority: Signer<'info>, // The authority of the 'from' account
    pub token_program: Program<'info, Token>,
}
```

### CPI with Signer Seeds (for PDAs)

When a PDA needs to "sign" a CPI call, you use `CpiContext::new_with_signer`. The signer seeds must match the seeds used to derive the PDA.

```rust
// PDA signing a token transfer
pub fn pda_transfer(ctx: Context<PdaTransfer>, amount: u64) -> Result<()> {
    // The seeds for our program's PDA
    let seeds = &[
        b"vault",
        ctx.accounts.authority.key().as_ref(),
        &[ctx.accounts.vault.bump],
    ];
    let signer_seeds = &[&seeds[..]];

    let cpi_accounts = Transfer {
        from: ctx.accounts.vault.to_account_info(), // The PDA is the 'from' account
        to: ctx.accounts.destination.to_account_info(),
        authority: ctx.accounts.vault.to_account_info(), // PDA is also the authority
    };
    
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer_seeds, // Provide the seeds here
    );

    token::transfer(cpi_ctx, amount)?;
    Ok(())
}

#[derive(Accounts)]
pub struct PdaTransfer<'info> {
    // PDA vault holding tokens
    #[account(
        mut,
        seeds = [b"vault", authority.key().as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info, Vault>, // Assume Vault stores its bump
    
    #[account(mut)]
    pub destination: Account<'info, token::TokenAccount>,
    
    pub authority: Signer<'info>, // The user initiating the transfer
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Vault {
    pub bump: u8,
    // other fields...
}
```

-----

## Tricky Parts and Common Pitfalls

  * **Forgetting `mut`**: If you need to change data in an account (e.g., `my_account.data += 1`), you **must** add `#[account(mut)]`. Forgetting this is the most common error, resulting in a read-only transaction error.
  * **PDA Seed Mismatches**: The seeds in `#[account(seeds = ...)]` must perfectly match the seeds used on the client-side to generate the PDA address. Any difference will cause an error.
  * **`UncheckedAccount` Security**: Using `/// CHECK:` is a promise that you are manually verifying the account. If you don't, you can introduce critical security vulnerabilities, like allowing a user to pass in a fake mint account.
  * **`init` vs. `mut`**: An instruction can either initialize an account (`init`) or mutate an existing one (`mut`), but not both on the same account. You need separate instructions for creation and updates.
  * **Closing Accounts**: When using `#[account(close = destination)]`, ensure the `destination` account is mutable, as it will be receiving the lamports from the closed account.

-----

## Next Steps: Zero-Copy Deserialization with `AccountLoader`

For a deeper understanding of high-performance account handling, the logical next step is to master **zero-copy deserialization**. While `Account<T>` is convenient, it deserializes the entire account on every access, which can be slow and consume a lot of compute units for large accounts.

`AccountLoader` solves this by mapping the account's raw byte buffer directly into a struct without any intermediate copying. This is crucial for programs that handle large state or need to optimize every bit of performance. You'll learn how to work with Rust's lifetime mechanics and smart pointers (`Ref`, `RefMut`) to safely access and modify parts of a large account without loading the whole thing.