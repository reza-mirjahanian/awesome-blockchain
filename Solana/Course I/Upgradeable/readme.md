Solana programs do not have constructors
----------------------------------------
```
anchor deploy
```
![alt text](image.png)


Run `anchor deploy` again:
![alt text](image-1.png)
The program was deployed to the same address, but this time it was upgraded, not deployed.

The program id has not changed, the program got overwritten.

Solana programs are mutable by default
--------------------------------------
This might come as a shock to Ethereum developers where immutability is assumed.

What is the point of a program if the author can just change it? It is possible to make a Solana program immutable. The assumption is that the author will deploy the mutable version first, and after time goes by and no bugs are discovered, then they will redeploy it as immutable.

Functionally, this is no different than an administrator controlled proxy where the owner later forfeits ownership to the zero address. But arguably, the Solana model is a lot cleaner because a lot can go wrong with Ethereum proxies.

Another implication: **Solana does not have delegatecall, because it doesn't need it**.

The primary use of delegatecall in Solidity contracts is to be able to upgrade the functionality of a proxy contract by issuing delegatecalls to a new implementation contract. However, since the bytecode of a program in Solana can be upgraded, there is no need for delegatecalling to implementation contracts.

Yet another corollary: **Solana does not have immutable variables the way Solidity interprets them (variables that can only be set in the constructor)**.


Running tests without redeploying the program
---------------------------------------------


anchor by default redeploys the program, let's demonstrate how to run the tests without redeploying.

Alter the test to be the following:

```rust
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

import fs from 'fs'
let idl = JSON.parse(fs.readFileSync('target/idl/deployed.json', 'utf-8'))

describe("deployed", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  // Change this to be your programID
  const programID = "6p29sM4hEK8ZFT5AvsGJQG5nKUtHBKs13iVL6juo5Uqj";
  const program = new Program(idl, programID, anchor.getProvider());

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});

```

Before you run the test, we suggest clearing the terminal of the Solana logs and restarting the solana-test-validator.

Now, run the test with:

```bash
anchor test --skip-local-validator --skip-deploy
```

Now look at the logs terminal: