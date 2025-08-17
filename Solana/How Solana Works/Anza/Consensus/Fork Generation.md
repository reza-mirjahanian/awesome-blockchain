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



# Fork Generation in Anza Consensus

## 🔍 Core Concept
A **fork** occurs when the blockchain temporarily splits into multiple competing chains due to simultaneous block proposals. Anza's consensus protocol automatically resolves these divergences through its *fork choice rule*.

> **Critical distinction**:  
> Forks ≠ Protocol upgrades. This refers to *temporary chain divergences*, not permanent hard/soft forks from rule changes.

## ⚙️ How Forks Are Generated

### 🌐 Primary Causes
- **Network latency**: Blocks propagating at different speeds across nodes
- **Simultaneous proposals**: Multiple validators creating blocks at the same height
- **Malicious actors**: Deliberate attempts to create conflicting chains

### 🔄 Fork Generation Process
1. **Height collision**
   - Two validators produce valid blocks at identical height `H`
   - Example: `Block_A` and `Block_B` both extend `Block_H-1`

2. **Chain divergence**
   ```mermaid
   graph LR
     H-1 --> A[Block_A at H]
     H-1 --> B[Block_B at H]
   ```

3. **Network partitioning**
   - Subset 1 receives `Block_A` first → builds on `Block_A`
   - Subset 2 receives `Block_B` first → builds on `Block_B`
   - *Result*: Two competing chains at height `H+1`

## 🧠 Fork Resolution Mechanism

### 🔎 Fork Choice Rule
Anza selects the canonical chain using:
```
canonical_chain = chain with highest cumulative weight
```
Where **cumulative weight** = sum of validator weights along the chain

### ✅ Resolution Workflow
1. Validators observe competing chains
2. Each calculates cumulative weight for:
   - `Chain_A`: `weight(Block_A) + weight(H-1) + ...`
   - `Chain_B`: `weight(Block_B) + weight(H-1) + ...`
3. **Automatic convergence**:
   - All validators extend the chain with *highest cumulative weight*
   - Lower-weight chain becomes orphaned within 1-2 blocks

> 💡 **Key insight**:  
> Unlike longest-chain rules, Anza's *weight-based selection* prevents "selfish mining" attacks by prioritizing stake concentration over mere block count.

## ⚡ Real-World Example
**Scenario**: Height 100 collision
- `Chain_A`: Weight = 65 (Validator X: 40 + Validator Y: 25)
- `Chain_B`: Weight = 58 (Validator Z: 35 + Validator W: 23)

**Resolution**:
- Next validator *always* builds on `Chain_A`
- `Chain_B` orphaned after height 101
- Total resolution time: **< 5 seconds** (Anza's target block time)

## 🛠️ Critical Parameters
| Parameter | Role | Anza Default |
|----------|------|--------------|
| `WEIGHT_WINDOW` | Weight calculation period | 10 blocks |
| `FORK_TOLERANCE` | Max allowed divergence | 3 blocks |
| `FINALITY_DELAY` | Time until irreversible | 32 blocks |

> ⚠️ **Failure condition**:  
> If forks persist beyond `FORK_TOLERANCE`, the network triggers *emergency finalization* to force consensus.