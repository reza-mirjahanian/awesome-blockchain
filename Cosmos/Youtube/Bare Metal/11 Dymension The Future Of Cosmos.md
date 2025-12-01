## Dymension: A Modular Settlement Layer for Application-Specific RollApps

Dymension is a network of chains built to provide a **modular architecture** for the Cosmos ecosystem. It can be viewed as a modularized version of Ethereum, featuring execution shards, data shards, and a settlement layer connecting them.

### Core Architecture and Components

**1. The Dymension Hub (Settlement Layer)**

* The Dymension Hub is the central **settlement layer** for the ecosystem.
* It is an App Chain, a Cosmos blockchain.
* Its core mission is to service the other blockchains in the ecosystem, known as RollApps.
* **Bridging Hub:** All state information from connected chains is routed through the Dymension Hub. It acts as the "federal Supreme Court" of the ecosystem, formalizing the states of all participating blockchains.
* **Inherited Security:** RollApps inherit their decentralization and security from the Dymension Hub.

**2. RollApps (Execution Layer)**

* RollApps are **Application Specific Roll-Ups**. They are Cosmos blockchains **without** their own BFT consensus validator set (like Tendermint).
* **Execution Client (Diamond):** RollApps use an execution client called **Diamond** (the replacement for Tendermint). The Diamond client:
    * Executes application logic.
    * Produces blocks and batches of transactions.
    * Publishes data to a Data Availability (DA) layer.
    * Publishes state roots and a data reference to the Dymension Hub (the settlement layer).
* **VM Agnostic:** RollApps can use any Virtual Machine (VM) or application logic that is ABCI-compatible, including Wasm (CosmWasm), Ethermint (EVM), or native Go modules.
* **Low Latency:** By removing the consensus overhead of a distributed validator set, RollApps can achieve very low block times (e.g., 0.2 seconds on testnet).
* **Security Enforcement:** If a RollApp's sequencer performs an invalid state transition, the Dymension Hub will:
    * Slashes the sequencer's stake.
    * Prevents that sequencer from posting further state updates.
    * Automatically switches to another address in a permissionless setting.

***

### The RollApp Development Kit (RDK) and Deployment

Dymension provides the **RollApp Development Kit (RDK)**, which is forked from the Cosmos SDK, streamlining the deployment of application-specific blockchains.

**RollApp Deployment Process:**

1.  **Develop Logic with RDK:** Use the RDK to write the application logic. The RDK removes the standard Cosmos staking module, as RollApps do not manage a validator set.
2.  **VM Selection:** Use a simple make flag to enable **EVM** or **Wasm (CosmWasm)** support out of the box, offering flexibility over the Go route.
3.  **Launch Transaction:** Submit a **`create-rollapp`** transaction to the Dymension Hub, giving the RollApp a unique namespace and granting its Sequencer's public key the right to write state updates.
4.  **Run Sequencer:** Deploy a server (sequencer) running the Diamond execution client. This server starts posting state updates and data to the chosen layers.
5.  **Enable IBC:** Set up an **IBC connection** between the RollApp and the Dymension Hub. This requires running an IBC relayer (or outsourcing it as a service) to ensure trust-minimized bridging and immediate access to the entire IBC ecosystem's liquidity.

**Developer Flexibility and Economics:**

* **Gas Fees:** Developers can choose to accept a foreign denomination, such as **USDC**, as the gas fee, removing the necessity of bootstrapping a native token solely for securing the chain.
* **Token Model:** The RDK retains the minting and banking modules, allowing a RollApp to still deploy its own native token, customized minting methodology, and inflation schedule if desired.
* **Operational Overhead:** RollApps significantly reduce the operational and economic overhead compared to a Sovereign Cosmos chain, which must dedicate substantial resources to bootstrapping a validator set and designing security economics.

***

### Modular Architecture: Settlement and Data Availability (DA)

The Dymension protocol separates the concerns of execution, settlement, and data availability.

**Data Availability (DA) Agnosticism:**

