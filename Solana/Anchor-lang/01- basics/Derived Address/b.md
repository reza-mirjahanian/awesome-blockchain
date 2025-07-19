 ## **Program Derived Addresses (PDAs) in Anchor Framework**

### **Core Concepts**

PDAs are deterministically derived addresses that:
- **Cannot have private keys** (off-curve points)
- Are **owned by programs**, not users
- Enable programs to sign for themselves
- Created using seeds + program ID + bump

### **PDA Generation Methods**

```rust
// Method 1: find_program_address (includes bump search)
let (pda, bump) = Pubkey::find_program_address(
    &[b"vault", user.key().as_ref()],
    program_id
);

// Method 2: create_program_address (requires bump)
let pda = Pubkey::create_program_address(
    &[b"vault", user.key().as_ref(), &[bump]],
    program_id
)?;

// Method 3: Anchor's constraint (recommended)
#[account(
    seeds = [b"vault", user.key().as_ref()],
    bump
)]
pub vault: Account<'info, Vault>,
```

### **Anchor PDA Constraints**

#### **Basic PDA Validation**
```rust
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 8,
        seeds = [b"vault", user.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

#### **PDA with Stored Bump**
```rust
#[account]
pub struct Vault {
    pub authority: Pubkey,
    pub amount: u64,
    pub bump: u8,  // Store bump for efficiency
}

#[derive(Accounts)]
pub struct UseVault<'info> {
    #[account(
        mut,
        seeds = [b"vault", authority.key().as_ref()],
        bump = vault.bump,  // Use stored bump
    )]
    pub vault: Account<'info, Vault>,
    pub authority: Signer<'info>,
}
```

### **Seed Types & Patterns**

| Seed Type | Example | Use Case |
|-----------|---------|----------|
| Static bytes | `b"vault"` | Type identifier |
| Pubkey | `user.key().as_ref()` | Account association |
| Integer bytes | `id.to_le_bytes()` | Sequential IDs |
| String bytes | `name.as_bytes()` | Named resources |
| Discriminator | `DISCRIMINATOR` | Account type |
| Timestamp | `clock.unix_timestamp.to_le_bytes()` | Time-based |

```rust
// Complex seed combinations
#[derive(Accounts)]
#[instruction(game_id: u64, round: u8)]
pub struct GameRound<'info> {
    #[account(
        init,
        payer = player,
        space = 8 + GameRound::SIZE,
        seeds = [
            b"game",
            player.key().as_ref(),
            &game_id.to_le_bytes(),
            b"round",
            &[round],
        ],
        bump
    )]
    pub game_round: Account<'info, GameRound>,
    
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

### **PDA Signing Patterns**

#### **CPI with PDA Signer**
```rust
#[derive(Accounts)]
pub struct TransferFromVault<'info> {
    #[account(
        mut,
        seeds = [b"vault"],
        bump = vault.bump,
    )]
    pub vault: Account<'info, Vault>,
    
    #[account(
        mut,
        constraint = vault_token.owner == vault.key()
    )]
    pub vault_token: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub recipient_token: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

pub fn transfer_from_vault(ctx: Context<TransferFromVault>, amount: u64) -> Result<()> {
    let seeds = &[
        b"vault",
        &[ctx.accounts.vault.bump],
    ];
    let signer_seeds = &[&seeds[..]];
    
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.vault_token.to_account_info(),
            to: ctx.accounts.recipient_token.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        },
        signer_seeds,
    );
    
    token::transfer(cpi_ctx, amount)?;
    Ok(())
}
```

#### **Multiple PDA Signers**
```rust
pub fn complex_transfer(ctx: Context<ComplexTransfer>) -> Result<()> {
    // First PDA signer
    let vault_seeds = &[
        b"vault",
        ctx.accounts.authority.key().as_ref(),
        &[ctx.accounts.vault.bump],
    ];
    
    // Second PDA signer
    let escrow_seeds = &[
        b"escrow",
        ctx.accounts.trade_id.as_ref(),
        &[ctx.accounts.escrow.bump],
    ];
    
    // Combined signer seeds for CPI
    let signer_seeds = &[&vault_seeds[..], &escrow_seeds[..]];
    
    // CPI with multiple PDA signers
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.program.to_account_info(),
        accounts,
        signer_seeds,
    );
}
```

### **Advanced PDA Patterns**

