Program Derived Address (PDA) in the Anchor Framework
====================================================

What is a PDA?  
A **Program Derived Address** is a deterministic address that *looks* like a normal Solana public-key but is **not on the Ed25519 curve** (so it has no matching private key).  
Anchor uses PDAs to let a program “own” accounts whose addresses are predictable, verifiable, and secure.

Key points
- Deterministically derived from  
  – the program id  
  – an arbitrary list of **seeds** (static bytes or run-time data, e.g. `b"user-stats"`, `user.key().as_ref()`)  
  – a single **bump** byte (0-255) that makes the resulting point fall off the curve.

- **Only the program that owns the PDA can sign for it** via a Cross-Program Invocation (CPI).  
- **Anchor automatically finds the canonical bump** (the first valid bump starting at 255) when you use the `bump` constraint .

--------------------------------------------------------------------
1. Declaring a PDA in the program
--------------------------------------------------------------------

Anchor expresses PDAs through account **constraints** inside the `#[derive(Accounts)]` struct.

```rust
use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
mod my_app {
    use super::*;
    pub fn create_user_stats(ctx: Context<CreateUserStats>, name: String) -> Result<()> {
        let stats = &mut ctx.accounts.user_stats;
        stats.owner = ctx.accounts.signer.key();
        stats.name   = name;
        stats.points = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateUserStats<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,          // payer & tx signer

    #[account(
        init,                           // create if empty
        payer = signer,
        space = 8 + 32 + 4 + 200,       // discriminator + owner + vec len + name
        seeds = [b"user-stats", signer.key().as_ref()],
        bump,                           // Anchor finds canonical bump
    )]
    pub user_stats: Account<'info, UserStats>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct UserStats {
    pub owner: Pubkey,
    pub name:   String,
    pub points: u64,
}
```

Explanation of constraints
- `seeds` – list of byte slices that together with the program id uniquely determine the address.  
- `bump` – tells Anchor to compute the canonical bump and store it in the account discriminator for later verification.  
- `init` / `payer` / `space` – tell Anchor to CPI into the system program and allocate the account .

--------------------------------------------------------------------
2. Deriving the same PDA on the client
--------------------------------------------------------------------

Anchor’s TypeScript client uses the program’s IDL to re-compute PDAs automatically.

```ts
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MyApp } from "../target/types/my_app";

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const program = anchor.workspace.MyApp as Program<MyApp>;

const [userStatsPDA] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("user-stats"), provider.wallet.publicKey.toBuffer()],
  program.programId
);

console.log("Derived PDA:", userStatsPDA.toBase58());

await program.methods
  .createUserStats("Alice")
  .accounts({
    userStats: userStatsPDA,
    signer: provider.wallet.publicKey,
  })
  .rpc();
```

No bump is passed from the client; Anchor derives it deterministically .

--------------------------------------------------------------------
3. Security best practices
--------------------------------------------------------------------
- Always verify the PDA inside the instruction—Anchor does this for you when you use `seeds` and `bump`.  
- Never accept a user-supplied bump unless you re-derive the PDA and check it matches.  
- Use **canonical bump** (Anchor default) to avoid collisions and replay attacks .

--------------------------------------------------------------------
4. Multiple PDAs per user
--------------------------------------------------------------------
Need one PDA per order, per post, etc.?  
Add a dynamic seed such as an index, timestamp, or a uuid:

```rust
seeds = [b"order", signer.key().as_ref(), &order_id.to_le_bytes()],
```

Each new seed tuple yields a brand-new PDA .

--------------------------------------------------------------------
5. Quick reference
--------------------------------------------------------------------
| Task                         | Anchor code pattern                                               |
|------------------------------|--------------------------------------------------------------------|
| Declare PDA                  | `#[account(seeds=[...], bump, ...)]`                              |
| Create account at PDA        | add `init`, `payer`, `space` constraints                          |
| Canonical bump               | use `bump` only (Anchor finds 255..0)                             |
| Client-side derive           | `findProgramAddressSync(seeds, programId)`                        |
| Re-use same PDA              | identical seeds → identical address → idempotent                  |
| Cross-program PDA            | `seeds::program = other_program_id` constraint                    |

With these patterns you can build escrow wallets, user profiles, mint authorities, or any other resource whose address must be predictable and controlled solely by your program.