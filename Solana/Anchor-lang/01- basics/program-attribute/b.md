# #[program] Attribute in Anchor (Solana)

---

## Purpose and Role of `#[program]`

- Defines the module that contains **all** instruction handlers for your Anchor program.
- Each `pub fn` within the annotated module becomes a **transaction instruction**.
- Anchor auto-generates the **entrypoint**, dispatch logic, and IDL entries for these functions.

---

## Macro Expansion and Entrypoint Dispatch

### How It Works

Under the hood, `#[program]`:

1. Reads each public function in the module.
2. Generates a Solana BPF entrypoint that:
   - Deserializes instruction data.
   - Matches the instruction discriminator (first 8 bytes) to the correct handler.
   - Invokes your handler with `Context<YourAccounts>` and arguments.
3. Emits IDL definitions for front-ends and clients.

```rust
// Before macro expansion
#[program]
mod hello_anchor {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        ctx.accounts.new_account.data = data;
        Ok(())
    }
}

// After expansion (simplified pseudo-code)
entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (discriminator, rest) = instruction_data.split_at(8);
    match discriminator {
        [0,1,2,3,4,5,6,7] => {
            // dispatcher to initialize()
            let context = Context::new(program_id, accounts);
            let data: u64 = borsh::try_from_slice(rest)?;
            initialize(context, data)
        }
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
```

---

## Usage Patterns

### Single-Instruction Program

```rust
#[program]
mod counter {
    use super::*;
    pub fn initialize(ctx: Context<Init>, start: u64) -> Result<()> {
        ctx.accounts.counter.count = start;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub counter: Account<'info, Counter>,
    #[account(mut)] pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Counter { pub count: u64 }
```

### Multi-Instruction Program

```rust
#[program]
mod program_module {
    use super::*;
    pub fn foo(ctx: Context<FooCtx>, x: u32) -> Result<()> { /*...*/ Ok(()) }
    pub fn bar(ctx: Context<BarCtx>, y: bool) -> Result<()> { /*...*/ Ok(()) }
}

#[derive(Accounts)]
pub struct FooCtx<'info> { /* fields */ }

#[derive(Accounts)]
pub struct BarCtx<'info> { /* fields */ }
```

### Nested Modules

You can nest modules, but only the outer module annotated with `#[program]` is exposed as the program entrypoint.

```rust
#[program]
mod outer {
    pub mod inner {
        use super::*;
        pub fn baz(ctx: Context<BazCtx>) -> Result<()> { /*...*/ Ok(()) }
    }
}

#[derive(Accounts)]
pub struct BazCtx<'info> { /* ... */ }
```

---

## Comparison: Anchor vs Manual Solana Program

| Aspect                  | Anchor `#[program]`                         | Manual Solana (Rust BPF)            |
|-------------------------|----------------------------------------------|-------------------------------------|
| Entrypoint boilerplate  | Auto-generated                               | Hand-written with `entrypoint!`     |
| Instruction dispatch    | Macro-derived with discriminator matching    | Manual match on discriminator bytes |
| Account validation      | Via `#[derive(Accounts)]`                    | Manual deserialization & checks     |
| IDL generation          | Auto-published in `target/idl/*.json`        | None                                |
| Error handling          | Standardized `anchor_lang::error` macros     | Custom `ProgramError` conversions   |

**Pros/Cons**

| Pros                                                         | Cons                                                    |
|--------------------------------------------------------------|---------------------------------------------------------|
| Eliminates repetitive boilerplate                            | Must learn Anchor macros and conventions                |
| Seamless IDL/client generation for TypeScript & Rust clients | Slight compile-time overhead                            |
| Built-in account validation, PDAs, and error definitions     | Less visibility into the raw entrypoint dispatch logic  |
| Integration with testing framework and local validator CLI   | Limited to Rust; no native C++/Go runtime for on-chain  |

---

## Performance & Complexity (O())

- **Dispatch Overhead**: O(N) in number of instructions (match arms), typically N ≤ 20 → negligible.
- **Account Validation**: O(M) in number of accounts per instruction, M ≤ 10 → trivial relative to I/O.
- **Serialization**: O(S) in size of instruction arguments, S bytes → Borsh costs linear time.

Overall, the Anchor dispatch adds **constant-small** overhead compared to manual BPF.

---

## Tricky Parts & Edge Cases

- **Instruction Name Collisions**: Two functions with the same signature in nested modules may collide on discriminator. Use `#[discriminator(...)]` to override.
- **Optional Accounts**: Fields of type `Option<Account<'info, T>>` require careful checks; Anchor treats `None` as absence without error.
- **Remaining Accounts**: Use `ctx.remaining_accounts` when you need to pass accounts not declared in your `Accounts` struct.
- **Cross-Program Invocation (CPI)**: Ensure correct PDAs and seeds; `ctx.bumps` provides bump seeds.
- **Custom Discriminators**: Use `#[discriminator(0xCAFEBABE)]` above functions to set a fixed 8-byte ID.

---

## Calling Anchor Instructions in Other Languages

### TypeScript (Anchor Client)

```ts
import * as anchor from "@coral-xyz/anchor";
import idl from "./target/idl/my_program.json";

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = new anchor.Program(idl, PROGRAM_ID, provider);
await program.methods
  .foo(123)
  .accounts({ myAccount: pubkey, user: provider.wallet.publicKey })
  .rpc();
```

### Go (Using Solana Go SDK + Manual Discriminator)

```go
// Pseudo-code: load IDL, compute instruction data with Borsh
data := append(discriminator("foo"), borshMarshal(uint32(123))...)
instr := types.Instruction{
    ProgramID: programID,
    Accounts:  []types.AccountMeta{/*...*/},
    Data:      data,
}
tx, _ := solana.NewTransaction(..., instr)
```

### C++ (Hypothetical)

```cpp
// Compute 8-byte discriminator of "bar"
uint8_t disc[8] = { /* hash("global:bar")[..8] */ };
std::vector<uint8_t> data(disc, disc+8);
// append serialized args...
TransactionInstruction instr(programId, accountMetas, data);
```

---

## Best Practices

- **One Module per Program**: Avoid multiple `#[program]` on different modules.
- **Explicit Discriminators**: When refactoring, lock discriminators with `#[discriminator]`.
- **Limit Accounts**: Keep `Accounts` structs small; use `remaining_accounts` sparingly.
- **Test Each Instruction**: Use the Anchor test suite to run local validator checks.
- **Document IDL**: Keep your IDL up to date for front-end and multi-language clients.

---

Next Steps Suggestion:  
Explore **Cross-Program Invocation (CPI) Patterns in Anchor** to build modular on-chain composable protocols.