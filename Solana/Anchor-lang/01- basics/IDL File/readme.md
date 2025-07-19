# Program IDL File



An Interface Description Language (IDL) file for an Anchor program provides a **standardized JSON** file describing the program's instructions and accounts. This file simplifies the process of integrating your on-chain program with client applications.

Key Benefits of the IDL:

* Standardization: Provides a consistent format for describing the program's instructions and accounts
* Client Generation: Used to generate client code to interact with the program

The `anchor build` command generates an IDL file located at `/target/idl/<program-name>.json`.

The code snippets in the sections below highlight how the program, IDL, and client relate to each other.

## [Program Instructions](#program-instructions)

The `instructions` array in the IDL corresponds directly to the instructions defined in your program. It specifies the required accounts and parameters for each instruction.

ProgramIDLClient

The program below includes an `initialize` instruction, specifying the accounts and parameters it requires.

lib.rs

```
use anchor_lang::prelude::*;
 
declare_id!("BYFW1vhC1ohxwRbYoLbAWs86STa25i9sD5uEusVjTYNd");
 
#[program]
mod hello_anchor {
    use super::*;



    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        ctx.accounts.new_account.data = data;
        msg!("Changed data to: {}!", data);
        Ok(())
    }
}
 

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 8)]
    pub new_account: Account<'info, NewAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
 
#[account]
pub struct NewAccount {
    data: u64,
}
```

## [Program Accounts](#program-accounts)

The `accounts` array in the IDL corresponds to the structs in a program annotated with the `#[account]` attribute. These structs define the data stored on accounts created by the program.

ProgramIDLClient

The program below defines a `NewAccount` struct with a single `data` field of type `u64`.

lib.rs

```
use anchor_lang::prelude::*;
 
declare_id!("BYFW1vhC1ohxwRbYoLbAWs86STa25i9sD5uEusVjTYNd");
 
#[program]
mod hello_anchor {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        ctx.accounts.new_account.data = data;
        msg!("Changed data to: {}!", data);
        Ok(())
    }
}
 
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 8)]
    pub new_account: Account<'info, NewAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
 


#[account]
pub struct NewAccount {
    data: u64,
}
```

## [Discriminators](#discriminators)

Anchor assigns a unique 8 byte discriminator to each instruction and account type in a program. These discriminators serve as identifiers to distinguish between different instructions or account types.

The discriminator is generated using the first 8 bytes of the Sha256 hash of a prefix combined with the instruction or account name. As of Anchor v0.30, these discriminators are included in the IDL file.

Note that when working with Anchor, you typically won't need to interact directly with these discriminators. This section is primarily to provide context on how the discriminator is generated and used.

InstructionsAccounts

The instruction discriminator is used by the program to determine which specific instruction to execute when called.

When an Anchor program instruction is invoked, the discriminator is included as the first 8 bytes of the instruction data. This is done automatically by the Anchor client.

IDL

```
  "instructions": [
    {
      "name": "initialize",


      "discriminator": [175, 175, 109, 31, 13, 152, 155, 237],
       ...
    }
  ]
```

The discriminator for an instruction is the first 8 bytes of the Sha256 hash of the prefix `global` plus the instruction name.

For example:

```
sha256("global:initialize")
```

Hexadecimal output:

```
af af 6d 1f 0d 98 9b ed d4 6a 95 07 32 81 ad c2 1b b5 e0 e1 d7 73 b2 fb bd 7a b5 04 cd d4 aa 30
```

The first 8 bytes are used as the discriminator for the instruction.

```
af = 175
af = 175
6d = 109
1f = 31
0d = 13
98 = 152
9b = 155
ed = 237
```

You can find the implementation of the discriminator generation in the Anchor codebase [here](https://github.com/coral-xyz/anchor/blob/0e5285aecdf410fa0779b7cd09a47f235882c156/lang/syn/src/codegen/program/common.rs#L5-L19), for the [`gen_discriminator` method here](https://github.com/coral-xyz/anchor/blob/0e5285aecdf410fa0779b7cd09a47f235882c156/lang/syn/src/codegen/program/common.rs#L21-L24), which is used [here](https://github.com/coral-xyz/anchor/blob/0e5285aecdf410fa0779b7cd09a47f235882c156/lang/syn/src/codegen/program/instruction.rs#L33).