In Solana, there is no equivalent to `msg.sender`.

There is an equivalent to `tx.origin` but you should be aware that Solana transactions can have multiple signers, so we could think of it as having "multiple tx.origins".

To get the "`tx.origin`" address in Solana, you need to set it up by adding Signer account to the function context and pass the caller's account to it when calling the function.

Let's see an example of how we can access the transaction signer's address in Solana:

```rust
use anchor_lang::prelude::*;

declare_id!("Hf96fZsgq9R6Y1AHfyGbhi9EAmaQw2oks8NqakS6XVt1");

#[program]
pub mod day14 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let the_signer1: &mut Signer = &mut ctx.accounts.signer1;

        // Function logic....

        msg!("The signer1: {:?}", *the_signer1.key);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer1: Signer<'info>,
}

```

From the above code snippet, the `Signer<'info>` is used to verify that the `signer1` account in the `Initialize<'info>` account struct has signed the transaction.

In the `initialize` function, the `signer1` account is mutably referenced from the context and assigned to `the_signer1` variable.

Then lastly, we logged the `signer1`'s pubkey (address) using the `msg!` macro and passing in `*the_signer1.key` , which dereferences and access the `key` field or method on the actual value being pointed to by `the_signer1`.

Next is to write a test for the above program:

```rust
describe("Day14", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day14 as Program<Day14>;

  it("Is signed by a single signer", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().accounts({
      signer1: program.provider.publicKey
    }).rpc();

    console.log("The signer1: ", program.provider.publicKey.toBase58());
  });
});

```

In the test, we passed our wallet account as signer to the `signer1` account, then called the initialize function. Following that, we logged the wallet account on the console to verify its consistency with the one in our program.

**Exercise:** What did you notice from the outputs in **shell\_1** (commands terminal) and **shell\_3** (logs terminal) after running the test?