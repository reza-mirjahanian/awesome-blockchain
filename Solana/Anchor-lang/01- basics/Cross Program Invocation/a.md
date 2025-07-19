 ## **Core Concepts**

**Cross Program Invocations (CPIs)** enable Solana programs to call instructions on other programs, creating composable on-chain applications. When a program invokes another program's instruction, the runtime passes the call to the invoked program.

## **Fundamental Mechanics**

### **Invocation Process**
1. **Caller program** creates instruction data
2. **Runtime** validates signatures and accounts
3. **Callee program** executes with inherited privileges
4. **Result** propagates back through call stack

### **Key Components**

```rust
use solana_program::{
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction},
    program::invoke,
    program::invoke_signed,
    pubkey::Pubkey,
};
```

## **Basic CPI Implementation**

### **Simple Invoke**
```rust
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let source = next_account_info(account_info_iter)?;
    let destination = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;

    // Create transfer instruction
    let transfer_instruction = spl_token::instruction::transfer(
        token_program.key,
        source.key,
        destination.key,
        source.key,
        &[],
        100,
    )?;

    // Invoke the transfer
    invoke(
        &transfer_instruction,
        &[source.clone(), destination.clone(), token_program.clone()],
    )?;

    Ok(())
}
```

### **Invoke with Signer Seeds (PDA)**
```rust
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let pda_account = next_account_info(account_info_iter)?;
    let destination = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    let (pda, bump_seed) = Pubkey::find_program_address(
        &[b"vault", user.key.as_ref()],
        program_id,
    );

    // Create transfer instruction
    let transfer_ix = system_instruction::transfer(
        pda_account.key,
        destination.key,
        1_000_000, // lamports
    );

    // Invoke with PDA signer
    invoke_signed(
        &transfer_ix,
        &[pda_account.clone(), destination.clone(), system_program.clone()],
        &[&[b"vault", user.key.as_ref(), &[bump_seed]]],
    )?;

    Ok(())
}
```

## **Account Validation**

### **Required Checks Before CPI**
```rust
// Verify program ownership
if token_account.owner != &spl_token::id() {
    return Err(ProgramError::IncorrectProgramId);
}

// Verify account is writable if needed
if !destination.is_writable {
    return Err(ProgramError::InvalidAccountData);
}

// Verify signer if required
if !authority.is_signer {
    return Err(ProgramError::MissingRequiredSignature);
}
```

## **Advanced Patterns**

### **Factory Pattern with CPIs**
```rust
pub fn create_token_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let payer = next_account_info(account_info_iter)?;
    let token_account = next_account_info(account_info_iter)?;
    let mint = next_account_info(account_info_iter)?;
    let owner = next_account_info(account_info_iter)?;
    let rent = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;

    // Calculate space and rent
    let space = spl_token::state::Account::LEN;
    let rent_lamports = Rent::get()?.minimum_balance(space);

    // Create account
    invoke(
        &system_instruction::create_account(
            payer.key,
            token_account.key,
            rent_lamports,
            space as u64,
            token_program.key,
        ),
        &[payer.clone(), token_account.clone(), system_program.clone()],
    )?;

    // Initialize token account
    invoke(
        &spl_token::instruction::initialize_account(
            token_program.key,
            token_account.key,
            mint.key,
            owner.key,
        )?,
        &[
            token_account.clone(),
            mint.clone(),
            owner.clone(),
            rent.clone(),
            token_program.clone(),
        ],
    )?;

    Ok(())
}
```

### **Nested CPIs**
```rust
pub fn complex_operation(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    // First CPI: Create associated token account
    invoke(
        &associated_token_account_instruction,
        &[/* accounts */],
    )?;

    // Second CPI: Mint tokens
    invoke_signed(
        &mint_instruction,
        &[/* accounts */],
        &[&[/* signer seeds */]],
    )?;

    // Third CPI: Transfer tokens
    invoke(
        &transfer_instruction,
        &[/* accounts */],
    )?;

    Ok(())
}
```

## **CPI Depth & Compute Limits**

| **Limit Type** | **Value** | **Notes** |
|----------------|-----------|-----------|
| Max CPI Depth | 4 | Direct invocations only |
| Compute Units | 200,000 default | Can be increased to 1.4M |
| Stack Frame Size | 4KB | Per invocation |
| Instruction Size | 1232 bytes | Total instruction data |

