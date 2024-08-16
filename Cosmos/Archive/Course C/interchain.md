The interchain wants to address the main issues of previous blockchain projects and provide interoperability between chains to foster an **internet of blockchains**

*How is the interchain an internet of blockchains?* The interchain is a **network of interoperable blockchains**, each implemented with different properties suitable for their individual use cases. The interchain lets developers create blockchains that maintain sovereignty free from any "main chain" governance, have fast transaction processing, and are interoperable. With the interchain, a multitude of use cases becomes feasible.

To achieve this vision and type of network, the ecosystem relies on an **open-source toolkit**, including the [Inter-Blockchain Communication Protocol (IBC) (opens new window)](https://ibcprotocol.dev/), its implementation in the [Cosmos SDK (opens new window)](https://v1.cosmos.network/sdk), and [Tendermint (opens new window)](https://tendermint.com/)as the base layer providing distributed state finality. A set of modular, adaptable, and interchangeable tools helps not only to quickly create a blockchain but also facilitates the customization of secure and scalable chains.

## vertical scalability

In a blockchain context, vertical scalability is typically achieved through the optimization of the consensus mechanism and applications running on the chain. On the consensus side, the interchain achieves vertical scalability with the help of the Tendermint BFT. The Cosmos Hub currently conducts transactions in seven seconds. The only remaining bottleneck is then the application.

The consensus mechanism and application optimization of your blockchain can only take you so far. To overcome the limits of vertical scalability, the multi-chain architecture of the interchain allows for **one application to run in parallel** on different but IBC-coordinated chains, whether operated by the same validator set or not. This inter-chain, horizontal scalability theoretically allows for infinite vertical-like scalability, minus the coordination overhead.



In blockchain, a **validator** is one or more cooperating computers that participate in the consensus by, among other things, creating blocks.


### two-layer governance


Applications deployed on general-purpose blockchains all share the same underlying environment. When a change in an application needs to be made, it not only depends on the governance structures of the application but also on that of the environment. The feasibility of implementing changes depends on the governance mechanisms set by the protocol on which the application builds. The chain's governance limits the application's sovereignty. For this reason it is often called **two-layer governance**.


structure, but it exists atop the blockchain governance, and chain upgrades can potentially break applications. Application sovereignty is therefore diminished in two-layer governance environments.

The interchain resolves this issue, as developers can build a blockchain tailored to the application. There are no limits to the application's governance when every chain is maintained by its own set of validators. The interchain follows a **one-layer governance design**.


To summarize, this section has explored:

- A brief history of blockchain technology, leading to the appearance of purpose-built or application-specific blockchains limited by legacy platform characteristics which the interchain is designed to overcome.
- How the interchain provides blockchain interoperability for a better decentralized application (dApp) user experience, and an open-source toolkit of modular resources for a simplified, specialized dApp development experience.
- How the interchain responds to issues of scalability, using the horizontal scalability of its multi-chain architecture to deliver theoretically infinite capacity for vertical scalability.
- How the interchain increases sovereignty by providing a one-layer governance design in which each chain is maintained by its own set of validators, and improves both developer and user experience through the use of modular, interoperable systems.