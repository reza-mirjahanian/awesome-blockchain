# **Solana Accounts: Complete Reference**

## **What is a Solana Account?**
- A **stateful entity** on the Solana blockchain.
- Stores data and/or tokens.
- Identified by a unique **public key** (32 bytes).
- Each account has:
  - **Owner**: The program that controls the account.
  - **Lamports**: The native token balance (SOL).
  - **Data**: Arbitrary binary data stored in the account.
  - **Executable Flag**: Indicates if the account holds executable code.

---

## **Types of Accounts**
### **1. System Accounts**
- Created by the **System Program**.
- Used for basic operations like transferring SOL.
- Example:
  ```rust
  let system_account = Pubkey::new_unique();
  ```

### **2. Program Accounts**
- Holds executable code (smart contracts).
- Marked with the **executable flag**.
- Cannot store data or tokens directly but can own other accounts.

### **3. Token Accounts**
- Managed by the **Token Program**.
- Used to store SPL tokens (fungible or non-fungible).
- Must follow specific rules:
  - Owned by the Token Program.
  - Associated with a single mint (token type).
  - Example:
    ```rust
    let token_account = get_associated_token_address(&owner_pubkey, &mint_pubkey);
    ```

### **4. Data Accounts**
- Stores arbitrary data for programs.
- Often used by custom programs to persist state.
- Example:
  ```rust
  let data_account = Keypair::new();
  ```

### **5. Rent-Exempt Accounts**
- Accounts must hold enough SOL to be **rent-exempt**.
- Prevents data from being purged due to inactivity.
- Minimum balance calculated as:
  ```rust
  let rent_exempt_balance = Rent::default().minimum_balance(data_len);
  ```

---

## **Account Ownership**
- Every account has an **owner program**.
- The owner determines how the account can be modified.
- Common owners:
  - **System Program**: Default owner for new accounts.
  - **Token Program**: Owner of token accounts.
  - **Custom Programs**: Owners of program-derived accounts.

---

## **Account Lifecycle**
### **1. Creation**
- Accounts are created using the **System Program**.
- Requires funding with at least the rent-exempt amount.
- Example:
  ```rust
  invoke(
      &system_instruction::create_account(
          &payer_pubkey,
          &new_account_pubkey,
          rent_exempt_balance,
          data_len,
          &program_id,
      ),
      &[payer_account, new_account],
  );
  ```

### **2. Modification**
- Only the **owner program** can modify an account.
- Modifications include:
  - Updating data.
  - Transferring lamports.
  - Changing ownership.

### **3. Deletion**
- Accounts can be deleted to reclaim rent-exempt balance.
- Example:
  ```rust
  invoke(
      &system_instruction::close_account(
          &account_to_close_pubkey,
          &recipient_pubkey,
          &owner_pubkey,
      ),
      &[account_to_close, recipient_account, owner_account],
  );
  ```

---

## **Key Properties of Accounts**
| **Property**         | **Description**                                                                 |
|-----------------------|---------------------------------------------------------------------------------|
| **Public Key**        | Unique identifier for the account.                                             |
| **Lamports**          | Balance of SOL in the account (in lamports, where 1 SOL = 1 billion lamports). |
| **Data**              | Binary data stored in the account (size defined at creation).                  |
| **Owner**             | Program that controls the account.                                             |
| **Executable Flag**   | Indicates if the account holds executable code.                                |
| **Rent Epoch**        | Tracks when rent was last paid (for non-rent-exempt accounts).                 |

---

## **Tips and Tricks**
### **1. Rent-Exemption**
- Always ensure accounts are **rent-exempt** to avoid data loss.
- Use the `Rent` API to calculate the minimum balance:
  ```rust
  let rent = Rent::default();
  let required_lamports = rent.minimum_balance(data_len);
  ```

### **2. Associated Token Accounts**
- Use **Associated Token Accounts (ATA)** for SPL tokens.
- ATAs are deterministic and simplify token management.
- Derive ATA address:
  ```rust
  let ata = get_associated_token_address(&owner_pubkey, &mint_pubkey);
  ```

### **3. Program-Derived Addresses (PDAs)**
- PDAs are accounts owned by programs without private keys.
- Useful for storing program-specific data securely.
- Derive PDA:
  ```rust
  let (pda, bump_seed) = Pubkey::find_program_address(&[b"seed"], &program_id);
  ```

### **4. Account Size**
- Define the size of an account at creation.
- Ensure sufficient space for future data updates.
- Example:
  ```rust
  let data_len = 1024; // 1 KB
  ```

### **5. Cross-Program Invocations**
- Programs can interact with accounts owned by other programs.
- Use `invoke` or `invoke_signed` for cross-program calls.
- Example:
  ```rust
  invoke_signed(
      &instruction,
      &[account1, account2],
      &[&[b"seed", &[bump_seed]]],
  );
  ```

### **6. Avoid Reinitialization**
- Reinitializing an account can lead to security vulnerabilities.
- Always check if the account is already initialized before modifying it.

### **7. Debugging Accounts**
- Use the Solana CLI to inspect accounts:
  ```bash
  solana account <PUBKEY>
  ```
- Check balances, data, and ownership.

---

## **Common Errors and Solutions**
| **Error**                              | **Cause**                                      | **Solution**                                   |
|----------------------------------------|------------------------------------------------|-----------------------------------------------|
| **Insufficient Funds**                 | Not enough lamports to create or modify.       | Fund the account with more SOL.               |
| **Invalid Account Owner**              | Wrong program tries to modify the account.     | Ensure the correct program owns the account.  |
| **Account Not Rent-Exempt**            | Account balance below rent-exempt threshold.   | Add more lamports to make it rent-exempt.     |
| **Account Already Initialized**        | Attempting to reinitialize an existing account.| Check initialization status before modifying.|
| **Data Too Large**                     | Account size exceeds allocated space.          | Allocate sufficient space during creation.    |

---

## **Code Examples**
### **Creating a New Account**
```rust
let rent = Rent::default();
let required_lamports = rent.minimum_balance(data_len);

let create_account_ix = system_instruction::create_account(
    &payer_pubkey,
    &new_account_pubkey,
    required_lamports,
    data_len as u64,
    &program_id,
);

invoke(&create_account_ix, &[payer_account, new_account])?;
```

### **Deriving a PDA**
```rust
let (pda, bump_seed) = Pubkey::find_program_address(&[b"my_seed"], &program_id);
msg!("PDA: {}", pda);
msg!("Bump Seed: {}", bump_seed);
```

### **Closing an Account**
```rust
let close_account_ix = system_instruction::close_account(
    &account_to_close_pubkey,
    &recipient_pubkey,
    &owner_pubkey,
);

invoke(&close_account_ix, &[account_to_close, recipient_account, owner_account])?;
```

---

This reference provides a comprehensive guide to working with Solana accounts, covering their types, lifecycle, properties, and best practices.