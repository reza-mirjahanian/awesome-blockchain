Tendermint modules **attend to consensus and networking**

This frees developers to focus on the application level without descending into lower-level blockchain concerns such as peer discovery, block propagation, consensus, and transaction finalization.


![alt text](image.png)

A blockchain node for an application-focused Cosmos blockchain consists of a state-machine, built with the Cosmos SDK, and the consensus and networking layer, which are handled by [CometBFT](https://docs.cometbft.com/v0.37/)


### CometBFT
CometBFT is a blockchain application platform which supports state machines in any **language**. The language-agnostic CometBFT helps developers securely and consistently replicate deterministic, finite state machines.

CometBFT is maintained even when 1/3rd of all machines fail, by providing two components:

- A blockchain consensus engine.
- A generic application interface.

**CometBFT and the Interchain: A High-Performance Consensus Module**

**Overview**

CometBFT is a consensus module that provides a high-performance, consistent, flexible, and secure way to validate transactions on a blockchain network. It relies on Proof-of-Stake (PoS) with delegation and Practical Byzantine Fault Tolerance (PBFT) to ensure strict fork accountability.

**Key Components**

-   **Validators**: A subset of network nodes that participate in the transaction finalization process. Only the top 175 nodes, ranked by staked ATOM, are selected as validators.
-   **Voting Power**: Calculated as the total ATOM tokens staked by a validator and its delegates. Determines the frequency of block creation privileges.
-   **Block Creation**: Validators create blocks and broadcast them to other validators for confirmation.
-   **Block Confirmation**: Validators confirm candidate blocks, absorb penalties for failing to do so, reject invalid blocks, and accept valid blocks by returning their signature.

**Process**

1.  **Block Creation**: A validator creates a block and broadcasts it to other validators.
2.  **Block Confirmation**: Validators confirm the block and return their signature.
3.  **Block Finalization**: When sufficient signatures are collected, the block is finalized and broadcast to the wider network.

**Benefits**

-   **High Performance**: Aggressive block times are feasible due to dedicated validators with good network connectivity.
-   **Transaction Finality**: Provides a level of certainty when it comes to transaction finality, unmatched by probabilistic systems like Proof-of-Work (PoW).
-   **Scalability**: Can handle thousands of transactions per second, with confirmations taking seven seconds.
-   **Decentralization**: Avoids concentration of power by allowing the community of users to elect validators by staking ATOM and participating in rewards and risks.

**Incentives**

-   **Staking**: Users stake ATOM to support reliable nodes and participate in rewards and risks.
-   **Validator Selection**: Validators are incentivized to provide dependable service to maintain their ranking and voting power.

**Security**

-   **Strict Fork Accountability**: CometBFT ensures that validators are held accountable for their actions, preventing forks and maintaining network integrity.
-   **Penalties**: Validators absorb penalties for failing to confirm blocks or creating invalid blocks.
