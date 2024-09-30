**Limitations of PoS**:

*   Staked tokens reduce liquidity as they are locked up.
*   Applications built on PoS chains have limited influence on chain security.
*   Validators receive minimal rewards from the applications they support, leading to a **misalignment of incentives**.

**It leads to**: In PoS, validators have little incentive to engage with protocols or users, leading to a fragmented ecosystem.

In PoL, validators, protocols, and users must work together to maximize liquidity and benefits.

**Polaris EVM**: An EVM-compatible L1 built on the Cosmos SDK
Added a new runtime layer to separate the EVM from the Cosmos runtime, allowing for a fully native EVM environment.


**Bar Chain's Solution**

-   **Dual Token System:** Bar Chain introduces a dual token system consisting of:
    -   **BAR:** Used for gas fees and network security.
    -   **BGT:** Used for governance and on-chain incentivization.

**How the System Works**

1.  **Staking Process:** Users stake BAR with validators. Validators also stake BAR, activating their stake.
2.  **BGT Emission:** The staking of BAR leads to the emission of BGT, which is utilized to provide liquidity across various protocols within the Bar Chain ecosystem, such as AMM and lending pools.
3.  **Liquidity Providers:** Users providing liquidity earn BGT, which they can delegate to activated validators.
4.  **Increased Emissions:** The more BGT a validator receives, the higher the BGT emissions, creating a cycle of benefits.

**Benefits of the Mechanism**

-   **Enhanced Liquidity:** This system ensures liquidity within the ecosystem.
-   **Mutually Beneficial Relationships:** Validators benefit from increased BGT emissions tied to their support from liquidity providers, while protocols enjoy improved liquidity and security.
-   **Overall Ecosystem Health:** This symbiotic relationship contributes to the strength and efficiency of the Bar Chain ecosystem.
-   



## Aligning Protocols and Validators 🤝[​](https://docs.berachain.com/learn/pol/#aligning-protocols-and-validators-%F0%9F%A4%9D)

Because validators are given the responsibility of distributing governance tokens to Reward Vaults, when chosen to propose a block, it introduces a new dynamic where rewards are essentially shared with the ecosystem protocols.

Validators will share a stronger relationship with protocols, as their reward weight is determined by the governance tokens delegated to them, creating a symbiotic relationship.

Protocols can also convince Validators to start directing rewards to them by offering _Incentives_ in exchange for the `$BGT` rewards directed to their specific _Reward Vaults_.

----------

![alt text](image-3.png)


**Key Issues with Current Proof of Stake Systems**

-   **Token Locking:** In traditional proof of stake systems, users stake tokens to secure the network, which results in these tokens being locked and unavailable for use, leading to liquidity constraints.
-   **Value Mismatch:** There is a disconnect between validators (who secure the network) and protocols (which drive ecosystem activity). Validators receive minimal benefits beyond transaction fees for the support they provide to protocols.


### Tokens:
*   **BGT**: Non-transferable governance token
*   **Beer**: Gas token
*   **Honey**: Stablecoin

**Three core primitives**:

1.  BeEx (decentralized exchange)
2.  Perps (perpetual swaps)
3.  Bend (lending market)
4.  
----------------------

#### **How Polaris Works**

-   **Block Construction**:

    -   Polaris builds **EVM blocks** from data provided by **Comet** (Cosmos SDK).
    -   EVM state transitions are applied, and the block is stored on the Cosmos chain using an **IVL tree**.
-   **Three Main Processes**:

    1.  **Block Building**: Assemble a list of transactions and order them.
    2.  **Block Processing**: Apply EVM state transitions and store data.
    3.  **Block Commitment**: Commit the block to the canonical chain.
-   **ABCI (Application Blockchain Interface)**:

    -   Uses **Prepare Proposal**, **Process Proposal**, and **Finalize Block** steps.

* * * *

#### **Generalized Block Structure in Polaris**

-   **Three Main Constructs**:

    1.  **Block Builder**.
    2.  **Block Processor**.
    3.  **Storage Provider**.
-   **Message Passing**:

    -   Polaris uses **message passing** between the Cosmos and EVM sides to maintain **JSON-RPC compatibility** and handle **block building** and **storage**.

* * * *

#### **Pluggable Components**

-   **Client Diversity**:
    -   Polaris allows for **pluggable components** like different transaction pools, mempools, and RPC clients.
    -   Developers can mix and match components from different clients, increasing **client diversity**.

* * * *

#### **Pre-compiled Development Kit**

-   **Developer-Friendly Pre-compiles**:

    -   Polaris provides a **pre-compiled development kit** to make writing EVM extensions easier.
    -   Developers can write pre-compiles in a way that is familiar to **Solidity** or **Vyper** developers.
-   **Message Dispatcher**:

    -   The **message dispatcher** passes calls to the pre-compile manager, which interacts with Cosmos modules.

* * * *

#### **Advantages of Polaris**

-   **Client Diversity**:

    -   Polaris brings **genuine client diversity** to Cosmos for the first time.
    -   Transactions on Bearchain can utilize different **EVM clients**, enhancing security and reducing the risk of exploits.
-   **Base Layer Innovation**:

    -   Polaris allows for **custom application logic** at the base layer, enabling the creation of unique products like:
        -   **Flash Bara** (a flash loan product).
        -   **Slinky** (an Oracle).
        -   **IBC** and **Rollkit** integrations.

* * * *

#### **Modularity in Action**

-   **Workshop Example**:
    -   Demonstrated how **Polaris** integrates with **Celestia** (for data availability and consensus) and **Rollkit**.
    -   Also deployed an **OP Stack chain** using Polaris for settlement, showing full **EVM equivalence**.

* * * *

#### **Q&A Highlights**

-   **Polaris and Client Diversity**:

    -   Polaris enables **Cosmos chains** to integrate EVM easily and supports **client diversity** within those chains.
-   **Flash Bara**:

    -   Built on **Skip's Proposer-Builder Separation (PBS)**, integrated with **Flashbots interfaces** to reduce friction for Ethereum developers transitioning to Cosmos.