# Solana Workshop Breakdown

## Solana Network Overview

* **Technical Advantages:**
    * Fast confirmation times (approximately 400 milliseconds).
    * Low transaction fees (around 5,000 lamports per signature).
* **Network Operation:**
    * A validator leader receives and packs transactions into blocks.
    * Blocks are propagated across the network (25,000+ validators) via Turbine.
    * Parallel transaction execution due to stateless design.
    * Proof of History enhances speed.

## Solana Programming Model

### Accounts

* Fundamental units on Solana (analogous to files in an operating system).
* 256-bit unique addresses.
* Hold SOL balance (in lamports).
* Store arbitrary data (raw bytes, paid for with rent).
* **Key Features:**
    * Permissionless crediting of SOL and reading of data.
    * Only owners can debit SOL or modify data.
* **Components:**
    * Key (public address).
    * Lamports (SOL balance).
    * Data (raw bytes).
    * Executable flag (true for programs, false for data accounts).
    * Owner (program controlling the account).

### Programs

* Equivalent to smart contracts on other blockchains.
* Stateless and execute instructions.
* Read and write data to other accounts, not themselves.
* Written primarily in Rust (also C/C++, with libraries for Python, etc.).
* **Key Features:**
    * Parallel execution.
    * Ownership determines data modification rights.
    * Cross-program invocation (CPI) for inter-program communication.

### Instructions

* Commands sent to programs.
* **Components:**
    * Program ID (target program).
    * Keys (involved accounts).
    * Data (raw bytes).

### Transactions

* Bundles of instructions.
* **Components:**
    * Instructions.
    * Recent blockhash (for deduplication).
    * Fee payer address.
    * Signers (accounts authorizing changes).
* Atomic execution (all or nothing).

### Transaction Lifecycle

1. Client builds instructions and transaction.
2. Transaction sent to RPC client.
3. RPC forwards to validators.
4. Validators execute instructions using Solana runtime.
5. Instructions call programs, which update accounts.

## Tokens on Solana

* Combination of three programs:
    * Token Program (creates mints).
    * Associated Token Account (ATA) Program (manages token ownership).
    * Metadata Program (stores token information).
* **Mint:** An account controlling token balance (like a government mint).
* **Associated Token Account (ATA):**  Holds tokens owned by a wallet.
* **Metadata Account:** Stores token metadata (name, image, symbol).

### Creating an SPL Token

1. Create a mint account.
2. Initialize the mint.
3. Create an ATA.
4. Mint tokens to the ATA.

## NFTs on Solana

* Specialized SPL tokens.
* **Key Properties:**
    * Zero decimal places.
    * Supply of one.
    * Customizable metadata.
* **Master Edition:** Stores NFT metadata.
* **Collection:** An NFT grouping related NFTs.