# 🖥️ What Is a Validator?

> A validator is just a **computer running specialized software** that keeps the Solana network alive.

---

## 🎯 Key Roles of a Validator
- **Track every account** on the cluster  
- **Validate & vote on new transactions** before they become blocks  
- **Produce blocks** when chosen as the **leader**

---

## 🔍 Two Types of Nodes You Can Run
1. **Voting / Consensus Node**  
   - Participates in consensus  
   - Votes on blocks  
   - Requires substantial stake  
   - *This is what “validator” usually means.*

2. **RPC Node**  
   - Serves data to developers & apps  
   - **Does NOT vote** (for performance)  
   - Focus on high query throughput  

---

## 🏛️ Consensus Architecture: Proof of Stake (PoS)

> Token holders **stake** their SOL to a validator they trust.

- **Staked tokens stay yours** — you can **unstake any time**  
- **Rewards** are paid in SOL proportional to stake size  
- **Vote weight & block-production chances** ∝ stake amount  
- **Leader** = the validator currently producing blocks  

---

## ⚡ Solana ≠ Proof of Work (PoW)
| Proof of Stake (Solana) | Proof of Work (e.g., Bitcoin) |
|------------------------|------------------------------|
| Validators compete on **stake size**, **not compute power** | Miners compete on **solving cryptographic puzzles** |
| Single machine is fine | Farms of ASICs / GPUs |
| **Low energy usage** | **Massive energy consumption** |

---

## 🕒 Proof of History (PoH) — Solana’s Turbo Boost
- **Not a consensus mechanism**  
- Provides a **cryptographically verifiable clock**  
- Lets validators agree on **transaction order quickly**  
- Makes PoS finalization **blazingly fast**

---

## 🛡️ Your Responsibilities as a Validator
1. **Keep the system online 24/7**  
2. **Stay secure**  
   - Firewalls, key management, OS updates
3. **Upgrade promptly**  
   - New releases often contain critical fixes
4. **Monitor & alert**  
   - PagerDuty / Discord bots at 3 a.m.
5. **Engage in governance**  
   - Vote on network proposals  
   - Join operator forums & chats

---

## 🌱 Why Run One?
- **Decentralize** the network  
- **Earn staking rewards**  
- **Learn Solana internals** hands-on  
- **Join a passionate operator community**