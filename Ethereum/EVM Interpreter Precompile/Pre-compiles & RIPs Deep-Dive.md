# 🚀 **RollCall #2.3 – Pre-compiles & RIPs Deep-Dive**  
*February 7, 2024 – L2 standardization chaos, gas-cost drama, and Progressive Pre-compile dreams*

---

## 📍 **RIP Lifecycle in 30 Seconds**
- Status copied 1-for-1 from **EIP flow**  
  - Draft ➜ Review ➜ Final Call ➜ **Final** ✅  
- **Only Final** guarantees **byte-for-byte identical** spec across rollups

---

## 🏅 **RIP-7212 – SECP256R1 Pre-compile**
| Chain / Rollup | Status | Test-net | Main-net ETA |
|----------------|--------|----------|--------------|
| **Polygon** | 🟢 Implemented | ✅ | **NoPo** hard-fork (next weeks) |
| **Kakarot (ZK-EVM)** | 🟡 Circuits ready | — | Awaits **April VM upgrade** |
| **Optimism** | 🟡 Draft PR merged | — | **FED** hard-fork |
| **Cosmos-Evmos** | 🟢 Already live | ✅ | ✅ |

---

## 🗺️ **Address Map – Avoiding L1 Collision**
- **L1 pre-compile range** → `0x00…0001` – `0x00…00FF` (255 slots)  
- **RIP-dedicated range** → `0x00…0100` – `0x00…01FF` (next 256)  
  - RIP-7212 will live at **`0x00…0100`** (decimal 256)  
- **Rule of thumb**: pick next free slot, skip if not shipping

---

## 🛠️ **Implementation Tracking Registry**
- **JSON file** in `rip-registry` repo  
- **Tracks**:  
  - Rollup name  
  - Exact spec hash committed  
  - **Main-net** or **test-net** flag  
- **Default inclusion** – any EVM-ish chain welcome 🌐  
- **Manual PRs** to update status ✅

---

## 🧪 **Testing Strategy Matrix**
| Tool | Role | Fork or Re-use? | Notes |
|------|------|-----------------|-------|
| **ethereum/execution-spec-tests** | JSON vectors | **Fork** 📂 | Drop under `rollups/` folder |
| **Hive** | Integration | **Fork** | Add L2 EL definitions |
| **retesteth** | Deprecated | **Skip** 🚫 | EF phasing out |
| **Custom runners** | ZK-EVMs | **OK** | Must emit same JSON pre-/post-state |

---

## 💸 **Gas-Cost Chaos**
- **RIP-7212 spec** fixes price at **3 000 gas** (native) vs **~300 000 gas** (Solidity)  
- **ZK-EVMs** push back – circuits cheaper, want **different pricing**  
- **Current stance**  
  - **Exact price required** for “official” support  
  - **Higher / lower price** → ship at **different address** 🔄  
- **Future ideas**  
  - **Maximum cost ceiling** + **post-tx refund**  
  - **EOF + gas-observability removal** as long-term fix  
  - **Multi-dimensional fee market** ⏳

---

## 🧩 **Progressive Pre-compile Dream**
### 🎯 **Problem**
- Dapp wants **RIP-7212** but only **some L2s** have it natively  
- Manual address probing + fallback is painful 😖

### 💡 **Lightweight ERC / Universal Deployer**
- Deploy **Solidity fallback** at **predictable CREATE2 address**  
- Once native pre-compile ships → **swap contract code** in-place  
- **ZK rollups w/ exotic hashers** can’t use CREATE2 → need **manual deployment**

### 🔍 **Feature-Probe Pre-compile**
- **Single call** to `0x00…01FF`  
- Returns **bitmap / bitfield** of supported RIPs  
- **Building block** for **Progressive libraries** 🧱

---

## 🌌 **Verkle & Storage-Gas Tsunami**
- **Verkle target** = **Pectra +1** (likely 2025)  
- **Storage gas model** → **per-chunk cost** (code & state)  
- **Rollups may diverge**:  
  - Sparse-Merkle vs Verkle vs Patricia  
- **Visible only via gas observability** – **discouraged pattern** ⚠️

---

## 🗣️ **Open Questions Parking Lot**
- **Who decides** max pre-compile gas?  
- **Address exhaustion** policy (>256 RIPs)?  
- **Solidity fallback** – maintain audited repo?  
- **EOF-only chains** – viable or too opinionated?  

---

## 📚 **Quick Reference Cheat-Sheet**
- **GitHub**  
  - `ethereum/rips` – specs  
  - `ethereum/rip-registry` – live status JSON  
- **Tooling forks**  
  - `ethereum/execution-spec-tests` under `rollups/`  
  - `ethereum/hive` under `simulators/rollup/`  
- **Gas observability best practice**  
  - **Never hard-code** gas assumptions 📜


  # **🛠 Precompiles in Layer 2 Rollups & the RIP Process**  

---

## **1️⃣ Context & Purpose**  
**💡 Why Precompiles for L2?**  
- Precompiles = Native EVM functions implemented outside normal Solidity execution for **efficiency** and **low gas costs**.  
- RIP process brings **standardization** → ensures consistent function behavior across different L2s.  
- Aims to give developers **confidence** when deploying to multiple rollups.  

