
---

### 1) What does `#[account(mut)]` do, and why isn’t it on `new_account`?

* **`mut` = “writable”**
  Whenever your program is going to *change* an account’s data or balance (lamports), you must mark it as mutable so that the runtime opens it with write‐access.

  ```rust
  #[account(mut)]
  pub signer: Signer<'info>,
  ```

  Here you need `mut` because:

  1. You’ll **debit** lamports from the payer (`signer`) to fund the new account.
  2. You might also change some fields on the `signer` account in other contexts.

* **Why no explicit `mut` on `new_account`?**
  When you use `#[account(init, …)]`, Anchor implicitly treats that account as mutable.

  * `init` tells Anchor: “Create a *brand new* account here, with this space and funded by `payer`,” and under the hood it

    1. Allocates the account (`SystemProgram::create_account`)
    2. Assigns it as writable so you can immediately write its data.
  * Because `init` always implies “we’re going to write to it,” you don’t also need to tag `#[account(mut)]` on the same field.

---

### 2) What does the `init` argument in `#[account(init, …)]` do?

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

* **`init`**

  * **Creates** the account if it doesn’t exist.
  * **Allocates** exactly `space` bytes on‐chain (here `8` bytes for the Anchor discriminator + `8` bytes for your `u64` field).
  * **Funds** it with the minimum rent‐exempt lamports, charging them to `payer` (here `signer`).
  * **Sets** the account’s owner to your program (so you can later read/write its data).

* **`payer = signer`**

  * Specifies that `signer` must cover the rent‐exemption cost.

* **`space = 8 + 8`**

  * First `8` bytes are Anchor’s “discriminator” (a unique tag for your account type).
  * The next `8` bytes are for your `u64` field (`NewAccount.data`).

Once that runs, `new_account` is a freshly minted account of type `NewAccount`, ready for you to do:

```rust
ctx.accounts.new_account.data = data;
```

---

#### Putting it all together

1. **Signer must be mutable** because you’re *paying* lamports out of it.
2. **`new_account` doesn’t need an explicit `mut`** because `init` automatically makes it writable and allocates it.
3. **`init` handles creation, space allocation, funding, and ownership** so you don’t have to write all the boilerplate yourself.

