use anchor_lang::prelude::*;

declare_id!("7FBBdRPMWb7eE5pRdpi3GRCNjxwFNwv33BGz3PNHb4bt"); //This defines the program's public key (address) on Solana.

#[program]   // Defines the main module of your program, where all the instructions (functions) are implemented.
pub mod counter {
    use super::*; // Imports all items (functions, structs, traits, etc.) from the parent module into the current module.

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.bump = ctx.bumps.counter; // store bump seed in `Counter` account
        msg!("Counter account created! Current count: {}", counter.count);
        msg!("Counter bump: {}", counter.bump);
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter: {}", counter.count);
        counter.count = counter.count.checked_add(1).unwrap();
        msg!("Counter incremented! Current count: {}", counter.count);
        Ok(())
    }
}

#[derive(Accounts)] // used to validate accounts passed to your Solana program. It defines which accounts are required for a specific instruction and enforces constraints 
pub struct Initialize<'info> {  // This struct specifies the accounts needed for the initialize instruction.
    #[account(mut)]
    pub user: Signer<'info>, // 'Info ties the lifetime of account data (provided by Solana during transaction processing) to the Rust types.

    // Create and initialize `Counter` account using a PDA as the address
    #[account(
        init,
        seeds = [b"counter"], // optional seeds for pda
        bump,                 // bump seed for pda
        payer = user,
        space = 8 + Counter::INIT_SPACE
    )]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    // The address of the `Counter` account must be a PDA derived with the specified `seeds`
    #[account(
        mut,
        seeds = [b"counter"], // optional seeds for pda
        bump = counter.bump,  // bump seed for pda stored in `Counter` account
    )]
    pub counter: Account<'info, Counter>,
}

#[account]  //When you create an account (using the #[account] attribute), Anchor knows how to store and retrieve its state from Solana's blockchain.
#[derive(InitSpace)]  // space = 8 + 8 + 1 // discriminator + u64 + u8
pub struct Counter {
    pub count: u64, // 8 bytes
    pub bump: u8,   // 1 byte
}