---

## **2️⃣ The RIP Lifecycle**  

| **Status**       | **Meaning** |
|------------------|-------------|
| `Draft`          | Proposal being discussed; not finalized. |
| `Review`         | Near final, under active feedback. |
| `Finalized`      | **Locked specification**, safe to implement; no further changes allowed. |

**📌 Example:**  
- **RIP-7212**: The *only* finalized RIP with a new precompile assigned — Address **256**.

---

## **3️⃣ Current Implementation Status**  
**✅ Adopted or In Progress:**  
1. **Polygon**  
   - PIP-207 → Included in *Nopo* hard fork.  
   - Testnet live, mainnet shipping soon.  

2. **Caisync**  
   - Precompile + circuits ready.  
   - Awaiting VM upgrade (ETA April).  

3. **Optimism**  
   - Draft PR implemented.  
   - Scheduled for Fed hard fork.  

4. **Evmos (Cosmos)**  
   - Implementation complete.  

---

## **4️⃣ Precompile Address Management**  
**📍 Rules:**  
- **L1 Mainnet Precompiles:** 0–255 reserved.  
- **L2 RIP-Based Precompiles:** 256–511 allocated range.  
- Example: `RIP-7212` → Address `256` (First in L2 range).  

**⚠️ Guidelines:**  
- L2s *should* deploy at standardized addresses where possible.  
- Can skip addresses if feature isn’t implemented.  
- If needed, the range can be expanded to prevent collisions.  

---

## **5️⃣ Rollup Inclusion in the Global Registry**  
**🗂 Policy:**  
- _Default = Inclusion over Exclusion_ — anything resembling a rollup gets listed.  
- Covers even EVM-compatible chains *outside* of typical L2 scope.  

**📄 Registry Contents:**
- Only finalized RIPs.  
- Records which chains have shipped each RIP **to mainnet** (optionally testnet).  
- Stored as a `JSON` file in RIP repository:  

```json
[
  {
    "RIP": "7212",
    "implementations": {
      "Polygon": "Mainnet Pending",
      "Caisync": "Awaiting VM Upgrade",
      "Optimism": "Hardfork Scheduled"
    }
  }
]
```

---

## **6️⃣ Deployment Tracking Philosophy**
- **Two Options:**
  1. 🔍 *Stage-by-Stage Tracking* → Useful but high maintenance.  
  2. 🚀 *Only Mainnet Go-Live Tracking* → Simpler, avoids excessive policing.  

- **Decision:** Track **Mainnet Live** status (Testnets only if targeted use case exists).  

---

## **7️⃣ Cross-Layer Compatibility Challenges**  

> **Question:** How can an app still work seamlessly if an L2 *doesn’t* support a certain precompile?  

**Options:**  
1. **Manual Fallback**  
   - Each contract checks precompile existence → falls back to Solidity equivalent.  

2. **Lightweight ERC Standard**  
   - Community-maintained Solidity version with shared deployment addresses.  

3. **Progressive Precompile Model**  
   - Start with Solidity deployed at a fixed address → replace with native precompile when available.  

**🚫 Issue:** Some ZK-rollups have different `CREATE2` hash logic → identical addresses not always possible.

---

## **8️⃣ Capability Discovery Precompile**  
**Proposal:**  
- Add a special precompile to **query supported features**.  

Example:
```solidity
bool isSupported = CapabilityQuery(7212);
```
- Saves gas by checking feature availability **once** for multiple calls.  
- Candidate for a future **dedicated RIP**.

---

## **9️⃣ Gas Pricing Considerations**  
- **Rule:** RIP specifies gas cost → Rollup should match it.  
- **Reality:** Some may adjust gas pricing for their execution model.  

**💬 Potential Approach:**
- Categorize implementations:
  - **Exact Gas Cost Match**  
  - **Different Pricing** tier (documented).  

**Impact Risks:**
- Apps sometimes depend on gas costs → unexpected changes can break logic.  
- Disclaimer for developers: Gas costs **may differ** across rollups.  

---

## **🔟 Verkle Trees & Storage Model Shifts**  

**🌳 Verkle Upgrade Changes:**  
- Gas changes for storage → cost is **per data chunk**.  
- Code loading may also be more expensive.  

**Implications:**
- Rollups adopting **Verkle** will have different gas profiles.  
- Others may remain on legacy storage model → gas variability across chains.  

---

## **1️⃣1️⃣ Efficiency vs Universality Debate**  

| **Approach** | **Advantages** | **Drawbacks** |
|--------------|---------------|---------------|
| **Universal Solidity Fallbacks** | Works everywhere; ensures feature availability | Can be *10–20x more expensive*, impractical for gas-sensitive apps |
| **Native Precompile Only** | Low gas, high efficiency | Limited to L2s that implement the feature |
| **Hybrid (Progressive Precompile)** | Transition path from Solidity → Native | Requires careful address standardization |

**Example – RIP-7212:**  
- Native cost: ~`3,000–4,000` gas.  
- Solidity equivalent: ~`300,000` gas.  

---

