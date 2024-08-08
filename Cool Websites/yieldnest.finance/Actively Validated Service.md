An EigenLayer AVS is any blockchain-based system that uses EigenLayer's restaking mechanism to support unique validation methods across the Ethereum network. ([Click here for examples.](https://holesky.eigenlayer.xyz/avs)) These services can rapidly scale by taking advantage of the security of Ethereum validators.

AVSs aren't limited by the Ethereum Virtual Machine's design. They provide developers with a broader set of tools, enabling new capabilities beyond what's possible on Ethereum. AVSs can include:

-   Sidechains
-   New virtual machines
-   Oracle networks
-   Keeper networks
-   Bridges
-   Data availability layers
-   Privacy-preserving protocols
-   Cross-chain interoperability solutions
-   And more...

**Why is the AVS Model Important?**
-----------------------------------

EigenLayer allows Ethereum stakers to restake ETH or liquid staking tokens (LSTs) to secure additional services on the network. This shared security layer eliminates the need for new services to establish their own security from scratch, addressing a critical issue with web3 development: **fragmented security.**

-   Dapps often rely on multiple services (see the list above).
-   Each service typically has its own security system.
-   Vulnerabilities are compounded --- the entire dapp is at risk if any service is exploited.


EigenLayer achieves **pooled security** through restaking, combining Ethereum's security across all Actively Validated Services.

-   **Bootstrapped Security:** New AVSs can leverage the existing validator set and staked ETH of Ethereum, making it easier to secure their networks.
-   **Increased Cost of Attacks:** Corrupting the system becomes substantially more expensive because attackers must break the combined security of all AVSs, not just one.
-   **Free-Market Governance:** EigenLayer introduces a marketplace where validators are incentivized to support services, allowing for flexible resource allocation.
-   **Improved Efficiency:** Validators can restake their ETH for multiple AVSs, reducing the overall financial burden on the system and making it more economically sustainable.
-   **Enhanced Trust:** The total amount of staked capital in EigenLayer strengthens the overall trust and security of the ecosystem.

**How Do AVSs Operate?**
------------------------

EigenLayer's architecture has four key players:

-   **Restakers:** Individuals or entities that stake ETH or LSTs through EigenLayer.
-   **Operators:** Operators run specialized node software and perform validation tasks for AVSs. They register with EigenLayer, receive delegated stakes from restakers, and earn rewards for their services.
-   **Actively Validated Services:** Decentralized services and infrastructure modules that benefit from EigenLayer's pooled security.
-   **AVS Users:** End-users or applications that interact with EigenLayer AVSs.




**Examples of AVS**:

-   **Oracle Services**: Oracles are services that provide external data to smart contracts on the blockchain. Validators may be required to validate the data provided by oracles to ensure its accuracy before it is used by smart contracts.
-   **Keeper Networks**: Keepers are automated agents that perform various tasks on behalf of smart contracts, such as liquidating undercollateralized positions or rebalancing portfolios. Validators might be responsible for triggering and verifying these actions.
-   **Cross-Chain Bridges**: Validators may be involved in the operation of bridges between different blockchain networks, ensuring that assets transferred between chains are securely and accurately recorded.