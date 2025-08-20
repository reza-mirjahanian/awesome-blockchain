---

## 🧠 Foresight: Decentralized Prediction Market on Solana x Serum

---

### 🌟 **Project Vision & Why CLOB Over AMM?**

* **Goal:** Build a *decentralized prediction market* utilizing a *central limit order book (CLOB)* instead of AMM.
  - *Prediction market*: Trade outcomes on future events.
  - *Central Limit Order Book (CLOB)*: A matching system where users post buy/sell orders at specific prices.

#### ❓ Why Not Use AMMs for Prediction Markets?
- **AMM LP Shares** behave like complex financial products—difficult for general users.
- *If you LP in a prediction market AMM*:
  - Bet evenly (e.g., 50/50 on YES/NO): If one outcome happens, your LP shares' value can drop to zero.
  - Timing is crucial: Need to add/remove liquidity at the right moments.
- **Fee structure problem:**  
  - AMM platforms often charge 2% per trade, which wipes out most small edges.
  - *Compared to Betfair/Predicted*: Small spreads, only profits are taxed.
- **AMM Risk:**  
  - As the binary event nears resolution, LPs are forced to withdraw, or risk full loss.
  - Final moments: No one provides liquidity, and prices get stuck.

---

### 🏦 **Central Limit Order Book Advantages**

- **Sophisticated Market Making**:  
  - *Market makers* set exposure profiles and risk levels directly.
  - *Exposure*: How much they win/lose per outcome.

- **Lower Fee Model:**  
  - No repeated trading fees; only pay on net profits.
  - *Example*: Like Betfair—*You only get charged if you win.*

- **Flexible Trading:**
  - Secondary market: Winning shares can be sold for close to $1 (e.g., $.99) before official settlement.
  - *Liquidity*: Better for participants needing early exit.

---

### 🕹 **Demo UI – Prediction Market Experience**

- **Non-functional** (Demo): UI shows how to:
  - Bet on outcomes (soccer matches, Atlantis discovered, movie releases).
  - View the real order book per market.
  - Buy/sell YES or NO shares directly.

#### **Example Bet:**
1. *Buy* 1000 "YES Arsenal beats Manchester United" shares.
2. *Cost*: $370.
3. If Arsenal wins → $1,000 payout (*$630 profit*)
4. If loses → $0 (full loss).
5. *Can sell before the game ends* (if market value increases).

---

### 📈 **Order Matching & Market Structure**

- **One market per event**:  
  - Single order book (no separate YES/NO pools).
  - Simplified user flow: No minting/redeeming required.
- **Order Types Available:**
  - Market orders (instant fill at best price).
  - Limit orders (set your price).

#### **Trading API:**
- Programmatic access, like Mango.
- *Users interact via open orders accounts that abstract holdings and positions.*

```rust
// Pseudo-code: Placing an Order in Anchor (Rust)
use anchor_lang::prelude::*;
use serum_dex::instruction::NewOrderInstructionV3;

fn place_order(ctx: Context<PlaceOrder>, ...params...) -> ProgramResult {
    // Validate input...
    // Interact with Serum orderbook
    invoke(
        &serum_dex::instruction::new_order_v3(
            &serum_market_id,
            &open_orders_account,
            ...order_params...
        ),
        ...
    )
}
```

---

### 🛠 **Settlement & Oracle Design**

- **Current Approach:**  
  - Near-term: Use a DAO/voting mechanism with staked governance tokens.
  - Long-term: Move towards external, decentralized on-chain oracles.
    - Direct data feeds from verified APIs.
    - *Goal*: No central authority, rapid resolution, fair outcomes.

- **Challenge:**  
  - Existing oracle solutions (e.g., Augur) are *slow* (~months to resolve).
  - *User experience requirement*: Fast and reliable outcome settlement.

---

### 👉 **Developer Stack & Recommendations**

- **Solana + Serum + Anchor Framework**
  - *Solana*: High throughput blockchain for speed/scale.
  - *Serum*: Order book DEX backbone.
  - *Anchor*: Rust-based smart contract (program) development made easy.