#### **Hierarchical PDAs**
```rust
// Parent PDA
#[account]
pub struct Organization {
    pub admin: Pubkey,
    pub name: String,
    pub bump: u8,
}

// Child PDA referencing parent
#[account]
pub struct Department {
    pub organization: Pubkey,
    pub manager: Pubkey,
    pub name: String,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct CreateDepartment<'info> {
    #[account(
        seeds = [b"org", admin.key().as_ref()],
        bump = organization.bump,
    )]
    pub organization: Account<'info, Organization>,
    
    #[account(
        init,
        payer = admin,
        space = 8 + Department::SIZE,
        seeds = [
            b"dept",
            organization.key().as_ref(),
            department_name.as_bytes()
        ],
        bump
    )]
    pub department: Account<'info, Department>,
    
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

#### **Canonical Bump Pattern**
```rust
// Always use canonical bump for consistency
#[derive(Accounts)]
pub struct InitCanonical<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 8 + 1,
        seeds = [b"canonical", authority.key().as_ref()],
        bump,  // Anchor finds canonical bump
    )]
    pub pda_account: Account<'info, CanonicalPDA>,
    
    pub authority: SystemAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitCanonical<'info> {
    pub fn process(&mut self, bumps: &InitCanonicalBumps) -> Result<()> {
        self.pda_account.bump = bumps.pda_account; // Store canonical bump
        Ok(())
    }
}
```

### **PDA Size & Rent Optimization**

```rust
// Efficient PDA with minimal storage
#[account]
#[derive(Default)]
pub struct CompactPDA {
    pub data: u64,        // 8 bytes
    pub bump: u8,         // 1 byte
    // Total: 9 bytes + 8 discriminator = 17 bytes
}

// PDA with dynamic reallocation
#[derive(Accounts)]
#[instruction(additional_size: usize)]
pub struct ReallocPDA<'info> {
    #[account(
        mut,
        seeds = [b"dynamic", authority.key().as_ref()],
        bump = pda.bump,
        realloc = 8 + pda.data.len() + additional_size,
        realloc::payer = authority,
        realloc::zero = false,
    )]
    pub pda: Account<'info, DynamicPDA>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

### **Common PDA Pitfalls & Solutions**

| Issue | Problem | Solution |
|-------|---------|----------|
| **Bump Recalculation** | Performance overhead | Store bump in account |
| **Seed Order** | Mismatched PDAs | Document seed order |
| **String Seeds** | Inconsistent encoding | Use fixed encoding |
| **Integer Seeds** | Endianness issues | Always use `to_le_bytes()` |
| **Max Seed Length** | 32 bytes per seed limit | Hash long seeds |
| **Collision Risk** | Duplicate PDAs | Add unique discriminators |

### **Security Patterns**

#### **Authority Migration**
```rust
#[derive(Accounts)]
pub struct MigrateAuthority<'info> {
    #[account(
        mut,
        seeds = [b"vault", current_authority.key().as_ref()],
        bump = vault.bump,
        close = current_authority,  // Close old PDA
    )]
    pub vault: Account<'info, Vault>,
    
    #[account(
        init,
        payer = current_authority,
        space = 8 + Vault::SIZE,
        seeds = [b"vault", new_authority.key().as_ref()],
        bump
    )]
    pub new_vault: Account<'info, Vault>,
    
    #[account(mut)]
    pub current_authority: Signer<'info>,
    /// CHECK: New authority for vault
    pub new_authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
```

#### **PDA Access Control**
```rust
#[account]
pub struct ProtectedPDA {
    pub authority: Pubkey,
    pub delegates: Vec<Pubkey>,
    pub permissions: u8,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct AccessProtectedPDA<'info> {
    #[account(
        seeds = [b"protected", pda.authority.as_ref()],
        bump = pda.bump,
        constraint = pda.delegates.contains(&accessor.key()) 
            || pda.authority == accessor.key() @ ErrorCode::Unauthorized
    )]
    pub pda: Account<'info, ProtectedPDA>,
    
    pub accessor: Signer<'info>,
}
```

### **Performance Optimization**

```rust
// Batch PDA operations
#[derive(Accounts)]
#[instruction(count: u8)]
pub struct BatchInitialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    // Remaining accounts for PDAs
}

pub fn batch_initialize(ctx: Context<BatchInitialize>, count: u8) -> Result<()> {
    for i in 0..count {
        let seeds = &[b"batch", &[i]];
        let (pda, bump) = Pubkey::find_program_address(seeds, ctx.program_id);
        
        // Get account from remaining_accounts
        let pda_account = ctx.remaining_accounts
            .get(i as usize)
            .ok_or(ErrorCode::InvalidAccountIndex)?;
            
        // Manual PDA initialization
        let lamports = Rent::get()?.minimum_balance(ACCOUNT_SIZE);
        let ix = system_instruction::create_account(
            &ctx.accounts.payer.key(),
            &pda,
            lamports,
            ACCOUNT_SIZE as u64,
            ctx.program_id,
        );
        
        invoke_signed(
            &ix,
            &[
                ctx.accounts.payer.to_account_info(),
                pda_account.clone(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[&[b"batch", &[i], &[bump]]],
        )?;
    }
    Ok(())
}
```

### **Real-World PDA Implementations**

