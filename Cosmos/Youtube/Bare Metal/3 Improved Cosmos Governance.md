## Technical Content Refinement

https://en.wikipedia.org/wiki/Flash_Core_Module

### Validator Operational History

* **Early History:** The company began in 2013 with **Bitcoin Miners** (Proof-of-Work, POW), later transitioning to **Ethereum miners**.
* **Proof-of-Stake (PoS) Transition:** Moved into running **masternodes** for early PoS networks like **Dash** and **Pivx**.
* **Cosmos and DPoS:** Switched to **delegated Proof-of-Stake (DPoS)**. Started working with **Cosmos** testnets in **2018** and subsequently joined most **Tendermint** chains.
* **Multi-Ecosystem Validation:** Validation operations extend across ecosystems including **Polkadot/Substrate** chains and **Celo**. The **Cosmos Hub** represents the primary public participation in the PoS ecosystem.

***

### Polkadot vs. Cosmos Validation

Validation across Polkadot/Substrate and Tendermint/Cosmos chains presents key differences in both infrastructure and staking mechanics.

#### Infrastructure Differences
Running a validator on a **Polkadot Parachain** requires running **double nodes**—the Parachain node and the main **Polkadot Relay Chain node**—on the same virtual machine (VM), which necessitates a significant increase in resources.

#### Staking and Reward Differences
* **Cosmos:** Delegation is a direct process where delegators receive rewards based on their proportional stake.
* **Substrate/Polkadot:** Each elected validator receives a **set amount of rewards** regardless of delegations. Delegators split this fixed reward. This structure means that a validator who is **over-delegated** will distribute **less rewards** to each individual delegator.

***

### Validation Infrastructure and Private Cloud Setup

The validator maintains a robust, private cloud infrastructure across multiple geographic locations.

#### Data Center (DC) and Hardware
* Operates a **Tier 3 (Tr3) data center** in **Malta**.
* Infrastructure uses internal **Dell hosts** managed with **VMware vCenter** and a hypervisor to create VMS.
* The setup is a private cloud: the team **rents cabinets** and provisions its **own hardware**, ensuring full control and eliminating third-party access to the servers and VMs.

#### Diversification and Redundancy
* Backup/secondary cabinet locations are maintained in **North America** and **Northern Europe** for geographical diversification.
* This diversification is critical for **peer connectivity**, especially on nascent **testnets** where a low peer count can cause the Malta-based validators to miss blocks.
* The DC features multiple **ISPs**, a large **generator**, and a large **UPS** for power redundancy.

#### High Availability and Failover System
* **VM Failover (Hardware Level):**
    * VMS run with **vSphere**.
    * The system uses **IBM shared storage** instead of local SSDs.
    * If a host fails, VMs can easily move to another host and immediately restart, connecting to the shared storage.
* **Validator Failover (Signing Level):**
    * Multiple synced nodes are run for every network in all locations.
    * A **key failover process** is strictly followed to prevent the catastrophic failure of **double signing** if the main data center or key management system goes down. Synced nodes in backup locations can then be safely promoted to validators.

#### Storage Optimization for EVM Chains
Optimizing storage was necessary due to the intensive disk I/O demands of **EVM chains** such as **Binance Smart Chain (BSC)** and **Polygon**.

* **Performance Insight:** **Latency to disk** was identified as the critical factor, overriding raw Input/Output Operations Per Second (IOPS).
* **Protocol and Hardware Upgrades:**
    * Switched from **iSCSI** to **Fiber Channel** protocol for faster disk access.
    * Upgraded from SATA SSDs to **NVMe SSDs**.
* **Current Solution:** High-performance **Flash Core Modules** within the IBM shared storage are utilized to meet the demanding I/O requirements of the nodes.

***

### The Panic Monitoring Tool

**Panic** is an open-source, easy-to-deploy monitoring solution developed by the team and intended as a public good for node operators.

#### Functionality and Architecture
* **Ecosystem Support:** Monitors nodes across **Cosmos, Substrate, and Chainlink** ecosystems.
* **Data Sources:** Gathers metrics from:
    * **RPC Port** of blockchain nodes.
    * **Prometheus metrics** exported by nodes (e.g., Tendermint/Cosmos SDK).
    * **On-chain data** (e.g., governance proposals).
    * **GitHub repos** for new release notifications.
    * **Node Exporter** (on VMS) for system metrics.
* **Alerting Channels:** Integrates with professional services like **PagerDuty** and **OpsGenie**, which are necessary for **scalable team escalations**, in addition to initial support for Telegram and Twilio.
* **Architecture:** It is a **custom-developed stack** with its own front-end, designed from the ground up for crypto node management, and is not based on a modified Prometheus/Grafana setup.
* **Key Value:** The tool provides built-in, pre-tinkered **alert thresholds** to help eliminate frequent and disruptive **false positives**.

#### Sustainability and Development Status
* Panic V2 was built with the help of various grants, but current development is **paused** due to limited resources (crypto winter) and a lack of recurring funding.
* Monitoring solutions were not seen to receive the same level of appreciation or delegation-based public goods funding from chains as other contributions like wallet providers or explorers.
* The development team, possessing deep blockchain knowledge, was reallocated to focus on the company's own **Cosmos SDK-based chain**, which offers a profitable future. The existing tool remains functional and open-source for community contribution.

***

### Blockchain Governance and Public Goods Funding Reform

The team advocates for substantial reform in how decentralized governance and public goods funding are managed, especially within the Cosmos ecosystem.

#### Critique of Current System
* **Grant Inefficiency:** The current process relies heavily on applying for individual grants from private **foundations**, which is insufficient for long-term, ongoing support of public goods.
* **Community Pool Fatigue:** Spending from the **Cosmos Community Pool** via individual, full-community votes is inefficient and time-consuming. This unsustainable process leads to **apathy and fatigue** among voters.

#### Proposal for a Streamlined Governance Model
The proposed model integrates lessons from traditional governance and other ecosystems like Polkadot.

1.  **Dedicated Committees/Sub-DAOs:** Establish groups of respected **experts** focused on specific areas (e.g., funding, technical development).
2.  **Full Transparency:** All committee discussions and decision-making must be **transparent and public**.
3.  **Treasury/Budget System:** Move away from per-proposal voting by creating a **budget system** or treasury. This involves:
    * Allocating high-level funds via a high-level budget.
    * Designating dedicated, accountable people to manage the wise spending of those allocated funds.

#### Cosmos Hub Specifics (Prop 82)
* **Issue:** Proposal 82 was too **large and overwhelming**, combining controversial elements such as new technology proposals, new funding methods (minting), and multiple other items.
* **Recommendation:** Future proposals should be **split up** into smaller, singular, and more manageable items to facilitate informed voting and prevent overall rejection due to controversy in one area.
* **Recent Action:** The team successfully submitted a proposal to **increase the Community Tax** to ensure the Community Pool has the necessary funds; the next priority is establishing the mechanism for **how those funds will be spent**.


