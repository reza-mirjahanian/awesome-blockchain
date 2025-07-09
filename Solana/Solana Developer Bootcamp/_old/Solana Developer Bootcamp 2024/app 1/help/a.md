# Building Your First Solana Smart Contract: A Step-by-Step Guide

This document breaks down the process of creating a simple Solana smart contract using Anchor and Solana Playground.

## Introduction

This tutorial demonstrates building a smart contract to store favorite things on the Solana blockchain.  It covers saving, updating, retrieving data, and using digital signatures for access control.

## Concepts

* **Key Pairs:**  A public key (address) and a private key (for signing transactions).
* **Program Derived Addresses (PDAs):** Addresses derived from seeds (programmer-defined values) rather than public keys, functioning as a key-value store.  Seeds act as the primary key for data retrieval.

## Building the App

### Setting up Solana Playground

1. Open **beta.solpg.io**.
2. Click "Create a new project."
3. Name the project (e.g., "favorites").
4. Select the "Anchor" option.
5. Click "Create."
6. Delete the existing content in `lib.rs`.

### Coding the Smart Contract

1. **Import Anchor Prelude:**
   ```rust
   use anchor_lang::prelude::*;
   ```

2. **Define Program ID:** (Handled automatically in Solana Playground)

3. **Define Anchor Discriminator Size:**
   ```rust
   const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;
   ```

4. **Create the `favorites` Module:**

   ```rust
   #[program]   pub mod favorites {
       use super::*;

       pub fn set_favorites(ctx: Context<SetFavorites>, number: u64, color: String, hobbies: Vec<String>) -> Result<()> {
           // Functionality will be added here
       }
   }
   ```

5. **Define the `Favorites` Struct:**
   ```rust
   #[account(init_if_needed, payer = user, space = 8 + Favorites::INIT_SPACE, seeds = [b"favorites", user.key().as_ref()], bump)]
   pub favorites: Account<'info, Favorites>,

   #[derive(Accounts)]
   pub struct SetFavorites<'info> {
       #[account(mut, signer)]
       pub user: AccountInfo<'info>,


       pub system_program: Program<'info, System>,
   }

   #[derive(InitSpace)]   pub struct Favorites {
       pub number: u64,
       pub color: String,
       pub hobbies: Vec<String>,
   }


   ```

6. **Implement the `set_favorites` Function:**

   ```rust
   pub fn set_favorites(ctx: Context<SetFavorites>, number: u64, color: String, hobbies: Vec<String>) -> Result<()> {
       msg!("Greetings from {}", ctx.program_id);
       let user_public_key = ctx.accounts.user.key();
       msg!("User: {}", user_public_key);
       msg!("Favorite number is {}, favorite color is {}, and hobbies are {:?}", number, color, hobbies);

       ctx.accounts.favorites.set_inner(Favorites {
           number,
           color,
           hobbies,
       });

       Ok(())
   }   ```

### Building and Deploying

1. Click the "Build" button in Solana Playground.
2. Resolve any compilation errors.
3. Click the "Deploy" button.
4. Request an airdrop of SOL if needed using **faucets.solana.com**.

### Testing

1. Navigate to the "Test" tab in Solana Playground.
2. Input favorite number, color, and hobbies.
3. Select accounts:
    * User: Current wallet
    * Favorites: From seed (seeds: "favorites", current wallet)
    * Program: Automatically populated
4. Click "Generate."
5. Click "Test."
6. View the transaction on Solana Explorer.

## Security Considerations

* The provided code enforces that users can only modify their own favorites by linking the user's public key to the favorites account via seeds.  Attempting to write to another user's favorites account will result in a "seeds constraint was violated" error.

## Next Steps

The tutorial concludes by suggesting installing Solana locally and building more complex programs.