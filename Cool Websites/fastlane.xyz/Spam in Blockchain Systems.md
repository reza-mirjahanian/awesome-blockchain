

# Introduction to Spam in Blockchain Systems

The speaker begins by highlighting the significance of spam as a problem in blockchain systems:

- Spam has caused crashes in various blockchain networks, including Polygon and Solana.
- The problem is expected to worsen as more chains focus on:
  - Parallel execution
  - High throughput
  - Low transaction costs

## Root Causes of Spam

Two main factors contribute to spam:

1. **Transaction sequencing**
2. **Probabilistic arbitrage**
   - Spammers flood the blockchain hoping to position their transactions advantageously
   - Often used with on-chain solving

## Factors Influencing Spam Behavior

The speaker outlines several factors that influence spamming behavior:

- Transaction count
- Win rate relative to competitors
- Price of the gas token
- Gas usage per transaction
- Expected profit

### Spam Count Calculation

The speaker presents a simplified equation for determining spam count:

```
Spam Count ~ Expected Profit / (Gas Price * Gas Used * Token Price)
```

**Key observations:**
- As gas price and gas usage decrease, spam tends to increase
- Expected profit is a complex factor influenced by win rates relative to other searchers/solvers

### Correlation with Volatility

An important insight is presented:

- Volatility is correlated with both profit opportunities and gas prices
- During high volatility, profit tends to increase faster than gas prices
- This leads to more spam, even during periods of high gas prices

**Conclusion:** Fee markets alone may not be ideal for stopping spam

# Alternative Approaches to Mitigating Spam

## Velocity of Money Concept

The speaker introduces the concept of "velocity of money" as a background for understanding potential solutions:

- **Definition:** How many times funds can change hands or be used to purchase goods/services in a given period
- **Traditional Finance:** Velocity is capped by T+2 settlement
- **DeFi:** Velocity is virtually infinite due to immediate settlement and atomic transactions

### Implications for Spam

- Stable coin velocity in DeFi is 20-30 times higher than regular US Dollars
- High velocity reduces the capital requirements for spammers
- Spammers can quickly reuse funds after each transaction

## Cost of Capital

The speaker discusses the concept of cost of capital in relation to spam:

- **Definition:** Minimum return on investment required for profitability
- For spammers, the cost of capital is relatively low due to high velocity

## Proposed Solutions

The speaker suggests two main vectors for decreasing spam:

1. **Increase capital requirements**
2. **Decrease capital velocity**

### Implementation Ideas

1. **Reserve requirement:** Set aside more funds than the actual fee (e.g., 10x the fee)
2. **Lockup period:** Prevent reuse of funds for a certain number of blocks

### Benefits of This Approach

- Doesn't increase fees for regular users
- Disproportionately affects spammers who rely on rapid transaction sequences
- Can be parameterized to achieve desired spam reduction levels

## Case Study: Monad Blockchain

The speaker highlights Monad as an example of a blockchain implementing similar concepts:

- Separates inclusion/sequencing from execution
- Reserves max gas * gas price in block N
- Calculates actual gas used in block N+1
- Releases unused funds in block N+Z (where Z is a set number of blocks, possibly 10)

**Result:** Higher capital requirements and decreased velocity for potential spammers

# Multi-dimensional Fee Markets

The speaker introduces the concept of multi-dimensional fee markets as a potential solution:

- Focus on cost of capital rather than just fees
- Allows for targeted impact on spammers while minimizing effect on regular users

## User Behavior vs. Spammer Behavior

- **Regular users:**
  - Typically slow
  - Use front-ends
  - Wait for transaction results before initiating new transactions
- **Spammers:**
  - Extremely fast
  - Use algorithms instead of front-ends
  - Send multiple transactions without waiting for results

## Parameterization Challenges

The speaker presents a hypothetical scenario to illustrate the potential impact:

- Expected profit: $100
- Staking yield: 7%
- Block duration: 12 seconds
- Token price: $2000

By adjusting parameters such as:
- Number of blocks funds are locked
- Capital requirement multiplier

The speaker suggests it's possible to achieve significant spam reduction (e.g., 93%) without increasing fees for regular users.

# Conclusion and Future Directions

The speaker concludes by acknowledging the challenges in finding the right balance:

- Extreme parameters could negatively impact user experience
- Minimal parameters may not significantly reduce spam

**Proposed solution:** Develop dynamic parameters that adjust based on network conditions and spam levels

The speaker expresses interest in further exploring these concepts, particularly in relation to the Monad blockchain ecosystem and decentralized MEV (Maximal Extractable Value) solutions.