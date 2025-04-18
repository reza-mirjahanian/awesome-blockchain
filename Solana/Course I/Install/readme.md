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
### Do I need an initialize function?

No, that was autogenerated by the Anchor framework. You could name it whatever you like.


###  kill the local validator process and restart it with:
```bash
solana-test-validator --reset
```
### Run the test 
```bash
anchor test --skip-local-validator
```

### Find the log file by running
```bash
ls .anchor/program-logs/
```

Alternatively
```bash
solana logs
```

### Why do we run the test with --skip-local-validator?

When the tests run against a node, we will be able to query the node for state changes. If you are not able to get the node to run, it is okay to run `anchor test` without the `--skip-local-validator` flag. However, you will have a harder time developing and testing, so we recommend getting the local validator to work.

### change the Anchor version

```
avm install 0.29.0
avm use 0.29.0
```

### Error: Deploying program failed: Error processing Instruction 1: custom program error: 0x1

```
Error: Deploying program failed: Error processing Instruction 1: custom program error: 0x1
There was a problem deploying: Output { status: ExitStatus(unix_wait_status(256)), stdout: "", stderr: "" }.
```
If you get this error, your keys are not synced. Run `anchor keys sync`.


### Error: Your configured rpc port: 8899 is already in use

You ran `anchor test` without `--skip-local-validator` while the validator is running in the background. Either turn off the validator and run `anchor test` or run `anchor test --skip-local-validator` with the validator running. Skip local validator means skip the temporary one it creates for the project, not the one running in the background.



### Account has insufficient funds for spend

Run the command below to airdrop 100 SOL to your development address:

```
solana airdrop 100 J7t...zjK
```


### RPC request error: cluster version query failed

```bash
Error: RPC request error: cluster version query failed: error sending request for url (http://localhost:8899/): error trying to connect: tcp connect error: Connection refused (os error 61)
There was a problem deploying: Output { status: ExitStatus(unix_wait_status(256)), stdout: "", stderr: "" }.
```

This means the `solana-test-validator` is not running in the background. Run `solana-test-validator` in another shell.

### thread 'main' panicked at 'called `Option::unwrap()` on a `None` value'

```
thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', /Users/username/.cargo/git/checkouts/anchor-50c4b9c8b5e0501f/347c225/lang/syn/src/idl/file.rs:214:73
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

It's likely you didn't run `anchor build` yet.


### Error: target/idl/day\_1.json doesn't exist. Did you run `anchor build`?

Create a new project and name it day\_1 instead of day1. Anchor seems to silently insert underscores on some machines.

