A Cross-Program Invocation (CPI) is a direct, synchronous call from one Solana program to another within the same transaction. This allows programs to compose with each other, creating a powerful, lego-like ecosystem. Instead of reimplementing common logic like token transfers, a program can simply invoke the official Token Program.

-----

### \#\# How CPIs Work

When a program makes a CPI, it temporarily halts its own execution and passes control to the callee program. The Solana Runtime enforces a strict set of rules during this process:

1.  **Instruction**: The caller program constructs an `Instruction` data structure, specifying the `program_id` of the program to be called, a list of `accounts` required by the callee, and a `data` byte array containing the instruction logic and arguments for the callee.
2.  **Invocation**: The caller uses one of two built-in functions to execute the call: `invoke` or `invoke_signed`.
3.  **Execution**: The Runtime receives the call, loads the callee program, and executes its instruction.
4.  **Return**: After the callee finishes, control returns to the caller program, which can then continue its execution.

Crucially, **a CPI does not create a new transaction**. It's a single instruction executed within the context of the parent transaction. This means all changes are atomic; if the CPI fails, the entire parent transaction fails.

-----

### \#\# `invoke` vs. `invoke_signed`

The choice between `invoke` and `invoke_signed` depends entirely on who needs to sign the instruction for the callee program.

| Feature          | `invoke`                                                              | `invoke_signed`                                                                      |
| ---------------- | --------------------------------------------------------------------- | ------------------------------------------------------------------------------------ |
| **Purpose** | To call another program when no Program Derived Address (PDA) needs to sign. | To call another program and have a PDA, owned by the *caller*, sign the instruction. |
| **Signers** | Requires the original transaction signer's `AccountInfo` to be passed through. | The caller program "signs" on behalf of its PDA using the PDA's seeds and bump.        |
| **Use Case** | A dApp calls Program A, which then needs to call Program B, but Program B requires the user's signature. | A staking program needs to call the Token Program to transfer an NFT from a user's vault (a PDA) back to the user. |
| **Key Function** | `solana_program::program::invoke`                                     | `solana_program::program::invoke_signed`                                             |

-----

### \#\# Code Snippets: Anchor vs. Native Rust

Anchor drastically simplifies CPIs by auto-generating client-side code for your program's instructions.

#### Anchor CPI Example

Let's imagine a `caller_program` wants to call a `callee_program`. The `callee_program` has a simple instruction `do_something` that requires a user's signature and writes to a `data_account`.

**Callee Program (`callee_program`)**

```rust
use anchor_lang::prelude::*;

declare_id!("CALEESfK5o21cMepg9sH1a3q4fH3z1jT8gW6p2qE4a");

#[program]
pub mod callee_program {
    use super::*;
    pub fn do_something(ctx: Context<DoSomething>, data: u64) -> Result<()> {
        ctx.accounts.data_account.data = data;
        msg!("Callee was called with data: {}", data);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DoSomething<'info> {
    #[account(mut)]
    pub data_account: Account<'info, DataAccount>,
    // The user must sign this instruction
    pub user: Signer<'info>,
}

#[account]
pub struct DataAccount {
    pub data: u64,
}
```

**Caller Program (using Anchor CPI)**
This program defines a PDA and uses it to sign the call to the `callee_program`.

```rust
use anchor_lang::prelude::*;
// Import the callee program to get access to its instruction builders
use callee_program::{self, cpi::accounts::DoSomething, program::CalleeProgram, DataAccount};

declare_id!("CALLERp1nkeri3fH1a3q4fH3z1jT8gW6p2qE4a");

#[program]
pub mod caller_program {
    use super::*;

    // Use `invoke` because the original signer is passed through
    pub fn proxy_call(ctx: Context<ProxyCall>, data_to_pass: u64) -> Result<()> {
        let cpi_program = ctx.accounts.callee_program.to_account_info();
        let cpi_accounts = DoSomething {
            data_account: ctx.accounts.data_account.to_account_info(),
            // Pass the user's signer privilege through
            user: ctx.accounts.user.to_account_info(),
        };
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        callee_program::cpi::do_something(cpi_context, data_to_pass)?;
        Ok(())
    }

    // Use `invoke_signed` because the PDA is signing
    pub fn pda_signed_call(ctx: Context<PdaSignedCall>, data_to_pass: u64) -> Result<()> {
        let seeds = b"my-pda-seed";
        let bump = ctx.bumps.pda_signer;
        let signer_seeds = &[&seeds[..], &[bump]];

        let cpi_program = ctx.accounts.callee_program.to_account_info();
        let cpi_accounts = DoSomething {
            data_account: ctx.accounts.data_account.to_account_info(),
            // The PDA is the signer now
            user: ctx.accounts.pda_signer.to_account_info(),
        };
        // We need `with_signer` for `invoke_signed`
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts)
            .with_signer(&[&signer_seeds[..]]);

        callee_program::cpi::do_something(cpi_context, data_to_pass)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ProxyCall<'info> {
    #[account(mut)]
    pub data_account: Account<'info, DataAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub callee_program: Program<'info, CalleeProgram>,
}

#[derive(Accounts)]
pub struct PdaSignedCall<'info> {
    #[account(mut)]
    pub data_account: Account<'info, DataAccount>,
    /// CHECK: This PDA will sign the CPI.
    #[account(
        seeds = [b"my-pda-seed"],
        bump,
    )]
    pub pda_signer: AccountInfo<'info>,
    pub callee_program: Program<'info, CalleeProgram>,
}
```

