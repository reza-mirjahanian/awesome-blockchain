# 🔄 Solana Leader Rotation

---

## 🧩 **Core Concept**
*Why rotation matters*  
- Only **one validator** produces ledger entries at a time  
- Prevents conflicting ledgers but creates **censorship risk**  
- 🔄 Regular rotation minimizes malicious leader influence  

> *"A malicious leader can censor votes/transactions, but can't be distinguished from network failures."*

---

## 📅 **Leader Schedule Mechanics**
- Validators reject blocks not signed by the ***slot leader***  
- *Leader schedule* = identity list of all slot leaders for an epoch  
- 🕒 Schedule is:  
  - Computed **locally** and **periodically**  
  - Based on ledger state from **previous epoch** (1-epoch offset)  
  - Generated when root fork crosses **epoch boundary**  

### 🌐 **Epoch Boundary Example**
- With simplified 100-slot epochs:  
  - Root fork updates from slot `99` → `102` (skipping `100-101`)  
  - New schedule uses fork at slot `102`  
  - Active from slot `200` onward  

---

## ⚠️ **Partition Challenges**
- Partitions > epoch duration cause **inconsistent schedules**  
- Multiple valid schedules can exist during instability  
- 🔒 Ideal offset:  
  > *median partition duration + 6 standard deviations*  
  → Reduces inconsistency likelihood to **1 in 1 million**

---

## 🌱 **Genesis Schedule**
- Genesis config sets **first leader** for initial epoch  
- First leader scheduled for **first two epochs**  
- Epoch length must be ≥ Tower BFT's `max_rollback_depth`  

---

## 🧮 **Schedule Generation Algorithm**
1. Use `PoH tick height` to seed random algorithm  
2. Sample bank for **active staked accounts** with leader identities  
3. Sort by **stake weight**  
4. Select nodes using **stake-weighted ordering**  
5. Schedule becomes valid after configured ticks  

---

## 🛡️ **Attack Vectors & Mitigations**

### 🔐 **Seed Vulnerability**
- Predictable but **unbiasable** (no grinding attacks possible)

### 👥 **Active Set Manipulation**
- Leaders can:  
  - ❌ Ignore validator votes  
  - ❌ Reject blocks with certain votes  
- Mitigation: Sample over *active set duration* (multiple leaders' slots)

### 💰 **Staking Censorship**
- Leaders can block new stake transactions  
- Similar to vote censorship

### 🔑 **Key Loss Recovery**
- Cluster recovers from **ephemeral key loss**  
- Stake owners can vote directly via co-signing

---

## 📥 **Appending Entries Process**
- Epoch = schedule lifetime, split into slots (`T` `PoH ticks` each)  
- Leader transmits entries **only during assigned slot**  
- Validators **ignore out-of-slot entries**  
- Next leader must **fill previous slot** if:  
  - Leader down (no entries)  
  - Entries invalid (malicious/broken leader)  
  - ⚠️ Must coordinate repair requests before filling slots