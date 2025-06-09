# **Danksharding & The Future of Ethereum Scaling**

## **What is Danksharding?**

* **Core Purpose**: An advanced sharding design to massively increase the amount of data Ethereum can handle.
* ***A way for Ethereum to have more data pass through the network.***
* **Primary Goal**: To create a highly-specialized and cheap place for Layer 2 solutions (like Optimistic and ZK-Rollups) to post their transaction data.
* **Impact for Users**: Directly lowers transaction fees on Layer 2s by significantly reducing their data posting costs on Layer 1.
* **Origin of the Name**:
    * **Danksharding**: Named after Ethereum researcher **Dankrad** Feist.
    * **Proto-Danksharding**: Named after Ethereum researcher **Proto**lambda.

---

## **EIP-4844: Proto-Danksharding**

* **The EIP**: EIP-4844 is the technical specification for **Proto-Danksharding**.
* **What it is**: *Proto-Danksharding is the first step we get towards full sharding.*
* **Key Idea**: It implements the transaction format and verification rules required for sharding, but without implementing the full sharding system itself.
* **Benefits**:
    * Delivers significant data cost reductions for L2s much sooner than full Danksharding.
    * Provides a simpler implementation path, allowing client teams to deliver benefits faster.
    * It is a forward-compatible upgrade; once Proto-Danksharding is live, the path to full Danksharding only requires changes to the consensus layer, not the execution layer.
* **Official Resource**: `https://www.eip4844.com/`

---

## **The Evolution of Ethereum's Sharding Roadmap**

This represents a multi-year process of simplification and pragmatism.

1.  **Early Complex Designs (2014-2016)**
    * Initial ideas involved highly complex structures like **hypercubes** and **shards on top of shards** (*super-quadratic sharding*).
    * Included concepts like protocol-managed cross-shard transactions.
    * The philosophy was described as *"more of Ethereum trying to do everything."*

2.  **Move to Quadratic Sharding**
    * The design was simplified to a single layer of sharding: a **Beacon Chain** connected to a set of shards.

3.  **Direct Block Inclusion (c. 2019-2020)**
    * Further simplification where shard blocks are directly referenced in the Beacon Chain, eliminating the need to manage complex "shard chains" as separate entities.

4.  **The Rollup-Centric Roadmap (Data Sharding)**
    * This was a fundamental pivot in strategy.
    * **Old Idea**: Shards would contain transactions and execute them (execution sharding).
    * **New Idea**: Shards will only contain large amounts of data ("blobs"). They do not execute anything.
    * The new paradigm:
        * *Ethereum L1 provides scalable data and non-scalable computation.*
        * *Layer 2 Rollups convert scalable data and non-scalable computation into scalable computation.*

5.  **Danksharding**
    * The next major simplification, made possible by the rise of Proposer-Builder Separation (PBS).
    * Instead of each shard having its own proposer, a single **block proposer** chooses the data for *all* shards in a given slot.
    * This massively reduces consensus complexity and simplifies the economic model.

---

## **Proposer-Builder Separation (PBS)**

* **Context**: The rise of **MEV** (*Maximal Extractable Value*) has led to the professionalization of block creation. Building the most profitable block is a highly complex task.
* **The Separation of Roles**:
    * **Builder**: A highly specialized entity running powerful hardware. Their job is to communicate with searchers, find the most profitable MEV opportunities, and **build** a complete, optimal block.
    * **Proposer**: A regular validator. Their job is simple: listen to bids from multiple builders and **propose** the block from the builder who offers them the highest payment.
* **Why PBS Enables Danksharding**:
    * Danksharding requires a network participant capable of processing and handling a large amount of data (~16MB per block).
    * The **Builder** role is expected to be filled by sophisticated actors who already run the kind of high-end hardware necessary to handle this data load.
    * This allows the protocol to assume such a capable entity exists without requiring every single small validator to have the same capabilities, thus preserving decentralization.

---

## **Data Availability (DA)**