* Dymension Hub as a protocol decides which L1s are acceptable as **DA layers** based on safety guarantees.
* **Governance Vetting:** The Dymension Hub governance must approve L1s (like Celestia or other Cosmos chains like Osmosis) to be accepted as a valid reference source.
* **RollApp Choice:** The RollApp deployer is free to choose any of the governance-vetted DA layers. RollApps are incentivized to choose the most cost-effective solution (e.g., Celestia) because they pay for all the bytes they put on chain.
* **Scaling Vision:** This modularity reflects a Web2 architecture:
    * **Data Availability Layer (Database):** L1 blockchains that host the data.
    * **Settlement Layer (Back End):** Dymension Hub.
    * **Execution Layer (Front End):** RollApps, where user activity occurs.

***

### Transaction Finality and Bridging

Dymension Hub functions as a settlement layer for both optimistic and validity (ZK) Rollups, ensuring flexibility for future tech advancements.

**1. Latency and Confirmation**

* **Low Latency (Soft Confirmation):** The Diamond execution client rapidly batches blocks (e.g., 0.2 seconds). Users experience this low latency, receiving a soft guarantee that the sequencer accepted the valid transaction, even though it has not yet been posted to a BFT chain.
* **Settlement (Hard Confirmation):** Full settlement occurs after the batch data is accepted by the DA layer and the new state root is verified and updated on the Dymension Hub.

**2. Inter-Blockchain Communication (IBC) and Withdrawal**

* **Hub to RollApp:** Transactions (e.g., Interchain Accounts) are immediate, relying on the validity of the Hub's validator signatures.
* **RollApp to Hub (Withdrawal):** Because Dymension is primarily an optimistic system, a **dispute period** is required before a transaction (like a token withdrawal) is fully accepted by the Dymension Hub state machine.

**3. Incentivizing Fraud Proofs**

The system incentivizes market participants to monitor for fraudulent state transitions:

* **Sequencer Slashing:** The RollApp's sequencer must stake DYM tokens. If a valid fraud proof is submitted during the dispute period, the sequencer's stake is **slashed** and a portion is given to the fraud prover.
* **IBC Fast Withdrawal:** Market makers can offer a service for **fast withdrawals** by relaying the user’s funds immediately, thus accepting the risk of a potential rollback. This creates a service-based incentive to watch the chain for invalid state transitions, ensuring security without relying on the assumption of a viable business of pure "fraud-hunting."

***

### Ecosystem Services and Vision

**Shared Liquidity / DEX**

* Dymension aims to provide a **seamless developer experience** by treating liquidity as core infrastructure.
* The Hub will provide a built-in decentralized exchange (DEX) service with **concentrated liquidity**.
* This approach is designed to concentrate all trading activity and pricing into a few core liquidity pools (e.g., DYM/RollApp Token) to provide **stable pricing** for long-tail RollApp assets, avoiding fragmented liquidity across multiple AMMs.
* The creation of a liquidity pool is integrated into the RollApp deployment process.
* The Hub can broadcast on-chain Oracle prices from these concentrated pools to individual RollApps on every block.

**NFTs and Governance**

* NFTs can be used as a mechanism for **governance** on RollApps, replacing the traditional validator/stake-based control model.
* An NFT collection or community can become its own RollApp, with the NFTs granting governance control and receiving automatic **value accrual** based on the RollApp's usage and cash flow.
* The NFT community can use CosmWasm to enforce **permissioned smart contract deployment**, deciding which applications (e.g., only DeFi, no games) fit the RollApp's identity and ecosystem.

**Vision for the Cosmos Ecosystem**

* Dymension views the future of Cosmos as a **"web of shared security"** (Mesh Security) rather than a single product winning or being secured by a monolithic validator set (Interchain Security V1).
* It anticipates that Sovereign L1s will evolve into specialized **DA chains**, competing to have RollApps post data on their block space.
* Dymension occupies the **vertical layer** of settlement, sitting between the DA L1s and the execution L2s (RollApps). This architecture is positioned for greater scalability and ease of deployment than a purely horizontal network of sovereign blockchains.



