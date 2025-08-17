# ⚡ **Solana Transaction Lifecycle – Turbo Edition**

---

## 📦 **Transaction Anatomy**
> One packet, 1,232 bytes max, everything inside.

- **Signatures** 🔑  
  - 64 bytes each (max 12 → 768 bytes).  
- **Message** 📄  
  - **Header** (3 bytes):  
    - `num_required_signatures`  
    - `num_readonly_signed_accounts`  
    - `num_readonly_unsigned_accounts`  
  - **Accounts** (32 bytes each) – **compact-u16** length + ordered list.  
  - **Recent Blockhash** 🕑 – 32 bytes; expires after 150 slots (≈ 60 s).  
  - **Instructions** – **compact-u16** count + array of:  
    - `program_id_index` (u8)  
    - `accounts_indexes` (compact-u16 list)  
    - `data` (compact-u16 opaque bytes)

---

## 🚀 **Journey of a Single Transaction**

### 1️⃣ **User Signs**
- Wallet → RPC server (non-voting validator).

### 2️⃣ **RPC Routes**
- Uses **QUIC** to push tx to:
  - Current leader  
  - Next two leaders (leader schedule fixed per 2-day epoch).

### 3️⃣ **Leader Stages** (TPU pipeline)
1. **Fetch** 📥 – batches 128 packets.  
2. **SigVerify** ✔️ – dedupe & check sigs.  
3. **Banking** 🏦 – **parallel execution** via **Prio-Graph** scheduler:  
   - Priority fee (user-set) + vote weight.  
   - Lock accounts → execute → update state.  
4. **Broadcast** 📡 – shred + gossip to cluster.

> **No mempool** 🙅‍♂️ – Gulfstream keeps pending txs in RPC/validator memory.

---

## 🔒 **Locking & Parallelism Cheat-Sheet**
- **Read-lock** 🔍 – many at once.  
- **Write-lock** ✍️ – exclusive.  
- **Conflict?** → re-queued for next try.  
- **Result:** Non-overlapping txs fly in parallel lanes 🏎️🏎️🏎️.

---

## ✅ **Finality Flow**
- Supermajority confirms block → 31+ additional blocks → **done**.  
- RPC streams status back to frontend in real-time.