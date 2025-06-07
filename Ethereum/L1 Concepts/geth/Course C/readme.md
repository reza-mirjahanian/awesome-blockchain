---

## 🧠 **Ethereum Technical Walkthrough Overview**

### 🎯 **Scope of the Walkthrough**

* Part of Ethereum Attackathon (hosted on MiFi)
* Focused on the **Go-Ethereum (Geth)** client
* Walkthrough led by contributors from the **G Project**
* Coverage areas include:

  * Ethereum Virtual Machine (EVM)
  * Block processing and state transitions
  * Transaction validation
  * P2P networking and consensus
  * Relevant tooling and testing (e.g., `evm` binary, fuzzing)

---

## ⚙️ **Go-Ethereum Codebase: Core Concepts**

### 📁 Core Directories

* **`core/`** – Handles EVM, block processing, and state transitions
* **`consensus/`** – Consensus rules logic
* **`eth/`** – Node-level operations (syncing, P2P, RPC)
* **`p2p/`** – Low-level peer discovery and networking protocols

---

## 🔄 **EVM (Ethereum Virtual Machine)**

### 📌 Key Responsibilities

* Executes smart contracts
* Processes bytecode (e.g., `PUSH`, `ADD`, `SUB`, etc.)
* Must match behavior across clients to avoid *consensus failures*

### 🧩 Core Components

1. **`evm` Object**:

   * Entry point for transaction execution
   * Methods:

     * `Call()` for contract calls
     * `Create()` for new contracts

2. **`interpreter.go`**:

   * Contains the main EVM *run loop*
   * Executes opcodes sequentially
   * \~100 lines: *small, readable, but critical*

### 🧪 Example EVM Execution with `evm` Binary

```bash
evm --json --code 0x60406040 run
```

* `0x60 40` pushes `0x40` onto the stack
* JSON output shows opcodes executed and gas used

---

## 🧱 **Block Processing and State Transitions**

### 🔄 Main Flow

1. **Pre-Execution Hooks**
2. **Transaction Application**
3. **Post-Processing Logic**

### 🧾 Files to Explore

* `state_processor.go`: Manages full-block processing
* `state_transition.go`: Applies a single transaction (message)

### ✳️ Fork Gating

* Fork-dependent logic gated by:

  ```go
  if config.IsPrague(...) {
     // Execute only during Prague fork
  }
  ```
* Helps manage active/inactive features

---

## ⛽ **Gas Constraints and Consensus Safety**

### 🔥 Important Assumptions

* **Gas Limit**: Cannot exceed `uint64::MAX`
* **Memory Usage**: Quadratic cost for memory expansion
* **Consensus Bugs**:

  * Submissions that cause overflows with billions of gas are *not valid*
  * Focus on feasible gas usage scenarios

---

## 📜 **Transaction Validation**

### 🧬 Transaction Types

1. **Legacy** – Original type
2. **Access List** – Includes an address/storage key list
3. **Dynamic Fee (EIP-1559)** – Introduced `maxFeePerGas` and `maxPriorityFeePerGas`
4. **Blob Transactions** – Include “blob” data chunks

### ⚠️ Common Bug Areas

* **RLP Encoding Ambiguities**

  * E.g., empty list vs. `null`
  * May lead to inconsistent hashes → *consensus issues*

### 🔍 Fuzzing & Tests

* Go fuzzing used internally (e.g., for RLP parsing)
* Community can create **differential fuzzers** comparing Geth with other clients

---

## 🧃 **Transaction Pool (`txpool`)**

### 🔧 Architecture

* Located in `core/tx_pool/`
* Two pools:

  * **LegacyPool**: General transactions
  * **BlobPool**: Blob-only transactions

### 🛡️ Security Posture

* Attacks on **individual node’s txpool**: *Low priority*
* Only **network-wide impact** is considered critical

---

## 🌐 **Networking and P2P Layers**

### 🧭 Ethereum Node Protocol Stack

* Implemented in `eth/` and `p2p/`
* Includes:

  * **ETH Protocol (eth/68)** – Blocks and txs
  * **SNAP Protocol** – Efficient state syncing
  * **Discovery Protocol (v4/v5)** – Peer finding
  * **RLPx (devp2p)** – Underlying transport encryption (crypto attacks are out of scope)

### ✉️ Message Handling Structure

Each protocol follows this pattern:

```go
switch msg.Code {
  case GetAccountRange:
    // Handle state range request
  case GetBlockHeaders:
    // Handle header sync
}
```

---

## 📦 **Engine API (`ecatalyst/api.go`)**

### 🛠️ Purpose

* Bridges execution and consensus layers
* Exposed via **privileged internal JSON-RPC**
* Not internet-accessible

### 🔑 Methods of Interest

1. `ForkchoiceUpdatedV3()`
2. `NewPayloadV2()`
3. `GetPayload()`

### ❗ Security Consideration

* Attacks on this privileged API must exploit logic, not web-based vectors
* Crashes from malformed JSON are **low-priority**

---

## 📡 **Peer-to-Peer Message Flow**

### 🔄 Syncing & Chain Download

* Managed by `eth/downloader`
* Coordinates:

  * Header sync
  * Body retrieval
  * State sync via SNAP

### 🧪 Testing Message Handling

* Each peer's handler:

  ```go
  func handleMessage(msgCode uint, data []byte) error
  ```

---

## 🧪 **Testing & Fuzzing Infrastructure**

### ✅ Test Types

* **Execution Spec Tests** – External JSON-based tests
* **Fuzzers** – Internally written with Go fuzzing
* **Differential Testing** – Recommended against other Ethereum clients

---

## 🚫 **Out-of-Scope Vulnerabilities**

### ❌ DoS Without Novel Vector

* Volumetric denial-of-service (high traffic from attacker)
* Petabyte-scale memory attacks
* Invalid inputs to internal RPCs (e.g., `debug_*`)

### ❌ RPC Abuse

* User-facing JSON-RPC (e.g., `eth_*`, `debug_*`) exposed to the internet
* Expected to be protected by node operator

---

## 📚 **Helpful Resources**

### 🗂️ Ethereum P2P Protocol Specs

* GitHub: `ethereum/devp2p`
* Contains:

  * Message formats
  * Protocol diagrams
  * Encoding rules

### 📖 Deep Dive Suggested Areas

* `eth/protocols/eth/handler.go`
* `eth/protocols/snap/handler.go`
* `eth/fetcher/` – Handles tx propagation
* `p2p/discover/` – Discovery protocols

---

## 🧩 **Key Takeaways for Security Researchers**

### 🎯 High-Impact Targets

* EVM inconsistencies
* State transition logic errors
* Incorrect RLP parsing → consensus splits
* Vulnerabilities in engine API
* P2P message handling leading to network-wide faults

### ⚠️ Low-Impact (or Out of Scope)

* RPC crashing from malformed requests
* Node-local txpool manipulation
* Known limitations in RLPx crypto

---
