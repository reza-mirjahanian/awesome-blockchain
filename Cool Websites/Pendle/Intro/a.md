# 📈 Real Estate Investing Without Ownership

## *Introducing Yield Tokenization via Pendle*

---

# 💥 Real Estate Market Is Booming

## *But You Only Have \$1,000—Now What?*

* **Rental prices are skyrocketing**
* **Traditional property ownership** requires high capital
* *Can you collect rent without owning property?*
  **Yes — with Pendle & Yield Tokenization**

---

# 🔮 What Is Yield Tokenization?

## *Turning Yield-Bearing Assets Into Tradeable Parts*

* **Yield-bearing asset**: *An asset that produces income over time*

  * 🏠 *Example*: A rental property earns monthly rent
* Yield tokenization **splits this asset into two components**:

  * **Principal** – the right to own the underlying asset
  * **Yield** – the right to receive the income it generates

```text
Total Asset Value = Principal Value + Yield Value
```

---

# ⚖️ Analogy: Tokenizing a Rental Property

* Property Value: **\$105,000**
* After tokenization:

  * **PT (Principal Token)**: Right to claim property after 1 year → **Buy for \$100,000**
  * **YT (Yield Token)**: Right to collect 1 year of rent → **Buy for \$5,000**

*Result: Rent and ownership rights are separated and independently tradable.*

---

# 🔐 What Is a PT (Principal Token)?

## *Earn Fixed Yield by Buying at a Discount*

### Example with stETH:

* Market price of **stETH** = `1 ETH`
* Buy **PT-stETH** for `0.9 ETH` with 1-year maturity

```text
Hold PT-stETH → Redeem 1 ETH after 1 year
Profit: 1.0 - 0.9 = 0.1 ETH (~11% APY)
```

✅ *Principal is locked until maturity*
💼 *Behaves like a zero-coupon bond*

---

# 🎯 Understanding Fixed Yield

* **Value appreciation** from discounted PT to full asset value
* Predictable returns — *no exposure to yield volatility*

```text
Fixed Yield = (Redemption Value - Purchase Price) / Purchase Price
```

---

# 💸 What Is a YT (Yield Token)?

## *Capture Variable Income from the Asset*

### Back to the rental property example:

* Buy the **right to receive rent** for 1 year for **\$5,000**
* Rent fluctuates:

  * *If total rent > \$5,000 → You profit*
  * *If total rent < \$5,000 → You lose*

---

# 🔁 YT in Pendle: Real Example with stETH

* Buy **YT-stETH** for `0.1 ETH`
* Earn stETH yield over 1 year (variable)
* *Claim rewards throughout the period*

```text
If total rewards > 0.1 ETH → Profit
If total rewards < 0.1 ETH → Loss
```

🌀 Yield is **not fixed** — depends on staking returns
📆 Valid only *until maturity date*

---

# 🔓 Exit Anytime — No Lock-In!

## *Your Tokens, Your Choice*

* PT and YT are **ERC-20 tokens**
* **Fully liquid**: Trade anytime on Pendle’s secondary market
* Take profit early or cut losses — *you’re never stuck*

```text
Sell PT or YT on Pendle DEX before maturity
```

---

# 🧠 Summary of Concepts

## Yield Tokenization = Asset Split

| Component | Symbol | What You Get           | Risk/Reward                          |
| --------- | ------ | ---------------------- | ------------------------------------ |
| Principal | PT     | Full asset at maturity | **Fixed yield**, no variable rewards |
| Yield     | YT     | Ongoing income stream  | **Variable yield**, higher risk      |

🔁 Combine PT & YT to reconstruct the full asset
💱 Trade each part independently to suit your strategy

---

# 🛠️ Optional Code Insight: Interacting with PT/YT in Solidity

```solidity
IERC20 ptToken = IERC20(PT_ADDRESS);
IERC20 ytToken = IERC20(YT_ADDRESS);

// Claim yield
ytToken.transferFrom(user, contract, amount);
yieldDistributor.claimYield(user);

// Redeem principal at maturity
require(block.timestamp >= maturityDate);
ptToken.transferFrom(user, contract, amount);
underlyingAsset.transfer(user, amount);
```

---

# 📚 Topics To Explore Next

* **Yield Optimization Strategies**
* **Pendle Automated Market Maker (AMM)**
* **Speculating on Future Yield Curves**
* **Composing PT/YT into DeFi protocols**

