📜 Precompiled Contracts: The EVM's "Cheat Codes"
=================================================

### **What are Precompiled Contracts?**

-   **Specialized contracts** integrated directly into the Ethereum Virtual Machine (EVM).

-   They exist at **pre-defined, hardcoded addresses**.

-   Designed to perform computationally intensive operations **cheaply and efficiently**.

    -   *Example: Complex cryptographic calculations.*

-   They are implemented in the client software (e.g., Geth, Nethermind) rather than in EVM bytecode.

### **Key Characteristics**

-   **Fixed Addresses:** They occupy specific, low-numbered addresses (starting from `0x1`). These addresses are reserved and cannot be used by other contracts or accounts.

-   **Added via Hard Forks:** New precompiles are introduced during network upgrades (hard forks).

-   **Currently 9 Precompiles:** This number is expected to grow with future updates like the Cancun hard fork.

🔢 The 9 Precompiled Contracts
==============================

1.  **`0x1` - `ecrecover` (Elliptic Curve Recovery)** 🔑

    -   **Purpose:** Recovers the signer's address from an ECDSA signature and a message hash.

    -   **Use Case:** The most frequently used precompile, essential for signature verification in smart contracts (e.g., multisigs, meta-transactions).

2.  **`0x2` - `SHA-256` Hash Function** 🔒

    -   **Purpose:** Implements the standard SHA-256 hashing algorithm.

    -   **Input:** Any calldata.

    -   **Output:** The 32-byte SHA-256 hash of the input.

3.  **`0x3` - `RIPEMD-160` Hash Function** 🔒

    -   **Purpose:** Implements the RIPEMD-160 hashing algorithm.

    -   **Use Case:** Often used in conjunction with SHA-256 for generating Ethereum addresses.

4.  **`0x4` - `Identity` Function** 🔄

    -   **Purpose:** Returns its input data directly. Essentially a copy operation.

    -   **Use Case:** Can be cheaper than copying memory (`mcopy`) within a smart contract in certain scenarios. A gas optimization tool.

5.  **`0x5` - `ModExp` (Modular Exponentiation)** 🧮

    -   **Purpose:** Computes `(base^exponent) mod modulus` with arbitrary precision.

    -   **Use Case:** Crucial for various cryptographic protocols, including RSA verification and certain zero-knowledge proof schemes.

✨ The 9 Precompiled Contracts (Crypto Edition)
==============================================

6.  **`0x6` - `alt_bn128` Addition** ➕

    -   **Elliptic Curve:** `alt_bn128` (different from the one used for standard transaction signatures, `secp256k1`).

    -   **Purpose:** Performs point addition on the `alt_bn128` curve.

    -   **Input:** Coordinates of two points.

    -   **Output:** Coordinates of the resulting point.

7.  **`0x7` - `alt_bn128` Scalar Multiplication** ✖️

    -   **Purpose:** Performs scalar multiplication on the `alt_bn128` curve.

    -   **Input:** Coordinates of a point and a scalar value.

    -   **Output:** Coordinates of the resulting point.

8.  **`0x8` - `alt_bn128` Pairing Check** ✅

    -   **Purpose:** Executes a pairing operation, a fundamental building block for zk-SNARKs (Zero-Knowledge Succinct Non-Interactive Argument of Knowledge).

    -   **Use Case:** Enables privacy-preserving technologies and scalability solutions like ZK-Rollups on Ethereum.

9.  **`0x9` - `BLAKE2f` Compression Function** 💨

    -   **Purpose:** Implements the compression function `F` from the BLAKE2 cryptographic hash algorithm.

    -   **Use Case:** Required for interoperability with Zcash and other protocols that use BLAKE2.

🔮 Future Precompiles & Properties
==================================

### **Upcoming Additions (Cancun-Deneb Upgrade)**

-   **EIP-4844:** Introduces a precompile for **KZG Point Evaluation**.

    -   **Purpose:** To verify data "blobs" for Proto-Danksharding, significantly reducing rollup costs.

-   *Community discussions are also underway for precompiles related to other elliptic curves to support more cryptographic operations.*

### **Unique Properties of Precompiles**

-   **No Reverts:** If an error occurs during execution (e.g., invalid input), the precompile returns `0` or an empty value but **does not cause the parent transaction to revert**.

-   **No Bytecode:** Opcodes like `EXTCODESIZE` will return `0` when called on a precompile address. The EVM treats them as Externally Owned Accounts (EOAs) in this context.

-   **Non-Standard Calldata:** Precompiles do not follow the standard ABI encoding for their inputs. Their data parsing is defined directly by their underlying algorithms.

-   **Stateless:** Currently, all precompiles are stateless. They do not have their own storage.

✍️ Digital Signatures in Ethereum
=================================

### **Primary Signature Types**

-   **ECDSA (Elliptic Curve Digital Signature Algorithm)**

    -   **Curve:** `secp256k1`

    -   **Usage:**

        -   Securing all user transactions.

        -   Verifying signed messages off-chain and on-chain.

    -   **On-chain Verification:** Done using the **`ecrecover`** precompile.

-   **BLS (Boneh-Lynn-Shacham)**

    -   **Usage:** Used by validators in the **Consensus Layer** for attestations.

    -   **Key Feature: Signature Aggregation.**

        -   Thousands of validator signatures can be combined into a single, constant-size signature.

        -   This dramatically reduces network load and is critical for Proof-of-Stake consensus at scale.

✉️ Why Sign a *Message* Instead of a Transaction?
=================================================

*Signing messages allows for off-chain verification and gasless interactions.*

### **Common Use Cases**

1.  **Trusted Party Approvals (e.g., Bridges)** 🌉

    -   Bridge validators sign a message authorizing a user to withdraw funds on the destination chain. The user submits this signature to the smart contract.

