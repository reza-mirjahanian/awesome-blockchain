

### What is Comet BFT?

**Comet** is a framework for building blockchain applications, similar to how Django or Ruby on Rails are frameworks for web applications.

It consists of three core components:
1.  **Consensus:** How nodes agree on the next block.
2.  **Peer-to-Peer:** How nodes exchange data between each other.
3.  **ABCI (Application Blockchain Interface):** A special interface that separates the application from the other components.

As an application developer, you focus only on the application logic, without needing to focus on peer-to-peer or consensus.

**Key ABCI Methods:**
Comet uses ABCI methods to interact with the application:
* **CheckTx**: Called when Comet wants the application to verify a transaction.
* **PrepareProposal**: Called when Comet wants the application to form the next block.
* **ProcessProposal**: Called when Comet wants the application to verify a block it received from the network.
* **FinalizeBlock**: Called when Comet wants the application to commit the new state.

The application can be any arbitrarily complex state machine, such as **CosmWasm**, which enables smart contracts written in the Rust programming language.

***

### Comet BFT Position and Benefits

In the Interchain stack, Comet is at the bottom.

**Reasons to choose Comet BFT:**
* It has been in production since **2019**, is battle-tested, passed multiple security audits, and was verified by **Jepsen**.
* It provides **instant finality**.
* It is relatively fast, capable of processing up to **500 transactions per second** according to the latest report (in a 200-node, non-globally distributed network).
* It is also **developer-friendly** because it is written in **Go** and utilizes **ABCI**.

An estimated 99% of Cosmos projects are using Comet (if they are using the Cosmos SDK). Comet is also being used outside of the Cosmos Network and the web3 sphere.

***

### Current State and Features

**Proposer-Based Timestamps (PBTS)**
* PBTS replaces the Doval algorithm, providing more accurate block timestamps, which is beneficial for time-sensitive applications.
* It also prepares the way for **Signature Aggregation**.

**ABCI 2.0**
* Allows Cosmos SDK to implement **optimistic block execution**.
* Crucially, it allows attaching **arbitrary data to votes**, which can be used by the next block proposer.
* This unlocks powerful use cases like **decentralized price oracles**, **threshold encryption for transaction privacy**, and **cross-chain coordination**.

**Other Work:**
* Performance optimization (in collaboration with the Osmosis team).
* Bandwidth reduction.
* The addition of the **BLS curve**.
* A new **release support policy** has been established.

**Spam Attack Advisory:**
Due to recent spam attacks, a guide was written to help teams mitigate these issues, advising them to **upgrade to the latest version** for all improvements and bug fixes.

***

### New and Upcoming Features

**Mempool Lanes (Expected in v1.1)**
* Mempool Lanes change the mempool from a single list of transactions to a **map of sub-lists**, where each sub-list can have a **different, application-defined priority**.
* This aids in **spam protection** by allowing higher-priority transactions to be disseminated and included in blocks faster than lower-priority transactions.

**QUIC Transport Protocol**
We are working on adopting **QUIC** as the new transport protocol to replace TCP. QUIC offers neat features:
* Built-in streams, removing the need for custom out-of-protocol multiplexing.
* Zero handshake and no head-of-line blocking.
* TLS integration, which removes the need for custom encryption.

**Dynamic Optimal Graph (DOG) Algorithm**
* This algorithm extends the base flood algorithm with a mechanism to **eliminate cycles** in transaction paths within the network.
* Preliminary results in a 200-node network showed a **bandwidth reduction of 75%**, with network usage dropping from 3.2 GB/s to 1.3 GB/s as cycles were eliminated.

**Other Ideas Being Explored:**
* Threshold signature aggregation
* Soft upgrades (meaning no hard forks)
* Pre-confirmation
* Pluggable crypto

The vision for **Comet V2** is significant improvement across the board, including better **network, memory, and disk** usage, leading to greater **stability** under a high volume of transactions.