### **Compute Unit Management**
```rust
use solana_program::compute_budget::ComputeBudgetInstruction;

// Request additional compute units
let compute_ix = ComputeBudgetInstruction::set_compute_unit_limit(400_000);

// Include in transaction
let tx = Transaction::new_signed_with_payer(
    &[compute_ix, your_instruction],
    Some(&payer.pubkey()),
    &[&payer],
    recent_blockhash,
);
```

## **Security Considerations**

### **Account Substitution Attack Prevention**
```rust
pub fn secure_cpi(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let token_program = next_account_info(account_info_iter)?;
    
    // CRITICAL: Always verify program IDs
    if token_program.key != &spl_token::id() {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Verify account ownership before CPI
    if token_account.owner != token_program.key {
        return Err(ProgramError::IllegalOwner);
    }

    // Safe to proceed with CPI
    invoke(&instruction, &accounts)?;
    
    Ok(())
}
```

### **Reentrancy Protection**
```rust
#[derive(BorshSerialize, BorshDeserialize)]
pub struct ProgramState {
    pub reentrancy_flag: bool,
}

pub fn protected_function(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let state_account = next_account_info(account_info_iter)?;
    let mut state = ProgramState::try_from_slice(&state_account.data.borrow())?;

    // Check reentrancy flag
    if state.reentrancy_flag {
        return Err(ProgramError::InvalidAccountData);
    }

    // Set flag
    state.reentrancy_flag = true;
    state.serialize(&mut &mut state_account.data.borrow_mut()[..])?;

    // Perform CPI
    invoke(&instruction, &accounts)?;

    // Clear flag
    state.reentrancy_flag = false;
    state.serialize(&mut &mut state_account.data.borrow_mut()[..])?;

    Ok(())
}
```

## **Common CPI Patterns**

### **Token Operations**
```rust
// Transfer SPL tokens
pub fn transfer_spl_tokens(
    amount: u64,
    source: &AccountInfo,
    destination: &AccountInfo,
    authority: &AccountInfo,
    token_program: &AccountInfo,
) -> ProgramResult {
    invoke(
        &spl_token::instruction::transfer(
            token_program.key,
            source.key,
            destination.key,
            authority.key,
            &[],
            amount,
        )?,
        &[source.clone(), destination.clone(), authority.clone(), token_program.clone()],
    )
}

// Burn tokens
pub fn burn_tokens(
    amount: u64,
    account: &AccountInfo,
    mint: &AccountInfo,
    authority: &AccountInfo,
    token_program: &AccountInfo,
) -> ProgramResult {
    invoke(
        &spl_token::instruction::burn(
            token_program.key,
            account.key,
            mint.key,
            authority.key,
            &[],
            amount,
        )?,
        &[account.clone(), mint.clone(), authority.clone(), token_program.clone()],
    )
}
```

### **Associated Token Account Creation**
```rust
pub fn create_ata(
    payer: &AccountInfo,
    wallet: &AccountInfo,
    mint: &AccountInfo,
    ata: &AccountInfo,
    system_program: &AccountInfo,
    token_program: &AccountInfo,
    ata_program: &AccountInfo,
) -> ProgramResult {
    invoke(
        &spl_associated_token_account::instruction::create_associated_token_account(
            payer.key,
            wallet.key,
            mint.key,
            token_program.key,
        ),
        &[
            payer.clone(),
            ata.clone(),
            wallet.clone(),
            mint.clone(),
            system_program.clone(),
            token_program.clone(),
            ata_program.clone(),
        ],
    )
}
```

## **Error Handling**

### **CPI Error Propagation**
```rust
pub fn handle_cpi_errors(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    match invoke(&instruction, &accounts) {
        Ok(_) => msg!("CPI succeeded"),
        Err(e) => {
            msg!("CPI failed: {:?}", e);
            return Err(e);
        }
    }

    // Custom error handling
    let result = invoke(&risky_instruction, &accounts);
    if let Err(error) = result {
        match error {
            ProgramError::InsufficientFunds => {
                msg!("Not enough funds, trying fallback");
                invoke(&fallback_instruction, &accounts)?;
            }
            _ => return Err(error),
        }
    }

    Ok(())
}
```

## **Testing CPIs**

