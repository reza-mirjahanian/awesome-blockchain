### PIP-43: Replace Tendermint with CometBFT in Heimdall

**Abstract**  
Heimdall, a crucial component of Polygon’s Proof-of-Stake (PoS) architecture, currently relies on Peppermint, a custom fork of Tendermint v0.32.7, to handle consensus. This implementation is outdated and creates technical debt. This proposal recommends upgrading Heimdall’s consensus layer by replacing Tendermint with CometBFT v0.38.x, which brings modernized consensus features, including ABCI 2.0 (Application Blockchain Interface v2.0, codenamed ABCI++). This upgrade will streamline transaction processing, improve state synchronization, and allow more efficient handling of external data – such as side transactions – using new mechanisms like vote extensions.

**Motivation**  
Heimdall’s reliance on an outdated version of Tendermint limits scalability and requires ongoing maintenance of the Peppermint fork. CometBFT offers better modularity, scalability, and flexibility, reducing tech debt. It introduces new consensus features such as two-phase block processing, application-driven proposal handling and validation, and advanced state sync capabilities. These will allow higher throughput, improved node synchronization times, and better data consistency. Peppermint’s side transaction logic can be mapped onto vote extensions in ABCI 2.0, enabling a thinner fork of CometBFT, with a plan to eliminate the fork in the next version.

**Rationale**  

#### Architectural Overview:

**Current Architecture (Tendermint-based Peppermint):**  
1. **Consensus Layer:** Uses standard Tendermint design with a linear process: blocks are proposed, validated, and finalized in a single step. Blocks contain transactions processed by the application layer after commitment. External transactions (e.g., state sync, checkpointing) are handled as side transactions with special mechanisms.  
2. **Application Layer:** Handles Polygon-specific needs like checkpointing to Ethereum and validator management, relying on side transactions to embed external chain data.

**Upgraded Architecture (CometBFT with ABCI 2.0):**  
CometBFT redefines consensus-application interaction via ABCI 2.0, enabling dynamic control over block creation, validation, and transaction processing.

1. **ABCI 2.0 (ABCI++):**  
   - **PrepareProposal:** Application can optimize blocks before proposal (e.g., reorder transactions, remove garbage), reducing block processing time.  
   - **ProcessProposal:** Validators pre-validate proposed blocks, rejecting invalid ones early for added security and efficiency.  
   - **ExtendVote / VerifyVoteExtension:** Vote extensions allow validators to append application-specific data (e.g., checkpoint hashes) in the precommit phase, replacing Heimdall’s side transaction system. This enables direct handling of external data through consensus without bespoke consensus-level code.

2. **State Synchronization:** Enhanced consensus-level state sync enables faster catch-up for new nodes/validators, improving resilience and onboarding time.

3. **Two-Phase Block Commit:** Block creation and validation occur in two steps: proposal and partial validation (via PrepareProposal/ProcessProposal), followed by final commit. This increases resilience to network latency and enables optimizations like batch processing or delayed execution.

**Specification**  

1. **ABCI 2.0 Overview:**  
   - **PrepareProposal:** Allows block proposers to optimize blocks at the application level (e.g., reorder transactions, remove unnecessary ones) for performance.  
   - **ProcessProposal:** Enables validators to pre-validate blocks before commitment, adding security and efficiency.  

2. **Vote Extensions:** Replaces Heimdall’s side transaction system. Validators append arbitrary data (e.g., cross-chain checkpoints, state sync updates) to precommit votes, enabling flexible external data integration without custom consensus logic.

3. **State Sync Enhancements:** Improved protocol uses snapshots for faster state catch-up, reducing resource needs for new nodes/validators.

4. **Backward Compatibility Considerations:** Requires migration to a new chain due to dependencies on a newer Cosmos SDK version and changes in CometBFT v0.38.x block format.

**Backwards Compatibility**  
The upgrade depends on a newer cosmos-sdk version. CometBFT v0.38.x has block format changes compared to Peppermint:  
- Some fields added/removed  
- Hash calculation methods changed  
These affect light-client logic and hash/signature verification. Code is not backward compatible, requiring a new chain launch (hard fork). Legacy data (e.g., TTBOOK) remains accessible via archive nodes on the old chain (heimdall-137).

**Security Considerations**  
The upgrade alters Heimdall’s consensus interaction, requiring focus on cryptographic key handling, vote extension implementation, and two-phase commit. Thorough testing, audits, and benchmarking are needed. The upgrade follows a 1:1 migration path, maintaining functionality while rolling out new features.

**Alignment and Clarification on Tendermint vs. CometBFT:**  
CometBFT forked from Tendermint Core in early 2023 and is now the canonical Tendermint implementation for the Interchain ecosystem, backed by the Interchain Foundation (ICF).  
The Tendermint Core repository is no longer actively maintained; its engineering team fully transitioned to CometBFT.

**Polygon’s Fork of CometBFT for Heimdall:**  
~70% of the current peppermint fork will be eliminated, as side transactions can be handled via vote extensions in CometBFT v0.38.x.  
The remaining ~30% (Ethereum-compatible crypto using uncompressed secp256k1) must stay in the fork due to a name collision preventing upstreaming.  
A strategy exists to upstream this crypto to CometBFT, enabling Polygon to use vanilla CometBFT in a future Heimdall version.

**Need for a New Chain (Hard Fork):**  
Block format changes in CometBFT since Tendermint Core v0.32.7 render legacy databases unusable:  
- Fields added/removed  
- Hash calculation methods changed  
- Impacts light-client logic and signature verification  
TTBOOK data in /var/lib/heimdall/data is rarely used but can be preserved via archive nodes on the old chain (heimdall-137) post-migration.

**Benefits of CometBFT v0.38.x:**  
- **ABCI++:** Enables application-driven block optimization and validation.  
- **State Sync:** New nodes with empty /var/lib/heimdall/data can fetch app snapshots from peers, install them, and only blocksync recent blocks, reducing sync time and eliminating need to copy ~370 GiB of data.  
- **Advanced Pruning:** New configuration options control growth of /var/lib/heimdall/data.  
- **Active Maintenance:** CometBFT is maintained and modernized (e.g., releasing v1.0.0-rc1).  
- **Additional Features:** Proposer-based timestamps (PBTS), data companion, etc.