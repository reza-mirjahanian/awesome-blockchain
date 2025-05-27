# Account Attributes in Solana Programs

## **Core Account Attributes**

### **1. `#[account]` Macro**

The fundamental attribute that marks a struct as a Solana account:

```rust
#[account]
pub struct UserAccount {
    pub owner: Pubkey,
    pub balance: u64,
    pub created_at: i64,
}
```

**Generated Features:**
- Implements `AccountSerialize` and `AccountDeserialize`
- Adds 8-byte discriminator automatically
- Implements `Owner` trait
- Implements `Discriminator` trait

### **2. Account Initialization Attributes**

#### **`init`**
Creates a new account:

```rust
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 8 + 8  // discriminator + pubkey + u64 + i64
    )]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

#### **`init_if_needed`**
Creates account only if it doesn't exist:

```rust
#[account(
    init_if_needed,
    payer = user,
    space = 8 + 32
)]
pub config: Account<'info, Config>,
```

**⚠️ Security Warning:** Requires `constraint` to prevent reinitialization attacks.

### **3. Space Calculation**

| Data Type | Size (bytes) | Example |
|-----------|--------------|---------|
| `bool` | 1 | `is_active: bool` |
| `u8/i8` | 1 | `status: u8` |
| `u16/i16` | 2 | `count: u16` |
| `u32/i32` | 4 | `timestamp: u32` |
| `u64/i64` | 8 | `balance: u64` |
| `u128/i128` | 16 | `large_amount: u128` |
| `Pubkey` | 32 | `owner: Pubkey` |
| `String` | 4 + len | `name: String` |
| `Vec<T>` | 4 + (len * size_of_T) | `items: Vec<u64>` |
| `Option<T>` | 1 + size_of_T | `data: Option<u64>` |

```rust
// Complex space calculation example
#[account]
pub struct ComplexAccount {
    pub owner: Pubkey,              // 32 bytes
    pub name: String,                // 4 + string length
    pub balances: Vec<u64>,          // 4 + (8 * vec length)
    pub metadata: Option<Metadata>,  // 1 + size of Metadata
    pub is_active: bool,             // 1 byte
}

// Space calculation
const SPACE: usize = 8 +  // discriminator
    32 +                  // owner
    4 + 32 +              // name (max 32 chars)
    4 + (8 * 10) +        // balances (max 10 items)
    1 + 64 +              // metadata option
    1;                    // is_active
```

### **4. PDA (Program Derived Address) Attributes**

#### **Basic PDA**
```rust
#[account(
    init,
    payer = user,
    space = 8 + 32,
    seeds = [b"user-stats", user.key().as_ref()],
    bump
)]
pub user_stats: Account<'info, UserStats>,
```

#### **PDA with Custom Bump**
```rust
#[account(
    init,
    payer = user,
    space = 8 + 32 + 1,
    seeds = [b"vault", user.key().as_ref()],
    bump = vault_bump
)]
pub vault: Account<'info, Vault>,
```

#### **Finding PDA Without Init**
```rust
#[account(
    seeds = [b"config"],
    bump = config.bump
)]
pub config: Account<'info, Config>,
```

### **5. Mutability Attributes**

```rust
#[derive(Accounts)]
pub struct UpdateBalance<'info> {
    #[account(mut)]  // Account data can be modified
    pub user_account: Account<'info, UserAccount>,
    
    #[account(mut)]  // SOL balance will change
    pub payer: Signer<'info>,
    
    // No mut - read-only access
    pub config: Account<'info, Config>,
}
```

### **6. Constraint Attributes**

#### **Basic Constraints**
```rust
#[account(
    mut,
    constraint = user_account.owner == user.key() @ ErrorCode::Unauthorized,
    constraint = user_account.balance > 0 @ ErrorCode::InsufficientBalance
)]
pub user_account: Account<'info, UserAccount>,
```

#### **Complex Constraints**
```rust
#[account(
    mut,
    constraint = {
        let clock = Clock::get()?;
        user_account.unlock_time <= clock.unix_timestamp
    } @ ErrorCode::AccountLocked,
    constraint = user_account.is_active @ ErrorCode::AccountInactive
)]
pub user_account: Account<'info, UserAccount>,
```

### **7. Reallocation Attributes**

```rust
#[derive(Accounts)]
#[instruction(new_size: u16)]
pub struct Realloc<'info> {
    #[account(
        mut,
        realloc = 8 + new_size as usize,
        realloc::payer = user,
        realloc::zero = false  // Don't zero out new memory
    )]
    pub data_account: Account<'info, DataAccount>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

### **8. Close Account Attribute**

```rust
#[derive(Accounts)]
pub struct CloseAccount<'info> {
    #[account(
        mut,
        close = receiver,  // Send lamports to receiver
        constraint = account.owner == user.key()
    )]
    pub account: Account<'info, UserAccount>,
    
    #[account(mut)]
    pub receiver: SystemAccount<'info>,
    pub user: Signer<'info>,
}
```