#### **DEX Market PDAs**
```rust
// Market PDA with multiple components
#[derive(Accounts)]
pub struct InitializeMarket<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + Market::SIZE,
        seeds = [
            b"market",
            base_mint.key().as_ref(),
            quote_mint.key().as_ref(),
        ],
        bump
    )]
    pub market: Account<'info, Market>,
    
    #[account(
        init,
        payer = payer,
        space = 8 + OrderBook::SIZE,
        seeds = [
            b"orderbook",
            market.key().as_ref(),
            b"bids"
        ],
        bump
    )]
    pub bids: Account<'info, OrderBook>,
    
    #[account(
        init,
        payer = payer,
        space = 8 + OrderBook::SIZE,
        seeds = [
            b"orderbook",
            market.key().as_ref(),
            b"asks"
        ],
        bump
    )]
    pub asks: Account<'info, OrderBook>,
    
    pub base_mint: Account<'info, Mint>,
    pub quote_mint: Account<'info, Mint>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

#### **Staking Pool PDAs**
```rust
#[derive(Accounts)]
pub struct CreateStakeAccount<'info> {
    // Global pool PDA
    #[account(
        seeds = [b"pool", pool.mint.as_ref()],
        bump = pool.bump,
    )]
    pub pool: Account<'info, StakingPool>,
    
    // User's stake account PDA
    #[account(
        init,
        payer = staker,
        space = 8 + StakeAccount::SIZE,
        seeds = [
            b"stake",
            pool.key().as_ref(),
            staker.key().as_ref()
        ],
        bump
    )]
    pub stake_account: Account<'info, StakeAccount>,
    
    // Pool's token vault PDA
    #[account(
        mut,
        seeds = [
            b"vault",
            pool.key().as_ref()
        ],
        bump = pool.vault_bump,
    )]
    pub vault: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub staker: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

### **Testing PDA Logic**

```typescript
import { PublicKey } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";

describe("PDA Tests", () => {
  it("derives correct PDA", async () => {
    const [pda, bump] = await PublicKey.findProgramAddress(
      [
        Buffer.from("vault"),
        provider.wallet.publicKey.toBuffer(),
      ],
      program.programId
    );
    
    // Initialize PDA
    await program.methods
      .initialize()
      .accounts({
        vault: pda,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();
    
    // Verify PDA data
    const vaultAccount = await program.account.vault.fetch(pda);
    assert.equal(vaultAccount.bump, bump);
  });
  
  it("handles PDA collisions", async () => {
    // Test with different seeds that might collide
    const seeds1 = [Buffer.from("test"), Buffer.from([1])];
    const seeds2 = [Buffer.from("tes"), Buffer.from("t1")];
    
    const [pda1] = await PublicKey.findProgramAddress(seeds1, programId);
    const [pda2] = await PublicKey.findProgramAddress(seeds2, programId);
    
    assert.notEqual(pda1.toString(), pda2.toString());
  });
});
```

### **PDA Best Practices**

**DO:**
- ✅ Store bumps in PDA accounts
- ✅ Use canonical bumps (from `find_program_address`)
- ✅ Document seed ordering
- ✅ Use type-safe seed generation
- ✅ Validate PDA ownership in constraints

**DON'T:**
- ❌ Hardcode bump values
- ❌ Use variable-length strings without validation
- ❌ Forget endianness for number seeds
- ❌ Exceed 32 bytes per seed
- ❌ Use more than 16 seeds total

### **Advanced PDA Techniques**

#### **Hashmap Pattern**
```rust
// Simulate hashmap using PDAs
#[derive(Accounts)]
#[instruction(key: String)]
pub struct HashMapSet<'info> {
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + 32 + value.len() + 4,
        seeds = [
            b"hashmap",
            namespace.key().as_ref(),
            &anchor_lang::solana_program::hash::hash(key.as_bytes()).to_bytes()
        ],
        bump
    )]
    pub entry: Account<'info, HashMapEntry>,
    
    pub namespace: Account<'info, Namespace>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

#### **Time-locked PDAs**
```rust
#[derive(Accounts)]
pub struct CreateTimeLock<'info> {
    #[account(
        init,
        payer = creator,
        space = 8 + TimeLock::SIZE,
        seeds = [
            b"timelock",
            creator.key().as_ref(),
            &unlock_timestamp.to_le_bytes()
        ],
        bump
    )]
    pub timelock: Account<'info, TimeLock>,
    
    #[account(mut)]
    pub creator: Signer<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}
```

### **Next Steps: Cross-Program Invocation (CPI) with PDA Authority**

The next advanced topic is mastering **CPI patterns with PDA authority** for building composable programs. This includes:
- Complex CPI scenarios with multiple PDA signers
- Authority delegation patterns
- Program-owned program accounts
- Recursive CPI with PDAs
- Security considerations for PDA-based CPIs
- Performance optimization for CPI-heavy operations