

---

# 🧭 `#[account]` Attribute in Anchor

The `#[account]` attribute in Anchor is used to:

* Annotate **account struct definitions** stored on-chain
* Annotate **account arguments** in instruction context structs
* Define **constraints**, **initialization rules**, and **ownership checks**

---

## 🔹 1. `#[account]` on **Struct Definitions**

This marks the struct as **serializable and storable on-chain**. Anchor:

* Automatically assigns an **8-byte discriminator**
* Handles **serialization/deserialization**
* Enforces **ownership rules** during execution

### ✅ Example:

```rust
#[account]
pub struct MyAccount {
    pub authority: Pubkey,
    pub count: u64,
}
```

### ⚠️ Rules:

* All fields must be serializable (e.g. `Pubkey`, `u64`, etc.)
* You **must** include the 8-byte discriminator in the `space` calculation

---

## 🔹 2. `#[account(...)]` on Instruction Context Struct Fields

Defines **runtime constraints** and **initialization rules** for each account.

### 🚨 Anchor runs these constraints *before* calling the instruction handler.

---

## 🔸 Common Constraints and Modifiers

| Constraint           | Purpose                                                        | Example                                                     |
| -------------------- | -------------------------------------------------------------- | ----------------------------------------------------------- |
| `init`               | Initializes the account                                        | `#[account(init, payer = user, space = 8 + 8)]`             |
| `mut`                | Marks the account as mutable                                   | `#[account(mut)]`                                           |
| `seeds`              | Uses seeds to derive a PDA                                     | `#[account(seeds = [b"vault", user.key().as_ref()], bump)]` |
| `bump`               | Used with `seeds` to derive PDA bump seed                      | `#[account(seeds = [...], bump)]`                           |
| `payer`              | Specifies who pays rent for new account                        | `#[account(init, payer = user)]`                            |
| `has_one`            | Enforces account field equals another account's key            | `#[account(has_one = authority)]`                           |
| `constraint = expr`  | Arbitrary Rust boolean expression                              | `#[account(constraint = data.count < 10)]`                  |
| `close = account`    | Closes the account and sends lamports to `account`             | `#[account(mut, close = recipient)]`                        |
| `owner = program_id` | Ensures the account is owned by a specific program             | `#[account(owner = token::ID)]`                             |
| `zero`               | Ensures account is zeroed out (used for CPI receiver accounts) | `#[account(zero)]`                                          |

---

## 🔹 3. Initialization Constraints (`init`)

Used when creating accounts.

### Example:

```rust
#[account(
    init,
    payer = user,
    space = 8 + 8
)]
pub my_account: Account<'info, MyAccount>,
```

#### Required fields with `init`:

* `payer`
* `space`
* (optional) `seeds`, `bump`

### 🧠 Trick: PDA + `bump`

```rust
#[account(
    init,
    seeds = [b"vault", user.key().as_ref()],
    bump,
    payer = user,
    space = 8 + 8
)]
pub vault: Account<'info, Vault>,
```

To access the bump later:

```rust
let (_pda, bump) = Pubkey::find_program_address(&[b"vault", user.key().as_ref()], program_id);
```

Or store it in the account:

```rust
pub struct Vault {
    pub bump: u8,
    pub data: u64,
}
```

---

## 🔹 4. `has_one` Constraint

Ensures a field inside an account matches another account’s public key.

```rust
#[account(mut, has_one = authority)]
pub my_account: Account<'info, MyAccount>,
pub authority: Signer<'info>,
```

Means: `my_account.authority == authority.key()`

**⚠️ Fails at runtime if not equal.**

---

## 🔹 5. `close = account`

Sends all lamports to another account and marks the closed account as uninitialized.

```rust
#[account(mut, close = user)]
pub my_account: Account<'info, MyAccount>,
```

**Best practice:** Use when cleaning up PDAs.

---

## 🔹 6. `constraint = expression`

Allows arbitrary Rust expressions to assert invariants.

```rust
#[account(
    mut,
    constraint = my_account.count < 100 @ MyError::MaxExceeded
)]
pub my_account: Account<'info, MyAccount>,
```

Supports custom error codes:

```rust
#[error_code]
pub enum MyError {
    #[msg("Value too high.")]
    MaxExceeded,
}
```

---

## 🔹 7. `zero`

Used for CPI-allocated accounts where the data is expected to be zeroed.

