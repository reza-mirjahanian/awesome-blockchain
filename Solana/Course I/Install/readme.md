### Install the Solana cli

```bash
# install solana 
sh -c "$(curl -sSfL https://release.solana.com/v1.16.25/install)"
```
-----------

### Install Anchor
```bash
# install anchor
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force

avm install latest
avm use latest

```

### Initialize and build an Anchor Program
```bash
anchor init day1 # use day_1 if you have a mac
cd day1
anchor build
```
### Configure Solana to run on localhost
```bash
solana config set --url localhost
```

### Run the test validator node
Run the following command in a new shell, not in the Anchor project. But do not close the shell where you ran anchor build. This is running a local (test) Solana node instance on your machine:
```bash
# shell 2

solana-test-validator
```

### Ensure the program\_id is synced with the Anchor key
```bash
# shell 1

anchor keys sync
```

You may also need to airdrop yourself some local Sol by running `solana airdrop 100 {YOUR_WALLET_ADDRESS}` in the terminal. You can get your wallet address by running `solana address` in the command line.

----------

Hello World
-----------


```rust
use anchor_lang::prelude::*;

declare_id!("...");

#[program]
pub mod day_1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello, world!"); // **** NEW LINE HERE ****
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
```


```bash

```

```bash

```

```bash

```


```bash

```