- **Why Anchor?**  
  - Simplifies smart contract logic.
  - Easier for new devs.
  - Well-supported by the ecosystem.

```shell
# Anchor scaffold: Initialize a new project
anchor init foresight-prediction-market
cd foresight-prediction-market
anchor build
anchor deploy
```

---

### 📅 **Development Timeline & Community**

- **Launch Timeline:**
  - *Devnet version*: ~1 month
  - *Mainnet*: 1-2 months

- **Community Tools:**
  - Twitter: **@forsightMKT**
  - Contributor-friendly; will open community channels soon.

- **Contribution Invites:**  
  - Open to developer contributions, community testers, feedback on features.

---

### 🚀 **Future Features & Experiments**

- **Token Launch Innovations**:
  - *Example*: “Vickrey Auction” for fair token distribution (sealed bids, all at clearing price).
  - Seeks to avoid unfair launches (wallet farming, "button race” scenarios).

- **Multi-Bracket Markets:**
  - Not just YES/NO; e.g., “Who will be the next mayor of New York?” (multiple outcomes tradable).

- **UX Focus:**  
  - Hide token mint/burn/redeem mechanics—*make it feel like simple trading*, not DeFi puzzles.

---

### ⚡ **Real-World & Ecosystem Context**

- **Current DeFi PM Challenges:**
  - Even *Vitalic Buterin* found using Augur confusing—high complexity, poor UX.
  - "Mint vs. buy" dilemma: Should you buy your preferred token, or mint all and sell the rest? (Not obvious, creates friction.)
- **Goal:**  
  - Ditch unnecessary complexity—users interact as with traditional markets.

---

### 🔗 **Relevant Concepts & Side Info**

- **Prediction Market**:  
  - *A mechanism that aggregates collective knowledge by letting users bet on future events*.
  - Used in everything from politics to sports to crypto events.

- **Serum DEX**:  
  - *On-chain, high-performance order book*, composable with other Solana dApps.

- **DAO-Based Resolution**:  
  - *DAO*: *Decentralized Autonomous Organization*, whose token holders vote on system parameters/outcomes.
  - *Can also be used to judge contentious event outcomes, where oracles are limited*.

---

### 💡 **Code Snippet: Simulating Automatic Order Matching**

```typescript
// TypeScript: Matching YES/NO Orders in Backend
function matchOrders(buyOrders, sellOrders) {
  const trades = [];
  while(buyOrders.length && sellOrders.length && buyOrders[0].price >= sellOrders[0].price){
    const tradePrice = (buyOrders[0].price + sellOrders[0].price) / 2;
    const qty = Math.min(buyOrders[0].qty, sellOrders[0].qty);
    trades.push({price: tradePrice, qty});
    buyOrders[0].qty -= qty;
    sellOrders[0].qty -= qty;
    if (buyOrders[0].qty === 0) buyOrders.shift();
    if (sellOrders[0].qty === 0) sellOrders.shift();
  }
  return trades;
}
```

---

### 🧬 **Design Principles & User Philosophy**

- **No complex liquidity games or yield traps**
- *Serve real betters/traders*—not just DeFi speculators.
- *Emphasize speed, fairness, and clarity*—no insider advantage.

---

### 🛡 **Security & Audit Focus**

- **Order book logic**: Well-abstracted via Serum’s proven infrastructure.
- **Custom business logic** (mint/redeem YES/NO shares, settlement): Being kept clean/minimal for easy review.

---

### 🗂 **Supplementary Ecosystem Tools**

- **Serum Tx History Tracker:**  
  - View historical trade data for all Serum markets.
  - *Open-source repo, useful for analysts and devs.*

---

### 🌍 **Get Involved & Stay Updated**

- Follow project updates & participate via Twitter.
- Early community builders will help shape governance and oracles.
- *Direct feedback welcomed to address real-world DeFi market user pain points.*

---