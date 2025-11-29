

## What is Tendermint: Overview

* **Tendermint** combines a **Byzantine Fault Tolerant (BFT) consensus engine** with a **generic application interface**.
* This separation enables any application (e.g. blockchain application logic) to run on top of a secure consensus layer — meaning you don’t need to implement consensus logic yourself if your app runs on Tendermint.
* The pattern is often referred to as “consensus engine + application layer.”

---

## Why BFT Matters

* In distributed systems, nodes may fail — including behaving maliciously. “BFT” refers to tolerance of this: the system continues to operate correctly even if up to a threshold of nodes fail or act maliciously.
* Tendermint’s BFT engine ensures consistency (all non-faulty nodes agree on the same state) even in the presence of faulty or malicious participants.

---

## Core Components & Architecture

* **Consensus Engine**: handles proposal, voting, and commit of blocks.
* **Application Interface**: typically via an Application Blockchain Interface (ABCI). This allows application logic to plug into the consensus engine.
* **Networking Layer**: handles peer-to-peer connections between nodes, message propagation, etc.
* **Block Structure**: blocks contain transactions; after consensus, blocks are committed and application logic processes transactions.

This modular architecture decouples application logic from consensus, enabling flexibility in building diverse applications (smart contracts, custom logic, etc.) on the same consensus foundation.

---

## How Tendermint Consensus Works — Step by Step

1. **Propose Phase**

   * Among the validators, one is selected as the *proposer* for the next block (round).
   * The proposer assembles a block of pending transactions and broadcasts a **PROPOSAL** message to other validators.

2. **Prevote Phase**

   * Validators examine the proposed block. If it’s valid (transactions, state, etc.), they broadcast a **PREVOTE** message indicating they accept this block as valid.
   * If validators refuse (e.g. block invalid), they send a PREVOTE for “nil” (i.e. no block).

3. **Precommit Phase**

   * If a block receives +⅔ of PREVOTE messages (i.e. two-thirds majority of validators), validators broadcast a **PRECOMMIT** for that block.
   * If +⅔ of validators send PRECOMMIT, the block is committed.

4. **Commit & Application Execution**

   * Once committed, the block becomes part of the canonical chain.
   * The application layer (via ABCI) executes the transactions in the block, updates application state, processes results, etc.

5. **Next Round / Next Block**

   * The process repeats: new block proposals, new voting, etc.

This cycle ensures that all honest validators agree on the same block and state transitions.

---

## Safety and Liveness Guarantees

* **Safety**: Under the assumption that less than one-third of validators are Byzantine (faulty or malicious), the algorithm guarantees that no two non-faulty validators commit different blocks for the same height.
* **Liveness**: As long as a majority (or required threshold) of validators are alive, connected, and responsive, the system continues to produce new blocks.

Thus, Tendermint provides finality: once a block is committed, it cannot be reverted by a fork (given the BFT assumptions).

---

## Advantages of Tendermint Design

* **Finality**: Immediate finality on commit — unlike probabilistic consensus (e.g. Proof-of-Work) where reorganizations are possible.
* **Modularity**: Application developers don’t need to reimplement consensus; they focus on application logic.
* **Performance**: BFT consensus typically allows for lower latency and higher throughput compared to PoW-type consensus, because block confirmation doesn’t rely on probabilistic mining.
* **Deterministic State Transition**: Because of the deterministic consensus + application model, every honest node will end up in exactly the same state — no “eventual consistency” ambiguities.

---

## Use Cases & Implications

* Ideal for permissioned or permissionless blockchains where finality and correctness are more important than open mining.
* Useful when building custom blockchain applications (smart contracts, asset tracking, private chains) — developers get a robust consensus engine out of the box.
* Particularly suitable where regulatory or enterprise-grade guarantees are needed: determinism, fast finality, security under adversarial conditions.

---

## Key Concepts to Remember

* **Validator set**: the nodes responsible for proposing and voting on blocks.
* **Proposer rotation**: the role of proposer rotates among validators to avoid centralization or over-reliance on one node.
* **Two-thirds threshold**: both prevote and precommit phases rely on a supermajority (⅔) to ensure agreement and prevent forks.
* **Application-agnostic consensus**: using ABCI lets the consensus engine be decoupled from business logic, enabling a variety of applications on the same foundation.
* **Deterministic finality**: once a block is committed via consensus, its state transitions are final and irreversible under the honest-validator assumption.

---

## Why This Approach Matters Compared to Other Consensus Mechanisms

* Unlike PoW (Proof-of-Work), Tendermint doesn’t waste computational resources on mining.
* Unlike probabilistic consensus (with potential chain reorganizations), Tendermint gives deterministic finality: you know immediately when a transaction is final.
* For blockchain applications needing consistency, reliability, and quick finality (e.g. finance, supply chain, enterprise), this approach balances decentralization (through validator set) and finality/performance.

---

## Practical Considerations When Building on Tendermint

* Validator management: choosing validators, managing their keys, handling validator joining/leaving, ensuring honest behavior.
* Network conditions: requires that honest validators remain connected and responsive to maintain liveness.
* Application logic via ABCI must be deterministic — nondeterministic behavior can lead to state divergence among nodes.
* Handling malicious validators: BFT assumes up to a certain threshold can be faulty — beyond that, safety/liveness guarantees may break.

---

## Summary of the Consensus Workflow (Pseudocode-style)

```
while (true) {
  proposer = select_next_validator()
  block = proposer.create_block(pending_transactions)
  broadcast(PROPOSAL, block)

  collect prevotes from validators
  if (prevotes >= 2/3) {
      broadcast(PRECOMMIT, block)
  } else {
      broadcast(PRECOMMIT, nil)
  }

  collect precommits from validators
  if (precommits >= 2/3) {
      commit(block)
      apply_transactions(block)
      start next height
  } else {
      go to next round (same height), repeat
  }
}
```

---

## Final Thoughts on Tendermint’s Value

Tendermint provides a robust, well-structured BFT consensus engine — allowing developers to build blockchain applications with deterministic finality, high performance, and modular architecture. By separating consensus from application logic, it reduces complexity for developers while maintaining strong safety and liveness guarantees.


**CometBFT is a modern fork of Tendermint Core**, designed as a more modular and performant version of the original consensus engine. While Tendermint is the original project, CometBFT is its successor, aiming to improve scalability, flexibility, and developer experience through features like a modular design and the ABCI++ interface. Essentially, CometBFT is the community-supported evolution of Tendermint, meant to replace it in future blockchain applications. 

| Feature  | Tendermint | CometBFT |
| --- |  --- |  --- |
| **Relationship** | Original project | Fork and successor to Tendermint |
| **Modularity** | Less modular, tightly coupled components | More modular, allows for greater flexibility and easier integration |
| **Performance** | Baseline performance | Improved performance with optimizations to reduce message overhead and improve block validation efficiency |
| **Developer Experience** | Legacy interface | Includes modern features like ABCI++ for easier development in any programming language |
| **Current Status** | Being phased out in favor of CometBFT | The active and forward-looking project for the Interchain Stack |
| **Maintenance** | Requires significant resources to maintain and update, especially with its legacy forks | Designed for easier maintenance and reduced tech debt |