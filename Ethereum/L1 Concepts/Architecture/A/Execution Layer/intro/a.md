# 🧠 Ethereum Execution Layer — Complete Technical Reference



---

## 🔧 What is the Execution Layer?

The **Execution Layer** is the part of Ethereum responsible for:

* **Processing transactions**
* **Managing Ethereum state (accounts, balances, storage)**
* **Executing smart contracts via the EVM (Ethereum Virtual Machine)**

Since **The Merge**, the EL works in tandem with the **Consensus Layer (CL)**. EL handles execution, while CL handles PoS consensus.

---

## 🏗️ Architecture Overview

```
+-----------------------+
|     User / Wallet     |
+-----------------------+
           |
           v
+-----------------------+
|     JSON-RPC / API    |
+-----------------------+
           |
           v
+-----------------------+
|   Execution Client    |  ←→  Consensus Client (via Engine API)
+-----------------------+
| - EVM                 |
| - Tx Pool             |
| - State Trie (Patricia) |
| - Storage (LevelDB)   |
+-----------------------+
```

---

## 🧬 Core Components

### 1. **Ethereum Accounts**

#### Types:

* **Externally Owned Accounts (EOA)**
* **Contract Accounts**

| Field       | Description                   |
| ----------- | ----------------------------- |
| nonce       | Number of txs sent            |
| balance     | Ether balance                 |
| storageRoot | Merkle root of storage trie   |
| codeHash    | Hash of the contract bytecode |

#### Example

```json
{
  "nonce": 3,
  "balance": "0xde0b6b3a7640000",
  "storageRoot": "0x...",
  "codeHash": "0x..."
}
```

---

### 2. **Transactions**

#### Fields:

| Field               | Description                   |
| ------------------- | ----------------------------- |
| nonce               | Tx count of sender            |
| gasPrice / EIP-1559 | Base fee, tip                 |
| gasLimit            | Max gas allowed               |
| to                  | Recipient address or contract |
| value               | Ether sent                    |
| data                | Input to contract call        |
| v, r, s             | Signature components          |

#### Types:

* **Legacy (pre-EIP-1559)**
* **EIP-1559 typed (type 2)**

#### Example (EIP-1559):

```json
{
  "type": "0x2",
  "maxFeePerGas": "0x59682f00",
  "maxPriorityFeePerGas": "0x59682f00",
  "gas": "0x5208",
  "to": "0x...",
  "value": "0x0",
  "data": "0x...",
  "nonce": "0x0",
  "chainId": "0x1",
  "accessList": [],
  "v": "...", "r": "...", "s": "..."
}
```

---

### 3. **Ethereum State**

* Represented as a **Merkle Patricia Trie**
* Stored in **LevelDB/RocksDB** by clients
* Each account is a node; storage of contract is a separate trie.

#### Trie Layers:

1. State Trie (rooted at `stateRoot`)
2. Storage Trie per contract
3. Code stored by `codeHash`

---

### 4. **EVM (Ethereum Virtual Machine)**

#### EVM Execution:

* Stack-based VM (256-bit words)
* Executes `EVM bytecode`
* Uses **gas accounting** to prevent abuse

#### Gas Cost Table (sample):

| Opcode   | Name       | Gas Cost       |
| -------- | ---------- | -------------- |
| `ADD`    | Arithmetic | 3              |
| `SLOAD`  | Storage    | 100            |
| `SSTORE` | Storage    | 20,000 or less |
| `CALL`   | Msg call   | 700 + dynamic  |

#### Example:

```evm
PUSH1 0x2
PUSH1 0x3
ADD
```

Result: 5 on top of stack

#### Solidity to EVM:

```solidity
function add() public pure returns (uint) {
    return 2 + 3;
}
```

Compiles to:

```evm
PUSH1 0x2
PUSH1 0x3
ADD
```

---

### 5. **Gas and Fees (EIP-1559)**

* **Base Fee**: Adjusted every block
* **Tip**: Goes to block proposer
* **Max Fee**: Cap on total user willing to pay

#### Effective Fee:

```solidity
effective_gas_price = min(max_fee_per_gas, base_fee + tip)
```

---

### 6. **Storage Layout**

Contract storage is a key-value store where:

* Each slot is 32 bytes
* Mapped using `keccak256`

#### Mappings:

```solidity
mapping(uint => uint) data;
```

Slot of `data[1]`: `keccak256(1 . slot_of_mapping)`

---

## 🧠 State Transition Function `𝛾`

Formally:

```math
𝛾(state, transaction) = new_state
```

Steps:

1. Check nonce, balance, signature
2. Deduct gas
3. Execute EVM code
4. Apply state changes
5. Refund unused gas

---

## ⚙️ JSON-RPC & Engine API (Post-Merge)

### JSON-RPC Examples:

```bash
eth_getTransactionByHash
eth_getBlockByNumber
eth_call
eth_sendRawTransaction
```

