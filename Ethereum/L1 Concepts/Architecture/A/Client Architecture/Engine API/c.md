
---

## ⚙️ ETH Engine API: Overview

The **Engine API** bridges the **Consensus Layer (CL)** and **Execution Layer (EL)** in Ethereum post-Merge. Two of the most critical methods in this API are:

* `engine_forkchoiceUpdatedV1` or `V2`
* `engine_newPayloadV1` or `V2`

These methods manage:

* **Which chain to build on** (fork choice)
* **What block to execute** (new payload)

---

## 🧱 1. Foundation: Ethereum Post-Merge Architecture

* The **Consensus Layer (CL)** (Beacon Chain) determines the canonical chain.
* The **Execution Layer (EL)** executes transactions and maintains the state.
* The **Engine API** enables communication between CL and EL.

```plaintext
[Consensus Layer]
      ⇅ Engine API
[Execution Layer]
```

---

## 🧩 2. `engine_forkchoiceUpdatedV1`: Fork Choice Update

### 🔍 Purpose

Informs the Execution Layer of:

* **Head block hash**: The block at the head of the canonical chain.
* **Safe block hash**: The latest justified checkpoint.
* **Finalized block hash**: The last finalized block.

```json
{
  "jsonrpc": "2.0",
  "method": "engine_forkchoiceUpdatedV1",
  "params": [
    {
      "headBlockHash": "0xabc...",
      "safeBlockHash": "0xdef...",
      "finalizedBlockHash": "0x123..."
    },
    {
      "payloadAttributes": {
        "timestamp": "0x5f5e100",
        "prevRandao": "0x456...",
        "suggestedFeeRecipient": "0xabc..."
      }
    }
  ],
  "id": 1
}
```

### 📘 Parameters

| Field                | Description                               |
| -------------------- | ----------------------------------------- |
| `headBlockHash`      | Chosen head block of the fork             |
| `safeBlockHash`      | Safe, justified block                     |
| `finalizedBlockHash` | Finalized block                           |
| `payloadAttributes`  | Optional, hints for next block production |

### ✅ Returns

| Field           | Description                      |
| --------------- | -------------------------------- |
| `payloadStatus` | Status of fork choice processing |
| `payloadId`     | ID used in `engine_getPayload`   |

```json
{
  "payloadStatus": {
    "status": "VALID",
    "latestValidHash": "0xabc...",
    "validationError": null
  },
  "payloadId": "0x123abc..."
}
```

### 🧠 Notes

* If `payloadAttributes` is **present**, it signals block production.
* A new block proposal begins if validator is the block proposer.

---

## 🧩 3. `engine_newPayloadV1`: New Execution Payload

### 🔍 Purpose

Submits a **new block payload** for execution and validation.

### 📘 Example

```json
{
  "jsonrpc": "2.0",
  "method": "engine_newPayloadV1",
  "params": [
    {
      "parentHash": "0x...",
      "feeRecipient": "0x...",
      "stateRoot": "0x...",
      "receiptsRoot": "0x...",
      "logsBloom": "0x...",
      "prevRandao": "0x...",
      "blockNumber": "0x...",
      "gasLimit": "0x...",
      "gasUsed": "0x...",
      "timestamp": "0x...",
      "extraData": "0x...",
      "baseFeePerGas": "0x...",
      "blockHash": "0x...",
      "transactions": [ "0x...", "0x..." ]
    }
  ],
  "id": 1
}
```

### 📘 Parameters

| Field           | Description                    |
| --------------- | ------------------------------ |
| `parentHash`    | Hash of parent block           |
| `blockHash`     | Unique hash of the new block   |
| `transactions`  | RLP-encoded transactions       |
| `prevRandao`    | Beacon chain randomness        |
| `baseFeePerGas` | EIP-1559 base fee              |
| `feeRecipient`  | Block producer reward receiver |

### ✅ Returns

```json
{
  "status": "VALID",
  "latestValidHash": "0x...",
  "validationError": null
}
```

