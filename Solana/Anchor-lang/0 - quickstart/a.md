
--------------------------------------------------
1.  `#[account(mut)]`
--------------------------------------------------

What it means  
• “I am going to **write** to this account in this instruction (change its lamports or its data), so please pass it to the program as *writable*.”  
• At the Solana level that just toggles the `is_writable` bit in the instruction’s account-metadata list.

Why does `signer` need `mut`?  
– When you create `new_account`, rent-exempt lamports are debited from `signer`.  
– Lamports changing ⇒ the account must be writable ⇒ `#[account(mut)]`.

Why doesn’t `new_account` have a separate `mut`?  
– It’s already declared with `init`.  
– `init` implies **both** “this account is writable” **and** “it will be newly created”.  
  (You could still add `mut` explicitly, but it would be redundant.)

Nice mnemonic  
`init` = create + writable  
`mut`  = just writable

--------------------------------------------------
2.  `#[account(init, …)]`
--------------------------------------------------

What `init` does for you  

1. Checks the account **does not exist yet** (lamports == 0, owner == System Program).  
2. Calculates rent-exempt minimum and creates the account (`system_program::create_account`).  
3. Sets the account’s owner to **your program**.  
4. Zero-initialises the account’s data and writes the discriminator.  
5. Marks it writable (the implicit `mut` we talked about).

Required extra arguments  
• `payer = <some-account>` – who funds the rent.  
• `space = <bytes>` – how many bytes to allocate.  
(You can also add `seeds`, `bump`, `owner = xyz`, etc. for PDAs or custom owners.)

So in your snippet

```
#[account(init, payer = signer, space = 8 + 8)]
pub new_account: Account<'info, NewAccount>,
```

Anchor will:

1. Take lamports from `signer` (needs `signer` to be mutable).  
2. Create `new_account` with 16 bytes: 8 bytes for the Anchor discriminator + 8 bytes for the `u64 data` field.  
3. Make the program you’re writing the owner of that account.

--------------------------------------------------
Cheat-sheet
--------------------------------------------------
Attribute        | What Anchor does
-----------------|--------------------------------------------------------------
`init`           | Create account, mark writable, zero-init, require `payer` & `space`
`mut`            | Require the account to be passed as writable (already existing)
`signer` (type)  | Enforce that the caller signed the Tx; if lamports change ⇒ add `mut`
(no attr)        | Read-only account – cannot touch lamports or data

--------------------------------------------------
Wrap-up
--------------------------------------------------
• Use `mut` whenever the account *already exists* and you plan to change its data/lamports.  
• Use `init` when the account does *not* exist yet and you want Anchor to create it for you (it includes mutability by default).



----

The `#[derive(Accounts)]` macro is used to annotate a struct that specifies the accounts required for a particular instruction, where each field represents a separate **account**.

```rust
#[derive(Accounts)]
pub struct UpdateData<'info> {
    // Account must exist and be mutable (no init)
    #[account(mut)]
    pub existing_account: Account<'info, NewAccount>,
    
    // Create new account
    #[account(init, payer = signer, space = 8 + 8)]
    pub new_account: Account<'info, NewAccount>,
    
    // Signer who pays and may receive/send SOL
    #[account(mut)]
    pub signer: Signer<'info>,
    
    // System program doesn't need mut (just used for CPI)
    pub system_program: Program<'info, System>,
}
``` 


```rust
use anchor_lang::prelude::*;
 
// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("11111111111111111111111111111111");
 
#[program]
mod hello_anchor {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        ctx.accounts.new_account.data = data;
        msg!("Changed data to: {}!", data); // Message will show up in the tx logs
        Ok(())
    }
}
 
#[derive(Accounts)]
pub struct Initialize<'info> {
    // We must specify the space in order to initialize an account.
    // First 8 bytes are default account discriminator,
    // next 8 bytes come from NewAccount.data being type u64.
    // (u64 = 64 bits unsigned integer = 8 bytes)
    #[account(init, payer = signer, space = 8 + 8)]
    pub new_account: Account<'info, NewAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
 
#[account]
pub struct NewAccount {
    data: u64
}

``` 
