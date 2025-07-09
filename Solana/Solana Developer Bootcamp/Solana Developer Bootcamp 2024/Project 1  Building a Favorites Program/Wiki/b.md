# 🧠 Solana Smart Contract with Anchor (via Solana Playground)

## 🛠 Project Setup

1. **Go to** `beta.solpg.io`.
2. **Create a new project**:

   * Name it: `favorites`
   * Choose template: `Anchor`
3. **Edit `lib.rs`**:

   * Delete all existing code.
   * Start fresh from scratch.

---

## 📦 Anchor Prelude Import

```rust
use anchor_lang::prelude::*;
```

> This brings all commonly used Anchor items into scope.

---

## 📛 Program ID and Constants

```rust
declare_id!("..."); // Auto-filled in Solana Playground

const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;
```

> Anchor writes 8-byte discriminators to every account — identifies the account type.

---

## 🔧 Turning Rust Module into Anchor Program

```rust
#[program]
pub mod favorites {
    use super::*;

    pub fn set_favorites(
        ctx: Context<SetFavorites>,
        number: u64,
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        msg!("Greetings from {:?}", ctx.accounts.user.key());
        msg!(
            "Favorite color: {}, hobbies: {:?}",
            color, hobbies
        );

        ctx.accounts.favorites.set_inner(Favorites {
            number,
            color,
            hobbies,
        });

        Ok(())
    }
}
```

* `#[program]` macro turns a Rust module into a Solana smart contract.
* `set_favorites` becomes an **instruction handler**.
* `ctx: Context<...>` includes account data.
* Writes data to chain via `set_inner`.

---

## 🧾 Account Data Structure

```rust
#[account]
#[derive(InitSpace)]
pub struct Favorites {
    #[max_len(50)]
    pub color: String,

    #[max_len(5)]
    #[max_len(50)]
    pub hobbies: Vec<String>,

    pub number: u64,
}
```

* `#[account]` tells Anchor it's stored on-chain.
* `#[derive(InitSpace)]` allows automatic size calculation.
* `#[max_len]` defines max space for variable-size types.

---

## 📬 Accounts Context for Instruction Handler

```rust
#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account(mut, signer)]
    pub user: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE,
        seeds = [b"favorites", user.key().as_ref()],
        bump
    )]
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,
}
```

* `#[derive(Accounts)]` defines required accounts for this instruction.
* **Fields:**

  * `user`: must be mutable & signer.
  * `favorites`: derived address (PDA), initialized if needed.
  * `system_program`: required to create accounts.
* **Seeds** + `bump` = program-derived address (PDA).

---

## 🧪 Logging (optional but useful)

```rust
msg!("Greetings from {:?}", ctx.accounts.user.key());
msg!("Favorite color: {}", color);
msg!("Hobbies: {:?}", hobbies);
```

* Logs will be visible in **Solana Explorer**.

---

## ✅ Writing to Blockchain

```rust
ctx.accounts.favorites.set_inner(Favorites {
    number,
    color,
    hobbies,
});
```

> This stores structured data under a PDA owned by the program.

---