```rust
#[account(zero)]
pub token_account: Account<'info, TokenAccount>,
```

---

## 🔹 8. `owner = Pubkey`

Checks the account owner. Common when working with external programs like Token Program.

```rust
#[account(owner = token::ID)]
pub token_account: AccountInfo<'info>,
```

---

## 🧪 Edge Cases & Gotchas

### ✅ PDA Must Be Recreated Exactly

Mismatch in seeds/bump → runtime error: “Account not created by program”

### ✅ `AccountInfo` vs `Account`

* `Account<'info, T>`: Deserializable, struct-safe
* `AccountInfo<'info>`: Raw access, lower-level

### ✅ No Default Field Initialization

With `init`, fields are NOT zero unless you do it explicitly in the handler:

```rust
ctx.accounts.my_account.count = 0;
```

---

## ⚙️ Complexity (O-notation)

| Operation             | Complexity            |
| --------------------- | --------------------- |
| Constraint checks     | O(1) each             |
| PDA derivation        | O(n) with seed length |
| Account serialization | O(data size)          |

---

## 🔁 Comparison Table

| Topic/Usage             | `#[account]`               | `AccountInfo` | `Account<T>`    |
| ----------------------- | -------------------------- | ------------- | --------------- |
| Declarative constraints | ✅ Yes                      | ❌ No          | ✅ Yes           |
| Deserializes data       | ✅ Automatically            | ❌ Manual      | ✅ Automatically |
| Used in CPIs            | ✅ With care (e.g., zeroed) | ✅ Often       | ✅ Usually       |
| Custom constraints      | ✅ With `constraint =`      | ❌ No          | ✅ Yes           |

---

## 🧱 Space Calculation Best Practice

```rust
#[account]
pub struct Example {
    pub authority: Pubkey, // 32 bytes
    pub count: u64,        // 8 bytes
    pub bump: u8,          // 1 byte
}
```

**Space = 8 (discriminator) + 32 + 8 + 1 = 49 bytes**

Pad to nearest 8-byte boundary for alignment → use 56

---

## 🛠️ Code Snippets

### Full Initialization Example

```rust
#[derive(Accounts)]
pub struct Create<'info> {
    #[account(
        init,
        seeds = [b"data", user.key().as_ref()],
        bump,
        payer = user,
        space = 8 + 32 + 8
    )]
    pub data_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

---

## ➕ Pros / ➖ Cons

| Pros                                               | Cons                                                          |
| -------------------------------------------------- | ------------------------------------------------------------- |
| ✅ Enforces safety & constraints at runtime         | ❌ Limited to what Anchor supports — not full Rust flexibility |
| ✅ Easier PDA derivation and validation             | ❌ Complex macro errors can be cryptic                         |
| ✅ Prevents many common security bugs automatically | ❌ Constraints can introduce hard-to-debug behavior            |

---

Continuing with the next advanced topic:

---

# 🧠 **Program Derived Addresses (PDAs) and Seeds in Anchor**

---

## 📌 What is a Program Derived Address (PDA)?

A **Program Derived Address (PDA)** is a **deterministic, non-signing account address** that can only be controlled by a **program**.

> A PDA is not associated with a private key — only your program can sign for it.

---

## 🔍 Why Use PDAs?

* To **store data** associated with a program in a predictable location
* To **enforce program-only access** to sensitive accounts (e.g., vaults, metadata)
* To **create namespaced addresses** using seeds (e.g., one account per user)

---

## 📐 PDA Formula

```rust
Pubkey::find_program_address(&[seeds...], program_id)
```

Solana runtime:

* Takes seeds
* Adds program ID
* Finds a valid address (no collision with private keys)
* Returns `(pda, bump)`

> The **bump** is used to find an address that is *off-curve* (unclaimable by private key)

---

## 🛠️ PDA Creation in Anchor (using `#[account]`)

### Example: Initialize a PDA account

```rust
#[account(
    init,
    seeds = [b"vault", user.key().as_ref()],
    bump,
    payer = user,
    space = 8 + 8
)]
pub vault: Account<'info, Vault>,
```

### Store `bump` inside account:

```rust
#[account]
pub struct Vault {
    pub bump: u8,
    pub amount: u64,
}
```

Then in your handler:

```rust
ctx.accounts.vault.bump = *ctx.bumps.get("vault").unwrap();
```

---

