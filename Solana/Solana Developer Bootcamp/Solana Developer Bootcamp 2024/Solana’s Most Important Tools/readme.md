# **Your A-Z Guide to Solana's Most Important Tools** 

Must use tools:
Create-Solana-Dapp: https://github.com/solana-developers/...
BetterCallSol: https://bettercallsol.dev/
Solana Playground: https://beta.solpg.io/
Explorers: https://solana.fm/ https://solscan.io/ 
Anchor Lang: https://www.anchor-lang.com/
Metaboss: https://metaboss.rs/

Solana CLI: https://docs.solana.com/cli/install-s...
SPL-token CLI: https://spl.solana.com/token
Solana Program Library: https://spl.solana.com/

Solathon: https://github.com/SuperteamDAO/solathon
Solana-py: https://michaelhly.com/solana-py/
Solnet: https://github.com/bmresearch/Solnet
Sol4k: https://github.com/sol4k/sol4k
Solana Go: https://github.com/gagliardetto/solan...
Solana Unity: https://github.com/magicblock-labs/So...

Wallet Adapter: https://github.com/solana-labs/wallet...
Mobile Wallet adapter: https://github.com/solana-mobile/mobi...
Solana Pay: https://docs.solanapay.com/
Seed vault SDK: https://github.com/solana-mobile/seed...

Seahorse: https://seahorse-lang.org/
CandyMachine: https://docs.metaplex.com/programs/ca...
Sugar: https://github.com/metaplex-foundatio...
Metaboss: https://metaboss.rs/

Switchboard: https://switchboard.xyz/
Pyth: https://pyth.network/
Openbook: https://openbookdex.com/
Umi: https://github.com/metaplex-foundatio...
Clockwork: https://clockwork.xyz/
Amman: https://github.com/metaplex-foundatio...

Shank: https://github.com/metaplex-foundatio...
Solita: https://github.com/metaplex-foundatio...
Oxylana: https://github.com/cavemanloverboy/ox...
Bokken: https://github.com/Blade-Labs-Corp/bo...
Sealevel attacks: https://github.com/coral-xyz/sealevel...
Security practices: https://www.soldev.app/course/securit...

*Every tool explained!*

---

## **Introduction**  
- As a developer, **knowing what tools to use** is half the battle.  
- The other half is figuring out how to use them.  
- This guide provides a **high-level overview** of every major tool on Solana:  
  - Front-end development  
  - Smart contract/program development  
  - NFT-related tools  

---

## **Playground and Explorers**  
### **Playground**  
- **What it is:** A browser-based development environment.  
- **Purpose:** Build, test, and deploy programs on the Solana blockchain.  
- **Features:**  
  - Templates  
  - Tutorials for quick starts  

### **Explorers**  
- **Official Explorer:** Functional but lacks features and user-friendliness.  
- **Alternatives:**  
  - **Solscan**  
  - **Solana FM**  

---

## **Command Line Interfaces (CLIs)**  
### **Solana CLI**  
- **Purpose:** Comprehensive tool for interacting with the Solana blockchain.  
- **Capabilities:**  
  - Send transactions  
  - Communicate with validators  
  - View data  

### **SPL Token CLI**  
- **Purpose:** Interact with the token program.  
- **Use Cases:**  
  - Launch tokens  
  - Mint tokens  
  - Transfer tokens  

### **Solana Test Validator**  
- **Purpose:** Set up a local Solana network on your machine.  
- **Advantages:**  
  - No transaction fees  
  - Fast testing  
- **Compatibility Issues:**  
  - Not well-supported on Windows  
  - Recommended: Use **MacOS**, **Linux**, or **WSL (Windows Subsystem for Linux)**  

---

## **Solana Program Library (SPL)**  
- **What it is:** A collection of commonly used programs (e.g., token and swap programs).  
- **Benefits:**  
  - Saves time by providing pre-deployed and maintained programs.  
  - Refer to the **Solana Program Library documentation** for usage.  

---

## **SDKs (Software Development Kits)**  
### **Solana Web3.js**  
- **What it is:** The most popular JavaScript SDK.  
- **Purpose:** Write JavaScript applications to interact with Solana.  
- **Use Cases:**  
  - Node.js scripts  
  - React apps  

### **Other SDKs**  
- **Solana.py:** Python SDK (Community effort).  
- **Solana:** Another Python SDK.  
- **SolGo:** Go SDK.  
- **Sol4k:** C SDK.  
- **Unity SDK:** For Unity-based applications.  

---

## **Create Solana dApp**  
- **What it is:** A CLI tool for creating Solana apps.  
- **Supported Frameworks:**  
  - React.js  
  - Next.js  
  - Vue.js (upcoming)  
  - React Native (upcoming)  
- **Anchor Projects:** Supported for creating apps.  

---

## **Wallet Adapters**  
### **Wallet Adapter**  
- **Purpose:** Libraries for interacting with Solana wallets.  
- **Features:**  
  - Add wallet support to your app.  
  - Send transactions.  

