**Cross Program Invocations (CPI) in Solana**

---

## 1. Basics of CPI

* **Definition:** CPI allows one Solana program (caller) to invoke another program (callee) within a single transaction.
* **Key Components:**

  * `invoke` and `invoke_signed` functions
  * `Instruction` struct
  * `AccountInfo` slices

```rust
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
};
```

### 1.1 `invoke` vs. `invoke_signed`

| Function        | Use Case      | Signature                                                                                        |
| --------------- | ------------- | ------------------------------------------------------------------------------------------------ |
| `invoke`        | Basic CPI     | `invoke(ix: &Instruction, accounts: &[AccountInfo]) -> ProgramResult`                            |
| `invoke_signed` | CPI with PDAs | `invoke_signed(ix: &Instruction, accounts: &[AccountInfo], seeds: &[&[&[u8]]]) -> ProgramResult` |

---

## 2. Constructing an Instruction

1. **Program ID**: `Pubkey` of callee.
2. **Accounts**: List of `AccountMeta` (pubkey + is\_signer + is\_writable).
3. **Data**: Serialized instruction data.

```rust
let ix = Instruction {
    program_id: target_program_id,
    accounts: vec![
        AccountMeta::new(caller_account.key.clone(), true),
        AccountMeta::new_readonly(system_program::id(), false),
    ],
    data: MyInstruction::DoSomething.try_to_vec()?,
};
invoke(&ix, &[caller_account.clone(), system_program.clone()])?;
```

---

## 3. Edge Cases & Tricks

* **Max Account Limit**: \~20 accounts per CPI; split calls if exceeding.
* **Rent-Exemption**: Ensure accounts are rent-exempt before transferring lamports.
* **Account Mutability Errors**: Matching `is_writable` flag between `Instruction` and actual `AccountInfo`.
* **Signer Seeds Mismatch**: Wrong PDA seeds => `invoke_signed` failure.

```rust
// Edge: Dynamic account list
let mut metas = vec![];
for acc in dynamic_accounts.iter() {
    metas.push(AccountMeta::new(*acc.key, false));
}
let ix = Instruction { program_id, accounts: metas, data };
invoke(&ix, dynamic_accounts)?;
```

---

## 4. Comparisons with Similar Concepts

| **Aspect**     | **Solana CPI**                              | **Ethereum `CALL`**                    |
| -------------- | ------------------------------------------- | -------------------------------------- |
| Gas Model      | Prepaid transaction fees                    | Per-instruction gas                    |
| Account Access | Explicit `AccountMeta` list                 | Implicit via address access            |
| Signatures     | Caller signs once; PDAs via `invoke_signed` | Each call may require msg.sender check |
| Composability  | Native cross-program                        | Via `delegatecall` or `call`           |

---

## 5. Real-World Use Cases

### 5.1 SPL Token Transfer

```rust
let spl_ix = spl_token::instruction::transfer(
    &spl_token::id(),
    source.key,
    dest.key,
    authority.key,
    &[],
    amount,
)?;
invoke(&spl_ix, &[source.clone(), dest.clone(), authority.clone(), token_program.clone()])?;
```

### 5.2 Token Swap via Serum

* Build Serum `NewOrder` instruction
* CPI into `serum_dex` program

```rust
let data = serum_dex::instruction::NewOrderInstructionV3 { /* ... */ };
let ix = Instruction::new_with_bytes(
    serum_dex::id(),
    &data.pack(),
    accounts_meta,
);
invoke(&ix, &[/* all AccountInfos */])?;
```

---

## 6. Pros & Cons

| **Pros**                | **Cons**                            |
| ----------------------- | ----------------------------------- |
| Reuse existing programs | Increased transaction size          |
| Modular code design     | Slight performance overhead         |
| Secure boundary checks  | Complexity in PDA management        |
| Native composability    | Potential account limit bottlenecks |

---

## 7. Tricky Parts Explained

* **Account Order**: Must match between `Instruction.accounts` and passed `AccountInfo` slice.
* **PDA Lifetime**: Derived PDAs are deterministic; changing seed invalidates.
* **Rent & Allocation**: CPI cannot allocate new accounts; use system program.

```rust
// Allocating within CPI:
let allocate_ix = system_instruction::create_account(
    caller.key,
    new_account.key,
    rent_lamports,
    space,
    &program_id,
);
invoke(&allocate_ix, &[caller.clone(), new_account.clone(), system_program.clone()])?;
```

---

## 8. Data Tables

**Instruction Data Size Limits**

| **Field**        | **Max Size**       | **Notes**                     |
| ---------------- | ------------------ | ----------------------------- |
| Accounts Vec     | \~20 entries       | Trigger nested CPI otherwise  |
| Instruction Data | \~1232 bytes total | Max packet size minus headers |

---

## 9. Related Official Documentation

* [Solana Program Library](https://docs.solana.com/developing/on-chain-programs/overview)
* [CPI Details](https://docs.solana.com/developing/programming-model/cross-program-invocation)

---

**Next Steps:** *Advanced: Writing and optimizing cross-program invocations using Anchor’s CPI macros and account compression techniques*
