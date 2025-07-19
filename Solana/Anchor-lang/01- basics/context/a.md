

---

# 📌 Instruction Context in Anchor (Solana)

---

## 🔹 Overview

In Anchor, the **instruction context** refers to the **Rust struct** that defines:

* The **accounts** involved in a transaction/instruction.
* The **constraints** and checks to validate them.
* How Anchor deserializes and enforces access control.

These context structs are annotated with `#[derive(Accounts)]`.

---

## 🔹 Syntax: Basic Pattern

```rust
#[derive(Accounts)]
pub struct MyInstructionContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    
    #[account(mut, has_one = signer)]
    pub data_account: Account<'info, MyData>,
    
    pub system_program: Program<'info, System>,
}
```

---

## 🔹 Account Attributes & Constraints

| Attribute                | Description                             | Example                                                      |
| ------------------------ | --------------------------------------- | ------------------------------------------------------------ |
| `mut`                    | Marks account as writable               | `#[account(mut)]`                                            |
| `signer`                 | Requires account to sign tx             | `#[account(signer)]`                                         |
| `has_one = xyz`          | Enforces foreign key to another account | `#[account(has_one = signer)]`                               |
| `seeds` & `bump`         | PDA derivation constraints              | `#[account(seeds = [b"seed", signer.key().as_ref()], bump)]` |
| `constraint`             | Custom logic validation                 | `#[account(constraint = my_func(&account)?)`                 |
| `close = recipient`      | Close account & send rent               | `#[account(close = signer)]`                                 |
| `init`, `init_if_needed` | Initialize account (optionally)         | `#[account(init, payer = signer, space = 8 + size)]`         |

---

## 🔹 Common Context Types

| Type                      | Purpose                                      |
| ------------------------- | -------------------------------------------- |
| `Account<'info, T>`       | Mutable, deserialized state (PDA or owned)   |
| `AccountInfo<'info>`      | Raw Solana account info                      |
| `Signer<'info>`           | Enforces signature                           |
| `Program<'info, T>`       | For referencing other programs (like System) |
| `UncheckedAccount<'info>` | Bypasses checks – use with caution           |
| `Sysvar<'info, T>`        | Clock, Rent, RecentBlockhashes, etc.         |

---

## 🔹 Account Composition Patterns

### ✅ Basic Account Read

```rust
#[derive(Accounts)]
pub struct ReadData<'info> {
    pub user: Account<'info, MyData>,
}
```

### ✅ Account Write + Sign

```rust
#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(mut, has_one = user)]
    pub data: Account<'info, MyData>,
}
```

### ✅ PDA Creation with Init

```rust
#[derive(Accounts)]
pub struct CreatePDA<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    
    #[account(
        init,
        payer = payer,
        space = 8 + MyData::LEN,
        seeds = [b"mydata", payer.key().as_ref()],
        bump
    )]
    pub my_pda: Account<'info, MyData>,
    
    pub system_program: Program<'info, System>,
}
```

---

## 🔹 Edge Case: Multiple Constraints

```rust
#[account(
    mut,
    seeds = [b"profile", authority.key().as_ref()],
    bump = profile.bump,
    has_one = authority,
    constraint = profile.status == Status::Active @ CustomError::NotActive,
)]
pub profile: Account<'info, Profile>;
```

✅ Combines:

* Seed derivation
* Authority check
* Status constraint
* Custom error handling

---

## 🔹 Edge Case: Optional Accounts

```rust
#[derive(Accounts)]
pub struct OptionalAccountExample<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    
    /// Optional account - only passed in under certain conditions
    pub maybe_account: Option<Account<'info, MyData>>,
}
```

🔸 Use pattern matching inside instruction:

```rust
if let Some(acc) = ctx.accounts.maybe_account.as_ref() {
    // use acc
}
```

---

## 🔹 Accessing Bump Seeds

```rust
let bump = *ctx.bumps.get("my_pda").unwrap();
```

```rust
ctx.accounts.my_pda.bump = bump;
```