### **Unit Test Setup**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use solana_program_test::*;
    use solana_sdk::{
        signature::{Keypair, Signer},
        transaction::Transaction,
    };

    #[tokio::test]
    async fn test_cpi() {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new(
            "my_program",
            program_id,
            processor!(process_instruction),
        );

        // Add external programs
        program_test.add_program(
            "spl_token",
            spl_token::id(),
            processor!(spl_token::processor::Processor::process),
        );

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Test CPI transaction
        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        banks_client.process_transaction(tx).await.unwrap();
    }
}
```

## **Performance Optimization**

### **Batch Operations**
```rust
pub fn batch_transfer(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    transfers: Vec<TransferParams>,
) -> ProgramResult {
    // Validate all accounts first
    for transfer in &transfers {
        validate_transfer_accounts(transfer)?;
    }

    // Execute all transfers
    for transfer in transfers {
        invoke(
            &spl_token::instruction::transfer(
                &spl_token::id(),
                &transfer.source,
                &transfer.destination,
                &transfer.authority,
                &[],
                transfer.amount,
            )?,
            &[/* accounts */],
        )?;
    }

    Ok(())
}
```

## **CPI vs Direct State Manipulation**

| **Aspect** | **CPI** | **Direct State** |
|------------|---------|------------------|
| **Security** | Program validates | Manual validation |
| **Composability** | High | None |
| **Gas Cost** | Higher | Lower |
| **Complexity** | Lower | Higher |
| **Maintenance** | Easier | Harder |

## **Real-World Examples**

### **DEX Swap Router**
```rust
pub fn swap_tokens(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount_in: u64,
    minimum_amount_out: u64,
) -> ProgramResult {
    // Get best route
    let route = calculate_best_route(amount_in)?;

    // Execute swaps through multiple DEXs
    for hop in route {
        match hop.dex_type {
            DexType::Raydium => {
                invoke(
                    &raydium_swap_instruction(hop.pool_id, hop.amount),
                    &[/* raydium accounts */],
                )?;
            }
            DexType::Orca => {
                invoke(
                    &orca_swap_instruction(hop.pool_id, hop.amount),
                    &[/* orca accounts */],
                )?;
            }
        }
    }

    // Verify minimum output
    let final_amount = get_token_balance(output_account)?;
    if final_amount < minimum_amount_out {
        return Err(ProgramError::Custom(1)); // Slippage
    }

    Ok(())
}
```

### **Lending Protocol Integration**
```rust
pub fn leveraged_position(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    collateral_amount: u64,
    leverage: u8,
) -> ProgramResult {
    // Deposit collateral
    invoke(
        &lending_protocol::deposit(collateral_amount),
        &[/* deposit accounts */],
    )?;

    let borrow_amount = collateral_amount * (leverage - 1) as u64;

    // Borrow against collateral
    invoke_signed(
        &lending_protocol::borrow(borrow_amount),
        &[/* borrow accounts */],
        &[&[/* PDA seeds */]],
    )?;

    // Swap borrowed assets back to collateral token
    invoke(
        &dex::swap(borrow_amount, collateral_token),
        &[/* swap accounts */],
    )?;

    // Re-deposit for compound leverage
    invoke(
        &lending_protocol::deposit(borrow_amount),
        &[/* deposit accounts */],
    )?;

    Ok(())
}
```

## **Debugging CPIs**

### **Logging Best Practices**
```rust
pub fn debug_cpi(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    msg!("Starting CPI to program: {}", target_program.key);
    msg!("Instruction data: {:?}", instruction_data);
    
    // Log account states before CPI
    for account in accounts.iter() {
        msg!("Account {}: lamports={}, owner={}", 
            account.key, 
            account.lamports(), 
            account.owner
        );
    }

    let result = invoke(&instruction, accounts);
    
    match &result {
        Ok(_) => msg!("CPI successful"),
        Err(e) => msg!("CPI failed with error: {:?}", e),
    }

    // Log account states after CPI
    for account in accounts.iter() {
        msg!("Post-CPI Account {}: lamports={}", 
            account.key, 
            account.lamports()
        );
    }

    result
}
```

## **Next Steps**

**Program Derived Addresses (PDAs) and CPI Signing**: Deep dive into advanced PDA patterns, including multi-level PDA hierarchies, canonical bump optimization, and complex signing scenarios with multiple PDAs in single CPIs. Understanding PDA-based program authority delegation and building secure escrow systems with deterministic addressing.