https://ccvalidators.com/

### CryptoCrew Infrastructure and Validation Architecture

**Operational Setup**
CryptoCrew operates as an infrastructure provider within the Interchain ecosystem, validating over 30 chains and placing a heavy focus on IBC (Inter-Blockchain Communication) relaying. The infrastructure utilizes bare metal nodes located in partnering data centers.
*   **Distribution:** The network spans six data center partners across 12 different regions within Europe.
*   **Hardware:** The fleet consists of a mix of rental servers and co-location deployments (owned hardware).

**Validator Architecture: Horcrux and MPC**
The core of the validator architecture relies on **Horcrux**, a remote signer software developed by Strangelove Ventures.
*   **Multi-Party Computation (MPC):** Horcrux acts as an MPC threshold signer. The Tendermint validator private key is split into shards.
*   **Cluster Configuration:** These key shards are distributed across decentralized nodes (a signer cluster). A threshold of these shards must actively participate to sign a block.
*   **Benefits:**
    *   **Resilience:** There is no single point of failure. If a region or data center goes offline, the remaining nodes in the cluster continue to provide signatures.
    *   **Rapid Recovery:** Nodes can be spun up in new data centers within minutes to replace offline instances without compromising the key.
*   **Latency Constraints:** To maintain consensus efficiency, latency between signer nodes generally needs to remain below approximately 60 milliseconds. Concentrating the cluster within the European continent satisfies this requirement while maintaining geographic diversity.

**Signature Performance vs. Liveliness**
While 100% uptime is ideal, it is not strictly necessary for network health.
*   **Tendermint Liveliness Threshold:** The network requires 66% of voting power to be online to continue producing blocks.
*   **Performance:** Due to the distributed nature of the signer cluster, signature quotes may hover around 96%. Anything above 95% is considered sufficient for validator performance without risking network liveliness. This is an acceptable trade-off for the increased security and redundancy provided by the MPC setup.

### The Risks of White Label Validators

**Definition and Mechanism**
"White Label" validation occurs when a service provider runs the physical node and manages the infrastructure, while a customer (often an influencer or DAO) markets the validator under their own brand.
*   **Key Custody:** To produce blocks, the infrastructure provider *must* possess the Tendermint consensus key.
*   **Security Implications:** If a single white-label provider manages a significant portion of the network's stake, a technical failure or malicious action by that provider correlates to a massive network outage.

**Case Study: Allnodes on Terra Classic**
A specific instance highlights the severity of these risks:
1.  **Centralization:** The provider "Allnodes" managed over 40% of the voting power on the Terra Classic network.
2.  **Key Management Security Failure:** It was discovered that the provider generated the account keys (seed phrases/mnemonics) for their customers during the registration process. This meant the provider potentially had access to customer funds and governance voting rights, posing a severe security threat (e.g., governance attacks or unauthorized fund transfers).

### Proposed Solution: Separating Block Production from Governance

To mitigate the centralization risks of white-labeling, the roles of "Block Producer" and "Governor" should be decoupled at the protocol level.

**The Concept**
*   **Block Producers (Infrastructure Providers):** These entities focus solely on the technical aspects: running secure, sturdy infrastructure, managing upgrades, and ensuring uptime. They are rewarded specifically for block production.
*   **Governors (Influencers/Community Leaders):** These entities focus on social engagement, community building, and governance decisions. They do not need to manage hardware or hold consensus keys.
*   **Delegation Split:** Delegators would split their delegation, assigning voting power to a Governor and infrastructure power to a Block Producer.

**Benefits**
*   **Eliminates Shared Risk:** Infrastructure providers no longer need access to governance keys or wallet seeds.
*   **Competence Alignment:** Tech experts focus on tech; community leaders focus on governance.
*   **Reduced Friction:** This removes the need for non-technical entities to use white-label services that compromise key security.

### IBC Relaying and Incentives

**IBC Mechanics**
The Inter-Blockchain Communication protocol functions similarly to TCP/IP for blockchains. It relies on asynchronous execution:
1.  **Initiation:** A transaction is initiated on Chain A.
2.  **Confirmation:** The transaction is confirmed on Chain A.
3.  **Relaying:** A relayer picks up the packet and submits an IBC update message to Chain B.
4.  **Finalization:** The action is finalized on Chain B.

**Relayer Software Implementations**
Competition in open-source relayer software drives improvement. Key implementations include:
*   **Go Relayer (rly):** Developed by Strangelove Ventures. Written in Go. Widely used standard.
*   **Hermes:** Developed by Informal Systems. Written in Rust. Generally considered easier to configure and robust.
*   **TS Relayer:** Developed by Confio. Written in TypeScript. Currently dormant, but possesses potential for browser-based relaying (integration into web front-ends).

**Incentivization Issues**
Currently, relayers pay transaction fees on both source and destination chains to deliver packets, often without direct compensation. This is not a sustainable business model.
*   **ICS-29 (Fee Middleware):** A protocol upgrade that allows users to pay fees for IBC transactions, which are then directed to the relayers. This is currently being rolled out on chains like Evmos and Juno.

### Public RPC Infrastructure and Load Balancing

**The Availability Problem**
Accessing reliable public RPC endpoints is a significant friction point for dApp developers.
*   **Pruning:** To save storage, many nodes prune historical state (e.g., only keeping the last 100 blocks).
*   **Inconsistency:** A developer querying an endpoint for older data (e.g., block 5) may hit a pruned node that cannot return the data, causing application errors.

**The Aggregation Solution**
Tools like **Cosmos Directory** (built by Eco Stake) address this by acting as a health-checking load balancer.
1.  **Scanning:** The system scans all RPC endpoints listed in the chain registry.
2.  **Health Checks:** It verifies if the nodes are syncing, if they are archival, and their latency.
3.  **Routing:** Requests are routed only to healthy, capable nodes. This creates a reliability layer similar to Infura but aggregated across decentralized providers.