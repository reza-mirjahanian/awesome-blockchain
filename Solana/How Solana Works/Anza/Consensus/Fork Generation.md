# 🌿 Fork Generation in Solana

> **TL;DR**:  
> Solana **does not wait** for global agreement before producing the next block → **forks are normal**.  
> Validators **vote** → one fork **wins**, the rest **vanish**.

---

## 1. Forks 101  
- **Definition**: competing chains that share the same parent block.  
- **Cause**: leader rotation + network latency → two leaders may append to the same parent.  
- **Outcome**: only one fork is **finalized**; abandoned blocks are **discarded**.

---

## 2. Leader-Rotation Forking Model  

### 2.1 Slot-Based Leader Schedule  
- Every **slot** has **exactly one** designated leader.  
- A leader **must** start its slot by replaying **any missing ticks** to link back to the last slot it **personally observed**.  
- Possible PoH streams per slot → **only two**:  
  1. **Entries + ticks** from the actual leader.  
  2. **Ticks only** (virtual ledger produced when the leader is offline or unseen).

### 2.2 “Skip-List” Forks  
- Forks are limited to **there / not-there** skips at slot boundaries.  
- **Same-slot blocks are identical** (double-block = slashable).  
- Distinct forks differ **only by skipped slots**.

---

## 3. Forking Example  

| Slot | Fork 1 | Fork 2 | Fork 3 |
|------|--------|--------|--------|
| 1    | Block 1 | Block 1 | Block 1 |
| 2    | —       | —       | Block 2 |
| 3    | Block 3 | Block 3 | —       |
| 4    | —       | Block 4 | —       |
| 5    | Block 5 | —       | —       |

- **Fork 1** skipped slots 2 & 4.  
- **Fork 2** skipped slot 2.  
- **Fork 3** skipped slots 3–5.

---

## 4. Message Flow — 11 Steps  

1. 🚪 **Ingest** transactions by current leader.  
2. 🔍 **Filter** valid transactions.  
3. ⚙️ **Execute** transactions, update local state.  
4. 📦 **Package** into entries tied to current PoH slot.  
5. 📡 **Transmit** signed **shreds** to validators.  
   - Stream starts with **catch-up ticks** to bridge any gap.  
   - Empty **ticks** act as liveness heartbeats.  
6. 🔄 **Retransmit** by validators to peers & downstream.  
7. ✅ **Re-validate** & re-execute on every validator.  
8. #️⃣ **Compute** resulting state hash.  
9. 🗳️ **Vote** at specific PoH tick counts.  
   - Vote = signature of state hash.  
   - Votes gossiped cluster-wide.  
10. 📊 **Leader executes votes** & re-broadcasts.  
11. 👀 **Validators observe** their own + all other votes.

---

## 5. Partitions & Fork Creation  

### 5.1 Tick-Driven Boundaries  
- Forks appear **at vote tick counts**.  
- Next leader may **not have seen** the latest vote slot → emits **virtual ticks**.  
- All nodes generate these ticks at cluster-defined rate `Z` hashes/tick.

### 5.2 Validator Choices  
- **Greedy fork choice** (Tower BFT) to maximize rewards.  
- **Ignore** forks from wrong leaders or **slash** the offender.  
- Once a validator **commits** to a fork, **all conflicting forks below that tick count can be discarded**.

---

## 6. Visualizing Validator Time-Line  

```
PoH Stream → time flows ↓
L1  L2  L3  L4  L5
E1  E2  x   E4  E5
    x   E3  xx  x
```
- `E` = actual leader entries.  
- `x` = ticks only (virtual).  
- Same `E` appearing twice at one slot → **slashable**.

---

## 7. Leader’s View at Slot Start  

1. Identify **latest voted slot** locally.  
2. Emit **missing ticks** to link current slot to that slot.  
3. Propose fork anchored to **that prior fork + virtual ticks**.