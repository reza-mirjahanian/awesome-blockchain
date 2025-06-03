## 🧠 Masterclass on **EIP-1559 in EVM** (Ethereum)

---

### 🧱 1. **Foundational Concepts**

#### What is EIP-1559?

**EIP-1559** is an Ethereum Improvement Proposal that changes the fee market mechanism.

**Before EIP-1559**:

* Gas fees are set by users.
* Bidding system (first-price auction).
* Network congestion ⇒ high, unpredictable fees.

**After EIP-1559** (post-London Hard Fork, August 2021):

* Introduces **Base Fee + Tip (Priority Fee)**.
* Base fee is **burned**.
* Tip is sent to the **miner (PoW) or validator (PoS)**.

---

### ⚙️ 2. **New Fee Mechanism Structure**

| Component         | Description                                         |
| ----------------- | --------------------------------------------------- |
| **Base Fee**      | Algorithmically adjusted; minimum fee to include tx |
| **Priority Fee**  | Tip to incentivize validator (set by user)          |
| **Max Fee**       | Maximum a user is willing to pay (`maxFeePerGas`)   |
| **Effective Fee** | `min(maxFeePerGas, baseFee + priorityFee)`          |

---

### 🧮 3. **Transaction Format Changes**

New fields added:

```json
{
  "type": "0x2",
  "maxFeePerGas": 1000000000,
  "maxPriorityFeePerGas": 200000000
}
```

---

### 🧪 4. **Examples in Native RPC / JSON**

#### Legacy Transaction (pre-EIP-1559)

```json
{
  "gasPrice": "0x12a05f200", // 5 Gwei
  ...
}
```

#### EIP-1559 Transaction (type 0x2)

```json
{
  "type": "0x2",
  "maxFeePerGas": "0x2540be400", // 10 Gwei
  "maxPriorityFeePerGas": "0x3b9aca00", // 1 Gwei
  ...
}
```

---

### 🧮 5. **Fee Calculation Algorithm**

```text
effectiveGasPrice = min(maxFeePerGas, baseFee + priorityFee)
```

#### Rust Example

```rust
fn calculate_effective_gas_price(max_fee: u64, priority_fee: u64, base_fee: u64) -> u64 {
    std::cmp::min(max_fee, base_fee.saturating_add(priority_fee))
}
```

---

### 🔄 6. **Base Fee Adjustment Algorithm**

* Target block size: 15M gas
* Max block size: 30M gas
* **Adjustment Rule**:

```text
baseFee' = baseFee * (1 + (gasUsed - targetGas) / targetGas)
```

Clamped to 12.5% per block.

#### Go Example

```go
func updateBaseFee(baseFee, gasUsed, targetGas uint64) uint64 {
    delta := int64(gasUsed) - int64(targetGas)
    feeDelta := baseFee / 8 // max 12.5%
    return baseFee + uint64((int64(feeDelta)*delta)/int64(targetGas))
}
```

---

### 🔥 7. **Fee Burn Mechanism**

* **Base Fee is burned** (removed from supply).
* Reduces ETH inflation.
* Can make ETH deflationary during high usage.

#### Example: Fee Burn per Block (Pseudo)

```solidity
uint256 feeToBurn = baseFee * gasUsed;
burn(feeToBurn);
```

---

### 💡 8. **Comparison with Legacy Model**

| Feature          | Legacy (pre-1559)   | EIP-1559                  |
| ---------------- | ------------------- | ------------------------- |
| Fee Mechanism    | First-price auction | Base fee + Tip            |
| Volatility       | High                | Lower                     |
| User Experience  | Complex             | Simpler, more predictable |
| Miner Revenue    | All fees            | Tip only                  |
| Deflationary ETH | ❌ No                | ✅ Possible                |

---

### 🧱 9. **Use Cases**

#### 1. **Efficient Wallet Estimation**

* Wallets can auto-set `baseFee + recommendedTip`.

#### 2. **Better Fee Prediction for DApps**

* Algorithms can calculate tip based on urgency.

#### 3. **Burn Tracking in Block Explorers**

* Tools like Etherscan show ETH burned per block.

---

### ⚠️ 10. **Edge Cases**

#### a. `maxFeePerGas < baseFee`

* Transaction is **rejected**.

#### b. `maxFeePerGas` very high, `priorityFee` too low

* Validator may **deprioritize**.

#### c. Network spike: block uses full 30M gas

* Base fee can **increase 12.5%** for next block.

#### Rust Example

```rust
fn is_transaction_valid(max_fee: u64, base_fee: u64) -> bool {
    max_fee >= base_fee
}
```

---

### 🧠 11. **Advanced Concepts**

#### a. **Inclusion Criteria for Validators**

```text
effectiveGasPrice ≥ baseFee
```

Validators sort by `priorityFee`.

#### b. **Simulation in Custom Chains**

* Many L2s and testnets customize the base fee algorithm.

#### c. **MEV & EIP-1559**

* MEV (Miner Extractable Value) still exists.
* Tip gives MEV-extracting validators incentive.

---

### 🧪 12. **Code: Simulate a Full EIP-1559 Block in Rust**

```rust
struct Transaction {
    max_fee: u64,
    priority_fee: u64,
}

struct Block {
    base_fee: u64,
    gas_limit: u64,
    gas_used: u64,
    transactions: Vec<Transaction>,
}

fn execute_block(mut block: Block) {
    for tx in block.transactions.iter() {
        let effective_price = std::cmp::min(tx.max_fee, block.base_fee + tx.priority_fee);
        if tx.max_fee < block.base_fee {
            println!("Rejected: max_fee too low");
        } else {
            println!("Accepted with effective price: {}", effective_price);
        }
    }
}
```

---

### 🔄 13. **Interaction with Other EIPs**

| EIP      | Interaction Description                            |
| -------- | -------------------------------------------------- |
| EIP-2718 | Transaction type abstraction (enables type 0x2 tx) |
| EIP-2930 | Access List transactions (also compatible)         |
| EIP-4844 | Proto-danksharding includes similar concepts       |

---

### 🔬 14. **Security Considerations**

* **Base fee manipulation** is hard due to protocol limits.
* **Tip bidding wars** can still happen for MEV.

---

### ✅ Summary Table

| Concept              | Description                            |
| -------------------- | -------------------------------------- |
| Type                 | 0x2                                    |
| Fields               | `maxFeePerGas`, `maxPriorityFeePerGas` |
| Base Fee Adjust Rate | ±12.5% per block                       |
| Max Block Gas        | 30M                                    |
| Target Block Gas     | 15M                                    |
| Tip Receiver         | Validator (PoS)                        |
| Base Fee Burned?     | ✅ Yes                                  |
| Transaction Validity | `maxFeePerGas ≥ baseFee`               |

---

Next topic or deeper focus (e.g., base fee update simulation, EVM gas accounting under EIP-1559, fork impact) — just say the word.
