

### Workshop Introduction

*   **Topic**: Comet BFT

    *   **Definition**: Byzantine Fault Tolerant State Machine Replication engine (BFT SMR) or Blockchain engine.



### Agenda

*   **Part 1: Theoretical (Researcher Perspective)**
    *   Comet BFT as a full-fledged State Machine Replication (SMR) solution.
    *   State Machine Replication (SMR) explained.
    *   Transition from SMR to Blockchain.
    *   ABCI (Application Blockchain Interface): Connecting Comet with applications.

*   **Part 2: Engineering (Practical Application)**
    *   Introduction of a new feature in Comet BFT: Propose Based Time Stamp (PBTS).
    *   Comet's method for calculating Block times in practice.


### State Machines and Blockchain

*   **Argument**: Comet BFT is a fully fledged Byzantine Fault Tolerance State Machine Replication solution.
*   **State Machine Definition**:
    *   Automata: Model of computation, describing program operation/algorithm.
    *   Simple abstraction.
    *   Components:        *   Finite set of States (S).
        *   Initial State (S₀).
        *   Deterministic State Transition Function (F):
            *   Input: State (S) and Input.
            *   Output: New State (S') and optional Output.
        *   Focus on state transition (S to S').
        *   Input & Output names: *Input* and *Output*.

*   **Execution of State Machine**
    *   Sequence of Inputs (Transactions):
        *   Transactions (T₁, T₂, T₃, ..., T<0xE2><0x82><0x99>).
        *   Finite but potentially large sequence.
    *   Associated set of States:
        *   S₀, S₁, S₂, S₃, ..., S<0xE2><0x82><0x99> (Subset of all possible states).
    *   Outputs (Transaction Results):
        *   Transaction 1 Result 1, Transaction 2 Result 2, etc.
        *   Names: Action, Events, Transaction Results (Comet).
    *   State Transition Function (Simplified):
        *   Input: State (Sᵢ<0xE2><0x82><0x8B><0xE2><0x82><0x99>₁) and Transaction (Tᵢ).
        *   Output: New State (Sᵢ) and Result (Rᵢ).
        *   Specific execution with transactions.

*   **Determinism**
    *   Importance for State Machines.
    *   Deterministic Programs:
        *   Same input = same output.
        *   Simple programs are inherently deterministic.
    *   Sources of Non-Determinism in Complex Programs:
        *   Time.
        *   Random value generation.
        *   Parallelism/Multi-threading (Scheduling order).
        *   Programming language, architecture, OS specifics.
            *   Example: Go's map iteration.
        *   Process interaction: Network, Inter-process communication, Database, Files.
    *   Goal: Achieve determinism for State Machine Replication.
    *   Computers are fundamentally deterministic.

*   **State Machine Replication (SMR)**
    *   Motivation: Desire for SMR as a feature.
    *   Concept:
        *   Multiple copies (replicas/instances) of a state machine.
        *   Run in independent processes (same/different machines/continents).
        *   Start from same initial state (S₀).
        *   Crucial Requirement: Replicas receive the *exact same sequence* of transactions in the *same order*.
    *   Outcome of SMR:
        *   Identical execution across all replicas.
        *   Enables deployment of distributed applications.
    *   Key Properties:
        *   **High Availability**:
            *   Fault Tolerance: Continues operation despite crashes, malicious behavior, etc.
            *   Program continues running despite failures.
            *   Similar to DNS high availability but with stronger consistency.
        *   **Strong Consistency**:
            *   Querying any replica at any execution point (e.g., index 10) returns the same state.
            *   Utmost consistency level achievable in distributed systems (Internet).
    *   Client Perspective:
        *   Interacting with any replica feels like interacting with a centralized server.
        *   Consistent outputs from any replica.
        *   Client unaware of replication, perceives a single, gigantic server.

*   **Visual Representation of SMR**    *   State Machine with three states (S₀, S₁, S₂, S₃).
    *   Clients on the right submit unordered transactions.
    *   "Black Box" (Comet BFT) responsible for:
        *   Ordering transactions.
        *   Delivering transactions to all replicas in the same order.
    *   Replica Examples:
        *   Top Left: State 0 -> Tx 1 -> State 1 -> Tx 2 -> State 2 -> Tx 3 -> State 3; next Tx 4.
        *   Top Right: State 0 -> Tx 1 -> State 1 -> Tx 2 -> State 2; next Tx 3.
        *   Bottom Left: State 0 -> Tx 1 -> State 1; next Tx 2.
    *   State Consistency Illustration:
        *   Querying State 1 on all replicas returns the same State 1.
        *   Querying State 2 returns State 2 for top replicas, "don't know State 2 yet" for bottom left (until Tx 2 processed).
    *   Black Box Replacement: Comet BFT.
        *   Four nodes (Comet BFT).
        *   Four applications (one per node).
        *   Comet BFT orders transactions from external clients and delivers them to all application instances in the same order.

*   **Historical Context and References**
    *   **Informal Introduction**: Leslie Lamport (1978) - *Time, Clocks, and the Ordering of Events in a Distributed System*.
        *   6-page paper, foundational for distributed systems.
    *   **Main Reference**: Fred Schneider (1990) - *Implementing Fault-Tolerant Services Using the State Machine Replication Approach*.
        *   Tutorial, 40 pages, Engineer-focused, less theoretical.
    *   **Byzantine Failure Model**: Leslie Lamport, Shostak, and Pease (1982) - *The Byzantine Generals Problem*.
        *   Introduced "Byzantine" terminology.
        *   15-page paper, first half very insightful with examples.

*   **Alternative Names for SMR**
    *   **Active Replication**: Name used ~10-12 years ago.
        *   Opposite of *Passive Replication* (Primary-Backup).
        *   Passive Replication:            *   Transactions to Primary.
            *   Backups get state copies.
            *   Primary failure = backup election, state recovery nightmare.
        *   Active Replication (SMR):
            *   Everyone is primary.
            *   Everyone receives and executes all transactions.
            *   Everyone updates state.
    *   **Distributed Ledgers**: 1990s-2000s name.
        *   Ledger book analogy with ordered entries.
        *   Directly maps to SMR:
            *   State = Account balances.
            *   Transactions = Ledger entries.
    *   **Blockchains**: Modern term for SMR packaging.
        *   New name, new packaging for SMR.
    *   *Distributed Ledgers and Blockchains are effectively the same as SMR.*
        *   Different names, same concept.
        *   Distributed Ledger emphasizes accounting/finance.
        *   State Machine Replication - Research/technical perspective.

### From SMR to Blockchain

*   **Transition Steps & Differences**
    *   Starting point: State Machine.
        *   State -> Transaction -> Next State -> Result.
    *   Blockchain: Batching Transactions into Blocks.
        *   Block = Multiple Transactions (e.g., Block H with Tx 1, 2, 3).
        *   Still state transitions and results, but batched.
        *   Results also batched (Results H).

*   **Atomicity Change**
    *   **SMR Atomicity**: Transaction level.
        *   Function in state transition:
            *   State I -> State I+1.
            *   Crash during transaction processing = remain in State I.
            *   Move to State I+1 only after complete transition and output.
    *   **Blockchain Atomicity**: Block level.
        *   Block as atomic unit.
        *   Process *all* transactions in a block or *none*.
        *   Introduces Blockchain State (Big S) and Application State (Small s).
            *   State 0 (application) = small s, State 0 (blockchain) = Big S.
            *   State H-1 (application) = state before block H.
            *   State AH (application) = state after block H (after applying block transactions and results).

*   **Block Headers**
    *   Blocks not just transaction batches, have *headers*.
    *   Headers contain information:
        *   Signatures from previous block.
        *   Link to previous block.
        *   Other metadata.
    *   Header as Input:
        *   Header (H) is *also* an input to state transition.
        *   State transition function receives Header + Transactions.
        *   State can change *just* by delivering the Header.
    *   Process Flow (Comet BFT/Cosmos Ecosystem):
        *   Process Header.
        *   Process each Transaction in block (Tx 1, 2, 3...).
        *   Each transaction produces a result (Result 1, 2, 3...).
        *   Client provides Transactions and receives Results.
        *   **Communication between Blockchain and Application**:
            *   Comet BFT (blockchain) delivers Header.
            *   Application delivers *Updates* back to Comet BFT.
            *   Updates allow application to reconfigure Comet (parameters, etc.).
    *   State Sequence:
        *   State 0 -> State H-1 (via Header) -> State AH (via Transactions) -> State Next Block.
        *   5 Transitions (Header + 3 Tx + Finalization) instead of 3 for single transactions.

### ABCI: Application Blockchain Interface

*   **Purpose**: Interface between Comet BFT and the Application State Machine.
    *   Comet BFT: Consensus Engine (orders transactions).
    *   Application: State Machine being replicated.
    *   Communication Standard:
        *   Uses Protocol Buffers (protobufs).
        *   Language-agnostic: Comet BFT (Go), application can be in any language.
        *   Socket communication.
        *   RPC (Remote Procedure Call) Protocol.
    *   Roles:
        *   Comet BFT: RPC Client (calls methods).
        *   Application: RPC Server (executor).
    *   Communication Flow:
        *   Comet sends *Request* messages to application.
        *   Application replies with *Response* messages.
        *   Request = Input, Response = Output (e.g., Input in Request, Output in Response).

*   **Deployment Options**
    *   Same Process: Comet and application in same process (Go applications, SDK-based apps).
    *   Different Processes: Comet and application in separate processes.
        *   Socket communication allows for different machines (possible, but less common).

*   **ABCI Methods (Relevant to SMR/Blockchain)**
    *   **Block Execution in Comet BFT/ABCI (up to Comet 0.37)**:
        *   **BeginBlock(header)**:
            *   First method call for each block.
            *   Input: Block Header.
                *   `LastCommitInfo` (signatures from previous block).
                *   Misbehavior evidence.
                *   Other block metadata.
        *   **DeliverTx(transaction)**:
            *   Called for *each* transaction in the block.
            *   Input: Transaction.
            *   Output (Application returns): Transaction Result.
        *   **EndBlock(height)**:
            *   Called after all `DeliverTx` for the block.
            *   Comet BFT indicates no more transactions for this block.
            *   Output (Application returns): *Updates*.
                *   **State Updates**.
                *   **Validator Updates**: Dynamically change validator set.
                *   **Consensus Parameters**: Global Comet BFT parameters (block size, keys, etc.).
    *   **FinalizeBlock (Comet 0.38+)**:
        *   Merges `BeginBlock`, `DeliverTx` (repeatedly), and `EndBlock` into a single call.
        *   Inputs/Outputs are conceptually the same, just a single method call.
    *   **Commit()**:
        *   Crucial for atomicity.
        *   Called after `FinalizeBlock` (or `EndBlock` pre-0.38).
        *   Comet decides on a block, writes to blockchain, delivers to application.
        *   Application returns results, validator updates, etc.
        *   Comet *writes these results to disk* before `Commit()`.
        *   `Commit()` signal to application: state transition finalized.
        *   Crash and recovery: Comet checks if `Commit()` was called. If not, re-executes block delivery.
    *   **Visual Representation (ABCI Methods)**:
        *   BeginBlock: Header delivery to application.
        *   DeliverTx: Transaction delivery, application returns result.
        *   EndBlock: Application returns updates to Comet.
        *   Commit: Atomicity point, state persistence guarantee.

*   **Application Bootstrapping & State Initialization**
    *   **Genesis File**: First state (State 0) for every state machine/blockchain.
    *   Comet and chain provide Genesis file to applications.
    *   Applications start from the same Genesis state.
    *   **Info() Method**:
        *   Invoked when Comet starts or restarts.
        *   Comet asks application for its latest state (height).
        *   Example:
            *   Comet at block 15, application says "State 10": Comet delivers blocks 11, 12, 13, 14, 15.
            *   Comet at block 15, application says "State 15": Comet only needs to deliver block 16 onwards.

*   **Transaction Validation**
    *   **CheckTx(transaction)**:
        *   Crucial for transaction validation.
        *   **Transactions are transparent to Comet BFT**: Comet doesn't know transaction content.
        *   Different from Bitcoin, Ethereum etc., where ordering layers know transaction format.
        *   Comet sends *every* received transaction to application via `CheckTx`.
        *   Application validates transaction.
        *   Response Code:
            *   Code 0: Transaction is valid. Comet should order and deliver.
            *   Code != 0: Transaction invalid. Comet ignores and drops transaction.
        *   Problem: Byzantine Proposer can ignore `CheckTx` and include invalid transactions in blocks. (Until version 34 - corrected later)
    *   **PrepareProposal(block)** and **ProcessProposal(proposal)** (Introduced in Comet 0.37):
        *   **PrepareProposal**:
            *   When Comet prepares to order transactions for a block.
            *   Sends proposed transactions to the application.
            *   Application decides what to do:
                *   Remove invalid transactions.
                *   Reorder transactions (e.g., account creation before account operation).
            *   Byzantine Proposer problem still exists: can still manipulate the block.
        *   **ProcessProposal**:
            *   Comet receives a proposal (block).
            *   Sends proposal (header, transactions) to the application.
            *   Application response:
                *   Accept: Application accepts the block. Comet signs and accepts it.
                *   Reject: Application rejects the block. Comet sends a reject vote.
            *   Deterministic operation is critical: Correct applications must accept valid proposals.
            *   Purpose: Allows correct applications to reject proposals with Byzantine content (e.g., random garbage).

*   **Conclusion (Part 1)**
    *   Comet BFT is a fully fledged State Machine Replication solution.
    *   ABCI is the "magical glue" between application and Comet BFT.
    *   Reasons for ABCI's effectiveness:
        *   Transactions and Results are just bytes (transparent to Comet).
            *   Advantage: Broader range of applications possible. Comet doesn't care about transaction content.
            *   Disadvantage: Performance potentially lower than systems where ordering layer understands transactions (like Bitcoin, Ethereum).
        *   Application State is opaque to Comet.
            *   Application can store state in any way (files, database, IPFS).
            *   Comet only needs two things at the end of each block height:
                *   Last committed height.
                *   App Hash: Hash of application state.
                *   Purpose of App Hash: Detect non-determinism between replicas. Mismatched app hashes lead to block rejection and network errors.
        *   Application configures Comet BFT.
            *   Application defines validator set, voting powers, consensus parameters.
            *   Dynamic reconfiguration possible at every block end.
            *   Highly dynamic system, reconfigurable every block time interval.

### Block Time Computation - Proposal Based Time Stamp (PBTS)

*   **Transition to Part 2 of Workshop (Practical)**
    *   Hands-on section with Docker setup (instructions on GitHub repo).
    *   Focus: How Comet computes timestamps for blocks.
    *   Introduction of Proposal Based Time Stamp (PBTS) - a new method.

*   **Time as a Problem in Distributed Systems**
    *   Reference: Lamport's 1978 paper (*Time, Clocks, and Ordering of Events*).
    *   Ideal Scenario (If synchronized clocks existed): Simple workshop time announcement (2:30 for 90 mins).
    *   Reality: Clock synchronization is extremely difficult.
        *   Atomic clock synchronization requires physics (relativity).
        *   Human time (minutes, seconds) vs. computer timescales (nanoseconds).
        *   Nanosecond-level clock synchronization across distributed systems is impossible.
    *   Time as Source of Non-Determinism:
        *   Program outputs change based on time even with same input (e.g., program run 10 years ago).
        *   Slight clock differences lead to different outputs in distributed executions.
    *   Blockchain Need for Time:
        *   Distributed applications, blockchains *expect* block timestamps.
        *   Want to trust block times (even though deterministic time is desired).
    *   Comet BFT (and other blockchains) include a Timestamp Field in Blocks.
    *   Important Caveat: *Do not fully trust block times*. Clock time is just a *reference*, not perfectly reliable in distributed systems.

*   **BFT Time (Byzantine Fault Tolerant Time)**
    *   Introduced in Comet BFT (and likely in other blockchains) ~2015/2016.
    *   Goal: Produce monotonically increasing time (time always increases).
    *   Time defined by correct validators.
    *   Mechanism:
        *   Every validator proposes a time for the next block.
        *   During pre-commit step (last consensus step):
            *   Validator reads local clock time.
            *   Includes timestamp in pre-commit message.
        *   Upon receiving pre-commit messages from >2/3 of validators:
            *   Collect timestamps from messages.
            *   Calculate the *median* of received timestamps.
            *   Median becomes the block timestamp.
        *   Rationale: Optimistic assumption that validators run NTP clients for clock synchronization.
        *   Best effort time synchronization, no absolute real-time guarantees.

*   **Limitations of BFT Time**
    *   Pre-commit Message Timestamps:
        *   Timestamps in pre-commit messages are non-deterministic (vary per process).
        *   Prevents Signature Aggregation: Signature aggregation requires identical values being signed. Time variation breaks value sameness.
    *   No Real-Time Guarantee:
        *   Relies on validator local clocks, assumptions about synchronization.
    *   Inter-Blockchain Communication (IBC) Issues:
        *   Multiple blockchains = multiple validator sets = different time generation.
        *   IBC errors due to timestamp discrepancies (block timestamp from one chain appears in "future" of another).
    *   "If you have one clock, you have time. If you have two clocks, you have nothing." - Problem with multiple time sources in distributed systems.

*   **Proposal Based Time Stamp (PBTS)**
    *   New protocol in Comet BFT V1 release to address BFT Time limitations.
    *   Key Idea: Proposer includes its *local time* in the proposed block.
    *   Validation: Other validators *validate* the proposed block time against their *local clocks*.
    *   Synchronous Requirements: PBTS relies on assumptions about clock synchronization and message delays.
    *   Improved Fault Tolerance: PBTS tolerates Byzantine behavior from *less than two-thirds* of voting power (BFT Time tolerated less than one-third). Doubled fault tolerance degree.

*   **Synchronous Requirements (PBTS)**
    *   **Precision**: Maximum difference between clocks of two correct processes at the same time.
        *   Conservative default: 500 milliseconds (0.5 seconds).
        *   Can be adjusted.  Real networks often have better synchronization.
    *   **Message Delay**: Maximum time for a message to travel from proposer to destination.
        *   Needed to bound time differences between block time (proposer's time) and validator's receive time.
        *   Default: 15 seconds.

*   **PBTS Workflow (Simplified)**
    *   Proposer (e.g., Validator 2) takes its local time and puts it into the block as timestamp.
    *   Other validators receive the proposed block.
    *   Validators compare the block timestamp to their *local receive time*.
    *   Important Parameters:
        *   Lower Bound for Acceptable Time Difference: -Precision (-0.5 seconds).
        *   Upper Bound for Acceptable Time Difference: Message Delay + Precision (15.05 seconds).
    *   Validation Logic:
        *   Accept block time if:  (Block Timestamp - Receive Time) is within [ -Precision, Message Delay + Precision ].
        *   Reject block time if:
            *   Block timestamp is too far in the future (difference > Upper Bound). - Proposal is "not timely - Prevoting"
            *   Block timestamp is too far in the past (difference < Lower Bound). - Reject TimeStamp.

*   **PBTS Benefits**
    *   Improved security by tolerating more Byzantine validators in terms of time manipulation.
    *   More reliable block timestamps under Byzantine conditions, assuming synchronous requirements are met.

*   **PBTS Practical Considerations and Configuration**
    *   PBTS is *not enabled by default* in Comet BFT V1.
    *   Must be explicitly enabled in configuration.
    *   Parameters (Precision, Message Delay) are configurable.
    *   Runbook available for monitoring PBTS performance, identifying issues, and adjusting parameters.

*   **Formal Verification of PBTS**
    *   PBTS protocol has been formally verified using TLA+ and Quint.

### Recap & Conclusion

*   **Comet BFT: Full-fledged State Machine Replication solution.**
    *   Supports diverse use cases.
*   **Comet BFT V1.0 Release (upcoming months):**
    *   Refined, production-ready.
    *   Key Improvements/Features:
        *   **Propose Based Time Stamp (PBTS)**.
        *   ABCI improvements.
        *   Bandwidth (BWI) savings and reduced disk usage for validators.
        *   BLS key support.
        *   Data companion API.
        *   New mempool implementation.
        *   Performance improvements (measured recently).

*   **Next Steps (V1.1 and beyond - Next Year)**
    *   Mempool Lanes (Spam mitigation). Experimental in V1.
    *   "Dog" - new transaction propagation protocol (mempool replacement).
    *   P2P improvements for faster connections (QUIC transport).
    *   Modular Comet BFT:
        *   Plugabble crypto algorithms.
        *   Different database backends.
        *   Software Programming improvements for modularity.

