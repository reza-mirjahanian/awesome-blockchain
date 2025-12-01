https://chorus.one/

## Proof of Stake (PoS) Ecosystem: Evolution and Architecture

The foundation of Chorus One was built on the premise that Proof of Stake (PoS) networks would require infrastructure providers (validators) to secure them.

### Validator and Venture Capital Operations

The core business for Chorus One involves running validator infrastructure for various PoS networks. This naturally involves a due diligence process for onboarding new networks, which is similar to the due diligence required for venture capital (VC) investment.

**Chorus One Venture Focus:**
The dedicated VC fund was established after accumulating a treasury from staking revenues. The fund is dedicated to investing in opportunities that align with the core validation business, specifically:
* New networks featuring a staking function.
* Middleware that facilitates staking adoption.
* Tooling that advances the broader Proof of Stake ecosystem.

### Software Development and Technical Initiatives

Chorus One's software efforts are divided into two main areas:
* **DevOps and Infrastructure Software Engineering:** Focuses on maintaining and running the validation infrastructure.
* **Infrastructure/Protocol Contributions:** Past projects include building early Tendermint-to-Solidity bridging code for Ethereum and expanding the Lido protocol to Solana.

**Historical Challenge: Data Infrastructure in Cosmos**
Early on, projects like the dashboard **Anthem** struggled due to the difficulty in accessing reliable and especially historical data within the Cosmos ecosystem, a common issue known as the **"perennial RPC problem."**

**Current Software Focus:**
The primary development project is an institutional product: an **API** allowing delegators to directly stake with Chorus One and manage their rewards, moving toward a more secure and reliable interface for large clients.

***

## The State of Proof of Stake

Proof of Stake is now an established and proven consensus model. This is evidenced by:
* Multiple networks running for several years, securing billions of dollars in value.
* Ethereum's successful transition to PoS (The Merge).
* A growing percentage of the overall crypto market capitalization running on PoS networks.

### Liquid Staking Dynamics

Liquid staking protocols gained significant adoption due to the market's desire for **capital efficiency** and the ability to use staked tokens in DeFi.

| Feature | Delegation Model (Cosmos/Tezos) | Proof-of-Work (PoW) or Early PoS (Ethereum) | Liquid Staking Protocol |
| :--- | :--- | :--- | :--- |
| **Delegation** | **In-Protocol:** Relationship is natively encoded in the protocol. | **Out-of-Protocol:** Not natively supported, creating incentive to pool funds externally. | **Out-of-Protocol:** Serves as a pooled custody solution. |
| **Transparency** | High | Low, as pooling happens via centralized exchanges or custom contracts. | Varies, can reduce transparency if validators are not linkable in-protocol. |

**Impact on Validators and Decentralization:**
While liquid staking can increase leverage in DeFi, it can also benefit network decentralization by:
* Shifting stake control from individual investors to the protocol (often a DAO).
* Giving a market share to high-quality validators who lack the resources for self-promotion or reputation building, as the protocol's technical due diligence selects them.

**Effect of Ethereum Withdrawals (Shanghai Upgrade):**
The introduction of native unstaking on Ethereum is expected to **increase the overall percentage of staked ETH** (currently around 20%). The ability to withdraw reduces the necessity of liquid staking as an exit mechanism, thus leveling the playing field. This may entice more users toward direct staking with single providers or running a home validator, reducing the **"liquidity premium"** historically enjoyed by liquid staking tokens.

***

## New Staking Architectures

### Eigenlayer and Restaking on Ethereum

Eigenlayer introduces the concept of **restaking** to Ethereum, fundamentally changing how token security is leveraged.

* **Traditional Model:** Top-down security (e.g., sharding in Ethereum or Polkadot), where one single token secures the entire ecosystem. This model was often criticized in Cosmos because the native token (Atom) did not automatically capture the value of its ecosystem's security.
* **Eigenlayer's Innovation:** A **bottom-up** approach where participants can **opt-in** to running and securing additional services (Actively Validated Services or AVS) using their existing staked ETH.
* **Mechanism:** Restaking allows validators to get more yield by securing multiple pieces of infrastructure, accepting increased slashing risk in return.

***

## Cosmos Ecosystem and App Chain Strategy

The Cosmos ecosystem, with its **Inter-Blockchain Communication (IBC)** protocol and **Cosmos SDK**, is significantly more mature in its **App Chain** thesis compared to newer Ethereum Layer 2 stacks (e.g., Op Stack).

### The App Chain Movement

Major applications, such as dYdX, are migrating from shared-blockspace environments (like Ethereum Layer 2s) to their own sovereign chains (App Chains) built on Cosmos.

**Motivation for Migration:**
The primary drivers for building an App Chain are the need for:
* **Customizability:** Tailoring the chain parameters to the application's specific requirements.
* **Sovereignty:** Full control over the block space and governance of the chain.

**Optimal Path for New Projects:**
The most sensible approach for a new application is an evolutionary one:
1.  **Start on Shared Space (Ethereum/Layer 2):** Deploy on a shared-space environment to quickly achieve **product-market fit (PMF)** and gain traction without the burden of security.
2.  **Migrate to App Chain (Cosmos):** Only move to a sovereign chain once the application becomes too large or requires the customizability and control that a dedicated chain provides.

**The Bootstrapping Challenge:**
Starting directly with a new App Chain without established users or PMF presents a significant challenge, requiring the team to simultaneously bootstrap:
* The application and user base.
* The validator set and network security.

### Exciting Projects in Cosmos

| Project | Focus | Technical Value / Contribution |
| :--- | :--- | :--- |
| **Osmosis** | Decentralized Exchange (DEX) | Known for continuous innovation and leveraging the Cosmos stack for unique features like **in-protocol MEV capture**. |
| **dYdX** | Perpetual Trading Exchange | Represents a major influx of **liquidity** and a large user base migrating to Cosmos. |
| **Kyve (Kaif)** | Decentralized Data Storage | Solves the critical problem of **state data persistence** and accessibility for chains. Used to create a more decentralized and trustworthy process for **State Sync** (validator onboarding). |
| **Numia** | Data Middleware/Analytics | Working to provide essential **data tooling** (similar to Dune on Ethereum) to improve the visibility of on-chain activity, which historically aids application and ecosystem growth. |

### Future of App Chain Security Models

The Cosmos ecosystem is rapidly introducing various shared security and scaling solutions, including Interchain Security (ICS), Mesh Security, Alliance, and rollups (Rollaps).

**Drivers for App Chain Deployment Choice:**
While technical merits (e.g., scalability, security model) are important, the choice of where an App Chain is deployed is heavily influenced by non-technological factors:
* **Business Development (BD):** Ecosystems that provide dedicated, hands-on support, grants, and incentives to new projects are highly effective in attracting new applications.
* **Ecosystem Alignment:** Choosing a system based on existing community alliances, potential synergies, and token incentives.
* **Community Support:** The willingness of an ecosystem's community to support and utilize the new chain.

The decision is often a pragmatic business calculation involving **ecosystem support and community fit** as much as it is a pure technical decision.


