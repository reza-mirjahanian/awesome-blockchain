# 💸 Priority Fees on Solana – Quick Guide

> Skip the queue by **tipping the network**.

---

## 🧭 Transaction Lifecycle in 3 Steps
1. **Create** – user builds & signs tx  
2. **Send** – RPC forwards to current + next 2 leaders  
3. **Execute** – leader schedules → runs → propagates via **Turbine**

---

## 🧮 Fee Formula
```
totalFee = baseFee + priorityFee
priorityFee = computeBudget × computeUnitPrice (µLAMP)
```

| Component | Default | Max |
|-----------|---------|-----|
| **Base fee** | 5 000–10 000 lamports / sig | — |
| **Compute budget** | 200 k CU | 1.4 M CU / tx |
| **Block CU limit** | — | 48 M CU |

---

## 🛠️ How to Add Priority Fees (JS)

```js
import { ComputeBudgetProgram } from "@solana/web3.js";

const priorityIx = [
  ComputeBudgetProgram.setComputeUnitPrice({ microLamports: 10_000 }),
  ComputeBudgetProgram.setComputeUnitLimit({ units: 300_000 }),
  // ...your actual instructions
];

const tx = new Transaction().add(...priorityIx);
```

> 🔝 **Order matters** – put `setComputeUnitLimit` **before** heavy instructions.

---

## 🎯 Best Practices
- 🪜 **Always** set limit before heavy ops  
- 📊 Use `getRecentPrioritizationFees` or Helius `getPriorityFeeEstimate` for live µLAMP suggestions  
- 🪄 Let wallets auto-fee when possible (Phantom, Solflare)  
- 🧹 Request only the CU you need – unused CU isn’t refunded

---

## ⚡ Helius Priority Fee API Cheat
```js
const fee = await fetch(rpc, {
  method: "getPriorityFeeEstimate",
  params: [{
    accountKeys: ["<program-id>"],
    options: { includeAllPriorityFeeLevels: true }
  }]
});
```

---

## 🪄 TL;DR
- **Priority fees = optional tip for faster inclusion**  
- **Compute budget** caps runtime work  
- **Tiny JS snippet** = instant speed-up