### Engine API (for EL ↔ CL communication):

* `engine_forkchoiceUpdatedV1`
* `engine_getPayloadV1`
* `engine_newPayloadV1`

Used for:

* Syncing blocks between layers
* Executing and validating blocks

---

## ⛓️ Block Structure (Execution Payload)

| Field        | Description               |
| ------------ | ------------------------- |
| parentHash   | Hash of previous block    |
| stateRoot    | Merkle root of state trie |
| transactions | List of txs               |
| receiptsRoot | Merkle root of receipts   |
| logsBloom    | Bloom filter of logs      |
| gasUsed      | Total gas consumed        |

---

## 📦 Real-World Projects Using EL Concepts

| Project                                       | Usage                                              |
| --------------------------------------------- | -------------------------------------------------- |
| **Foundry**                                   | Testing framework for contracts, spins up EL nodes |
| **Flashbots**                                 | Uses txpool, gas mechanics for MEV extraction      |
| **Ethereum Clients (Geth, Nethermind, Besu)** | Full EL implementation                             |
| **Hardhat**                                   | Simulates EVM locally, uses tx/state/evm modules   |

---

## ⚖️ Pros and Cons

| Feature                 | Pros                              | Cons                        |
| ----------------------- | --------------------------------- | --------------------------- |
| Modular architecture    | Easy upgrades, separates concerns | Adds cross-layer complexity |
| EVM standardization     | Broad compatibility               | Performance bottlenecks     |
| Gas metering            | Prevents abuse, ensures fairness  | Developer overhead          |
| Deterministic execution | Same input → same output          | No native concurrency       |

---

## ⏱️ Complexity Overview

| Operation        | Complexity     | Notes                    |
| ---------------- | -------------- | ------------------------ |
| Account lookup   | O(log N)       | Trie-based lookup        |
| Contract call    | O(depth × gas) | Depth-limited, gas-bound |
| Storage access   | O(log N)       | MPT + DB query           |
| Transaction exec | O(1) to O(n)   | Depends on bytecode      |

---

## 🧪 Tricky Parts and Edge Cases

1. **Gas Refund Mechanics** (e.g., `SSTORE` refund rules)
2. **Precompiled Contracts** — e.g., `0x01` (ECDSA), `0x05` (ModExp)
3. **Self-Destruct behavior** post-EIP-6780
4. **Out-of-Gas Reverts** don't persist state changes
5. **Re-entrancy via delegatecall**

---

## 💡 Comparisons

| Execution Layer | Ethereum | Solana                 |
| --------------- | -------- | ---------------------- |
| VM              | EVM      | Sealevel (parallel)    |
| Gas             | Yes      | No, uses compute units |
| Concurrency     | No       | Yes                    |
| Determinism     | Full     | Full                   |
| Performance     | Moderate | High throughput        |

---

## 📚 Related Standards & EIPs

| EIP      | Title                            | Purpose                            |
| -------- | -------------------------------- | ---------------------------------- |
| EIP-1559 | Fee market change                | Introduced base/tip fee            |
| EIP-2718 | Typed transactions               | Enables tx type extension          |
| EIP-2930 | Access lists                     | Reduce gas cost for known accesses |
| EIP-3541 | Disallow code starting with 0xEF | Prepares for EOF                   |

---

## ⛓️ Example Flow

```plaintext
User signs tx →
→ tx enters mempool →
→ selected in block →
→ block sent to engine_newPayload →
→ EVM runs, state updated →
→ new stateRoot calculated
```

---

## 📥 Code Example: Raw Transaction (EIP-1559)

```js
const tx = {
  type: 2,
  nonce: 0,
  to: "0xRecipientAddress",
  value: ethers.utils.parseEther("1.0"),
  gasLimit: 21000,
  maxFeePerGas: ethers.utils.parseUnits("100", "gwei"),
  maxPriorityFeePerGas: ethers.utils.parseUnits("2", "gwei"),
  chainId: 1
};

const signedTx = await wallet.signTransaction(tx);
await provider.sendTransaction(signedTx);
```

---

## 📊 Data Table: EVM Opcodes Summary

| Opcode   | Name     | Gas     | Stack Inputs | Stack Outputs |
| -------- | -------- | ------- | ------------ | ------------- |
| `ADD`    | Addition | 3       | 2            | 1             |
| `SLOAD`  | Storage  | 100     | 1            | 1             |
| `SSTORE` | Storage  | 20k/5k  | 2            | 0             |
| `CALL`   | Msg call | dynamic | 7            | 1             |

---

## 🧭 Next Steps Suggestion:

**Next Advanced Topic:**
📘 **Ethereum Virtual Machine (EVM) Deep Dive: Internals, Bytecode, Gas Profiling, and Optimizations**
This topic builds directly on the Execution Layer knowledge and dives deep into contract performance, disassembly, and runtime behavior.

Would you like to proceed with that next?
