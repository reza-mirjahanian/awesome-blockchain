# Program Derived Address


Program Derived Addresses (PDA) refer to a feature of Solana development that allows you to create a unique address derived deterministically from pre-defined inputs (seeds) and a program ID.


## [Anchor PDA Constraints](#anchor-pda-constraints)

When using PDAs in an Anchor program, you generally use Anchor's account constraints to define the seeds to derive the PDA. These constraints serve as security checks to ensure that the correct address is derived.

The constraints used to define the PDA seeds include:

* `seeds`: An array of optional seeds used to derive the PDA. Seeds can be static values or dynamic references to account data.
* `bump`: The bump seed used to derive the PDA. Used to ensure the address falls off the Ed25519 curve and is a valid PDA.
* `seeds::program` - (Optional) The program ID used to derive the PDA address. This constraint is only used to derive a PDA where the program ID is not the current program.

The `seeds` and `bump` constraints are required to be used together.

### [Usage Examples](#usage-examples)

Below are examples demonstrating how to use PDA constraints in an Anchor program.



The `seeds` constraint specifies the optional values used to derive the PDA.

#### [No Optional Seeds](#no-optional-seeds)

* Use an empty array `[]` to define a PDA without optional seeds.

```
#[derive(Accounts)]
pub struct InstructionAccounts<'info> {
    #[account(

        seeds = [],
        bump,
    )]
    pub pda_account: SystemAccount<'info>,
}
```

#### [Single Static Seed](#single-static-seed)

* Specify optional seeds in the `seeds` constraint.

```
#[derive(Accounts)]
pub struct InstructionAccounts<'info> {
    #[account(


        seeds = [b"hello_world"],
        bump,
    )]
    pub pda_account: SystemAccount<'info>,
}
```

#### [Multiple Seeds and Account References](#multiple-seeds-and-account-references)

* Multiple seeds can be specified in the `seeds` constraint. The `seeds` constraint can also reference other account addresses or account data.

```
#[derive(Accounts)]
pub struct InstructionAccounts<'info> {
    pub signer: Signer<'info>,
    #[account(

        seeds = [b"hello_world", signer.key().as_ref()],
        bump,
    )]
    pub pda_account: SystemAccount<'info>,
}
```

The example above uses both a static seed (`b"hello_world"`) and a dynamic seed (the signer's public key).

## [PDA seeds in the IDL](#pda-seeds-in-the-idl)

Program Derived Address (PDA) seeds defined in the `seeds` constraint are included in the program's IDL file. This allows the Anchor client to automatically resolve account addresses using these seeds when constructing instructions.

This example below shows the relationship between the program, IDL, and client.


The program below defines a `pda_account` using a static seed (`b"hello_world"`) and the signer's public key as a dynamic seed.

```
use anchor_lang::prelude::*;
 
declare_id!("BZLiJ62bzRryYp9mRobz47uA66WDgtfTXhhgM25tJyx5");
 
#[program]
mod hello_anchor {
    use super::*;
    pub fn test_instruction(ctx: Context<InstructionAccounts>) -> Result<()> {
        msg!("PDA: {}", ctx.accounts.pda_account.key());
        Ok(())
    }
}
 
#[derive(Accounts)]
pub struct InstructionAccounts<'info> {

    pub signer: Signer<'info>,
    #[account(

        seeds = [b"hello_world", signer.key().as_ref()],
        bump,
    )]
```