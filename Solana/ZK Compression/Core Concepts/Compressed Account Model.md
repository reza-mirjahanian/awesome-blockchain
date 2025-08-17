# 🗜️ Compressed Account Model – ZK Compression

> **Compressed accounts** are **Solana accounts on a diet**:  
> keep the semantics, slash the rent.

---

## 1️⃣ Regular vs Compressed Accounts

| Feature | Regular Account | Compressed Account |
|---------|-----------------|--------------------|
| **Identifier** | `Pubkey` (32 B) | **Hash** (32 B) *(or optional `address`)* |
| **State storage** | Lives in **validator RAM** | Lives as **calldata in the ledger** |
| **Rent cost** | 💸 **Paid per byte** | 🪙 **Almost zero** *(only root stored on-chain)* |
| **Mutability** | In-place edits | **New hash per write** *(append-only)* |
| **Lookup** | Direct key access | **Merkle proof + root** |

---

## 2️⃣ Anatomy of a Compressed Account

### 🔑 Required Fields
- **Hash** – *always* the primary key.  
- **Data** – program state wrapped in `AccountData`.

### 🏷️ Optional Fields
- **Address** – permanent, unique PDA-style id.  
  > Use it for **NFTs**, skip it for **fungible tokens**.

---

### 📦 `AccountData` Layout (Anchor-friendly)

| Offset | Field | Purpose |
|--------|-------|---------|
| 0–7 | **Discriminator** (8 B) | Anchor type tag |
| 8… | **Data** | Actual program state |
| tail | **DataHash** | ZK integrity (ignore for now) |

> Same feel as a classic PDA, but the **owner** is the program that derived it.

---

## 3️⃣ Address vs Hash – When to Use What

| Use-Case | Use `Address` | Use Hash Only |
|----------|---------------|---------------|
| **Unique assets** (NFTs, singleton PDAs) | ✅ Yes | ❌ |
| **Fungible balances** | ❌ Overhead | ✅ Cheaper |
| **Ephemeral state** | ❌ | ✅ |

> Changing data → **new hash** → Merkle path changes;  
> the `address` stays constant if you need stable lookup.

---

## 4️⃣ Sparse State Trees – Behind the Scenes

- **Every compressed account** = leaf in a Merkle tree.
- **On-chain footprint** = single 32-byte **state root** (fingerprint).
- **Proof size** = logarithmic in #accounts, not linear.

> Validators verify proofs instead of storing full accounts.

---

## 5️⃣ Quick Mental Model

```text
[Program Logic]
     │
     ├─ "Write"  → emit new account data as tx calldata
     │
     ├─ "Read"   → provide Merkle proof + root
     │
     └─ "Verify" → on-chain program checks proof against stored root
```

---

# **Compressed Account Model: Core Concepts**  

## **What is the Compressed Account Model?**  
A **space-efficient** way to store account data on-chain by **offloading computation** while maintaining cryptographic security.  

### **Key Components**  
1. **On-Chain Verifier**  
   - A lightweight smart contract that validates proofs.  
   - Only stores a **root hash** of the compressed data.  

2. **Off-Chain Data Store**  
   - Bulk data (e.g., account states) is stored off-chain.  
   - Accessed via **zero-knowledge (ZK) proofs**.  

3. **Merkle Trees for Compression**  
   - Data is hashed into a **Merkle tree**.  
   - Only the **root commitment** is stored on-chain.  

## **How It Works**  

### **1. Data Compression (Off-Chain)**  
- Accounts are batched and hashed into a Merkle tree.  
- The **Merkle root** represents the compressed state.  

```rust  
// Example: Generating a Merkle root from account data  
let accounts = vec![account1, account2, account3];  
let merkle_root = compute_merkle_root(accounts);  
```  

### **2. On-Chain Verification**  
- A user submits a **proof** (e.g., Merkle proof) to verify data.  
- The on-chain verifier checks the proof against the stored root.  

> **Note**: The verifier **does not store full data**, only the root hash.  

### **3. State Updates**  
- When an account changes, a new Merkle root is computed.  
- Only the **updated root** is stored on-chain.  

## **Advantages**  
✔ **Reduced Storage Costs** – Less on-chain data means lower fees.  
✔ **Scalability** – Supports millions of accounts without bloating the chain.  
✔ **Security** – Cryptographic proofs ensure data integrity.  

## **Use Cases**  
- **DeFi**: Efficiently track user balances.  
- **NFTs**: Store metadata off-chain while keeping ownership on-chain.  
- **Gaming**: Handle large game states with minimal on-chain footprint.  

## **Trade-offs**  
⚠ **Off-Chain Dependency** – Requires external data availability.  
⚠ **Proof Generation Overhead** – ZK proofs add computational cost.