### **Mobile Wallet Adapter**  
- **Purpose:** Wallet adapter for mobile apps.  
- **Use Cases:**  
  - React Native apps  
  - Progressive Web Apps (PWAs)  

### **Seed Vault SDK**  
- **Purpose:** Interact with the Seed Vault on Solana Saga phones.  
- **Target Audience:** Wallet developers only.  

---

## **Solana Pay**  
- **What it is:** A protocol for creating and consuming transaction requests.  
- **Features:**  
  - QR code-based transactions.  
- **Use Case:** Popular for in-person commerce.  

---

## **Frameworks for Solana Programs**  
### **Anchor**  
- **What it is:** A framework for creating Solana programs.  
- **Purpose:** Simplifies program development by generating Rust code.  
- **Advantages:**  
  - Speeds up development.  
  - Reduces the need for extensive Rust coding.  

### **Seahorse**  
- **What it is:** A Python-based framework for Solana programs.  
- **Status:** In beta and less widely used.  

---

## **SPL Name Service**  
- **Purpose:** Work with Solana domains (e.g., sending money to `r.o`).  

---

## **Off-Chain Storage Solutions**  
### **RWeave and Shadow Drive**  
- **What they are:** Decentralized, permanent, and cost-effective storage solutions.  
- **Use Case:** Ideal for storing large NFT collections (e.g., 10,000 assets).  

---

## **NFT Management Tools**  
### **Sugar CLI**  
- **Purpose:** Manage NFT assets and JSON files.  

### **Candy Machine CLI**  
- **Purpose:** Deploy NFT collections.  

### **Metaboss**  
- **What it is:** A Swiss Army knife for NFTs.  
- **Capabilities:**  
  - Burn NFTs  
  - Airdrop NFTs  
  - Update metadata  
  - Decode metadata  

---

## **Testing Tools**  
### **Aman**  
- **What it is:** A wrapper around the Solana Test Validator.  
- **Purpose:** Test SDKs and apps using Metaplex.  
- **Features:**  
  - Forked Explorer for local testing.  

---

## **Gasless Transactions**  
### **Octane**  
- **What it is:** A gasless transaction relayer.  
- **Purpose:** Allow developers to pay gas fees on behalf of users.  

---

## **Anchor Space Calculator**  
- **Purpose:** Calculates the space (in bytes) required for fields in Anchor programs.  

---

## **Executable NFTs (xNFTs)**  
- **What they are:** NFTs with embedded code.  
- **Supporting Wallet:** Backpack Wallet.  

---

## **OpenBook**  
- **What it is:** An exchange for submitting trade orders.  
- **Use Case:** Build trading applications with access to multiple orders.  

---

## **Oracles**  
### **Switchboard and Pyth**  
- **What they are:** Oracles for fetching off-chain data (e.g., asset prices) on-chain.  
- **Use Case:** Reliable and secure data for trading and exchange applications.  

---

## **JavaScript Frameworks**  
### **Umi**  
- **What it is:** A Solana framework for creating JavaScript clients.  
- **Advantages:** Bundles common interfaces for easier development.  

---

## **Cron Job Engine**  
### **Clockwork**  
- **What it is:** A cron engine for Solana.  
- **Use Cases:**  
  - Schedule transactions.  
  - Automate repetitive tasks (e.g., daily tax withdrawals).  

---

## **IDL Tools**  
### **Shank and Solita**  
- **Purpose:** Generate and manage IDLs (Interface Definition Language).  
- **How They Work:**  
  - **Shank:** Annotates Rust programs to generate IDLs.  
  - **Solita:** Converts IDLs into TypeScript APIs.  

---

## **Rust-Based Tools**  
### **Oxyana**  
- **What it is:** A Rust-based framework for front-end, smart contract, and unit test development.  

### **Cargo Build BPF**  
- **What it is:** A command for compiling native Rust Solana programs.  

---

## **Browser-Based Tools**  
### **Better Call Sol**  
- **What it is:** A browser-based tool for defining and sending transactions.  
- **Use Case:** Interact with programs directly from a UI.  

---

## **Security and Data Structures**  
### **Sea-Level Attacks**  
- **What they are:** Common exploits in Solana's programming model.  
- **Importance:** Developers must understand these to build secure programs.  

### **Concurrent Merkle Trees**  
- **What they are:** Data structures for building apps with compression (e.g., compressed NFTs).  

---

## **Debugging Tools**  
### **Bakan**  
- **What it is:** A debugging app for native and Anchor Solana programs.




### Playground and Explorers