| Status     | Meaning                            |
| ---------- | ---------------------------------- |
| `VALID`    | Payload is valid and accepted      |
| `INVALID`  | Payload is invalid and rejected    |
| `SYNCING`  | Execution client not ready         |
| `ACCEPTED` | Accepted but not verified yet (V3) |

---

## ⚔️ 4. Interaction Between Fork Choice and New Payload

### 🎯 Flow

1. CL runs fork choice rule.
2. Calls `engine_forkchoiceUpdatedV1` to notify EL.
3. If validator is proposer:

   * Includes `payloadAttributes`.
   * Gets `payloadId`.
4. CL builds block using `engine_getPayload(payloadId)`.
5. Submits block via `engine_newPayloadV1`.

---

## 🧪 5. Edge Cases

### ❌ Invalid Fork Choice

```json
{
  "status": "INVALID",
  "latestValidHash": "0xdeadbeef...",
  "validationError": "Unknown parent"
}
```

→ Returned when:

* `headBlockHash` has an unknown parent
* Reorg to invalid chain
* Finalized block reverts

---

### ⏳ Execution Not Synced

```json
{
  "status": "SYNCING"
}
```

→ EL isn’t ready to process fork choice or payload.

---

## 🆚 6. Comparison: Fork Choice vs New Payload

| Aspect               | `forkchoiceUpdated`                 | `newPayload`                   |
| -------------------- | ----------------------------------- | ------------------------------ |
| Purpose              | Update head/finalized chain in EL   | Submit new block for execution |
| Direction            | CL → EL                             | CL → EL                        |
| Initiates proposal?  | Yes (if proposer)                   | No                             |
| Block data included? | No (unless via `payloadAttributes`) | Yes (full block data)          |
| Returns              | Fork choice status + `payloadId`    | Validation result of block     |

---

## 💻 7. Native Use Example (Go, simplified)

```go
type ForkChoiceState struct {
    HeadBlockHash      string
    SafeBlockHash      string
    FinalizedBlockHash string
}

type PayloadAttributes struct {
    Timestamp             string
    PrevRandao            string
    SuggestedFeeRecipient string
}

func engineForkChoiceUpdated(state ForkChoiceState, attr *PayloadAttributes) {
    payload := map[string]interface{}{
        "jsonrpc": "2.0",
        "method": "engine_forkchoiceUpdatedV1",
        "params": []interface{}{state, attr},
        "id": 1,
    }

    response := sendJSONRPC(payload)
    handleResponse(response)
}
```

---

## 🧠 8. Advanced: How It Affects Block Production

| Event                 | Triggered Method           | Result                                                  |
| --------------------- | -------------------------- | ------------------------------------------------------- |
| Head update in CL     | `engine_forkchoiceUpdated` | EL updates internal chain state                         |
| Validator is proposer | `forkchoiceUpdated` + attr | EL starts building block (async, tracked via payloadId) |
| Block built           | `engine_getPayload`        | CL fetches built block                                  |
| Block sent back       | `engine_newPayload`        | EL executes and validates                               |

---

## 🚦 9. Payload Status Handling (Rust pseudocode)

```rust
match payload_status.status.as_str() {
    "VALID" => println!("Block is valid."),
    "INVALID" => {
        eprintln!("Invalid block: {:?}", payload_status.validation_error);
    },
    "SYNCING" => {
        eprintln!("Execution engine still syncing.");
    },
    _ => println!("Unhandled status."),
}
```

---

## 📜 10. Summary Table

| Method                     | Input                                | Output             | Use Case                        |
| -------------------------- | ------------------------------------ | ------------------ | ------------------------------- |
| `engine_forkchoiceUpdated` | Forkchoice state + payloadAttributes | Status + payloadId | Notify EL of new head/finalized |
| `engine_newPayload`        | Full block (execution payload)       | Payload status     | Submit a block for execution    |
| `engine_getPayload`        | payloadId                            | Block payload      | CL fetches EL-built block       |

---
