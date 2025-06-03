

---

## 🔷 Foundational Concepts

### What is EIP-1559?

**EIP-1559** stands for *“Fee market change for ETH 1.x”*. It was implemented via the **London hard fork** on August 5, 2021 (block 12965000).

### Goals:
- Make gas pricing more predictable.
- Reduce overpayment by users.
- Burn part of the base fee, reducing ETH supply → potential deflation.
- Improve UX by removing first-price auction dynamics.

### Key Terminology:

| Term             | Description |
|------------------|-------------|
| **Base Fee**     | The minimum price per gas required for inclusion in a block. Dynamically adjusted per block. |
| **Priority Fee** | Optional tip paid to miners for faster inclusion. Also called `tip`. |
| **Gas Limit**    | Maximum amount of gas allowed in a block. Now has a **target** and **maximum** value. |
| **Burned ETH**   | Base fee portion of transaction is burned, not given to miners. |

---

## 🧱 Legacy Gas Pricing (Pre-EIP-1559)

Before EIP-1559, gas pricing used a **first-price auction model**:

```solidity
// Legacy transaction example
{
  "to": "0x...",
  "value": "1 ether",
  "gasPrice": "20 gwei"
}
```

- Users had to guess a competitive `gasPrice` to get included.
- Often led to overpayment or long delays during congestion.

---

## 🛠️ How EIP-1559 Works

### New Transaction Structure:

```json
{
  "to": "0x...",
  "value": "1 ether",
  "maxPriorityFeePerGas": "2 gwei",  // optional tip
  "maxFeePerGas": "20 gwei"          // max user willing to pay
}
```

The total effective fee = `min(baseFee + priorityFee, maxFee)` × gasUsed

### Base Fee Adjustment Algorithm:

Every block adjusts the base fee based on demand:

```python
if gasUsed > targetGasUsed:
    baseFee = baseFee * (1 + (delta / targetGasUsed))
else:
    baseFee = baseFee * (1 - (delta / targetGasUsed))

# delta = gasUsed - targetGasUsed
```

Where:
- `targetGasUsed = gasLimit / 2`
- `delta` determines whether the base fee increases or decreases.
- `baseFee` can increase by at most 1/8th (12.5%) per block.

### Block Gas Target vs Limit

- **Target Gas**: 15 million (default gas limit / 2)
- **Max Gas**: 30 million (hard cap)

This elasticity allows blocks to expand during high usage but still maintain average stability.

---

## 💻 Code Examples

Let’s look at real-world code examples across multiple environments: native Solidity, Go, Rust, and C++.

### 📌 Solidity (Transaction Example)

```solidity
function sendTx() public payable {
    (bool success,) = address(this).call{value: msg.value, gas: 21000}("");
    require(success);
}
```

When interacting with this function, you would specify:

```javascript
const tx = {
  to: contractAddress,
  value: ethers.utils.parseEther("0.1"),
  maxPriorityFeePerGas: ethers.utils.parseUnits("2", "gwei"),
  maxFeePerGas: ethers.utils.parseUnits("20", "gwei")
};
```

---

### 🦀 Rust (Using `ethers-rs`)

```rust
use ethers::{
    prelude::*, 
    providers::{Provider, Http}, 
    types::{transaction::eip2930::TransactionRequest, U256},
};

let provider = Provider::<Http>::try_from("https://mainnet.infura.io/v3/YOUR_INFURA_KEY").unwrap();

let tx = TransactionRequest::new()
    .to("0x742d35Cc6634C0532925a3b844Bc454E4438f44e")
    .value(1e18)
    .gas_price(20e9 as u128)
    .chain_id(1);

// Signer signs and sends the transaction
let wallet = LocalWallet::new(&mut rng);
let signed_tx = wallet.sign_transaction(&tx, &provider.get_chainid().await.unwrap()).await?;
let pending_tx = provider.send_raw_transaction(signed_tx.raw_transaction()).await?;
```

