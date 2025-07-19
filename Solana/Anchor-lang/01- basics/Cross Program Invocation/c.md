Cross-Program Invocations (CPIs) on Solana  
==========================================

What a CPI is  
A **Cross-Program Invocation** (CPI) is simply one on-chain program **calling an instruction of another program**.  
Think of it as an internal “API call” that lets you compose functionality instead of re-implementing it.

Why CPIs matter  
- **Composability** – Combine existing programs (Token, System, DEX, Oracle, etc.) to build richer products.  
- **Security & Re-use** – Rely on battle-tested code instead of writing token transfers, swaps, or NFT burns yourself.  
- **Compute efficiency** – Solana’s runtime is optimized for CPIs; chaining programs is cheaper than monolithic logic.

---

### 1. Anatomy of a CPI

Every CPI needs four things:

| Component         | Description                                                                 |
|-------------------|------------------------------------------------------------------------------|
| **Program ID**    | Address of the program you want to invoke (e.g., `spl_token::id()`).        |
| **Accounts**      | List of `AccountMeta` that the callee instruction requires.                 |
| **Instruction data** | Borsh-serialized arguments for the callee.                                  |
| **Signer seeds**  | Optional: seeds + bump if the caller must sign via a PDA (`invoke_signed`). |

---

### 2. Raw vs. Anchor-style CPIs

#### A. Raw Solana (native)

Use `invoke` or `invoke_signed`:

```rust
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
};

pub fn raw_transfer(
    from: &AccountInfo,
    to: &AccountInfo,
    authority: &AccountInfo,
    amount: u64,
) -> ProgramResult {
    let ix = Instruction::new_with_borsh(
        spl_token::id(),
        &spl_token::instruction::TokenInstruction::Transfer { amount },
        vec![
            AccountMeta::new(*from.key, false),
            AccountMeta::new(*to.key, false),
            AccountMeta::new_readonly(*authority.key, true),
        ],
    );
    invoke(&ix, &[from.clone(), to.clone(), authority.clone()])
}
```

- `invoke_signed` is identical but adds a `signers_seeds: &[&[&[u8]]]` parameter when the caller signs via a PDA.

#### B. Anchor ergonomic CPIs

Anchor auto-generates a `cpi` module for every program in your `Cargo.toml`.  
Example: transferring tokens with the SPL Token program.

```rust
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};

#[program]
pub mod my_program {
    use super::*;
    pub fn proxy_transfer(ctx: Context<ProxyTransfer>, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: ctx.accounts.from.to_account_info(),
            to:   ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)
    }
}

#[derive(Accounts)]
pub struct ProxyTransfer<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}
```

- **Type safety** – Anchor checks discriminators and account constraints.  
- **Less boilerplate** – no need to hand-craft `Instruction` objects.

---

### 3. Signing via PDA (CPI with seeds)

When a PDA must act as the signer:

```rust
let vault_key = ctx.accounts.vault.key();
let authority_seeds = &[b"vault", vault_key.as_ref(), &[vault_bump]];
let signer_seeds = &[&authority_seeds[..]];

let cpi_ctx = CpiContext::new_with_signer(
    ctx.accounts.token_program.to_account_info(),
    Transfer {
        from: ctx.accounts.vault_token_account.to_account_info(),
        to:   ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.vault.to_account_info(),
    },
    signer_seeds,
);
token::transfer(cpi_ctx, amount)?;
```

- Anchor computes the bump automatically if you declared `seeds` and `bump` in the account context.

---

### 4. Calling another Anchor program

If both caller and callee are Anchor programs:

```rust
// In Cargo.toml:
// callee = { path = "../callee", features = ["cpi"] }

let cpi_accounts = callee::cpi::accounts::MyInstruction {
    user: ctx.accounts.user.to_account_info(),
    vault: ctx.accounts.vault.to_account_info(),
    system_program: ctx.accounts.system_program.to_account_info(),
};
let cpi_ctx = CpiContext::new(
    ctx.accounts.callee_program.to_account_info(),
    cpi_accounts,
);
callee::cpi::my_instruction(cpi_ctx, param1, param2)?;
```

- Enabling the `cpi` feature exposes the `callee::cpi` module.

---

### 5. Security & best-practice checklist

| ✅ Do                                                                              | ❌ Don’t                                                    |
|------------------------------------------------------------------------------------|-------------------------------------------------------------|
| Validate program IDs and account ownership inside constraints               | Trust external account data without checks            |
| Use `CpiContext::new_with_signer` with correct seeds when PDAs sign               | Hard-code program-id strings inline                         |
| Handle CPI errors with `?` so they bubble up                                      | Ignore compute budget limits; batch CPIs when possible      |
| Use events to log CPI calls for debugging                                         | Make assumptions about external account state               |

---

### 6. One-liner quick start

```rust
// Transfer SOL from a PDA vault
let seeds = &[b"vault", &[bump]];
let signer = &[&seeds[..]];
let cpi_ctx = CpiContext::new_with_signer(
    ctx.accounts.system_program.to_account_info(),
    system_program::Transfer {
        from: ctx.accounts.vault.to_account_info(),
        to:   ctx.accounts.user.to_account_info(),
    },
    signer,
);
system_program::transfer(cpi_ctx, lamports)?;
```

---

Further reading  
- Step-by-step tutorial (official): https://solana.com/docs/intro/quick-start/cross-program-invocation   
- Anchor CPI docs: https://www.anchor-lang.com/docs/basics/cpi 