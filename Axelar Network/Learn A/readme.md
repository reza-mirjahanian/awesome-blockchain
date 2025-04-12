Understanding Axelar
====================


-   Axelar is more than a bridge: it offers the ability to program cross-chain logic and pass arbitrary data.
-   The upcoming Axelar VM will allow the permissionless connection of new chains.
-   Axelar's Interchain Token Service is in testnet, which supports cross-chain transfers of native tokens rather than wrapped ones.
-   Updated tokenomics have been proposed for AXL. They would allow the network to sustainably support more chains by decreasing the rate of token inflation per added chain.
-   As of this writing, 10 of the top 15 Axelar chains by activity are EVM-based. Axelar is focused on onboarding even more EVM and Ethereum-based chains.

---

The fragmentation of liquidity, services, and users creates [friction](https://axelar.network/blog/axelar-virtual-machine-future-of-interoperability) across ecosystems and sovereign networks. Axelar's technology aims to allow cross-chain functions that are more complex than simply transferring wrapped assets to different blockchains. Axelar addresses this by focusing on full-stack interoperability --- with full-stack meaning that Axelar not only supports bridging any information/asset but also permissionless [overlay](https://axelar.network/blog/how-overlay-networks-could-change-web3) programmability, executing smart contracts, and dApps across networks.

The Axelar community is working to scale the number of connected networks (55 at the time of publication) through a three-pronged approach: adjusting the network's economic structure, releasing the Axelar Virtual Machine to support permissionless connections, and exploring more efficient solutions like light clients. These initiatives aim to connect Axelar to hundreds of chains.

--------


Technology
----------

Axelar has three components:

-   A decentralized network. This is built primarily on open-source Cosmos technologies.
-   A set of gateway smart contracts that provide the connectivity between the Axelar network and its interconnected external chains.
-   A software development kit (SDK) of developer tools and APIs, including [Axelarscan](https://axelarscan.io/), a block explorer used to track cross-chain transactions.


### Network Architecture

The [Axelar network](https://axelar.network/blog/an-introduction-to-the-axelar-network) is built using the [Cosmos SDK](https://v1.cosmos.network/sdk), CometBFT, and CosmWasm VM. The [Cosmos SDK](https://docs.cosmos.network/main) is an open-source software development kit (SDK) for building sovereign, multi-asset, public, PoS blockchains. It's used to build a custom application layer, or state machine, while CometBFT is used to securely replicate that state machine on all nodes in the network. CometBFT, an application-agnostic engine, handles the networking and consensus layers through two main components:

-   A consensus algorithm, i.e., Tendermint.
-   A socket protocol, i.e., the [Application Blockchain Interface](https://cosmos-network.gitbooks.io/cosmos-academy/content/cosmos-for-developers/tendermint/abci-protocol.html) (ABCI).

Tendermint is used to validate requests on the source chain and confirm changes on the destination chain. Tendermint consensus provides instant finality and Byzantine fault tolerance. While this specific consensus approach only verifies cross-chain communication, Axelar can connect diverse forms of consensus. For example, Axelar is one of a few cross-chain protocols able to connect EVM and Cosmos chains.



#### Consensus & Security Approaches

Axelar's network uses a **Delegated Proof-of-Stake (DPoS)** consensus mechanism. Validators produce new blocks, participate in multiparty signing, and vote on external chain states. Tokenholders stake AXL by delegating tokens to a validator's staking pool. Only the top 75 validators are in the active set, a parameter that can be adjusted through onchain governance. Both delegating to validators and running a validator are permissionless.

Every PoS consensus mechanism runs the risk of concentrating voting power among a few dominant stakers. Axelar mitigates this risk with [quadratic voting](https://www.mdpi.com/2078-2489/13/6/305/htm) for its consensus mechanism. With quadratic voting, voting power does not increase linearly with stake. For Axelar validators to increase their voting power, they must increase their delegated stake exponentially.

In addition, Axelar applies network functions that enable the suspension of traffic from malicious interconnected chains and contract limits that cap how much can be transferred over a time period. The efficacy of these functions is improved by Axelar's hub-and-spoke network topology. During the [Multichain collapse](https://axelar.network/blog/hub-and-spoke-architecture), cross-chain swap services built using Axelar were able to stay safe and liquid by isolating compromised connections.