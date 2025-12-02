## Validator Operations and Chain Strategy

The Frens Validator team consists of around 10-12 members, with four dedicated to technology and validator operations, ensuring updates and maintaining tools like the auto-compound bot.

### Chain Selection Criteria

The process for selecting chains to validate focuses on several key questions to ensure alignment and value:

* **Team and Product Belief:** Do we believe in the team and the product?
* **Ecosystem Value:** Will the product provide additional value to the broader Cosmos ecosystem? Is the product exciting?
* **Community Necessity:** Is our community present on this chain, and do they have a necessity to have us as a validator on that chain to represent their interests and maintain trust?

### Multi-Chain Operations and Strategic Focus

Frens Validator maintains a multi-chain mindset, currently validating on approximately 22 chains within and outside the Cosmos ecosystem. Recently onboarded chains include Solana and Near.

* **Near Protocol:** It is tightly connected to Ethereum and the Cosmos ecosystem, with plans to launch **Inter-Blockchain Communication (IBC)**, making its integration with the Interchain highly relevant.
* **Solana:** Interest remains in how the ecosystem will progress following the FTX event, viewing it as a space where opportunity may lie.

### Exciting Cosmos Projects

Several projects are exciting for their innovative approaches and forthcoming launches:

* **Sorc Chain:** A project focused on the automotive industry. It adopted a **product-first approach**—building a device that tracks underlying road data before launching a blockchain. This is seen as the correct approach, reversing the common trend of starting with a blockchain and then trying to figure out the off-chain data part. They have a partnership with Suzuki to implement the device in new cars, where users earn $SOR tokens for driving. This platform is open for building automotive-tailored applications on top.
* **Archway:** A forthcoming layer one smart contract chain.
* **dYdX:** The perpetuals decentralized exchange is migrating to become an independent Cosmos chain.

***

## Security Models and Interchain Security (ICS)

The Cosmos ecosystem is an experimental hotbed for various security models, including IBC and the Cosmos SDK paired with mechanisms like Interchain Security (ICS), Dymension, Saga, Mesh, and Alliance.

### Interchain Security V1 (ICS V1)

ICS V1, which recently launched, operates by requiring **Hub validators to run additional validators/code** for a consumer chain once a governance proposal passes.

**Challenges of ICS V1:**

* **Scaling Overhead for Small Validators:** While minimal for large validators (e.g., those running 20+ chains), running an additional 2-5 consumer chains adds significant overhead (monitoring, updates, governance participation) for smaller validators who may only run on two or three chains.
* **Consumer Chain Supremacy:** The core question is: What if a consumer chain (like Neutron), which features added complexity (e.g., liquid staking, lending protocols), becomes **larger and more valuable than the Cosmos Hub itself**? In this scenario, the rationale for that chain to *borrow* the Hub's security becomes unclear, especially when alternative security models like Mesh Security exist.

### ICS Adoption Status

* The initial chains confirmed to launch with ICS include **Stride** and **Neutron**, both focused on liquid staking.
* **Quicksilver** initially planned to launch with ICS but later stepped back from the commitment, though they did not rule it out for the future.

***

## Decentralized Finance: Liquid Staking Solutions

Liquid Staking (LS) is considered an elementary and inevitable part of the Cosmos ecosystem and the broader proof-of-stake industry. It is expected to grow into a multi-billion-dollar industry.

### Importance and Advantages

* **Addressing Unbonding Periods:** LS addresses the fundamental constraint of proof-of-stake networks where assets are locked for an unbonding period. It allows users to stake while maintaining immediate liquidity.
* **Capital Efficiency:** Liquid staked tokens can be used as collateral or yield-generating assets in DeFi protocols.

### Risks and Centralization

The primary risk associated with LS is **centralization**. If a single protocol becomes too large, it may acquire substantial voting power over the underlying blockchain (analogous to Lido's influence in Ethereum). Avoiding single points of failure is critical.

### Differentiation Between LS Solutions

A variety of LS solutions is necessary to provide redundancy and cater to different user needs, mitigating the risk of a single protocol dominating the market.

| Feature | Stride | Quicksilver | P-Stake |
| :--- | :--- | :--- | :--- |
| **Governance/Validator Selection** | The protocol community votes on a reputable validator set. | The user decides which validator they want to use, retaining full governance power. | Validators are pre-picked; the user cannot choose. |
| **Risk Profile** | Seen as potentially more secure due to a minimalistic blockchain design (no smart contracts deployed). | Shifts more decision-making power (and thus certain risks) to the user. | Lower user decision-making required. |
| **Architecture** | Own independent chain secured by the Hub (via ICS). | Independent chain. | Independent protocol. |

### The Case for Native Staking

Despite the advantages of liquid staking, allocating a portion of assets to **native staking** on the Hub is still recommended for risk hedging and profitability:

1.  **Risk Mitigation:** Native staking is the most secure way to stake assets on the chain.
2.  **Cost and Profitability:** It is the most profitable and cheapest method, as it avoids the service fees charged by liquid staking protocols.
3.  **Airdrop Eligibility:** Staking ATOM natively on the Hub provides the highest likelihood of exposure to potential airdrops, a guarantee that LS solutions have not definitively solved.

***

## Economic Optimization: Maximal Extractable Value (MEV)

MEV is defined as the value that can be extracted by validators/searchers through their ability to include, exclude, or re-order transactions within a block.

### Role in Validator Profitability

MEV is viewed as a crucial, necessary component for the future of validator operations.

* **Decreasing Inflation Challenge:** Validator profitability currently relies heavily on token inflation. As inflation decreases and transaction fees remain low (e.g., on chains like Osmosis), many validators would become heavily unprofitable.
* **New Income Stream:** Democratizing MEV access and sharing the revenue with stakers can create a vital new income stream to compensate for the reduction in inflationary rewards.

### Governance and Architectural Solutions

The key challenge is determining how to split the extracted MEV assets and manage the infrastructure without introducing centralization.

#### 1. Proposal Builder Separation (PBS)

PBS is an architectural solution that separates the process of **building** the block from the process of **proposing** the block.

* **Reward Clarity:** PBS can clarify who is doing most of the work in abstracting MEV revenue, which aids in a fairer determination of reward splits.
* **Operational Decoupling:** It allows chain upgrades (e.g., software or security patches) to happen independently of MEV infrastructure upgrades. Currently, validators must rely on MEV providers to offer a custom node build with their MEV Tenderment dependency on time for an upgrade, introducing operational risk and exposure to bugs. PBS would allow the proposer's node to connect to the builder via a consistent API.

#### 2. MEV Philosophy

MEV should be viewed as an **economic good**—the use of free block space. Like any innovation (e.g., the automobile), it is not inherently "good" or "bad." The goal is to figure out how to use block space to **maximize upside** while **minimizing harm** to the users (e.g., preventing sandwich attacks).

