# 🚨 ACDE #199 – **Pectra**, **BLS Pre-compiles**, & **Block-Gas-Limit Drama**  


---

## 🧪 **Pectra Devnet Status Dashboard**
| Devnet | Status | Sunset Date | Notes |
|--------|--------|-------------|-------|
| **Pectra Devnet-3** | 🔴 **Offline** | Oct 18 | Debugging complete |
| **Pectra Devnet-4** | 🟢 **Live since Oct-18** | TBD | EF-Ops, Teku, Lodestar, Nethermind actively patching |
| **Pectra Devnet-5** | 🚧 **Pending** | Post-Devcon | EIP-2537 gas re-pricing + misc fixes |

---

## 🎯 **Open Question: Public Pectra Testnet @ Devcon?**
- **Goal floated on prior calls** → public testnet for Devcon (Nov 12-15)  
- **ACDE-199 outcome** → **zero mention** 🙊  
- **Market implied odds** 📉 slim-to-none

---

## 🔑 **EIP-2537 – BLS12-381 Pre-compile Rollercoaster**

### 🧰 **What it Does**
- Adds **native BLS curve ops** to EVM  
- Enables:  
  - 🔐 **Signature aggregation** (like Beacon Chain)  
  - 🧮 **Cheaper ZK-proof verification**  
- **Missing piece** for Ethereum-native BLS wallets & rollups

### 💰 **Gas-Pricing Saga**
- **Aug proposal** → 2× across-the-board hike  
- **Paul Bica’s dissent** → “Trash the baselines, start from scratch”  
- **Oct pivot** → full **re-benchmarking** cycle w/ all client teams 🔄

### 🪞 **Kev’s Reality Check**
> “Apps already entrenched on BN254—**will anyone migrate?**”  
- **Risk**: ghost-town pre-compile after multi-month dev effort 👻  
- **Mitigation**: outreach sprint to dApp teams promised by EF researchers

---

## ⛓️ **Related Cryptographic Curves Cheat-Sheet**
| Curve | Pairing Friendly? | Ethereum Today | Notes |
|-------|-------------------|----------------|-------|
| **BN254** | ✅ | Pre-compiled at 0x08 | zk-SNARKs (Groth16) |
| **BLS12-381** | ✅ | EIP-2537 (Pectra) | Beacon Chain native |
| **BLS12-377** | ✅ | Not in EVM | Used in some ZK-rollups |
| **BW6-761** | ✅ | Research only | Cycles w/ BLS12-377 |

---

## 🧱 **Block-Gas-Limit Debate – TL;DR**

### 📊 **Current Numbers**
- **Target** = 15 M gas  
- **Hard cap** = 30 M gas  
- **Proposal** = ramp to **60 M over 2 yrs** via **EIP-7783**

### 🔧 **EIP-7783 Mechanics**
- **Linear ramp** (~0.65 % / day)  
- **No hard-fork required** – validator-set coordination only  
- **Rollback trigger** if network health degrades 📉

### 🎙️ **Pushback Bingo Card**
| Objection | Who | Core Argument |
|-----------|-----|---------------|
| **Missing data** | Multiple | “Show us the benchmarks!” |
| **Call-data spam** | Tony | **EIP-7623** must land first |
| **History bloat** | Lightclient | **History Expiry** prerequisite |
| **Home staker doom** | Chorus | ARM boxes & 1 Gbps links at risk |

### 🗳️ **Validator Agency Moment**
- **Tim’s mic-drop** → “Protocol devs **don’t control** gas limit—**validators do**.”  
- **Precedent**: miners raised limit 3× (2016-2021)  
- **Post-Merge**: no validator-led increases yet; **economic pressure rising** 📈

---

## 🔮 **Adjacent EIP Watch-List**
| EIP | Nickname | Status | Blocker |
|-----|----------|--------|---------|
| **EIP-7742** | Blob scaling helper | ✅ In Pectra | None |
| **EIP-7623** | Call-data cost bump | 🔍 Draft | Tony’s review |
| **EIP-7790** | Gradual gas-limit tool | 🔄 Replaced by 7783 | Obsoleted |
| **EIP-7002** | Max EB increase | 🐛 Bugs keep coming | More audits |

---

## 🏠 **Home Staker Survival Kit (Unofficial)**
- **Hardware floor**  
  - 8-core CPU  
  - 32 GB RAM  
  - 2 TB NVMe (gen 4)  
  - 1 Gbps symmetrical fiber  
