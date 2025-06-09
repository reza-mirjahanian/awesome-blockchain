---

# **Proto-Danksharding (EIP-4844): Ultimate Mastery Guide**

---

## **1. Foundational Concepts**

### **1.1. What is Proto-Danksharding?**
- **Proto-Danksharding (EIP-4844)** introduces a new transaction type—**blob-carrying transactions**—to scale Ethereum L1 by lowering data availability costs for rollups.
- It is the **precursor** to full Danksharding. It adds "blobs" for cheap data storage, but without full data sharding.

---

### **1.2. Core Terminology**
- **Blob:** Large, cheap, transient chunks of binary data attached to special transactions (not directly readable by EVM).
- **KZG Commitment:** A cryptographic primitive for polynomial commitment, enabling succinct verification of blob inclusion.
- **Data Availability:** Ensures that data for L2 (e.g., rollups) is available for fraud-proofs or ZK proofs.
- **Danksharding:** The next-gen sharding proposal using single proposer and blob data, fully sharding block data for maximum throughput.

---

### **1.3. Proto-Danksharding vs. Danksharding**

| Feature            | Proto-Danksharding (EIP-4844) | Danksharding (Future)         |
|--------------------|-------------------------------|-------------------------------|
| Blobs per Block    | 1–3 (soft/hard cap)           | 64+ (scalable across shards)  |
| Blob Lifetime      | 4096 epochs (~18 days)        | Longer/Customizable           |
| Execution Layer    | No direct EVM access          | No direct EVM access          |
| Proposer           | Single proposer               | Single proposer               |
| Data Structure     | Monolithic chain + blobs      | Many data-only shards + blobs |
| Purpose            | Rollup DA, test infra         | Full throughput scaling        |

---

## **2. Step-by-Step Structure**

---

### **2.1. Blob Transactions**

#### **Blob Transaction Fields**
- `blob_versioned_hashes[]` — Reference to one/more blobs (by KZG commitment)
- `blobs[]` — The actual blob data (off-EVM, only for DA)
- `max_fee_per_blob_gas` — User gas bid for blob inclusion
- `blob_signature` — KZG polynomial openings for compact proof

#### **Blob Fundamentals**
- Each blob ≈ 128 KB (field elements)
- Strict gas metering and caps
- Blobs decoupled from state-execution—**not accessible from smart contracts**

---

#### **Blob Transaction Lifecycle**

1. **User/rollup creates a blob-carrying transaction**
2. **Blobs are encoded, committed via KZG, and gossip across P2P**
3. **Block proposer includes references + proof (not blob data) in the block body**
4. **Nodes store blobs for ~18 days, gossip data for DA**
5. **Verification leverages KZG polynomial commitments for integrity**

---

#### **Go Code Example: Blob Transaction Construction**

```go
type Blob struct {
    Data []byte   // Must be 128KB
}

type BlobTx struct {
    Nonce        uint64
    Blobs        []Blob
    KZGProofs    [][]byte
    MaxBlobGas   uint64
}

func CreateBlob(data []byte) (Blob, error) {
    if len(data) != 128*1024 {
        return Blob{}, errors.New("invalid blob size")
    }
    return Blob{Data: data}, nil
}
```
---

#### **Rust Code Example: KZG Commitment for a Blob**

```rust
use kzg::{KZGCommitment, Blob};
fn commit_blob(blob_data: &[u8]) -> KZGCommitment {
    // Input should be converted into field elements for KZG
    let blob = Blob::from_bytes(blob_data);
    KZGCommitment::commit(&blob)
}
```

---

### **2.2. Data Availability and Integrity**

- **Blob Storage:** P2P layer ensures DA by gossiping and storing blobs for a set duration.
- **KZG Commitments:** Proofs ensure that blobs referenced are available and unmodified.
- **Pruning:** Blobs are pruned after expiry, state remains unaffected.

---

### **2.3. Gas, Pricing, and Economics**

| Parameter                  | Value (Proto-Danksharding) |
|----------------------------|----------------------------|
| Blob Gas Target / Block    | 0.5M                       |
| Blob Gas Hard Cap / Block  | 1M                         |
| Blob Gas Base Fee          | Separate dynamic fee       |
| Storage Duration           | ~18 days (temporary)       |
| Cost (per blob)            | Cheaper than calldata      |

#### **How is Blob Gas Calculated?**
- Modeled after EIP-1559, with **separate blob fee market**
- Blobs do **not** compete with normal txs for blockspace

---

#### **Edge Case: Blob Gas Overflow**
- If blob demand spikes, base fee increases, restricting excessive blob inclusion.

---

### **2.4. Rollup Integration**

- **Rollups submit proofs/data as blobs, reference them in L2 contracts**
- L2 contracts verify data from blobs using KZG
- **No native read from EVM**, relies on off-chain actors to fetch blob content

---

### **2.5. Node Implementation**

- **Execution Layer**: Parses and verifies blob-carrying tx, enforces base fee, cap
- **Consensus Layer**: Includes blob commitment in block proposal, ensures inclusion proofs
- **Networking/P2P**: Enhanced blob propagation protocol (eth/4844)

---

