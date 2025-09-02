🪙 Native Rollups & The Future of Ethereum L2s
==============================================

### A New Architecture for Layer 2 Scaling

🚨 **Problems with Current Rollups**
------------------------------------

Today's Layer 2 rollups, while innovative, face significant challenges that distance them from the core ethos of Ethereum.

-   **Complex Custom Verification**

    -   Rollups rely on thousands of lines of bespoke code for fraud proofs (Optimistic) or SNARK verifiers (ZK).

    -   *Sub-point:* This introduces a high risk of vulnerabilities in custom ZK circuits or fraud proof game logic.

-   **Centralization & Governance Overhead**

    -   To manage risks, most rollups introduce centralization as a safeguard.

    -   *Examples:* Security councils with upgrade powers, centralized sequencers.

    -   *Sub-point:* This is a departure from Ethereum L1's immutability and permissionless nature.

-   **Manual EVM Upgrades**

    -   When Ethereum mainnet hard forks (e.g., Pectra upgrade with EIP-7702), each rollup team must manually implement the changes.

    -   *Sub-point:* This creates a lag, where rollups are not fully equivalent to the L1, with some teams taking longer than others to upgrade.

-   **Expensive On-Chain Verification**

    -   The final verification of proofs on L1 is a costly process, contributing to rollup transaction fees.

-   **Fragmented, Siloed Systems**

    -   Each rollup operates as a separate system with its own trust assumptions, rather than as a true extension of Ethereum.

💡 **What are Native Rollups?**
-------------------------------

Native rollups propose a fundamental architectural shift: **directly using Ethereum's own execution engine to verify L2 state transitions.**

-   **Core Mechanism: The `execute` Precompile**

    -   A new precompile (a low-cost, complex operation built into the EVM) is introduced to the Ethereum L1.

    -   This precompile exposes Ethereum's built-in EVM execution engine to any smart contract.

-   **How It Works**

    1.  A rollup bundles transactions into a `trace`.

    2.  Instead of building its own proof system, the rollup calls the L1 `execute` precompile with this trace.

    3.  The Ethereum L1 network itself handles the verification of the state transition.

    4.  Ethereum validators either re-execute the trace or, in the long term, verify a ZK proof for it.

-   **Analogy: "Programmable Execution Shards"**

    -   This model is akin to the original "sharding" concept, where rollups act as parallel execution environments whose state is directly validated by the main chain.

🔧 **The `execute` Precompile Explained**
-----------------------------------------

The `execute` precompile is the heart of the native rollup design. It's a function that L1 validators run to verify a rollup's state change.

**Key Inputs:**

-   `pre_state_root`: The rollup's state before the transactions.

-   `post_state_root`: The rollup's state after the transactions.

-   `trace`: The list of transactions and state changes.

-   `gas_used`: The precise amount of gas consumed by the transactions.

**Validation Process:**

1.  The precompile takes the `pre_state_root` and processes the `trace`.

2.  It confirms that the execution of the trace results in the *exact* `post_state_root`.

3.  It verifies that the reported `gas_used` is accurate.

**Resource Management:**

-   To prevent Denial-of-Service (DoS) attacks on the L1, there is a block-wide `execute_cumulative_gas_limit`.

-   This caps the total computational work from all native rollup verifications within a single L1 block.

💨 **Key Feature: Stateless Verification**
------------------------------------------

A critical innovation of native rollups is that L1 validators can verify rollup state without having to store the entire state of every rollup.

-   **No State Bloat:** This prevents the Ethereum L1 from becoming bloated with the state data of hundreds or thousands of L2s.

-   **How it Works:**

    -   Validators are provided with *state access proofs* alongside the transaction trace.

    -   These proofs allow them to temporarily reconstruct only the specific parts of the rollup's state needed for verification.

    -   Once verified, the state data can be discarded.

-   **Shift in Validator Workload:**

    -   This changes the task of validation from being **storage-intensive** to being **computation-focused**.

**Verification Approaches**
---------------------------

The rollout of native rollups is envisioned in two phases.

### 🚶 **Phase 1: Re-execution (Launch)**

This is the initial, simpler implementation designed as a proof of concept.

-   **Process:** L1 validators receive the rollup's transaction trace and literally re-execute every single transaction to verify the outcome.

-   **Scalability:** This is **not a scaling solution**. The computational load is high.

-   **Purpose:** To prove that the core concept works and that the L1 can be used to natively secure rollup execution. We could call these "pessimistic rollups."

### 🚀 **Phase 2: ZK-SNARK Verification (Endgame)**

This is the long-term, scalable vision.

-   **Process:** L1 validators receive a ZK proof alongside the trace. They can verify this proof computationally cheaply, confirming the state transition's correctness without re-executing it.

