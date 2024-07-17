**The interchain** is a network of independent blockchains, which are:

-   All powered by consensus algorithms with Byzantine Fault-Tolerance (BFT).
-   All connected through the Inter-Blockchain Communication Protocol (IBC), which enables value transfers, token transfers, and other communication between chains, all without the need to involve exchanges or make compromises regarding the sovereignty of each chain.

The interchain is also **a blockchain ecosystem** complete with protocols, SDK, tokens, wallets, applications, repositories, services, and tools.


----

Building on modular components, many of which you did not write yourself* \- does this increase the **potential for attacks**, and faulty or malicious nodes operating undetected? No need to worry.

The Cosmos SDK is built on the [object-capability model (opens new window)](https://docs.cosmos.network/v0.45/core/ocap.html). It not only favors modularity but also encapsulates code implementation. An object-capability model ensures that:

-   There is no way for objects in the memory to be discovered just by going through the composed objects of others.
-   The only way to have references to objects or to access services is to have passed the relevant object references.

-------
The **default consensus** mechanism available when developing with the SDK is CometBFT, which is based on [Tendermint Core (opens new window)](https://docs.tendermint.com/v0.34/tendermint-core/)

---------
The [Inter-Blockchain Communication Protocol (IBC) (opens new window)](https://ibcprotocol.dev/)is the basis for **interoperability** in the interchain. It leverages the instant finality of Tendermint to allow for the transfer of value (token transfers) and communication between heterogeneous chains. Blockchains with different applications and architecture specifications become interoperable whether or not they share a validator set.

The interchain implements a **modular architecture with two blockchain classes**: **hubs** and **zones**.

----

The Cosmos Hub
---------------

The Cosmos Hub was the first blockchain built with the Interchain Stack. It is a public Proof-of-Stake (PoS) blockchain with a native token, ATOM. ATOM is used to secure the network and pay transaction fees but is also the token that has funded the development of the Interchain Stack and other projects in the wider ecosystem.

The Cosmos Hub can be understood as a router facilitating transactions between the chains connected to it. For example, the Cosmos Hub allows for transaction **fees to be paid in different tokens**, as long as the zone trusts the Cosmos Hub and the other zones connected to it.