-   **[Solana Playground](https://playground.solana.com/)**: A browser-based development environment for building, testing, and deploying Solana programs.
-   **[Solana Explorer](https://explorer.solana.com/)**: Official Solana blockchain explorer.
-   **[SolScan](https://solscan.io/)**: An alternative explorer with additional features/better UX.
-   **[Solana.FM](https://solana.fm/)**: Another user-friendly Solana blockchain explorer.

### Command Line Interfaces

-   **[Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)**: A comprehensive tool for interacting with the Solana blockchain.
-   **[SPL Token CLI](https://spl.solana.com/token)**: For managing SPL tokens.
-   **[Solana Test Validator](https://docs.solana.com/developing/test-validator)**: Bundled with Solana CLI for setting up a local network.

### Solana Program Library

-   **[Solana Program Library (SPL)](https://spl.solana.com/)**: Includes commonly used programs like the Token and Swap Programs.

### SDKs and Development Tools

-   **[@solana/web3.js](https://github.com/solana-labs/solana-web3.js)**: JavaScript SDK for interacting with the Solana blockchain.
-   **[Solathon](https://github.com/SuperteamDAO/solathon)**: Python SDK for Solana.
-   **[Solana Py](https://github.com/michaelhly/solana-py)**: Another Python SDK.
-   **[SolNet](https://github.com/bmresearch/Solnet)**: .NET SDK.
-   **[Solana Go](https://github.com/gagliardetto/solana-go)**: Go SDK for Solana.
-   **[Sol4k](https://github.com/sol4k/sol4k)**: Kotlin/Java SDK.
-   **[Solana Unity](https://github.com/magicblock-labs/Solana.Unity-SDK)**: SDK for Unity.
-   **[Create-Solana-Dapp](https://github.com/solana-developers/create-solana-dapp)**: CLI tool for creating Solana apps in various frameworks.

### Wallet Adapters

-   **[Wallet Adapter](https://github.com/solana-labs/wallet-adapter)**: Libraries for interacting with Solana wallets in web applications.
-   **[Mobile Wallet Adapter](https://github.com/solana-mobile/mobile-wallet-adapter)**: For integrating Solana wallets in mobile applications.
-   **[Seed Vault SDK](https://github.com/solana-mobile/seed-vault-sdk)**: For intergrating the Seed Vault on the Saga into wallets.

### Frameworks for Solana Programs

-   **[Anchor](https://github.com/project-serum/anchor)**: A framework for Solana smart contract development.
-   **[Seahorse](https://github.com/coral-xyz/seahorse)**: A Python-based framework for developing Solana programs (currently in beta).

### Off-Chain Storage Solutions

-   **[Shadow Drive](https://shadowstorage.io/)**: Solana native decentralized storage solution.
-   **[Arweave](https://www.arweave.org/)**: A popular1 decentralized storage service.

### NFT Tools

-   **[Sugar](https://github.com/metaplex-foundation/sugar)**: A CLI tool for streamlined NFT creation and management.
-   **[CandyMachine](https://docs.metaplex.com/programs/candy-machine/overview)**: A CLI for creating and managing NFT drops on Solana.
-   **[Metaboss](https://metaboss.rs/)**: A versatile tool for managing NFTs on Solana, enabling tasks like burning, airdropping, and updating NFT metadata.

### Oracles

-   **[Switchboard](https://switchboard.xyz/)**: A decentralized oracle network, providing real-world data to smart contracts on various blockchains.
-   **[Pyth](https://pyth.network/)**: A specialized oracle solution for bringing high-fidelity, time-sensitive data to the blockchain.

### Miscellaneous Tools

-   **[Helpers](https://github.com/solana-developers/helpers)**: Helper functions - get custom errors, request airdrops, manage keypairs.
-   **[Openbook](https://openbookdex.com/)**: An innovative decentralized exchange platform offering transparent and efficient trading experiences.
-   **[Umi](https://github.com/metaplex-foundation/umi)**: A Solana framework designed for creating JavaScript clients, bundling common functionalities and interfaces for efficient development
-   **[Clockwork](https://clockwork.xyz/)**: A platform providing automated smart contract operations and services for blockchain ecosystems.
-   **[Amman](https://github.com/metaplex-foundation/amman)**: A testing tool for Solana, enhancing the Solana test validator with additional features for SDKs and app testing, including a local explorer.
-   **[Oxylana](https://github.com/cavemanloverboy/oxylana)**: A Rust-based framework for building scalable and secure applications on Solana.

### Program dev tools

-   **[Shank](https://github.com/metaplex-foundation/shank)**: A tool for Solana smart contract development, automating the generation of Interface Definition Languages (IDLs) from Rust code.
-   **[Solita](https://github.com/metaplex-foundation/solita)**: Complements Shank by converting IDLs into TS interfaces, facilitating the integration of Solana programs with front-end applications.
-   **[Bokken](https://github.com/Blade-Labs-Corp/bokken)**: Tool for testing and debugging smart contracts, enhancing security and reliability.

### Security

-   **[Sealevel attacks](https://github.com/coral-xyz/sealevel-attacks)**: A repository documenting potential attack vectors and vulnerabilities within the Sealevel runtime on Solana.
-   **[Security practices](https://www.soldev.app/course/security-intro)**: An introductory course on security best practices for Solana blockchain development.