-   **Client Diversity:** Crucially, the protocol does **not** enshrine a single, standardized ZK-VM.

    -   Validators can run a variety of ZK execution clients (e.g., a "zk-Geth," "zk-Reth").

    -   This means a single rollup may need multiple proof formats to be generated for it, ensuring that no matter which ZK client a validator runs, they can verify the rollup. This maintains client diversity, a cornerstone of Ethereum's security.

⚖️ **Optimistic vs. ZK Native Rollups**
---------------------------------------

The native rollup design can accommodate both optimistic and ZK approaches, each with distinct trade-offs.

| **Feature** | **Optimistic Native Rollups** | **ZK Native Rollups** |
| --- |  --- |  --- |
| **Data Availability** | ✅ Can use off-chain DA (e.g., Celestia). Data is only provided on-chain during a dispute. | ❌ Data must be posted on L1. Validators need immediate access to generate/verify proofs. |
| **Finality** | 🐌 **Slow.** A long challenge window (1-7 days) is required before finality. | ⚡️ **Fast.** Enables near real-time settlement, with finality in one L1 slot (12 seconds). |
| **Economic Impact** | 💰 **Low Capital Efficiency.** The slow finality creates high costs for bridges and liquidity. | 💸 **High Capital Efficiency.** Fast settlement makes moving funds cheap and efficient. |
| **Use Case** | Well-suited for applications where finality speed is not critical. | Ideal for DeFi, payments, and other applications requiring fast, secure settlement. |

🏗️ **Supporting L1 Upgrades**
------------------------------

For the ZK native rollup vision to be fully realized, certain changes to Ethereum's core consensus mechanism are beneficial.

-   **Attester-Proposer Separation (APS)**

    -   A planned upgrade that separates the roles of block *proposers* (who order transactions) and *attesters* (who vote on validity).

    -   This creates a new specialized role: the **Execution Proposer**, who is responsible for block execution.

-   **One-Slot Delayed Execution**

    -   A change where a block is proposed in slot **N**, but executed and its state finalized in slot **N+1**.

-   **Why are these important?**

    -   Generating a ZK proof for an entire block of transactions is computationally intensive. Currently, a proposer has only milliseconds to do this within their 12-second slot.

    -   Delayed execution **gives the Execution Proposer a full 12 seconds** to generate the ZK proof.

    -   This levels the playing field, preventing centralization where only actors with extremely powerful, specialized hardware ("beefy provers") can participate.

✨ **Technical Benefits of Native Rollups**
------------------------------------------

This new architecture unlocks several powerful advantages for the entire Ethereum ecosystem.

-   **🛡️ Inherited L1 Security**

    -   Rollups no longer rely on their own custom, audited, and upgradeable proof systems. They inherit the full security, liveness, and decentralization of the Ethereum L1 validator set.

-   **🔄 Automatic EVM Equivalence**

    -   When the L1 EVM is upgraded, all native rollups instantly inherit the changes. No more manual updates or lag. A new opcode on L1 is immediately available on all native L2s.

-   **🤝 Trustless Cross-Rollup Composability**

    -   Because all native rollups are secured by the same L1 mechanism, it becomes possible to have trustless and potentially atomic interactions between them.

-   **⚙️ Operational Simplification**

    -   Rollups become true extensions of Ethereum, not fragmented systems with their own governance councils. This creates a more unified security model.

🚧 **Challenges & Roadmap**
---------------------------

The path to native rollups is ambitious and has several hurdles to overcome.

**Challenges:**

-   ***DA Overhead:*** ZK native rollups require more data to be posted on-chain compared to some existing L2 designs, which could increase costs until L1 data availability scales further.

-   ***Migration Complexity:*** Existing L2s like Arbitrum or Optimism would need to undergo a significant and complex refactoring to transition to a native model.

-   ***Standardization:*** The ecosystem would need to rally around new standards for rollup bridges and interfaces to ensure interoperability.

**Roadmap:**

-   **EIPs & Hard Forks:** The proposal requires several Ethereum Improvement Proposals to be accepted and implemented in a future hard fork.

-   **Timeline:** The community consensus is that this is a long-term vision, likely **not before 2026**.

-   **Phased Rollout:** The implementation will be gradual, starting with the simple re-execution model and evolving towards full ZK-SNARK verification over time.

🌌 **The Endgame Vision**
-------------------------

Native rollups blur the lines between L1 and L2, aiming for a more unified, secure, and efficient Ethereum.

**From a Fragmented Present...**

-   Separate trust models for each L2.

-   Complex and insecure bridging.

-   Governance fragmentation.

**...To a Unified Future**

-   ✅ **Unified Security:** All rollups share Ethereum's guarantees.

-   ✅ **Seamless Interoperability:** Cross-rollup transactions without added trust assumptions.

-   ✅ **Capital Efficiency:** Fast ZK settlement becomes a public good provided by the L1.

-   ✅ **Simplified Development:** Teams can launch secure, EVM-equivalent rollups without needing to be cryptography experts.

This vision transforms Ethereum into a true settlement layer that provides **security and execution verification as a service** to a vibrant ecosystem of L2s.