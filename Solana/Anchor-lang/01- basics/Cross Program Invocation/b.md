### **Overview of Cross-Program Invocations (CPI) in Solana**

Cross-Program Invocations (CPI) enable one Solana program to call instructions on another program during runtime, promoting composability. This is a core feature of Solana's programming model, allowing programs to interact without direct dependencies. CPI is handled at the runtime level via the Solana BPF (Berkeley Packet Filter) VM.

- **Key Mechanism**: CPI uses the `solana_program::program::invoke` or `invoke_signed` functions in Rust (or equivalents in other SDKs like Anchor). The invoking program prepares an instruction with accounts and data, then passes it to the runtime.
- **Account Handling**: Accounts are shared via `AccountInfo` structs. The invoked program receives a subset of accounts from the invoker.
- **Security Note**: CPI does not automatically transfer authority; programs must validate accounts and signatures explicitly.
- **Official Docs Reference**: Per Solana's official documentation (solana.com/docs/core/cpi), CPI is atomic within a transaction but can fail independently, rolling back changes if errors occur. Runtime enforces no reentrancy by default (programs can't call back into themselves directly).

#### **How to Perform a Basic CPI**
Use `invoke` for unsigned calls. The invoker must ensure the invoked program ID matches expectations.

**Code Snippet (Rust - Basic CPI)**:
```rust
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke,
    pubkey::Pubkey,
    system_instruction,
};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let payer = next_account_info(account_info_iter)?;
    let new_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?; // System program ID: solana_program::system_program::ID

    // CPI to create a new account via System Program
    invoke(
        &system_instruction::create_account(
            payer.key,
            new_account.key,
            1_000_000, // Lamports
            0,         // Space
            program_id,
        ),
        &[payer.clone(), new_account.clone(), system_program.clone()],
    )?;
    Ok(())
}
```

- **Use Case**: Creating a new account owned by the current program via System Program CPI.

#### **Signed CPI (invoke_signed)**
For invocations requiring signatures (e.g., PDA authority), use `invoke_signed` with seeds.

**Code Snippet (Rust - Signed CPI)**:
```rust
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::invoke_signed,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
    program_pack::Pack,
    spl_token::{self, state::Account as TokenAccount},
};

pub fn transfer_tokens(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    // Assume accounts: [source_token_acc, dest_token_acc, token_program, pda_authority]
    let source = &accounts[0];
    let dest = &accounts[1];
    let token_program = &accounts[2];
    let pda = &accounts[3]; // PDA that owns source

    let pda_seeds: &[&[u8]] = &[b"seed", program_id.as_ref()]; // Example seeds

    invoke_signed(
        &spl_token::instruction::transfer(
            token_program.key,
            source.key,
            dest.key,
            pda.key,
            &[],
            amount,
        )?,
        &[source.clone(), dest.clone(), pda.clone(), token_program.clone()],
        &[pda_seeds], // Sign with PDA seeds
    )?;
    Ok(())
}
```

- **Use Case**: Transferring SPL tokens using a PDA as authority via Token Program CPI.
- **Tip**: Always derive PDAs correctly to avoid signature failures (use `Pubkey::find_program_address`).

#### **Edge Cases and Code Snippets**
- **Edge Case: Insufficient Lamports**: CPI fails if the payer lacks funds. Handle with error checking.
  ```rust
  // Wrap in a result handler
  let result = invoke(...);
  if result.is_err() {
      // Custom error handling, e.g., return Err(MyProgramError::InsufficientFunds.into());
  }
  ```

