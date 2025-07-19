## **Program-Derived Addresses (PDAs) in the Anchor Framework**

---

### 1. Core Concepts  

| Term | What it is | Anchor Macro / API | Mandatory? |
|------|------------|--------------------|------------|
| **Seed** | Arbitrary byte slices (≤ 32 bytes each, ≤ 16 seeds) used to derive the address | `seeds = [b"tag", user.key().as_ref(), … ]` | Yes |
| **Bump** | 1-byte “nonce” (0-255) that makes the derived point fall off the ed25519 curve | `bump`, `bump_seed`, `Pubkey::find_program_address` | Yes |
| **PDA** | A public key **owned by a program**, incapable of holding a private key | Created implicitly; becomes the signer when `invoke_signed` / Anchor `ctx.accounts` is used | Yes |
| **Off-Curve** | Required property: PDA **must NOT** lie on the ed25519 curve ⇒ no real private key | Automatic when using `find_program_address` | Yes |

---

### 2. Why PDAs? (vs. “normal” keypairs)

| Feature | PDA | Normal Keypair |
|---------|-----|----------------|
| Private key exists? | **No** | Yes |
| Can sign tx directly? | No (needs program) | Yes |
| Deterministic? | Yes (seeds+bump) | Only if you store the secret |
| Access control | Enforced by program logic | Off-chain / multisig |
| Funds retrieval on program upgrade? | Automatic (same program id) | Manual transfer |
| CPI signing | Supported with `invoke_signed` | Requires secret sharing |

---

### 3. Creating & Using PDAs in Anchor (Rust)

```rust
// 1. Define account with seeds
#[account(
    init,               // create if not exists
    payer = user,       // who funds the rent
    space = 8 + 40,     // discriminator + data
    seeds = [b"vault".as_ref(), user.key().as_ref()],
    bump                // stores the bump automatically
)]
pub struct VaultAccount {
    /// Always store the bump if you plan to re-sign later
    pub bump: u8,
    pub authority: Pubkey,
    pub amount: u64,
}

// 2. Instruction context
#[derive(Accounts)]
pub struct InitVault<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault".as_ref(), user.key().as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info, VaultAccount>,

    pub system_program: Program<'info, System>,
}

// 3. Program logic
pub fn init_vault(ctx: Context<InitVault>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    vault.bump = *ctx.bumps.get("vault").unwrap(); // store bump
    vault.authority = ctx.accounts.user.key();
    Ok(())
}
```

---

### 4. Client-Side Derivation (TypeScript w/ Anchor)

```ts
// Program id from IDL or anchor.toml
const programId = new PublicKey("YourProgram1111111111111111111111111111111");

const [vaultPda, vaultBump] = await PublicKey.findProgramAddress(
  [
    Buffer.from("vault"),
    user.publicKey.toBuffer(),
  ],
  programId
);

console.log("Vault PDA:", vaultPda.toBase58(), "bump", vaultBump);
```

---

### 5. `invoke_signed` & Cross-Program Invocation (CPI)

```rust
pub fn sweep_sol(ctx: Context<Sweep>) -> Result<()> {
    let vault = &ctx.accounts.vault;
    let seeds = &[
        b"vault".as_ref(),
        ctx.accounts.authority.key.as_ref(),
        &[vault.bump],
    ];
    let signer_seeds = &[&seeds[..]];       // &[&[&[u8]]]

    let ix = system_instruction::transfer(
        &ctx.accounts.vault.key(),          // from PDA
        &ctx.accounts.destination.key(),
        vault.amount,
    );

    anchor_lang::solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.destination.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        signer_seeds,
    )?;
    Ok(())
}
```

---

### 6. Edge Cases & Tricky Parts 🚧

| Gotcha | Explanation / Fix |
|--------|-------------------|
| Seeds over 32 bytes | Split, hash, or shorten before `find_program_address`. |
| Using `init_if_needed` + `bump` mismatches | Always pass **identical seeds** and `bump` (store bump in the account). |
| Multiple bumps | Only last byte is bump; store only one. |
| Close PDA account | Use `close = recipient` in Anchor; bump not required for closing. |
| PDA owns Token Account | Must set account’s owner to `token_program_id`; PDA acts as “Authority”. |
| Changing seed data (e.g., user key) | Impossible ⇒ derive a new PDA and migrate data. |
| Off-Curve fudge | Never attempt to derive via `Pubkey::create_program_address` with custom bump loops; always use `find_program_address`. |

---

### 7. Common Patterns

1. **Escrow Vaults**  
   Seed: `[b"escrow", order_id.to_le_bytes()]`  

2. **Associated Token PDA (Anchor helper)**  
   Anchor: `#[account(associated_token::mint = mint, associated_token::authority = user)]`  

3. **Program-Authority for Minting**  
   Seed: `[b"mint_authority"]` → becomes mint’s `mint_authority` & `freeze_authority`.

4. **Metadata PDAs (Metaplex)**  
   Seed sequence: `[b"metadata", metadata_program_id, mint]`

---

### 8. Advanced Tips & Tricks

• **Dynamic Seeds** – Use GUIDs or incrementing counters to avoid collisions while remaining deterministic.  
• **Fewer CPIs** – Store bump inside account to skip `find_program_address` during runtime.  
• **Safety Macro** – `#[account(seeds = [...], bump, realloc = new_size, realloc::payer = user)]` handles resizing without re-deriving bump.  
• **Events** – Emit bump + seeds bytes to help indexers reconstruct signer seeds off-chain.  
• **Zero-Copy PDAs** – Combine zero-copy deserialization with PDAs for cost-efficient programs.

---

### 9. Real-World Projects Using PDAs

| Project | Purpose of PDA | Main Seed(s) |
|---------|----------------|--------------|
| Serum DEX | Market state, request/ event queues | `["serum_market", mint_a, mint_b]` |
| Candy Machine (Metaplex) | Candy machine state & config | `["candy_machine", uuid]` |
| Mango Markets | Margin accounts & groups | `["mango_group", group_num]` |
| SPL Governance | Realm & Proposal accounts | `["governance", realm_name]` |

---

### 10. Pros / Cons Table

| Pros | Cons |
|------|------|
| No private key to compromise | Irreversible seed mistakes ⇒ funds locked |
| Deterministic; easy to recreate | Seeds immutable; migration overhead |
| Enables on-chain “signatures” for CPIs | Limited to ≤ 16 seeds, 512 total bytes |
| Seamless with Anchor macros | Requires bump management |

---

### 11. Cheat-Sheet Reference

```
Max seeds per PDA..........16
Max seed length............32 bytes
Total cumulative bytes.....512
Bump range.................0-255
Derivation fn..............Pubkey::find_program_address
Signer in CPI..............invoke_signed / cpi_ctx.signer_seeds
Anchor seed syntax.........seeds = [..], bump
```

---

### **Next Step for Deeper Expertise → “Zero-Knowledge Proofs inside Solana Programs (ZK-Sol)**