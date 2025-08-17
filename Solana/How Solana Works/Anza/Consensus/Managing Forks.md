# 🧩 Managing Forks in Consensus

## 🔄 Fork Selection Algorithm  
Agave validators use a **weight-based fork selection algorithm**:  
- Always chooses the fork with the *heaviest stake weight*  
- Weight calculated from **validator votes** on bank hashes  
- Prioritizes forks with **sufficient super-majority stake**  

## ⚙️ Block Replay Process  
When processing forks:  
1️⃣ Receive new blocks via **gossip protocol**  
2️⃣ Replay blocks on *all viable forks*:  
   - Validate transactions & signature verification  
   - Execute smart contracts  
   - Update fork-specific ledger state  
3️⃣ Maintain **parallel computation** for competing forks  

## 🔒 Fork Resolution  
- Validators **lock votes** on specific banks by signing:  
  `BankHash = Hash(slot, blockhash, validator_identity)`  
- Confirmed when fork achieves:  
  > 🔐 **2/3+ super-majority stake commitment**  
- Unconfirmed forks are **orphaned** when:  
  - Heavier fork emerges  
  - Stale for > `MAX_LOCKOUT_HISTORY` slots  

## ⚖️ Weight Comparison Metrics  
Validators evaluate forks using:  
- **Stake weight** of committed validators  
- **Vote recency** (newer votes prioritized)  
- **Blockhash ancestry** depth  

## ⚡ Optimistic Confirmation  
- **Early lock** possible when:  
  ```python
  if fork_stake > (2/3 * total_stake) and block_depth > 32:
      confirm_optimistically()
  ```  
- Allows faster finalization before full epoch commitment  

## 🛡️ Slashing Protection  
- **Double-voting prevention**:  
  - Validators sign every vote with `(slot, bankhash)`  
  - Conflicting signatures trigger automatic slashing  
- **Surround vote detection** enforced via vote timestamps  

## 🔁 Fork Switching  
Validators dynamically switch forks when:  
1. New fork has **>1.25x weight** of current chain  
2. Receives **conflicting optimistic confirmation**  
3. Current fork exceeds **failure threshold** (e.g., 32 skipped slots)


## 🌳 Fork Basics  
- **Fork** = sequence of slots branching from a root  
- **Blockstore** = tree of all forks  

> Example forks  
> - `{0,1,2,4,6,8}`  
> - `{0,1,3,5,12,13}`  
> - `{0,1,3,5,7,9,10,11}`  

---

## ✂️ Pruning Strategy  

1. **Root slot**  
   - Reached *max lockout depth* → **immutable**  
2. **Validator action**  
   1. Drop forks **not** rooted in local root  
   2. **Squash** remaining nodes into root  
3. **Keep ancestors**  
   - Up to cluster-wide **Super Majority Root (SMR)** for RPC needs  

---

## 🗳️ Walk-Through  

**Step 1**  
- Votes: `0→1→3→5→7→9`  
- Local root: `3`  
- SMR: `0`  
- Pruned view:  

```
3
└─5─7─9
```

**Step 2**  
- Next vote: `10` → roots `5`  
- SMR moves to `3`  
- Pruned view:  

```
5
├─7─9
└─10
```