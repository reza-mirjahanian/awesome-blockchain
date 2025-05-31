

---

## 🧱 1. **Foundations: What Is Archive Data in Ethereum?**

### ✅ **Definitions**

* **Full Node**: Stores all blocks and can respond to current state queries (latest balances, smart contract storage).
* **Archive Node**: Extends Full Node by storing all historical states — not just current state, but **every past state of every account, contract, and storage slot**.

| Node Type    | Stores Blocks | Current State | Historical State |
| ------------ | ------------- | ------------- | ---------------- |
| Full Node    | ✅             | ✅             | ❌                |
| Archive Node | ✅             | ✅             | ✅                |

---

## 🔍 2. **Why Archive Nodes Are Important**

1. **Debugging Smart Contracts**

   * Inspect storage layout and values at **any block height**.
2. **Historical Data Analytics**

   * Analyze token balances at a specific block, e.g., for retroactive airdrops.
3. **EVM Debuggers & Tools**

   * Required for tools like Tenderly, Dune, Blockscout in full debug mode.
4. **State Re-execution**

   * Replay EVM execution to verify prior states.

---

## 🛠 3. **How Ethereum Stores State**

### 🧩 Ethereum State Trie (Merkle-Patricia Trie)

* **Trie Root** in each block header points to **current state**.
* Full nodes discard old state tries to save space.
* Archive nodes **retain all past tries**.

```txt
Block Header
└── stateRoot (Merkle-Patricia Trie)
     ├── Account Trie (address → account object)
     │    └── Storage Trie (storage slot → value)
```

---

## ⚙️ 4. **Running an Archive Node**

### With Geth

```bash
geth --syncmode full --gcmode archive
```

* `--syncmode full`: downloads the full chain and validates everything.
* `--gcmode archive`: disables pruning (GC = Garbage Collection) of historical states.

#### Storage Size

| Chain    | Archive Size (approx) |
| -------- | --------------------- |
| Ethereum | \~15 TB+              |
| Gnosis   | \~3 TB                |

> ⚠️ **Warning**: This is *very expensive* in disk and IO.

---

## 📦 5. **Accessing Archive Data**

### JSON-RPC APIs

#### `eth_getBalance`

Returns balance of an address **at a specific block**.

```bash
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  --data '{
    "jsonrpc":"2.0",
    "method":"eth_getBalance",
    "params":["0x742d35Cc6634C0532925a3b844Bc454e4438f44e", "0x5BAD55"],
    "id":1
}'
```

* `"0x5BAD55"` is the **block number** in hex.

#### `eth_getStorageAt`

Returns value at a storage slot **at a specific block**.

```bash
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  --data '{
    "jsonrpc":"2.0",
    "method":"eth_getStorageAt",
    "params": [
      "0xTokenContractAddress",
      "0xStorageSlotIndex",
      "0xBlockNumberHex"
    ],
    "id":1
}'
```

---

## 🧪 6. **Storage Slot Calculation Example**

### Solidity Storage Layout

```solidity
contract Simple {
    uint256 a = 100;
    uint256 b = 200;
}
```

* `a`: stored at slot `0x0`
* `b`: stored at slot `0x1`

### Get Historical Value of `a`

```bash
eth_getStorageAt("0xContractAddress", "0x0", "0xBlockNumber")
```

---

## 📈 7. **Comparing Archive vs Full Nodes: Performance and Use Cases**

| Aspect              | Full Node           | Archive Node                         |
| ------------------- | ------------------- | ------------------------------------ |
| Disk Usage          | \~800GB             | 15TB+                                |
| Historical Storage  | ❌                   | ✅ All states                         |
| Query Past Balances | ❌                   | ✅                                    |
| Debug Execution     | ❌                   | ✅                                    |
| Sync Speed          | Faster              | Slower                               |
| Ideal For           | Validators, Wallets | Block explorers, analytics platforms |

---

## 🧠 8. **Advanced: Replaying Historical Calls and Traces**

### `debug_traceTransaction`

Requires archive node for historical state.

```bash
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  --data '{
    "jsonrpc":"2.0",
    "method":"debug_traceTransaction",
    "params":["0xTxHash"],
    "id":1
}'
```

### `trace_call` (OpenEthereum/Nethermind)

Executes a call at a given block using historical state.

---

## 🧬 9. **Tools That Use Archive Data**

| Tool       | Archive Usage                               |
| ---------- | ------------------------------------------- |
| Etherscan  | Show balance/storage for past blocks        |
| Tenderly   | Replays transactions, shows stack traces    |
| Dune       | SQL-like interface on historical EVM data   |
| Blockscout | Block explorer with deep call trace support |
| Alethio    | Analytics on token and dApp activity        |

---

## 🧰 10. **Alternatives: When Not to Use an Archive Node**

### Use External Services

* **Infura** (`https://mainnet.infura.io/v3/<API_KEY>`)
* **Alchemy** (`https://eth-mainnet.alchemyapi.io/v2/<API_KEY>`)
* **Ankr**, **Chainstack**, **QuickNode**

✅ These provide archive APIs **without running your own archive node**.

---

## 🧠 11. **Expert Insight: Merkle Trie Versioning**

Archive nodes do not store every version of the trie — instead they store a versioned DB of every change across blocks:

### In Geth:

* **Trie nodes are versioned** using block numbers as keys.
* Key-value DB holds historical tries per block root.

---

## 🔄 12. **Comparison With Similar Concepts**

| Concept        | Purpose                                | Historical Access | Size    |
| -------------- | -------------------------------------- | ----------------- | ------- |
| Full Node      | Live chain + latest state              | ❌                 | \~800GB |
| Archive Node   | Full + all historical state            | ✅                 | \~15TB+ |
| Light Node     | Only headers and on-demand state proof | ❌                 | <5GB    |
| Snap Sync Node | Compressed state, faster sync          | ❌                 | \~300GB |

---

## 📌 13. **Edge Cases to Understand**

1. **`eth_call` at old block** on full node fails:

   * Returns empty or error without archive data.

2. **Storage slot overwritten**

   * You *can* still query old value if archive is present.

3. **Pruned Nodes**

   * Even if state exists in block, it's not queryable without archive mode.

---