2.  **Multisig / DAO Voting** 🗳️

    -   Members sign messages to approve a proposal off-chain. Once enough signatures are collected, a single transaction can be sent to execute the proposal.

3.  **Gasless "Login" / Terms of Service** ✅

    -   A user signs a message to agree to a platform's ToS, proving ownership of their account without sending a transaction and paying gas.

4.  **Entropy for Key Generation** 🎲

    -   A user's unique signature can be used as a source of entropy to derive other keys (e.g., encryption keys).

📜 EIP-191: Signed Data Standard
================================

### **The Problem: Replay Attacks**

-   Without a standard, a signed message could potentially be misinterpreted as a signed transaction, as they are both just strings of bytes. An attacker could "replay" a signature intended for one purpose as a transaction, draining funds.

### **The EIP-191 Solution**

-   **Prefix all messages** before hashing and signing them.

-   **Format:** `\x19Ethereum Signed Message:\n<length of message>`

-   This prefix ensures that the hash of a signed message can **never** collide with the hash of a valid Ethereum transaction, preventing replay attacks between message signing and transaction signing.

⚠️ Vulnerabilities in Basic Signature Schemes
=============================================

*A simple EIP-191 signature check can still be vulnerable.*

1.  **Cross-Chain Replay Attack** ⛓️

    -   **Problem:** A signature generated for one EVM chain (e.g., Ethereum Mainnet) could be valid on another (e.g., Polygon) if the contract and state are identical.

    -   **Solution:** Include the **`chainid`** in the signed message.

2.  **Cross-Contract Replay Attack** 📄

    -   **Problem:** If two different contracts use the same logic for message signing, a signature intended for `Contract A` could be replayed on `Contract B`.

    -   **Solution:** Include the **verifying contract's address** in the signed message.

3.  **Cross-Function Replay Attack** 🔧

    -   **Problem:** A signature intended for one function (`approve`) might be usable in another function (`transfer`) if they expect similar arguments.

    -   **Solution:** Include a unique identifier for the function or action (e.g., function selector, name) in the signed message.

📑 EIP-712: Typed Structured Data Hashing and Signing
=====================================================

*The ultimate standard for safe, user-friendly message signing.*

### **How It Works**

-   **Goal:** Present data to users in a readable, understandable format rather than a cryptic hex string.

-   **Mechanism:** Defines a structure for the data being signed, including types and names for each field. This is displayed clearly in wallets like MetaMask.

### **Core Components**

1.  **`Domain Separator`**

    -   A unique fingerprint for the signing context. It prevents replay attacks by default.

    -   **Includes:**

        -   `name`: The DApp's name.

        -   `version`: The contract version.

        -   `chainId`: The network ID.

        -   `verifyingContract`: The address of the contract.

2.  **`Type Hash`**

    -   A hash representing the structure of the message itself (e.g., `Permit(address owner, address spender, uint256 value)`). This prevents cross-function replays.

*By combining the `Domain Separator` and the `Type Hash` of the specific message, EIP-712 creates a globally unique, unambiguous message to be signed.*

💻 Smart Contract Bytecode: Init vs. Runtime
============================================

### **Two Types of Bytecode**

1.  **Init Code (Creation Code)**

    -   **What it is:** The bytecode used to **deploy** a contract. It includes:

        -   The constructor logic.

        -   The runtime code that will be stored on-chain.

        -   ABI-encoded constructor arguments appended at the end.

    -   **Execution:** Runs **only once** during the deployment transaction.

    -   **Size Limit:** `48 KB` (as of the Shanghai hard fork).

2.  **Runtime Code**

    -   **What it is:** The bytecode that is **stored on the blockchain** at the contract's address.

    -   **Execution:** Runs every time someone interacts with the contract.

    -   **Origin:** It's the data that the `Init Code` returns from its execution.

    -   **Size Limit:** `24 KB`.

⚙️ Anatomy of Init Code
=======================

*The `solc` compiler assembles `Init Code` into four distinct parts.*

```
[ Constructor ] -> [ Runtime Code ] -> [ Metadata ] -> [ Constructor Arguments ]

```

1.  **Constructor** 🏗️

    -   Initializes the `free memory pointer`.

    -   Parses constructor arguments from the end of the calldata.

    -   Runs the logic written in the `constructor()`.

    -   Copies the `Runtime Code` into memory and returns it to be stored on the blockchain.

2.  **Runtime Code** 🏃

    -   The actual contract logic that will live on-chain. This part is just data within the `Init Code`.

3.  **Metadata** 📊

    -   A CBOR-encoded data blob appended by the compiler.

    -   Contains a hash of the source code (typically IPFS or Swarm).

    -   Allows for source code verification. The hash is unique to the exact code and compiler settings.

4.  **Constructor Arguments** 🎁

    -   ABI-encoded arguments passed to the constructor during deployment.

🔬 A Closer Look at Runtime Code
================================

*Runtime code is primarily a **function dispatcher** and the function bodies.*

### **Execution Flow**

1.  **Initialize Free Memory Pointer:** Every call starts by setting up memory.

2.  **Parse Calldata:** The dispatcher reads the first 4 bytes of the calldata to get the **function selector**.

3.  **Dispatch:** It compares the selector against the known function selectors in the contract.

    -   **Match Found:** It jumps to the code for that specific function.

    -   **No Match:** It jumps to the `fallback` or `receive` function (if one exists), or reverts.

4.  **Execute Function Body:**

    -   Parses arguments from calldata.

    -   Executes the function's logic (e.g., read/write storage, call other contracts).

5.  **Return Data:** Places the output in memory and returns it.