#### Native Rust CPI Example

Doing the same thing in native Rust is more verbose as you must manually construct the instruction and account lists.

```rust
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    msg,
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
};

// Assuming the same `callee_program` and its ID
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    // 0. Callee Program ID
    let callee_program_id_account = next_account_info(accounts_iter)?;
    // 1. Data account
    let data_account = next_account_info(accounts_iter)?;
    // 2. Signer (either a user or a PDA)
    let signer = next_account_info(accounts_iter)?;

    // Manually build the instruction for the `callee_program`
    let instruction = Instruction::new_with_borsh(
        *callee_program_id_account.key, // The program to call
        &instruction_data, // The data to pass (must be properly serialized!)
        vec![
            AccountMeta::new(*data_account.key, false),
            AccountMeta::new(*signer.key, true), // Mark the signer
        ],
    );

    // To use `invoke` (assuming `signer` is an external account)
    invoke(
        &instruction,
        &[
            data_account.clone(),
            signer.clone(),
            callee_program_id_account.clone(),
        ],
    )?;

    // To use `invoke_signed` (assuming `signer` is a PDA)
    let (pda_key, bump_seed) = Pubkey::find_program_address(&[b"my-pda-seed"], program_id);
    let pda_seeds = &[&b"my-pda-seed"[..], &[bump_seed]];

    invoke_signed(
        &instruction,
        &[
            data_account.clone(),
            signer.clone(),
            callee_program_id_account.clone(),
        ],
        &[&pda_seeds[..]],
    )?;

    Ok(())
}
```

-----

### \#\# Tricky Parts & Common Pitfalls

  - **Account Order Matters**: The slice of `AccountInfo`s you pass to `invoke` or `invoke_signed` **must** match the order the callee program expects them. A mismatch will cause an `InvalidAccountData` error.
  - **Privilege Escalation is Forbidden**: You cannot make an account writable or a signer in a CPI if it wasn't already marked as such in the original transaction. For example, if `account_x` is passed as read-only to your program, you cannot pass it as writable in a CPI.
  - **Reentrancy is Forbidden**: The runtime explicitly forbids a program from being in the call stack more than once. Program A calling Program B which then calls Program A is not allowed and will fail the transaction.
  - **Data Serialization**: The `data` field of the `Instruction` must be a byte array (`Vec<u8>`) that the callee program can correctly deserialize. In Anchor, this includes an 8-byte "discriminator" hash of the method name, followed by the serialized arguments. In native Rust, you're responsible for getting this encoding right.
  - **PDA Authority**: Only the program that owns a PDA (i.e., the program whose `program_id` was used to derive the address) can use `invoke_signed` for that PDA. Program A cannot create signer seeds for a PDA owned by Program B.
  - **Compute Budget**: Each CPI adds to the total compute units used by the transaction. Complex chains of CPIs can easily exceed the compute budget, causing the transaction to fail.

-----

### \#\# Real-World Usage

| Category         | Use Case                                                                                                    | Example Programs                                                                             |
| ---------------- | ----------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------- |
| **Token Operations** | A dApp needs to transfer SPL tokens on behalf of a user. It makes a CPI to the official **SPL Token Program**. | Candy Machine minting an NFT, DEXs swapping tokens, Staking protocols locking tokens.      |
| **DeFi Composability** | A yield aggregator (e.g., Tulip) calls a lending protocol (e.g., Solend) to deposit assets.              | The lending protocol might then call a DEX (e.g., Orca) to rebalance its pools via another CPI. |
| **NFT Management** | An NFT staking program makes a CPI to the **Token Metadata Program** to freeze the NFT, preventing its sale while staked. | Staking platforms like `Staking` on Metaplex.                                              |
| **Governance** | A DAO program (e.g., Squads) executes a successful proposal by making a CPI to a treasury program to transfer funds. | Any on-chain governance system that controls a treasury or program parameters.             |

-----

### \#\# Next Steps: Address Lookup Tables (ALTs)

Once you master CPIs, you'll quickly run into a major limitation: the **transaction size limit**. A standard transaction can only hold a limited number of account public keys. Complex CPIs, which require passing all necessary accounts for both the caller and the callee, can easily exceed this limit.

The next logical step is to learn about **Address Lookup Tables (ALTs)**. ALTs allow you to store a list of account public keys on-chain in a table. Instead of listing all 30, 40, or 50+ accounts in the transaction itself, you can just reference the on-chain ALT address, effectively compressing the transaction and enabling much more complex CPIs.