- **Software tips**  
  - Enable **snap sync** + **state expiry** flags early  
  - Run **Nethermind + Lighthouse** combo for lightest footprint 🦾

---

## 🎉 **Milestone Corner**
- **ACDE-200** incoming 🎂  
- **Ethereum roadmap reflections**: 200 fortnightly open calls → longest-running live governance experiment in crypto 📜


# 🌿 Ethereum ACDE #199 Key Discussion Points

## 🧪 Pectra Devnet Updates

### Current Devnet Status
- **Pectra Devnet 4** officially launched on *October 18th*
- Devnet 3 has been **shut down**
- Active debugging happening across multiple clients:
  - Aragon
  - Ethereum JS
  - Grandine

### Upcoming Devnet Plans
- Development teams focusing on **Devnet 4** for the next *1-2 weeks*
- Then shifting attention to **Devnet 5** with additional code changes
- Notable missing discussion: *Public Pectra testnet for Devcon* (November 12-15)
  - Previous calls mentioned this as a goal for developer testing at the conference
  - No updates provided in this meeting about public testnet plans

## 🔐 BLS Precompiles Discussion (EIP-2537)

### What Are BLS Precompiles?
- Precompile for **BLS 12-381 curve operations**
- Enables smart contract developers to utilize a *new cryptographic signature scheme*
- BLS curve is **already used on the Beacon Chain** for validator signature aggregation
- Would make:
  - Signature aggregation more efficient
  - Zero-knowledge proof generation more efficient

### ⚠️ Pricing Controversy
- Ongoing debate about appropriate pricing for BLS operations
- Previous proposal suggested *doubling costs* across the board
- **Paul (Barnabas)** argued:
  > "The baseline costs that were given for the EIP should be redone from scratch... rather than increasing the cost of these operations 2x across the board, start off with completely new baseline cost estimates"

### 🚩 Major Concern: Missing Use Cases
- Developer **Kev** raised critical question:
  > "Some of the app developers have infrastructure that make it pretty hard to switch away from BM 254 to BLS 12-381, so I'm not sure what the use case for this pre-compile is"
- Significant concern that:
  - Years of developer effort might address a need that *doesn't exist*
  - Implementation work is progressing without confirmed application demand
- Other developers committed to:
  - Reach out to application developers
  - Verify real-world use cases before final implementation

## ⛓️ Block Gas Limit Increase Debate

### Current Gas Limit Structure
- Ethereum blocks currently have a **30 million gas limit**
- Gas measures *computational work* required for operations
- Gas is converted to `gwei` (1 billionth of ETH) for transaction pricing
- Limit prevents blocks from becoming too large for network propagation

### Proposed Changes
- **EIP-7790**: Proposed by *Julio Ruffo* (Aragon developer)
  - Creates mechanism for *gradual* block gas limit increases
  - Linear increase approach (vs. previous "cliff-like" jumps)
  - Would allow better network health monitoring during increases
- **EIP-7783**: Follow-up proposal to implement a specific increase
  - Suggests *doubling* block gas limit from 30M to 60M
  - Could be implemented *after Pectra upgrade*
  - Doesn't require hard fork (asynchronous implementation possible)

### 📢 Developer Concerns & Counterpoints

#### Major Objections
- **Insufficient research** on appropriate increase amount
- **Tony Aladashvili** (Ethereum Foundation):
  > "Block gas limit increase shouldn't be implemented until developers also implement EIP-7623 which would increase the cost of call data"
- **Light Client** (Geth developer):
  > "If there is going to be a block gas limit increase, validators should first make progress on History Expiry"
- Concerns about impact on **home stakers** using resource-constrained devices

#### Validator Agency Insight
- **Tim** (call chair) made critical observation:
  > "It's not that big of a deal if developers cannot come to an agreement about these EIPs related to the block gas limit because at the end of the day, the block gas limit is something that Ethereum protocol developers don't control"
- Key clarification:
  - Block proposers (validators) ultimately control the gas limit
  - Validators can coordinate increases based on network demand
  - Historical precedent: Miners increased gas limits multiple times pre-Merge
- Opportunity for **staking community** to consider:
  - Whether to coordinate a gradual gas limit increase
  - Potential scalability benefits vs. home staker concerns
  - Most validators operate on professional-grade hardware