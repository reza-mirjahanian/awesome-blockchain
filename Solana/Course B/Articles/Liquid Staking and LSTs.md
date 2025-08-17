# 🏄‍♂️ Solana LSTs Crash-Course

> **LST** = Liquid Staking Token  
> Stake stays earning, tokens stay spendable 💸

---

## 🧩 LST Types on Solana
1. **Stake-Pool LSTs**  
   • Deposit SOL → get `spSOL`  
   • Pool auto-splits stake across many validators  
   • Example: *JitoSOL* (prioritizes MEV-friendly nodes)

2. **Validator LSTs**  
   • Deposit SOL → get `v_lstSOL`  
   • All stake goes to **one** validator  
   • Zero fees, faster rewards calc  
   • Extras: loyalty drops, no-loss lotteries 🎁

---

## 🧮 Token Mechanics
- **Design choices**:  
  • *Reward-bearing* (price grows) ✅ Solana only has this  
  • *Rebasing* (quantity grows) ❌ none yet

---

## 📊 Adoption Snapshot
| Chain | LST Share of Stake |
|-------|--------------------|
| Ethereum | ~70% |
| Solana | <5% |

**Why the gap?**  
- Path-dependence & network effects on ETH  
- Solana DeFi still bootstrapping  
- Big holders prefer own validator (legal/regulatory)  
- Users just learning LSTs 🤓

---

## 🏗️ DeFi Flywheel
- **Problem**: High DeFi lending rates compete with staking → security risk  
- **Solution**: LSTs let users **stake + lend** at once → deeper DeFi liquidity  
- After Jito/Jupiter airdrops: 📈 volume & activity spike

---

## 🌊 Sanctum & the Long-Tail
- **Sanctum Reserve**: >200 k SOL instant-unstake pool  
- **Sanctum Router**: swap any LST via Jupiter  
- **Sanctum Infinity**: coming soon for infinite-LST future ♾️

---

## 🔍 LST Models vs Ethereum
| Model | Example | Key Traits |
|-------|---------|------------|
| **Permissioned Operators** | stETH | 30 curated nodes, no collateral |
| **Permissionless Operators** | rETH | 8 ETH + RPL bond to run node |
| **Centralized Operator** | cbETH | Coinbase runs nodes for you |

---

## ⚙️ Governance & Risks Today
- **Stake pools**: multisig + **staker authority** (can redelegate, raise fees after 1 epoch)  
- **No programmatic slashing** on Solana yet 🛡️  
- Future: watch for slashing & two-tier staking ideas