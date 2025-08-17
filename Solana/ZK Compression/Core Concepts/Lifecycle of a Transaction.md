# Lifecycle of a Transaction in ZK Compression


# 🔄 Lifecycle of a ZK Compression Transaction

> Same Solana envelopes (`Transaction`, `VersionedTransaction`) – **new packing rules**.

---

## 1️⃣ Three Nuances vs Regular Accounts

### 📋 Instruction Payload
- **Explicit list** of every compressed account being **read or written**.
- For each account:
  1. Send its **current state** on-chain.
  2. Attach a **validity proof** (128 B).

### 🌳 State-Tree Access List
- Every **state tree** touched (even indirectly) must appear in Solana’s **account access list**.

### 🔐 Proof Types
- **Existence proof** – confirms the account is a leaf.  
- **Non-existence proof** – proves a PDA address is still **vacant** (needed before creation).

---

## 2️⃣ Transaction Formula

> **`(state, validityProof)` → state transition → `state'`**

---

## 3️⃣ Single Compressed PDA Update – Step-by-Step

### 🧾 Client Side
1. Fetch account + Merkle proof via RPC.  
2. Pack instruction with:
   - `address` (unchanged)
   - `owner` program id (unchanged)
   - `data` (old)
   - `data' - data` (delta)
   - **validity proof**

### 🧑‍💻 Instruction Layout

| Field | Size | Purpose |
|-------|------|---------|
| `address` | 32 B | Stable PDA |
| `owner` | 32 B | Program id |
| `data` | variable | Old account bytes |
| `data' - data` | variable | Delta to apply |
| `validity_proof` | 128 B | Groth16 proof |

---

## 4️⃣ On-Chain Protocol Flow

> All steps executed by **Light System Program** via CPI.

1. ✅ **Checks**  
   - Integrity (sum checks, etc.)
2. 🗑️ **Nullify**  
   - Mark old leaf as spent.
3. 🌱 **Append**  
   - Insert new leaf hash → update Merkle root.
4. 📡 **Emit**  
   - Log new compressed state to ledger.

---

## 5️⃣ RPC Aftermath

- **RPC node** indexes the log.
- Serves fresh compressed state via **ZK Compression RPC API**.



## Overview of ZK Compression Transactions
ZK Compression transactions maintain full compatibility with Solana's standard **Transaction** and **Versioned Transaction** formats. However, when involving *compressed accounts*, they introduce three key nuances compared to regular accounts:

1. **Instruction Specification**  
   Instructions must *explicitly list* all compressed accounts being read or written to. To interact with a compressed account (for reading or writing), the instruction requires sending the current account state on-chain and proving its validity.

2. **State Tree Specification**  
   Each unique *state tree* accessed (through any compressed account) must be specified. This follows Solana's standard on-chain account access lists.

3. **Validity Proof Requirement**  
   To read any compressed account state on-chain, the client must include a *validity proof* alongside the instruction data. This proof can validate:  
   - **A)** The validity of all specified read accounts.  
   - **B)** The non-existence of a specified *Program-Derived Address (PDA)* within the compressed address space (e.g., for creating a new compressed PDA account).

> 🔄 The transaction can be expressed as:  
> `(state, validityProof) -> state transition -> state'`

This represents a transition from an initial state to a new state, verified by the proof.

## Example: Updating a Single Compressed PDA Account
🔍 *Simplified Process: Read and Write Compressed Accounts*

Assume the client has previously created the compressed account and fetched its details from an RPC node. A custom Solana program handles the state transition from `Data` to `Data'`. The client packs instructions efficiently, sending the following data to the chain:

- **Address** (unchanged)  
- **Owner Program** (unchanged)  
- **Data**  
- **Difference** between new data and old data (`Data' - Data`)  
- **Validity Proof**

After the update:  
- The compressed account is fully represented, including its *PDA*.

## On-Chain Protocol Execution
When writing compressed state on-chain, a custom caller program invokes the *Light System Program* via **Cross-Program Invocation (CPI)**. The system program executes these steps:

1. **Runs Relevant Checks**  
   Includes processes like *sum checks* to ensure integrity.

2. **Nullifies the Old Leaf**  
   The "old" leaf of the compressed account being written to is nullified in the state tree.

3. **Appends New Hash and Advances Tree**  
   The new compressed account hash is appended to the state tree, and the tree's *state root* is advanced.

4. **Emits New State**  
   The updated compressed account state is emitted onto the Solana ledger.

📡 An *RPC node* then parses the transaction and compressed state, providing the read state to clients via the **ZK Compression RPC API**.

## Key Terms and Concepts
🔑 **Compressed Accounts**  
Accounts requiring special handling for reads and writes, involving state trees and validity proofs.

🔑 **State Tree**  
A data structure storing compressed account states; each unique tree must be specified in transactions.

🔑 **Validity Proof**  
Proof included with instruction data to confirm account states or PDA non-existence.

🔑 **PDA (Program-Derived Address)**  
An address type in the compressed space, used when creating new compressed accounts.

🔑 **Light System Program**  
The core program invoked via CPI for managing compressed state updates.

🔑 **RPC Node**  
A node that processes transactions and states, exposing data through the ZK Compression RPC API.