- **Edge Case: Reentrancy Prevention**: Solana runtime prevents direct reentrancy (a program can't CPI into itself). Attempting it errors with `ProgramError::ReentrancyNotAllowed`.
  ```rust
  // This will fail
  invoke(&my_instruction, accounts, &[&[b"self_call"]])?; // Self-CPI attempt
  ```

- **Edge Case: Account Borrowing Conflicts**: Mutable accounts can't be borrowed multiple times. Use `try_borrow_mut_lamports` carefully.
  ```rust
  // Tricky: Ensure no double mutable borrow
  if let Ok(mut lamports) = payer.try_borrow_mut_lamports()? {
      **lamports -= 1000; // Deduct before CPI
  }
  invoke(...)?; // CPI that might also mutate payer
  ```

- **Edge Case: Cross-Program Data Passing**: Pass serialized data in instructions. Deserialize in the invoked program.
  ```rust
  // Invoker
  let data = vec![0u8; 8]; // Example serialized data
  invoke(&Instruction { program_id: *target_program.key, accounts: vec![], data }, accounts)?;

  // Invoked program
  let amount = u64::from_le_bytes(instruction_data.try_into().unwrap());
  ```

- **Edge Case: Invoking Non-Existent Program**: Runtime returns `ProgramError::InvalidAccountData` if program ID is invalid.

#### **Comparisons with Similar Concepts**
- **Solana CPI vs. Ethereum Contract Calls**: 
  - Solana: Asynchronous, high-throughput (parallel execution via Sealevel runtime); accounts are explicit and shared.
  - Ethereum: Synchronous calls (e.g., `call` or `delegatecall`); state is global, leading to reentrancy risks (mitigated by checks like `nonReentrant` in Solidity).
  - Key Diff: Solana CPI is cheaper (no gas for loops) but requires manual account management.

- **Solana CPI vs. Cosmos IBC**: CPI is intra-chain (within Solana), while IBC is inter-chain. CPI focuses on composability within a single runtime.

#### **Pros/Cons Table**
| Aspect          | Pros                                                                 | Cons                                                                 |
|-----------------|----------------------------------------------------------------------|----------------------------------------------------------------------|
| **Performance** | High throughput; parallelizable via Sealevel.                        | Overhead from account serialization/deserialization.                 |
| **Security**    | Isolated failures; no automatic reentrancy.                          | Risk of unauthorized access if accounts aren't validated.             |
| **Composability** | Enables modular programs (e.g., DeFi legos).                        | Complex error propagation across calls.                              |
| **Cost**        | Low compute units (CU) per CPI (~200-500 CU).                        | Cumulative CU can exceed transaction limits in deep call stacks.     |
| **Ease of Use** | Straightforward in Rust/Anchor; promotes reuse of programs like SPL. | Manual handling of accounts and signatures prone to bugs.            |

- **Official Data**: Per docs, max CPI depth is implicitly limited by 200,000 CU per transaction (solana.com/docs/core/fees).

#### **Tricky Parts and Explanations**
- **Account Validation**: Always check `is_writable`, `is_signer`, and `owner` in the invoked program to prevent exploits. Tricky: Invokers can pass unexpected accounts; validate strictly.
  - **Tip**: Use `assert_eq!(account.owner, &expected_owner)` to enforce ownership.
- **Error Propagation**: CPI errors bubble up as `ProgramError`. Catch with `?` operator, but custom errors require mapping (e.g., via `ProgramError::from`).
- **PDA Signing Pitfalls**: Seeds must match exactly for `invoke_signed`; mismatches cause `InvalidSeeds` error. Tricky in multi-seed scenarios—use `create_program_address` for validation.
- **Compute Budget Limits**: Deep CPIs can hit 200,000 CU limit. Trick: Optimize by minimizing data size and using `request_additional_compute_budget`.
- **Realloc Pitfalls**: If CPI reallocates an account, ensure space is rent-exempt (use `Rent::get()?.minimum_balance`).
- **Common Bug**: Forgetting to clone `AccountInfo` before passing to CPI, leading to borrow errors.

#### **Real-World Usage and Projects**
- **SPL Token Program**: Widely used for CPI in DeFi (e.g., Serum DEX invokes Token Program for transfers).
- **Raydium AMM**: Uses CPI to swap tokens via Token Program, demonstrating composability.
- **Mango Markets**: Leverages CPI for perpetual futures, invoking oracles and token mints.
- **Project Example**: In a lending protocol like Solend, CPI calls the Token Program to mint/burn cTokens.
- **Stats from Docs**: Over 80% of Solana transactions involve CPI (e.g., to System or Token programs), per runtime metrics.

#### **Data Tables**
**CPI-Related Error Codes (from Official Docs)**:
| Error Code                  | Description                                      | Common Cause                          |
|-----------------------------|--------------------------------------------------|---------------------------------------|
| `InvalidArgument`           | Bad instruction data or accounts.                | Mismatched account count.             |
| `InvalidAccountData`        | Account not program-owned or invalid.            | Wrong program ID in CPI.              |
| `AccountBorrowFailed`       | Mutable borrow conflict.                         | Multiple mut refs in call chain.      |
| `ReentrancyNotAllowed`      | Attempted self-CPI.                              | Recursive call to own program.        |
| `InsufficientFunds`         | Not enough lamports for operation.               | Payer account underfunded.            |

**Compute Unit Costs (Approximate, from Docs)**:
| Operation              | CU Cost          | Notes                              |
|------------------------|------------------|------------------------------------|
| Basic `invoke`         | 200-300          | Without data.                      |
| `invoke_signed`        | 300-500          | Includes seed validation.          |
| Token Transfer CPI     | 1,000-2,000      | Varies by account size.            |
| Deep CPI (3 levels)    | Up to 10,000     | Risk of exceeding tx limit.        |

- **Tip**: Monitor with `solana compute-budget` CLI for optimization.

**Next Steps**: Program Derived Addresses (PDAs) in Solana