* **What it is**: It is not just about storing data. It's about achieving **network consensus** that a piece of data *was made available* for anyone to download during a specific time window.
* **DA vs. Storage (e.g., IPFS)**:
    * **IPFS**: A system for storing and retrieving data. A malicious publisher could selectively reveal data to only some peers.
    * **Ethereum DA**: A system that provides a hard, binary consensus guarantee. The entire network agrees on *whether or not* a piece of data with a specific hash was successfully published.
* **Why DA is Critical for Rollups**:
    * Rollups post transaction data to L1 to inherit its security.
    * This data ensures that if the rollup's operator (sequencer) becomes malicious or goes offline, users can use the L1 data to reconstruct the state and safely withdraw their assets.
    * Without this on-chain DA, a malicious operator could freeze all user funds, a vulnerability present in systems known as **Validiums**.
* **Data Pruning**:
    * The Ethereum consensus layer does **not** need to store this data forever.
    * The plan is to prune the data blobs after a set period (e.g., 30-60 days). This is enough time for any interested party (L2s, indexers, etc.) to retrieve and back it up.

---

## **How Danksharding Achieves Scalable DA**

### **1. Data Availability Sampling (DAS)**

* **The Challenge**: How can network validators be sure that a massive ~16MB block of data is fully available without every validator having to download all 16MB?
* **The Solution**: Validators only download a few, randomly selected small pieces (*samples*) of the data.
* **The Problem with the Naive Approach**: If an attacker withholds just 1% of the data, the chance of a small number of random samples detecting that specific missing piece is very low.

### **2. Reed-Solomon Encoding**

* **The Fix**: Before sampling, the data is mathematically transformed using **Reed-Solomon codes**.
* **How it Works**:
    1.  The original data (e.g., 16MB) is used to define a polynomial function, let's call it $P(x)$.
    2.  This polynomial is then "evaluated" at more points than necessary, expanding the data (e.g., from 16MB to 32MB). This is the *encoded data*.
* **The Magic Property**: Any **50%** of the *encoded data* can be used to reconstruct 100% of the original polynomial and thus the original data.
* **The Result**: An attacker can no longer hide a small piece of data. To make the data unavailable, they must withhold **more than 50%** of the encoded data. The probability that a validator's random samples all miss this massive hole is astronomically small.

```
// Simplified Conceptual Logic
function isDataAvailable(encodedData) {
  const SAMPLES_TO_CHECK = 30;
  for (let i = 0; i < SAMPLES_TO_CHECK; i++) {
    let randomIndex = Math.random() * encodedData.length;
    let sample = downloadSample(encodedData, randomIndex);
    if (!sample) {
      // A single failed sample does not mean failure
      // But if many fail, we can be statistically certain
      // the data is unavailable.
    }
  }
  // If we successfully get all our samples, we have a
  // very high statistical guarantee (e.g., 99.9999999%)
  // that at least 50% of the data is available, which is
  // enough to reconstruct the whole thing.
  return true;
}
```

---

## **KZG Commitments**

* **The Problem**: DAS ensures the data is available, but how do we know the data was *encoded correctly* as a polynomial in the first place? What if a builder just filled the 32MB of space with garbage?
* **The Solution**: **KZG (Kate-Zaverucha-Goldberg) Commitments**.
* **What they are**: A special type of cryptographic commitment, you can think of it as a *hash of a polynomial*.
* **Key Properties**:
    * **Binding**: A single, small commitment (like a hash) represents the entire polynomial.
    * **Proof of Evaluation**: A prover can generate a small proof that the polynomial $P(x)$, when evaluated at a point $z$, equals $a$ (i.e., $P(z) = a$), without revealing the entire polynomial.
* **How it's Used in Sharding**:
    1.  The block builder creates the polynomial for the shard data and publishes the KZG commitment.
    2.  When a validator requests a sample at point $z$, the builder provides the value $a$ and a small proof.
    3.  The validator can efficiently check that this sample `(z, a)` is consistent with the original commitment, guaranteeing that it lies on the one true polynomial. This prevents the builder from cheating.

