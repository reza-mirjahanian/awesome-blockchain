Berachain is a high-performance [EVM-Identical](https://docs.berachain.com/learn/#berachain-evm-identical-%E2%9F%A0) Layer 1 (L1) blockchain utilizing [Proof-of-Liquidity](https://docs.berachain.com/learn/#proof-of-liquidity-%F0%9F%A4%9D) (PoL) as a consensus mechanism, and built on top of a modular evm-focused consensus client framework named [BeaconKit](https://docs.berachain.com/learn/#beaconkit-%E2%9B%B5%E2%9C%A8).

EVM Identical ⟠[​](https://docs.berachain.com/learn/#evm-identical-%E2%9F%A0)
-----------------------------------------------------------------------------

Berachain's execution layer is identical to the Ethereum Virtual Machine (EVM) runtime environment seen on Ethereum Mainnet. This means that it uses existing unmodified [execution clients](https://docs.berachain.com/learn/help/glossary#execution-client) like Geth, Reth, Erigon, Nethermind, and more to handle executing smart contracts, and supports all the tooling that comes native with the EVM.

Identical means that whenever the EVM is upgraded, Berachain can adopt the latest version---for example, Dencun---straight out of the box. This includes compatibility with all RPC namespaces and endpoints and any improvements made to execution clients would give immediate improvements to Berachain.

Proof-of-Liquidity 🤝[​](https://docs.berachain.com/learn/#proof-of-liquidity-%F0%9F%A4%9D)
-------------------------------------------------------------------------------------------

Proof-of-Liquidity is a [consensus mechanism](https://docs.berachain.com/learn/help/glossary#consensus-mechanism) that establishes a framework to reward ecosystem liquidity that contributes to efficient trading, price stability, securing the chain, and increasing the network/user growth.

This framework makes it possible to strongly align the incentives of key stakeholders / [PoL participants](https://docs.berachain.com/learn/pol/participants) (validator, protocols, users) and contributes to the overall long-term health of the chain.

Beyond providing a great day-one dApp experience, the native dApps, such as [BEX](https://docs.berachain.com/learn/dapps/bex), [Bend](https://docs.berachain.com/learn/dapps/bend) and [Berps](https://docs.berachain.com/learn/dapps/berps), serve as reference implementations of how developers can build on-top of Proof-of-Liquidity.

Read more in [What Is Proof-of-Liquidity](https://docs.berachain.com/learn/what-is-proof-of-liquidity).

BeaconKit ⛵✨[​](https://docs.berachain.com/learn/#beaconkit-%E2%9B%B5%E2%9C%A8)
-------------------------------------------------------------------------------

BeaconKit is a modular framework developed by Berachain for building EVM [consensus clients](https://docs.berachain.com/learn/help/glossary#consensus-client). It integrates the benefits of CometBFT consensus, including increased composability, single slot finality (SSF), and more.

Read more in [What Is BeaconKit](https://docs.berachain.com/learn/what-is-beaconkit).