---

## 🔹 Custom Deserialization & Manual Checks

For advanced cases like CPI or unverified PDAs:

```rust
#[derive(Accounts)]
pub struct RawAccess<'info> {
    #[account(mut)]
    pub data: AccountInfo<'info>,
}
```

Then:

```rust
let data = MyData::try_from_slice(&ctx.accounts.data.data.borrow())?;
```

---

## 🔹 Comparisons: Anchor vs Raw Solana

| Feature         | Anchor Context                | Raw Solana                         |
| --------------- | ----------------------------- | ---------------------------------- |
| Deserialization | Auto-derived via macros       | Manual via `try_from_slice`        |
| Error messages  | More readable                 | Often cryptic                      |
| Account init    | `#[account(init)]` simplifies | Manual rent exemption & allocation |
| Constraints     | Declarative `#[account(...)]` | Manual `require!` & asserts        |

---

## 🔹 Tricky Parts & Gotchas

* **Missing signer flag**: Leads to runtime error.
* **Wrong PDA seeds**: Causes `ConstraintSeeds` error.
* **Wrong space**: Causes instruction failure.
* **Close fails**: if account not empty or recipient is invalid.
* **Using `AccountInfo` instead of `Account`**: Loses type guarantees.

---

## 🔹 Real-World Use Case: Token Vesting Contract

```rust
#[derive(Accounts)]
pub struct ClaimVesting<'info> {
    #[account(mut)]
    pub claimer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vesting", claimer.key().as_ref()],
        bump,
        has_one = claimer,
        constraint = vesting.schedule.claimable(now_ts) > 0 @ ErrorCode::NothingToClaim
    )]
    pub vesting: Account<'info, Vesting>,

    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub destination: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
```

---

## 🔹 Full Example: Minting a Token (Context + Handler)

```rust
#[derive(Accounts)]
pub struct MintToken<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut, has_one = authority)]
    pub mint: Account<'info, MintData>,

    #[account(mut)]
    pub destination: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> Result<()> {
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.destination.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        },
    );

    token::mint_to(cpi_ctx, amount)?;
    Ok(())
}
```

---

## 🔹 Best Practices

* Use **constraint macros** instead of `require!` for static checks.
* Prefer **`Account<T>`** over `AccountInfo` unless needed.
* Name context structs **after the instruction** for clarity.
* Group **related fields** together (`payer`, `programs`, etc.).
* Use **PDAs with seeds + bump** to avoid collision & loss.
* Mark only necessary accounts as `mut` or `signer`.

---

## 🔹 Related Concepts

| Concept                            | Description                                      |
| ---------------------------------- | ------------------------------------------------ |
| **CPI (Cross-Program Invocation)** | Uses context to invoke another program           |
| **Bumps**                          | Auto-generated seeds for PDAs                    |
| **Sysvars**                        | Pass `Clock`, `Rent`, etc., via context          |
| **#\[derive(Accounts)]**           | Expands into deserialization and validation code |
| **ProgramTest**                    | Can use context patterns for test inputs         |

---

## 🔹 Table: Common Anchor Constraints

| Constraint      | Purpose                         |
| --------------- | ------------------------------- |
| `mut`           | Writable account                |
| `signer`        | Must sign transaction           |
| `has_one`       | Cross-account authority         |
| `seeds`, `bump` | PDA derivation                  |
| `constraint`    | Custom Rust validation          |
| `init`          | Create new account              |
| `close`         | Close account & return lamports |

---

## 🔹 Next Steps: Advanced Topic

### 🔥 **Next Logical Topic: Cross-Program Invocations (CPI) with Anchor**

* Learn how to invoke other Anchor/Raw programs using **CPI context structs**, **CpiContext**, and **signer seeds**.
* Handle **token transfers**, **MintTo**, **Delegate**, **close\_account**, etc.
* Understand **`invoke_signed`** vs **`invoke`**, and how Anchor simplifies it.

Let me know if you want a full deep dive on **CPI in Anchor** next.
