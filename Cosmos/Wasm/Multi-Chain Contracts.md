
# Multi-Chain Contracts

## Different Chain, Same Contract

Since we make a few requirements of the host application, it is easy for any Cosmos SDK app to embed the  `wasm`  module and customize the permissions or fees as they wish. All code is designed to be agnostic to the details of the underlying chain, so just by writing a CosmWasm contract, you will soon be able to run on different chains on the Cosmos ecosystem.

[Regen Network](https://regen.network/)  plans to include CosmWasm support at launch. Several other chains are adding this support.

## Inter Blockchain Contracts[​](https://docs.cosmwasm.com/docs/architecture/multichain#inter-blockchain-contracts "Direct link to heading")

If you have heard anything about Cosmos, it is most likely about  [Inter-Blockchain Communication](https://ibcprotocol.org/). The power of  [Tendermint BFT consensus](https://tendermint.com/)  and their  [novel bonded proof of stake algorithm](https://blog.cosmos.network/what-does-the-launch-of-cosmos-mean-for-the-blockchain-ecosystem-952e14f67d0d)  are the foundation for a revolutionary protocol to allow trustless message-passing semantics between blockchains. No middleman, no timing issue, full security.

The potential means code on one chain can execute a transaction on another chain. But the code must be designed around a message-passing idiom. CosmWasm fully embraces the  [actor model](https://docs.cosmwasm.com/docs/architecture/actor)  and lends itself to IBC use. Messages are fire-and-forget, rather than awaiting a promise and worrying about race conditions and reentrancy attacks. As IBC stabilizes, we will be adding first-class support for IBC primitives into the  [CosmWasm](https://github.com/CosmWasm/cosmwasm)  libraries, as well as the  [Cosmos SDK module](https://github.com/CosmWasm/wasmd/tree/master/x/wasm)  that hosts it.


## Easy to Integrate[​](https://docs.cosmwasm.com/docs/architecture/multichain#easy-to-integrate "Direct link to heading")

Another design goal of CosmWasm was to be more of a library than a framework. This means it has a small surface area of required APIs and you can opt-in to most of the code. It is there to make life easy for you, but you can easily build it your own way as well.

This has two big benefits:

-   It makes it easier to add support for multiple languages to write contracts in. So we can add support for say,  [AssemblyScript](https://www.assemblyscript.org/)  or  [Go](https://github.com/golang/go), for those who prefer not to write in Rust.
    
-   Since it makes limited demands on the host system, it can be embedded in other frameworks, not just the Cosmos SDK. The core runtime logic  [`cosmwasm-vm`](https://github.com/CosmWasm/cosmwasm/tree/main/packages/vm)  is in Rust, and  [`wasmvm`](https://github.com/CosmWasm/wasmvm)  provides a generic Go binding to it. As Go and Rust are two of the most popular languages to write blockchains, this opens the door for many integrations. Of course, unless your chain is running on top of  [Tendermint](https://tendermint.com/)  or potentially another BFT Instant Finality Consensus algorithm like  [Babble](https://github.com/mosaicnetworks/babble), the contracts will not be able to interact via IBC.


