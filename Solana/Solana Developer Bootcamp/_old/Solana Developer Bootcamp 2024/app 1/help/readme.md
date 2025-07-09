### **Application Overview**  
https://beta.solpg.io/

**Objective**: Build a Solana smart contract to store/update/retrieve users' "favorites" on the blockchain using **Program Derived Addresses (PDAs)** and enforce access control via digital signatures.  

---

### **Core Concepts**  
- **Key Pairs**:  
  - **Public Key**: Acts as a wallet address.  
  - **Private Key**: Used to sign transactions.  
- **Program Derived Addresses (PDAs)**:  
  - Generated from **seeds** (e.g., strings, wallet addresses) instead of public keys.  
  - Enable key-value storage (e.g., `config`, `user_address + "Titanic"`).  

---

### **Setup & Tools**  
- **Solana Playground**: Web-based IDE for Solana development.  
- **Anchor Framework**: Simplifies Solana program creation with Rust.  

#### **Project Setup**  
1. Create a new project at [beta.solpg.io](https://beta.solpg.io).  
2. Name: `favorites`.  
3. Select **Anchor** as the framework.  

---

### **Code Structure**  

#### **Imports & Constants**  
```rust  
use anchor_lang::prelude::*;  
const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;  
```  

#### **Program Module**  
```rust  
declare_id!("YourProgramID"); // Auto-filled by Solana Playground  
#[program]  
pub mod favorites {  
    use super::*;  
    // Instruction handlers here  
}  
```  

---

### **Data Structures**  

#### **Favorites Account**  
```rust  
#[account]  
#[derive(InitSpace)]  
pub struct Favorites {  
    pub favorite_number: u64,  
    #[max_len(50)]  
    pub favorite_color: String,  
    #[max_len(5, 50)] // 5 items max, each 50 chars  
    pub hobbies: Vec<String>,  
}  
```  

#### **SetFavorites Instruction Accounts**  
```rust  
#[derive(Accounts)]  
pub struct SetFavorites<'info> {  
    #[account(mut, signer)]  
    pub user: Signer<'info>,  
    #[account(  
        init_if_needed,  
        payer = user,  
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE,  
        seeds = [b"favorites", user.key().as_ref()],  
        bump  
    )]  
    pub favorites: Account<'info, Favorites>,  
    pub system_program: Program<'info, System>,  
}  
```  

---

### **Instruction Handler**  
```rust  
pub fn set_favorites(  
    ctx: Context<SetFavorites>,  
    number: u64,  
    color: String,  
    hobbies: Vec<String>,  
) -> Result<()> {  
    msg!("Greetings from {}", ctx.program_id);  
    let user_key = ctx.accounts.user.key();  
    msg!("User: {} | Number: {} | Color: {} | Hobbies: {:?}", user_key, number, color, hobbies);  
    ctx.accounts.favorites.set_inner(Favorites {  
        favorite_number: number,  
        favorite_color: color,  
        hobbies,  
    });  
    Ok(())  
}  
```  

---

### **Deployment & Testing**  

#### **Steps**  
1. **Build**: Click "Build" in Solana Playground.  
2. **Deploy**:  
   - Ensure sufficient SOL via [faucet](https://faucet.solana.com).  
   - Click "Deploy" in Solana Playground.  
3. **Test**:  
   - Use the **Test** tab to invoke `set_favorites`:  
     - **Inputs**:  
       - `number`: e.g., `7`  
       - `color`: e.g., `"red"`  
       - `hobbies`: e.g., `["skiing", "bike riding"]`  
     - **Accounts**:  
       - `user`: Current wallet (signer).  
       - `favorites`: Generated from seed `favorites` + user address.  

---

### **Security Features**  
- **Seed Constraints**: Ensures `favorites` account is derived from the user’s public key.  
- **Signer Validation**: Only the wallet owner can update their `favorites` (enforced via `#[account(signer)]`).  

---

### **Key Observations**  
- **Logs**: Visible in Solana Explorer for debugging.  
- **Storage Costs**: Users pay SOL to create accounts (space = discriminator + data size).  
- **Atomic Transactions**: Changes occur only if all instructions succeed.