#### **Native Code Snippet — Blob Data Validation (Pseudo-Go)**
```go
func ValidateBlob(blob Blob) error {
    if len(blob.Data) != BLOB_SIZE {
        return errors.New("invalid blob size")
    }
    // Validate KZG commitment
    if !VerifyKZG(blob.Data, blob.KZGProof) {
        return errors.New("KZG verification failed")
    }
    return nil
}
```

---

## **3. Advanced Details**

---

### **3.1. KZG Polynomial Commitments**

- Enable **succinct proofs** that a blob is committed (included and not tampered)
- Security depends on trusted setup (for now); switching to transparent later

#### **Security Analysis**
- **Pro:** Compact, fast verification, secure DA
- **Con:** Trusted setup necessary (potential single point of failure if compromised)

---

### **3.2. Edge Cases & Limitations**

- **No direct blob read in EVM**—prevents L1 state bloat, ensures scalability
- Blobs are ephemeral: **rollups must fetch data before expiry**
- Rollups must coordinate DA checks **off-chain**
- **MEV:** No special MEV incentives, but rollup proposers may auction for blob space

---

### **3.3. Monitoring & Tooling**

- **Lighthouse / Prysm / Geth**—implement full support for blob-carrying txs, KZG validation
- **Blobscan:** Third-party tool to visualize blob/DA metrics

---

## **4. Comparisons**

### **Compared with Calldata**

| Feature               | Calldata                         | Blob (4844)                 |
|-----------------------|----------------------------------|-----------------------------|
| Storage Cost          | Expensive, forever in state trie | Cheap, ephemeral            |
| Execution Access      | Readable in EVM                  | Not readable                |
| Purpose               | Any tx data                      | Pure data availability      |
| Scaling               | No additional scaling            | Massive L2/L3 scaling       |

---

### **Compared with Full Danksharding**

| Aspect                | Proto-Danksharding           | Danksharding              |
|-----------------------|-----------------------------|---------------------------|
| Shards                | None                         | Dozens+                   |
| Blobs/block           | Few                          | Many (64+)                |
| Throughput            | ~0.5 MB/block                | Multi-GB/s                |
| Availability Sampling | No                           | Yes                       |

---

## **5. Code, Patterns, and Real-World Usage**

---

### **5.1. Blob-Carrying Transaction (Rust, EIP-4844 draft style)**

```rust
struct BlobTx {
    nonce: u64,
    kzg_commitments: Vec<KZGCommitment>,
    blob_data_hashes: Vec<[u8; 32]>,
    max_fee_per_blob_gas: u64,
    // ... signature, payload, ...
}

impl BlobTx {
    fn verify_blobs(&self, blobs: &Vec<Blob>) -> bool {
        // Map each blob to KZG, check ALL
        blobs.iter().zip(&self.kzg_commitments).all(|(blob, comm)| {
            comm.verify(blob)
        })
    }
}
```

---

### **5.2. Go: Rollup L2 Posting Data via Blobs**

```go
func SubmitRollupData(l2Data []byte, keys KZGKeys) *BlobTx {
    blob := CreateBlob(l2Data)
    proof := KZGCommit(blob.Data, keys)
    tx := BlobTx{
        Blobs: []Blob{blob},
        KZGProofs: [][]byte{proof},
        MaxBlobGas: DefaultGasPrice,
    }
    return &tx
}
```

---

### **5.3. Edge Case: Blob Expiry and Fallback**

- **Scenario:** L2 tries to verify proof after blob expiry → Failure!
- **Mitigation:** Rollup must:
    - Fetch and back up DA before deadline
    - Optionally store merklized data for fraud/validity proofs

---

## **6. Complexity & O() Analysis**

| Operation                  | Time Complexity | Notes                                           |
|----------------------------|----------------|-------------------------------------------------|
| Blob KZG Verify            | O(1)           | Succinct constant-time verification             |
| Blob Gossip/Storage        | O(N)           | N = number of nodes per epoch                   |
| Blob Lookup (by hash/ref)  | O(1)           | KZG commitment references                       |
| Blob Expiry/Pruning        | O(1)           | Time-triggered, not per-use                     |

---

## **7. Real-World Related Projects**

- **Optimism, Arbitrum, Base**: Already planning/prototyping with blobs to reduce L2 fees 10x+
- **Ethereum Mainnet**: All major clients support EIP-4844 as of Dencun upgrade

---

## **8. Tricky Pitfalls and Gotchas**

- **KZG Trusted Setup:** Watch for ceremony compromise
- **Blob-Only Data:** Any critical in-EVM data must still go through calldata
- **Networking:** P2P blob propagation must avoid flooding/bandwidth attacks
- **Rollup DAs:** If off-chain actor misses DA window, unrecoverable data loss

---

## **9. Further Advanced Topics**

- **Full Danksharding with Data Availability Sampling**
- **KZG to Verkle Upgrade Path**
- **Rollup Channels with Blobs & zkProof Integration**
- **Stateless Witnesses, Blob Pruning Strategies**
- **Cross-Shard Data Availability Implications**

---

## **10. Suggested Next Advanced Topic**

**Full Danksharding (EIP-4844 → EIP-4845+) & Data Availability Sampling (DAS): Implementation, Security, and Coding Patterns at Scale**

---