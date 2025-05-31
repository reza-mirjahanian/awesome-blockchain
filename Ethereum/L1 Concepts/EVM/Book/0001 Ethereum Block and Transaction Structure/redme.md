---

# 📦 Ethereum Block and Transaction Structure (with EVM Focus)

---

## 🧱 1. **Block Structure Overview**

Each Ethereum block consists of three main components:

* **Block Header**
* **Transactions**
* **Uncles (Ommer blocks)**

### Block = `RLP([Header, [Transactions], [Uncles]])`

---

## 📘 2. **Block Header Structure**

The block header is an RLP-encoded list of the following 15 fields (post-merge has 17 due to `baseFeePerGas` and `withdrawalsRoot`):

| Field              | Type         | Description                                 |
| ------------------ | ------------ | ------------------------------------------- |
| `parentHash`       | `32 bytes`   | Hash of the parent block header             |
| `ommersHash`       | `32 bytes`   | Keccak256 hash of the list of uncle headers |
| `beneficiary`      | `20 bytes`   | Address to receive mining rewards           |
| `stateRoot`        | `32 bytes`   | Root hash of the state trie                 |
| `transactionsRoot` | `32 bytes`   | Root of the transaction trie                |
| `receiptsRoot`     | `32 bytes`   | Root of receipts trie                       |
| `logsBloom`        | `256 bytes`  | Bloom filter for logs in receipts           |
| `difficulty`       | `BigInt`     | Block difficulty (0 post-merge)             |
| `number`           | `BigInt`     | Block number                                |
| `gasLimit`         | `BigInt`     | Max gas allowed in block                    |
| `gasUsed`          | `BigInt`     | Total gas used in block                     |
| `timestamp`        | `BigInt`     | Unix timestamp                              |
| `extraData`        | `<=32 bytes` | Arbitrary data                              |
| `mixHash`          | `32 bytes`   | Used for PoW (ignored post-merge)           |
| `nonce`            | `8 bytes`    | Used for PoW (ignored post-merge)           |
| `baseFeePerGas`    | `BigInt`     | (Post-EIP-1559) base fee                    |
| `withdrawalsRoot`  | `32 bytes`   | (Post-merge) root of withdrawals trie       |

---

## ⚙️ 3. **Transaction Formats**

There are **three** transaction types in Ethereum:

### ✅ Legacy (Pre-EIP-2718)

```txt
RLP([nonce, gasPrice, gasLimit, to, value, data, v, r, s])
```

### ✅ EIP-2930 (Access List)

```txt
0x01 || RLP([chainId, nonce, gasPrice, gasLimit, to, value, data, accessList, v, r, s])
```

### ✅ EIP-1559 (Dynamic Fee)

```txt
0x02 || RLP([chainId, nonce, maxPriorityFeePerGas, maxFeePerGas, gasLimit, to, value, data, accessList, v, r, s])
```

### 📌 Fields Explained

| Field                  | Description                                     |
| ---------------------- | ----------------------------------------------- |
| `nonce`                | Number of transactions sent by sender           |
| `gasPrice`             | Fixed fee (Legacy)                              |
| `maxPriorityFeePerGas` | Tip to miner (EIP-1559)                         |
| `maxFeePerGas`         | Total cap on fee (EIP-1559)                     |
| `gasLimit`             | Max gas for execution                           |
| `to`                   | Recipient address (`0x0` for contract creation) |
| `value`                | Ether value to transfer                         |
| `data`                 | Calldata (EVM bytecode or function input)       |
| `accessList`           | Storage keys/contracts to preload               |
| `v`, `r`, `s`          | Signature for ECDSA (secp256k1)                 |

---

## 🔁 4. **Transaction Execution → EVM**

### Lifecycle:

1. **Account nonce is checked**
2. **Sender balance >= gasLimit \* maxFeePerGas**
3. **Intrinsic gas is calculated**
4. **State transition begins**
5. **EVM executes code**
6. **Receipts generated**
7. **Logs emitted**
8. **State updated or reverted**

### Gas Breakdown

