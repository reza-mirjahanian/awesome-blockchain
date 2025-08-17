# 🌐 Solana Cluster Deep-Dive  

---

## 1. Cluster Basics  
> *A Solana cluster is a **co-operating set of validators** that:*
> * serve client transactions  
> * maintain the integrity of the **shared ledger**  

- **Many clusters can coexist**  
  - If two share a **common genesis block** → they attempt **convergence**  
  - If not → they **ignore each other**  
  - Transactions sent to the wrong cluster are **silently rejected**  

---

## 2. Creating a Cluster 🔨  

### 2.1 Step-by-step  
1. **Create the genesis config**  
   - References two public keys  
     - 🪙 **Mint**  
     - 🔑 **Bootstrap validator**  
2. **Bootstrap validator**  
   - Holds the private key → **appends first ledger entries**  
   - Initializes its state with the **mint’s account**  
   - Account balance = native tokens defined in genesis config  
3. **Second validator**  
   - Contacts the bootstrap validator → **registers**  
4. **Additional validators**  
   - Register with **any already-registered member**

---

## 3. Joining a Cluster 🚪  

- **Registration message** → sent to the **control plane**  
  - Implemented via **gossip protocol**  
  - Guarantees  
    - *Eventual* global knowledge  
    - *Uncensorable* information  
- **Sync time** ∝ **O(n²)**  
  - Slow algorithmically, but **robust**

---

## 4. Ledger Sharing & Replication 📋  

- **Validator life-cycle**  
  1. Receives **all entries** from the **current leader**  
  2. **Votes** to confirm validity  
  3. **Stores** the entries  
  4. **Deletes** its copy once *sufficient* redundant copies are observed  

---

## 5. Sending Transactions to a Cluster 📬  

```text
Client ──► any validator’s TPU port
         │
         ├──► if validator role → forwards to **leader**
         └──► if leader role → bundles + timestamps → pushes to **data plane**
```

- **Data plane** = the path where transactions are **validated & appended** to the ledger  

---

## 6. Confirming Transactions ⚡  

### 6.1 Speed  
- **Sub-second confirmation** for **thousands** of nodes  
- Roadmap: **hundreds of thousands** of validators  
- Confirmation time grows **logarithmically**  
  - Base ≈ 1000 → **+1 hop** per 1000× more validators  

### 6.2 Definition  
> **Confirmation** = time from **leader timestamp** ➜ **supermajority ledger votes seen**

---

## 7. Scalability Techniques 🚀  

| Technique | Purpose |
|-----------|---------|
| **VDF-timestamp + signature** | Prove time & leader identity |
| **Recursive batching** | Split & share tx sets efficiently |
| **Turbine Block Propagation** | Multi-level batching for massive scale |

### 7.1 Recursive Batch Sharing  
1. **Leader splits** transactions into **batches**  
   - *Example*: 60 tx ÷ 6 nodes → 10 tx / batch  
2. **Each node** shares its batch with **peers**  
3. **Reconstruct** full set once all batches collected  

### 7.2 Turbine Block Propagation  
- Apply the same **recursive splitting** on **another equal-sized set of nodes**  
- Enables scaling **beyond ~1,250 validators** to **hundreds of thousands**