### **9. Account Types Comparison**

| Type | Use Case | Validation | Example |
|------|----------|------------|---------|
| `Account<'info, T>` | Custom program accounts | Type + Owner check | `Account<'info, UserData>` |
| `Signer<'info>` | Transaction signers | Signature verification | `user: Signer<'info>` |
| `SystemAccount<'info>` | System-owned accounts | Owner = System Program | `payer: SystemAccount<'info>` |
| `Program<'info, T>` | Program accounts | Program ID check | `Program<'info, Token>` |
| `UncheckedAccount<'info>` | Any account (unsafe) | None | `any_account: UncheckedAccount<'info>` |
| `Sysvar<'info, T>` | System variables | Sysvar check | `Sysvar<'info, Clock>` |

### **10. Associated Token Account Attributes**

```rust
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Token, TokenAccount, Mint};

#[derive(Accounts)]
pub struct InitializeATA<'info> {
    #[account(
        init,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = user
    )]
    pub user_ata: Account<'info, TokenAccount>,
    
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
```

### **11. Advanced Patterns**

#### **Multi-Seed PDAs**
```rust
#[account(
    init,
    payer = user,
    space = 8 + 32 + 8,
    seeds = [
        b"user-game-stats",
        user.key().as_ref(),
        game_id.to_le_bytes().as_ref()
    ],
    bump
)]
pub game_stats: Account<'info, GameStats>,
```

#### **Canonical Bump Seeds**
```rust
#[account]
pub struct PDAAccount {
    pub bump: u8,  // Store bump in account
    pub data: u64,
}

#[account(
    seeds = [b"pda", user.key().as_ref()],
    bump = pda_account.bump,  // Use stored bump
)]
pub pda_account: Account<'info, PDAAccount>,
```

#### **Account Versioning**
```rust
#[account]
pub struct VersionedAccount {
    pub version: u8,
    pub data: AccountDataV2,  // Can migrate between versions
}

#[derive(Accounts)]
pub struct Migrate<'info> {
    #[account(
        mut,
        constraint = account.version == 1 @ ErrorCode::InvalidVersion,
        realloc = 8 + std::mem::size_of::<AccountDataV2>(),
        realloc::payer = payer,
        realloc::zero = false
    )]
    pub account: Account<'info, VersionedAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

### **12. Performance Considerations**

| Operation | Time Complexity | Space Complexity | Gas Cost |
|-----------|----------------|------------------|----------|
| `init` | O(1) | O(n) | High |
| `mut` access | O(1) | O(1) | Low |
| PDA derivation | O(1) | O(1) | Medium |
| `realloc` | O(n) | O(n) | High |
| `close` | O(1) | O(1) | Low |
| Constraint check | O(1) | O(1) | Low |

### **13. Security Best Practices**

#### **Owner Validation**
```rust
#[account(
    mut,
    constraint = account.owner == user.key() @ ErrorCode::Unauthorized,
    constraint = !account.is_frozen @ ErrorCode::AccountFrozen
)]
pub account: Account<'info, UserAccount>,
```

#### **Signer Seeds for PDA Signing**
```rust
#[derive(Accounts)]
pub struct TransferFromPDA<'info> {
    #[account(
        mut,
        seeds = [b"vault"],
        bump
    )]
    pub vault: Account<'info, Vault>,
    // ... other accounts
}

// In instruction handler:
let seeds = &[b"vault", &[ctx.bumps.vault]];
let signer = &[&seeds[..]];
// Use signer for CPI calls
```

### **14. Common Pitfalls and Solutions**

| Pitfall | Solution |
|---------|----------|
| Forgetting discriminator space | Always add 8 bytes for discriminator |
| Reinitialization attacks | Use constraints or `init` only once |
| Integer overflow in space | Use checked arithmetic |
| Missing `mut` for payers | Always mark payers as `mut` |
| Wrong PDA seeds order | Document seed order clearly |
| Unchecked account access | Prefer typed accounts over `UncheckedAccount` |

### **15. Zero-Copy Accounts**

For large data structures:

```rust
#[account(zero_copy)]
pub struct LargeData {
    pub data: [u64; 1024],
}

#[derive(Accounts)]
pub struct UseZeroCopy<'info> {
    #[account(mut)]
    pub large_account: AccountLoader<'info, LargeData>,
}

// Usage
let mut data = ctx.accounts.large_account.load_mut()?;
data.data[0] = 42;
```

### **16. Account Info Direct Access**

```rust
#[derive(Accounts)]
pub struct RawAccess<'info> {
    /// CHECK: Manual validation required
    pub raw_account: AccountInfo<'info>,
}

// Manual validation
if raw_account.owner != &crate::ID {
    return Err(ErrorCode::InvalidOwner.into());
}
```

---