| Action              | Intrinsic Gas          |
| ------------------- | ---------------------- |
| Transaction         | 21,000                 |
| Contract creation   | 32,000                 |
| `CALL`              | 700                    |
| Storage read        | 100                    |
| Storage write (new) | 20,000                 |
| Storage reset       | 5,000 (refund -15,000) |

---

## 🤝 5. **Uncles (Ommers)**

### Purpose

To reward miners for blocks that were valid but not included in the canonical chain (due to latency).

### Rules

* Max 2 uncles per block
* Uncle must be within 6 blocks of current block
* Uncles must be valid blocks with a valid header

### Rewards

| Recipient             | Reward                                    |
| --------------------- | ----------------------------------------- |
| Uncle miner           | `uncleReward = (8 - k) / 8 * blockReward` |
| Including block miner | `1/32 * blockReward` per uncle            |

---

## 🔗 6. **How Blocks Form the Blockchain**

* Each block references the previous via `parentHash`
* Blocks form a **linked list** (chain)
* Valid chain = chain with most accumulated difficulty (now total validator weight post-merge)
* Each block contains:

  * `transactionsRoot` (Merkle-Patricia Trie)
  * `stateRoot` (global account/storage state)
  * `receiptsRoot` (post-execution effects)

---

## 📂 7. **Receipts & Logs**

Each transaction generates:

* **Receipt**:

  * `status`: 1 (success) or 0 (failure)
  * `cumulativeGasUsed`
  * `logsBloom`
  * `logs`: array of event logs

* **Logs**:

  * `address`: emitting contract
  * `topics`: indexed event params
  * `data`: unindexed params

---

## 🧠 8. **Tricky Parts & Edge Cases**

* Transactions with `to = None` → contract creation; hash of the new contract = `keccak256(rlp([sender, nonce]))`
* Zero-value transactions still consume gas
* State root reflects post-execution world state, not just user balances
* Uncle headers are validated but their transactions are not replayed
* Same transaction hash is invalid (nonces must be unique)

---

## ⚖️ 9. **Comparison Tables**

### EIP-1559 vs Legacy Tx

| Feature            | Legacy             | EIP-1559                               |
| ------------------ | ------------------ | -------------------------------------- |
| Fee field          | `gasPrice`         | `maxFeePerGas`, `maxPriorityFeePerGas` |
| Base fee mechanism | ❌                  | ✅                                      |
| Miner tip          | part of `gasPrice` | `maxPriorityFeePerGas`                 |
| Overpayment refund | ❌                  | ✅                                      |

### Block vs Transaction Trie

| Feature      | Block Header Field | Trie Contents       | Trie Type       |
| ------------ | ------------------ | ------------------- | --------------- |
| State Trie   | `stateRoot`        | Accounts            | Merkle-Patricia |
| Tx Trie      | `transactionsRoot` | Tx ordered by index | Merkle-Patricia |
| Receipt Trie | `receiptsRoot`     | Receipt objects     | Merkle-Patricia |

---

## ⏱️ 10. **Complexity & Cost**

| Operation                          | Complexity | Notes                    |
| ---------------------------------- | ---------- | ------------------------ |
| Transaction insertion (RLP encode) | O(1)       | Encoding constant size   |
| Transaction Merkle proof           | O(log n)   | Depends on number of txs |
| Trie update                        | O(log n)   | N = number of keys       |
| State read/write                   | O(log n)   | Trie traversal           |

---

## 🔍 11. **RLP Encoding Basics**

Recursive Length Prefix encoding used for:

* Block headers
* Transactions
* Receipts
* Uncles

### RLP Rules

* Single byte < `0x80` = itself
* Strings:

  * If <55 bytes: `0x80 + len(data)` + data
  * Else: `0xb7 + len(len(data))` + len(data) + data
* Lists:

  * If total payload < 55 bytes: `0xc0 + len(payload)` + payload
  * Else: `0xf7 + len(len(payload))` + len(payload) + payload

---

## 🚀 Next Steps Suggestion

### 🔮 **Next Topic**: *Ethereum State Trie & Merkle Patricia Trie Structure*

A deep dive into how Ethereum encodes account states, proofs, and contracts in tries, including:

* Account object structure
* Storage trie per contract
* Proof generation and verification
* State syncing and light client proof mechanisms

---
