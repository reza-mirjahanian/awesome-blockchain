# ⚠️ ZK Compression – Limitations Cheat-Sheet

> Mix **compressed** and **regular** accounts.  
> Know the trade-offs **before** you compress.

---

## 🚫 When NOT to Compress

| Scenario | Keep Regular |
|----------|--------------|
| 🔁 **Hot-spot account** updated many times **per block** | e.g., shared AMM pool |
| 📈 **Lifetime writes ≫ 1 000×** | cost amortization fails |
| 📦 **Bulk read > 1 kB** inside **one tx** | exceeds tx limits |

---

## 📏 Transaction Size Cap

- **Solana limit**: **1 232 bytes**
- **ZK overhead**:
  - 128 B validity proof
  - Full account data you read or write

> Exceed → **tx fails**.

---

## 🔥 Compute Unit (CU) Budget

| Step | CU Cost |
|------|---------|
| ✔ Validity proof verify | ≈ **100 k** |
| ✔ State-tree hashing | ≈ **100 k** |
| ✔ Read/write **each** account | ≈ **6 k** |

### 📊 Quick Example
> Compressed token transfer ≈ **292 k CU**

### 🚧 Hard Ceilings
- **Per tx**: **1.4 M CU**
- **Per state-tree / block**: **12 M CU**
- **Global / block**: **48 M CU** → congestion ⇒ **priority fees**

---

## 💸 State Cost Per Write

> Compressed writes **nullify + append**.  
> Two micro-costs added to base fee.

### 🔢 Formula (per write)
```
Cost ≈ 2^depth × rent × rollover
```
- Default depth **26** → ≈ **0.0003 SOL** per write

### 🏦 Forester Reimbursement
- Fee covers validator’s **off-chain indexing** (Forester).

---

## ✅ Practical Rules

1. **Compress** for:
   - Rare writes, many reads
   - Large data rarely mutated
2. **Stay regular** for:
   - High-frequency state
   - Tight CU budgets