### **The Trusted Setup**

* **Requirement**: KZG commitments require a set of public parameters known as a **Structured Reference String (SRS)**.
* **The Secret**: This SRS is generated using a secret number (often called "toxic waste"). This secret must be destroyed. If anyone ever discovered it, they could create false proofs and potentially compromise system security.
* **The Ceremony**: To avoid trusting a single party, a **Multi-Party Computation (MPC) ceremony** is held.
    * 1.  A person generates a secret and creates the first version of the SRS. They then destroy their secret.
    * 2.  A second person takes the first SRS, mixes in their own new secret, and produces a second version. They then destroy their secret.
    * 3.  This continues for thousands of participants.
* **Security Guarantee**: The final SRS is secure as long as **at least one person** in the entire chain of participants was honest and destroyed their secret. This is known as an *$N-1$ of $N$* trust assumption.
* **How to be Trustless**: *Participate in the ceremony yourself*. If you know you acted honestly, you personally don't need to trust anyone else for the setup to be secure for you.

---

## **Scaling Impact by the Numbers**

* **Current L1 (Calldata)**
    * **Data per Block**: ~50-100 KB
    * **Mechanism**: Data is stored as part of transaction `calldata`, which is processed by the EVM and stored by all nodes forever. This is very expensive.

* **Proto-Danksharding (EIP-4844)**
    * **Target Data per Block**: **~1 MB** (via 2-4 blobs of 256KB each).
    * **Mechanism**: Introduces a new transaction type with a separate data "blob". All consensus nodes download these blobs, but they are pruned after ~1-2 months.
    * **Expected Impact**: A ~**10x** increase in data throughput available to rollups, leading to a major fee reduction.

* **Full Danksharding**
    * **Target Data per Block**: **~16 MB** (via 64 shards/blobs).
    * **Mechanism**: Full Data Availability Sampling (DAS). Validators no longer download all blobs; they sample them instead. The data load is distributed across the entire validator set.
    * **Expected Impact**: A further ~**16x** increase on top of Proto-Danksharding, making data for rollups extremely cheap.

---

## **A New Fee Market: Multi-Dimensional EIP-1559**

* **The Concept**: Different resources within Ethereum should have their own independent fee markets.
* **EIP-4844's Implementation**:
    * It creates **two separate fee markets** within each block.
    * **1. Execution Gas Market**: The existing EIP-1559 market for transaction computation, priced in **gas**.
    * **2. Blob Data Market**: A new EIP-1559 style market for data blobs, with its own base fee, priced in **blob gas**.
* **Why this is better**:
    * Demand for blockspace (computation) can be high while demand for data is low, and vice-versa.
    * Separating the markets prevents a surge in demand for one resource (e.g., during an NFT mint) from making the other resource (e.g., L2 data posting) unnecessarily expensive.

---

## **Open Problems & How to Contribute**

* **Engineering Challenges**:
    * **DAS Networking**: Designing, building, and optimizing the peer-to-peer networking layer to handle the rapid distribution and sampling of shard data. This is a major engineering effort.
    * **Decentralized History**: Building robust and decentralized systems (like the **Portal Network**) to ensure historical data remains accessible for all after the core protocol prunes it.
* **Research & Economics**:
    * **PBS Design**: Refining the Proposer-Builder Separation mechanism to be more robust, efficient, and censorship-resistant.
    * **Proof-of-Stake Evolution**: Integrating the sharding design with future upgrades to Ethereum's core consensus, such as **Single Slot Finality (SSF)**.
* **Layer 2 Ecosystem**:
    * **L2 Light Clients**: Creating lightweight clients that allow users to verify the state of rollups (Optimism, Arbitrum, Starknet, etc.) without having to trust the rollup's centralized sequencer.
    * **The Bear Market is the Builder Market**: Now is the time to read the specs, get involved with projects, and contribute to building out this infrastructure.