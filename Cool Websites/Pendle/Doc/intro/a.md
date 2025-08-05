# 💰 Understanding Yield with Pendle

---

## ❓ How Much Will You Earn Lending 1,000 USDC?

* **1%? 3%? 5%?**
* *Answer: You can't say for sure.*

### Why?

* **Yield fluctuates** just like token prices.
* Influenced by:

  * **Market cycles**: Bull = High Yield, Bear = Low Yield
  * **Liquidity conditions**, **demand/supply of borrowing**, **protocol incentives**, etc.

---

## 📉 Historical Yield Volatility

* Yield on platforms like **Compound** and **Aave** can vary dramatically.

* Example chart:

  ```plaintext
  2020 → USDC APY ~12%
  2021 → USDC APY ~1–2%
  2022 → USDC APY ~5%
  ```

* 📊 *Yield ≠ Constant. It’s dynamic and market-driven.*

---

# 🚀 Enter Pendle: Control Your Yield

## What Does Pendle Do?

* **Pendle is a permissionless yield-trading protocol.**
* *Empowers users to manage, speculate, or hedge their yield.*

### Key Benefit:

> **Maximize** yield in bull markets
> **Hedge** yield risk in bear markets

---

# 🔍 Pendle Core Concepts

---

## 1. 🔗 Yield Tokenization

### How It Works:

1. Start with a **yield-bearing token** (e.g. `stETH`)
2. Wrap it into **SY (Standardized Yield Token)**:

   ```plaintext
   stETH → SY-stETH
   ```
3. **Split** SY into:

   * **PT (Principal Token)** – claim base asset at maturity
   * **YT (Yield Token)** – claim all future yield until maturity

### Result:

```plaintext
SY-stETH → PT-stETH + YT-stETH
```

* You now *own the yield separately* from the principal.

---

## 2. ⚖️ Pendle AMM (Automated Market Maker)

* Facilitates **trading of PT and YT**.
* Users can:

  * Buy/sell **PT** to lock in **fixed yield**
  * Buy/sell **YT** to speculate on **future yield**
* *No need to understand AMM mechanics to trade.*

---

## 3. 🏛️ vePENDLE: Governance + Boosts

* **vePENDLE** = Locked PENDLE tokens
* Provides:

  * **Voting rights**
  * **Boosted yields** (especially for LPs)
  * **Influence** over emissions and incentives

---

# 🎯 Strategies Enabled by Pendle

---

## ✅ Strategy 1: Fixed Yield

* **Buy PT (Principal Token)**
* Hold until maturity
* *Know your return ahead of time*

```plaintext
Buy PT-stETH → Receive stETH at maturity → Lock in fixed APY
```

Great for:

* **Risk-averse** users
* **Whales** seeking predictable returns

---

## 📈 Strategy 2: Long Yield

* **Buy YT (Yield Token)**
* Get full exposure to **all future yield**
* *Bet on yield going up*

```plaintext
Buy YT-stETH → Receive all stETH yield until maturity
```

Higher potential returns, but:

* **Value decays over time**
* **Higher risk** if yield drops

---

## 🧪 Strategy 3: Enhanced Yield via LP

* **Provide liquidity** using a pair like `PT-stETH` + `SY-stETH`
* Earn:

  * **Fixed yield (from PT)**
  * **Floating yield + rewards (from SY)**
  * **Swap fees**
  * **PENDLE incentives**

### Optional Boosts:

* Enable **zero price impact mode** (preserves point exposure)
* Lock PENDLE or use **Liquid Lockers** (e.g. **Penpie**, **Equilibria**) to boost returns

---

## 🧩 Strategy 4: Combine & Customize

* Mix strategies for:

  * Hedging
  * Exposure balancing
  * Liquidity farming
* Example:

  ```plaintext
  50% PT for fixed income
  30% YT for yield speculation
  20% LP for additional yield stacking
  ```

---

## 🌐 Pendle = DeFi’s Yield Derivative Layer

* Mirrors **TradFi interest derivatives**, now in DeFi
* Unlocks access to a \$400T+ market logic
* Available to *anyone with DeFi access*

---
