# **Rust Client**

## **Installation**

Add the following dependency to your `Cargo.toml`:

```toml
anchor-client = "0.30.1"
```

## **Basic Usage**

### **Creating a Client Instance**

```rust
use anchor_client::{
    solana_sdk::signature::{Keypair, Signer},
    solana_sdk::commitment_config::CommitmentConfig,
    Client, Cluster,
};

let payer = Keypair::new();
let client = Client::new_with_options(
    Cluster::Localnet,
    &payer,
    CommitmentConfig::processed(),
);
```

## **Program Interaction**

### **Getting a Program Handle**

```rust
let program = client.program(program_id)?;
```

### **Reading Accounts**

```rust
let my_account: MyAccount = program.account(account_pubkey)?;
```

### **Sending Transactions**

```rust
let tx = program
    .request()
    .accounts(my_instruction::accounts::MyInstruction {
        account: account_pubkey,
        user: payer.pubkey(),
        system_program: system_program::ID,
    })
    .args(my_instruction::instruction::MyInstruction {
        data: 42,
    })
    .send()?;
```

## **Advanced Features**

### **Using Different Clusters**

- **Mainnet Beta**: `Cluster::Mainnet`
- **Devnet**: `Cluster::Devnet`
- **Testnet**: `Cluster::Testnet`
- **Localnet**: `Cluster::Localnet`
- **Custom**: `Cluster::Custom("https://api.custom.com".to_string())`

### **Transaction Options**

#### **With Additional Signers**
```rust
.signer(&another_keypair)
```

#### **With Custom Options**
```rust
.options(RequestOptions {
    commitment: Some(CommitmentConfig::confirmed()),
    ..Default::default()
})
```

### **Simulating Transactions**

```rust
let result = program
    .request()
    .accounts(accounts)
    .args(instruction_data)
    .simulate()?;
```

### **Getting Multiple Accounts**

```rust
let accounts: Vec<MyAccount> = program.accounts::<MyAccount>(vec![
    account1_pubkey,
    account2_pubkey,
])?;
```

## **Error Handling**

### **Anchor Errors**
```rust
use anchor_client::anchor_lang::error::Error;

match program.request().accounts(accounts).args(args).send() {
    Ok(signature) => println!("Transaction sent: {}", signature),
    Err(Error::AnchorError(e)) => println!("Anchor error: {:?}", e),
    Err(e) => println!("Other error: {:?}", e),
}
```

## **Working with PDAs**

### **Finding Program Derived Addresses**

```rust
use anchor_client::solana_sdk::pubkey::Pubkey;

let (pda, bump) = Pubkey::find_program_address(
    &[b"seed", user.pubkey().as_ref()],
    &program_id,
);
```

## **Event Subscription**

### **Listening to Program Events**

```rust
let (tx, rx) = std::sync::mpsc::channel();
let handle = program.on(move |_, event: MyEvent| {
    tx.send(event).unwrap();
});

// Process events
for event in rx {
    println!("Received event: {:?}", event);
}
```

## **Testing Utilities**

### **Creating Test Fixtures**

```rust
use anchor_client::solana_sdk::signature::Keypair;
use anchor_client::solana_sdk::signer::keypair::read_keypair_file;

let payer = read_keypair_file(&shellexpand::tilde("~/.config/solana/id.json"))
    .expect("Wallet keypair file not found");
```

### **Program Testing Example**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use anchor_client::solana_sdk::system_instruction;

    #[test]
    fn test_initialize() {
        let payer = Keypair::new();
        let client = Client::new(Cluster::Localnet, &payer);
        let program = client.program(program_id).unwrap();
        
        let account = Keypair::new();
        let tx = program
            .request()
            .accounts(initialize::accounts::Initialize {
                account: account.pubkey(),
                payer: payer.pubkey(),
                system_program: system_program::ID,
            })
            .args(initialize::instruction::Initialize { data: 100 })
            .signer(&account)
            .send()
            .unwrap();
            
        println!("Transaction: {}", tx);
    }
}
```

## **Utility Functions**

### **Getting Account Rent**

```rust
let rent = client.rpc().get_minimum_balance_for_rent_exemption(
    std::mem::size_of::<MyAccount>()
)?;
```

### **Checking Account Existence**

```rust
let account_exists = client
    .rpc()
    .get_account(&account_pubkey)
    .is_ok();
```

## **Configuration Options**

### **Custom RPC Client**

```rust
use anchor_client::solana_client::rpc_client::RpcClient;

let rpc_client = RpcClient::new_with_commitment(
    "https://api.mainnet-beta.solana.com".to_string(),
    CommitmentConfig::confirmed(),
);

let client = Client::new_with_rpc(
    rpc_client,
    &payer,
);
```

### **Request Builder Pattern**

The client uses a **builder pattern** for constructing requests:

1. **Start with** `program.request()`
2. **Add accounts** with `.accounts()`
3. **Add instruction data** with `.args()`
4. **Add additional signers** with `.signer()`
5. **Execute with** `.send()` or `.simulate()`