# 🚀 Advanced Yield Trading Strategies on Pendle

---

## 📈 Strategy #1: Buy **Principal Tokens (PTs)** Instead of Spot

### Why PTs?

* If you're already *long* on an asset (e.g., **ETH**)…

  * Swap spot ETH for **PT-stETH** on Pendle to:

    * ✅ **Outperform** ETH at maturity.
    * ✅ Get **downside protection** from ETH price drops.

### Key Benefits

* **Fixed yield** on your underlying asset.
* **Market risk ≈ spot holding**, but with yield upside.
* **More ETH at maturity** if prices decline.

> *Applies to all supported assets—not just ETH.*

---

## 🔁 Strategy #2: **Delta Neutral Yield Farming**

### What is Delta Neutral?

*A position that cancels out price exposure by balancing long and short exposure.*

### Example Setup

1. **Long Position**: Provide liquidity on Pendle (LP = similar to spot exposure).
2. **Short Position**: Short the same asset (e.g., ETH) on a centralized exchange.

```text
If ETH goes up ➡️ LP gains, short loses  
If ETH goes down ➡️ LP loses, short gains  
Net effect = 0 price exposure
```

### What You Earn

* 💰 LP Yield from Pendle
* 💸 Funding rates from your short position

> *Yields on top of yields—all with zero market exposure.*

---

## ⚡ Strategy #3: Use **Principal Tokens as Collateral**

### Why It Matters

Pendle PTs are integrated with many DeFi money markets.

### How It Works

1. Deposit PTs as **collateral**.
2. Borrow **ETH or USDC**.
3. Optionally:

   * Reinvest in more PTs → *Leveraged fixed yield*
   * Use capital elsewhere (hedging, new positions, etc.)

```text
Looping Strategy:
PT → Borrow ETH → Buy more PT → Repeat
```

⚠️ **Risks increase** with each loop. Manage your **liquidation risk** wisely.

---

## 🧠 Strategy #4: Smart Execution with **Limit Orders**

### Pendle’s Built-in Limit Order Feature

* Works like traditional exchanges
* **No price impact**
* Ideal for **large trades**

### Benefits

* Set PT or YT orders at a desired **implied APY**
* Automatically matched via AMM for best results

### Example: Yield Swing Trading with stETH

If you expect yield to range from **3% to 5%**, you can:

```text
🟢 Buy YT at 3% APY  
🔴 Buy PT + Sell YT at 5% APY  
```

> *Buy low, sell high. Even when the pool is out of range.*

### How To Set a Limit Order

1. Click the **Limit** tab
2. Enter:

   * Desired **Implied APY**
   * **Limit Expiry**
3. Place the order

> *Set, forget, and never miss a trade again.*

---

## 🧩 Strategy Summary

* **Buy PTs** for fixed yield + downside protection
* **Delta Neutral** setups earn yield without price risk
* **Use PTs/LPs as collateral** for leveraged positions
* **Limit Orders** offer precision and automation for trading yield

---
