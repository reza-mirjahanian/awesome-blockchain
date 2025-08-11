### Decentralized Sequencer Consensus Algorithms

## Decentralized Sequencer Consensus Algorithms



  
- **Focus Areas:**
  1. **Sequencers, Proofs, and Rollups:**
     - General overview and high-level concepts.
  2. **Decentralized Sequencers:**
     - Design and functionality.
  3. **Proof Design:**
     - Key takeaways from different companies' approaches.
  4. **Prov-friendly Sequencer Algorithm:**
     - Detailed examination of the algorithm and compromises.
  5. **Standardization Attempts:**
     - Efforts and insights from attempts at standardizing the process.

#### **Rollup Structure**

**Components:**
- **Sequencer:** Takes transactions.
- **Prover:** Creates a zero-knowledge proof.
- **Verifier Contract:** Verifies the proof.

**Process:**
- Transactions are signed and submitted by externally owned accounts.
- The prover generates proofs based on transaction data and state changes.
- Currently, sequencers are centralized, leading to implicit trust in their order.

#### **Decentralized Sequencers**

**Challenges:**
- **Order of Transactions:** 
  - Defined by the sequencer.
  - Can vary based on time of arrival or transaction fees.
- **Byzantine Fault Tolerance (BFT):**
  - Need for consensus even with potential node failures.
  - Causes can be non-malicious (e.g., software issues) or malicious (e.g., operator misconduct).

**Requirements:**
- **Deterministic Finality:**
  - Ensuring transactions are final and cannot be reversed.
- **Consensus Protocol:**
  - Must allow for agreement on the order of transactions and blocks.

#### **Consensus Protocols and Decentralization**

**Decentralization Levels:**
- Varying node numbers (e.g., 20-30 independent nodes).
- Initial steps towards full permissionless decentralization.

**Preferred Consensus Protocols:**
- **Single Block Finality:**
  - Essential for provers to work efficiently.
- **Proof of Stake (PoS):**
  - Enhances utility and ownership control.
  - Validators participate in the consensus.

#### **Proof Aggregation and Stateless Design**

**Proof Process:**
- **Leader Node:** 
  - Coordinates activities and communicates with RPC nodes.
- **Workers:** 
  - Execute transactions and create proofs.
- **Proof Aggregator:** 
  - Combines proofs into a smaller final proof.
- **Verifier Contract:** 
  - Verifies the final aggregated proof.

**Key Points:**
- **Stateless Nature:**
  - Leader, workers, proof aggregator, and consensus prover are stateless.
  - The verifier contract checks the correctness of proofs.

#### **Decentralized Sequence Consensus Algorithms**

**Information Flow:**
- **Finality:** 
  - Critical for deterministic finality in transactions.
- **Multiple EVM Implementations:**
  - Ensures agreement on state and block validity across different implementations.


### Execution and State Changes

- **EVMS Sync Requirement**:
  - Sequencer and Prover must work identically.
  - Same op codes, pre-compile, storage systems.

- **Potential Issues**:
  - Custom precompiles in decentralized sequencer may cause discrepancies.
  - If Prover doesn’t recognize a precompile, it reverts the action, causing inconsistencies.

- **Hard Fork Synchronization**:
  - Both Sequencer and Prover need simultaneous hard forks for updates.
  - Ensures both systems remain in sync.

### EVM Compatibility and State Changes

- **On-chain State Changes**:
  - ZK Prover proves transactions without special logic if actions occur via transactions.
  - Transactions must happen on-chain for proof.

- **Validator Set Changes**:
  - Must occur through transactions to ensure consistency.
  - Validator set details need to be stored in a deterministic storage slot for proof.

### Validator Set Contract

- **Storage Slot Calculation**:
  - Node address maps to a Boolean indicating if a node is active.
  - Prover calculates the storage slot using `keccak(0, node_address)` to prove a validator's status.

- **Balance Changes and Block Rewards**:
  - Balance changes due to block rewards must occur via transactions.
  - Use `coinbase` opcode to pay the coinbase account in a lightweight transaction.

### Paying Non-block Proposers

- **Challenge**:
  - Proving participation in the consensus protocol to pay non-block proposers.
  - Avoiding the lazy validator problem.

- **Slashing**:
  - Needs to be cheap and based on provable events, not popularity contests.
  - Immutable's approach: slash for non-participation in consensus protocol.

### Consensus and Validator Set Changes

- **Consensus Protocol**:
  - Must access validator information through a contract.
  - Validator set changes should be contract-based for transaction consistency.

- **Signature Algorithms**:
  - EC Recover is more cost-effective than BLS aggregation for a small number of validators.
  - BLS aggregation increases proving costs significantly.

- **Batch Proofs**:
  - Proving batches of blocks rather than individual blocks saves costs.
  - Validator changes at the start of a batch reduce the need for frequent consensus checks.

### Impact on Latency and Practical Implementation

- **Latency Considerations**:
  - Centralized sequencer may provide a quick soft commit.
  - Decentralized sequencer offers better data availability and quicker finality.

- **Decentralized Sequencer**:
  - Inclusion of transactions in a decentralized blockchain ensures data availability.
  - Reduced time to finality compared to centralized sequencers with soft commits.

- **Test and Implementation**:
  - Practical implementation and testing needed to validate theoretical advantages.
  - The approach suggests better instant finality with decentralized sequencers.