## 🧪 Runtime PDA Verification

When a PDA account is passed into the program, Anchor verifies:

* `vault.key() == Pubkey::create_program_address(&[b"vault", user.key().as_ref(), &[bump]], program_id)`
* If not, the transaction fails

---

## 🔄 Comparing: `Pubkey::create_program_address` vs `find_program_address`

| Function                 | Purpose                       | When to Use                  |
| ------------------------ | ----------------------------- | ---------------------------- |
| `find_program_address`   | Finds a valid PDA + bump      | When initializing PDAs       |
| `create_program_address` | Validates PDA with known bump | When verifying/checking PDAs |

---

## 🔐 PDA as Authority Example

```rust
#[account(
    mut,
    seeds = [b"vault", user.key().as_ref()],
    bump
)]
pub vault: Account<'info, Vault>,
```

You can then sign for the PDA inside the program:

```rust
let seeds = &[b"vault", user.key().as_ref(), &[vault.bump]];
let signer = &[&seeds[..]];

token::transfer(
    CpiContext::new_with_signer(
        token_program.to_account_info(),
        Transfer {
            from: source,
            to: dest,
            authority: vault.to_account_info(),
        },
        signer,
    ),
    amount,
)?;
```

---

## ⚠️ Tricky Parts

| Trap / Edge Case                                 | Why It Happens                         | How to Fix                                        |
| ------------------------------------------------ | -------------------------------------- | ------------------------------------------------- |
| ❌ Wrong bump causes `ProgramError::InvalidSeeds` | Using wrong bump in signer             | Always use `find_program_address` or `ctx.bumps`  |
| ❌ Seed mismatch during validation                | Change in seed logic without migration | Audit seed logic before deployment                |
| ❌ Trying to sign with PDA outside program        | PDAs can't sign externally             | Use PDA as authority inside the program only      |
| ❌ PDA not created with `init` or wrongly passed  | Not using `init` properly              | Use `init` with `seeds`, `bump`, `payer`, `space` |

---

## 🔁 Use Cases for PDAs

| Use Case         | Description                                              |
| ---------------- | -------------------------------------------------------- |
| Associated vault | Store user funds in a PDA derived from `["vault", user]` |
| Metadata         | Store asset data at `["metadata", mint]`                 |
| Escrow           | Time-locked contracts using `["escrow", offer_id]`       |
| DAO governance   | Vote records at `["vote", proposal_id, voter]`           |

---

## 🧮 O-Notation for PDA Operations

| Operation                               | Time Complexity                                                      |
| --------------------------------------- | -------------------------------------------------------------------- |
| `find_program_address()`                | O(n), where `n` is number of retry attempts to get off-curve address |
| PDA signature with `create_with_signer` | O(1)                                                                 |

---

## 🧠 Advanced PDA Trick: Multi-Signer Seeds

Use a PDA as a joint authority:

```rust
#[account(
    seeds = [b"multi", user1.key().as_ref(), user2.key().as_ref()],
    bump
)]
pub shared_account: Account<'info, SharedAccount>,
```

You can enforce 2-of-2 control using this PDA without creating a multisig program.

---

## 🧱 Full PDA Lifecycle

1. **Calculate PDA** in client using `find_program_address`
2. **Pass it into instruction** as an account with `#[account(init, seeds = [...], bump)]`
3. **Use it later as signer** with the same seeds and bump
4. **Close or mutate it** as long as the seeds still resolve

---

## 🧬 Summary Table: PDA vs Non-PDA

| Feature               | PDA                 | Regular Account           |
| --------------------- | ------------------- | ------------------------- |
| Signable              | ✅ (by program only) | ✅ (by private key holder) |
| Deterministic address | ✅ Yes               | ❌ No                      |
| Used as vaults        | ✅ Common            | ❌ Risky                   |
| Require seeds/bump    | ✅ Yes               | ❌ No                      |

---

## 📦 Code Snippet: Using PDA as Token Authority

```rust
let seeds = &[b"vault", user.key().as_ref(), &[ctx.accounts.vault.bump]];
let signer = &[&seeds[..]];

token::mint_to(
    CpiContext::new_with_signer(
        token_program.to_account_info(),
        MintTo {
            mint: mint.to_account_info(),
            to: token_account.to_account_info(),
            authority: vault.to_account_info(),
        },
        signer,
    ),
    1,
)?;
```

---



