Cross Program Invocations (CPI) in Solana refer to the ability of one on-chain program (smart contract) to call another on-chain program during the execution of a single transaction. This is a powerful feature that enables modular, reusable, and composable smart contract designs, allowing programs to delegate functionality or build on top of each other.

---

### What is Cross Program Invocation (CPI)?

- **Definition:** When a Solana program needs to execute instructions from another program, it performs a CPI.
- It happens during transaction runtime, where one program calls into another by invoking it with specific accounts and instruction data.
- The invoked program executes as a subroutine of the caller program within the same transaction.

---

### Why use CPI?

- **Code Reuse:** Instead of duplicating logic, a program can reuse functionality from existing programs.
- **Modularity:** Programs can be broken into smaller components.
- **Interoperability:** Programs can interact with and leverage others (e.g., token programs, NFTs, marketplaces).
- **Composability:** Complex applications can be built by combining simpler programs.

---

### How does CPI work?

When Program A wants to invoke Program B:

1. **Prepare the Instruction:** Program A constructs an `Instruction` with:
   - The program_id of Program B.
   - The accounts required by Program B.
   - The instruction data for Program B.

2. **Invoke:**
   - Program A calls `invoke` or `invoke_signed` (if using program-derived addresses) from the `solana_program::program` module.
   - This causes the runtime to execute Program B’s code as part of the current transaction.

3. **Execution Flow:**
   - Program B runs as a subroutine, performing its operations.
   - Control returns to Program A after Program B finishes.
   - Any state changes made by Program B are reflected on-chain.

---

### Key Points and Constraints:

- **Account Borrowing:**
  - Accounts passed to the CPI are borrowed; solana runtime enforces exclusive or shared access.
- **Signer Seeds:**
  - When invoking with `invoke_signed`, you provide signer seeds to sign for PDAs.
- **Instruction Limit:**
  - CPI calls add to compute units; deep invocation chains may hit compute budget limits.
- **Error Propagation:**
  - If Program B errors, it bubbles up and causes the whole transaction to fail unless caught within Program A.
- **Security:**
  - Proper validation of accounts and data must be done on both sides.
- **No Recursion:**
  - Programs cannot infinitely recurse CPI; max call stack is enforced by runtime.

---

### Example in Rust (simple CPI call)

```rust
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
    instruction::{AccountMeta, Instruction},
};

pub fn call_other_program(
    program_id: &Pubkey,             // Program A's id (caller)
    accounts: &[AccountInfo],        // Accounts passed to Program A
    other_program_id: Pubkey,        // Program B's id (callee)
) -> ProgramResult {
    // Prepare accounts for Program B
    let accounts_for_b = vec![
        AccountMeta::new(accounts[0].key.clone(), false),
        AccountMeta::new_readonly(accounts[1].key.clone(), true),
    ];

    // Prepare instruction data for Program B
    let data_for_b = vec![0, 1, 2, 3]; // example data

    let ix = Instruction {
        program_id: other_program_id,
        accounts: accounts_for_b,
        data: data_for_b,
    };

    // Call Program B
    invoke(&ix, accounts)?;

    Ok(())
}
```

---

### Useful Resources:

- [Solana Docs on Cross-Program Invocations](https://docs.solana.com/developing/runtime-facilities/calling-other-programs)
- [Anchor Framework: `invoke` and `invoke_signed`](https://book.anchor-lang.com/anchor_references/cross_program_invocations.html)
- Real-world examples: interacting with the SPL Token program, Metaplex NFT auctions, etc.

---