> Note: For EIP-1559 support, use `.max_fee_per_gas()` and `.max_priority_fee_per_gas()` instead of `.gas_price()`.

---

### 🟨 Go (Using `go-ethereum`)

```go
tx := types.NewTx(&types.DynamicFeeTx{
    ChainID:   big.NewInt(1),
    Nonce:     nonce,
    To:        &recipient,
    Value:     big.NewInt(1e18),
    Gas:       21000,
    GasFeeCap: big.NewInt(20e9),  // maxFeePerGas
    GasTipCap: big.NewInt(2e9),   // maxPriorityFeePerGas
    Data:      nil,
})
```

---

### 📘 C++ (libethereum Example)

Libethereum doesn't support EIP-1559 natively anymore, but here's a conceptual structure:

```cpp
evmc::message msg;
msg.kind = EVMC_CALL;
msg.to = recipient;
msg.value = 1_ether;
msg.gas = 21000;
msg.sender = sender;

// EIP-1559 fields must be handled manually
uint256 maxFeePerGas = 20_gwei;
uint256 maxPriorityFeePerGas = 2_gwei;
```

---

## 📊 Comparison with Pre-EIP-1559 Gas Model

| Feature                      | Pre-EIP-1559 (Legacy)               | Post-EIP-1559                          |
|----------------------------|--------------------------------------|----------------------------------------|
| Gas Price Field            | `gasPrice`                           | `maxFeePerGas`, `maxPriorityFeePerGas` |
| Miner Reward               | Full gasPrice × gasUsed              | Only `priorityFee` × gasUsed           |
| Predictability             | Low                                  | High                                   |
| Fee Burning                | No                                   | Yes (base fee burned)                  |
| First-price Auction        | Yes                                  | No                                     |
| Elastic Block Size         | No                                   | Yes                                    |

---

## 📈 Economic Impact

### ETH Supply Dynamics

With EIP-1559, every transaction burns some ETH:

- If issuance < burn → **deflation**
- If issuance > burn → **inflation**

Example:
- Avg block reward: ~2 ETH
- Avg burn: ~1–5 ETH/block (depends on congestion)

So, net supply change = rewards - burned

---

## 🧪 Edge Cases & Advanced Considerations

### ⚠️ When MaxFee < BaseFee

If user sets `maxFeePerGas < current baseFee`, the transaction will **not be mined** until base fee drops.

### ⏱️ Volatility During High Congestion

Even with EIP-1559, sudden spikes (like NFT mints) can cause base fee jumps up to 12.5% per block.

#### Example:

Block 1: baseFee = 100 gwei  
Block 2: full → baseFee = 112.5 gwei  
Block 3: full again → baseFee = 126.56 gwei  
... and so on.

### 🤖 Miner Behavior

Miners can’t manipulate base fee directly, but they can:
- Reorder transactions within a block.
- Include their own transactions with higher tips.
- Mine empty blocks if tips are too low.

### 🧩 Layer 2 Implications

Layer 2 rollups benefit from more predictable L1 gas prices, which helps them:
- Estimate batch submission costs better.
- Design more efficient fee markets.

---

## 🧬 EIP-1559 Variants Across Chains

| Chain        | EIP-1559 Support | Notes |
|-------------|------------------|-------|
| Ethereum    | Native (Post-London) | Core implementation |
| Binance Smart Chain | Implemented (BEP-1559) | Similar logic |
| Polygon     | Yes              | Modified for faster adjustments |
| Arbitrum    | Simulated        | Uses custom pricing model internally |
| Optimism    | Simulated        | Abstracts gas cost via fixed pricing |

---

## 🧠 Summary

You now understand:
- The **motivation** behind EIP-1559.
- Its **technical mechanics** including base fee calculation.
- How to build **transactions** that comply with EIP-1559 using various languages.
- The **economic impact** and **edge cases** involved.
- How it compares to **legacy systems** and behaves across chains.

You're now equipped to:
- Build dApps with accurate gas estimation.
- Analyze network conditions and optimize transaction submissions.
- Understand deflationary effects and miner incentives.

