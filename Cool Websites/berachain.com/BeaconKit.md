BeaconKit is a modular and customizable consensus layer for Ethereum based blockchains.

BeaconKit is an innovative framework that makes the [CometBFT](https://docs.cometbft.com/v0.38/) consensus algorithm available to any EVM execution environment. In other words, BeaconKit is a modular [consensus layer](https://docs.berachain.com/learn/help/glossary#consensus-client) adaptable for Ethereum-based blockchains.

By leveraging the [Engine API](https://docs.berachain.com/learn/help/glossary#engine-api), BeaconKit can be paired with any EVM [execution client](https://docs.berachain.com/learn/help/glossary#execution-client), allowing it to be [EVM identical](https://docs.berachain.com/learn/#berachain-evm-identical-%E2%9F%A0), fully supporting any EVM execution client without modifications.

The framework is built with modularity in mind to easily integrate different layers that may include a custom block builder, a rollup layer, a data availability layer, among others. This modularity enables the building of not only Layer 1 blockchains but also serves as a framework for Layer 2 solutions.


BeaconKit Advantages[​](https://docs.berachain.com/learn/what-is-beaconkit#beaconkit-advantages)
------------------------------------------------------------------------------------------------

-   Single slot finality (compared to Ethereum's ~13 minutes)
-   Optimistic payload building (executing block proposal in parallel with voting) reduces block times by up to 40%
-   Conformity to Eth2 modularity
-   Full EIP compatibility