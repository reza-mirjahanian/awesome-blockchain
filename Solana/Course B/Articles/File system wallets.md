### 👛 File System Wallets

* **File System Wallet**: *A type of digital wallet where cryptographic keys are stored directly on a user's device as a file. This provides the user with complete control over their keys, but also makes them fully responsible for their security.*

***

### 🛠️ Creating a Wallet

1.  **Generation**: *Use the Solana Command-Line Interface (CLI) to generate a new key pair.*
2.  **Command**: *Run the command `solana-keygen new --outfile ~/my-keypair.json` to create and save the key pair in a JSON file.*

***

### ⚙️ Using the Wallet

* **Signing Transactions**: *The wallet is used to sign transactions and interact with the Solana network.*
* **CLI Flag**: *When using the Solana CLI, specify your wallet file with the `--keypair` flag.*

***

### 🔒 Security

* **Private Key**: *It is crucial to keep your wallet file secure.* 🔑
* **Best Practices**: *Never share it with anyone and always keep a backup in a safe place.*

***

### ✨ Customization

* **Vanity Address**: *The Solana CLI allows for customization of your wallet address.*
* **Grind Subcommand**: *Use the `grind` subcommand with `solana-keygen` to create a wallet address that starts with specific characters.*