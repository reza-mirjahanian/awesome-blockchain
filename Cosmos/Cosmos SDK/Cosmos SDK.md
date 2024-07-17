
The creation of an application-specific blockchain with the Cosmos SDK is largely a process of selecting, configuring, and integrating well-solved modules, also known as "composing modules". This greatly reduces the scope of original development required, as development becomes mostly focused on the truly novel aspects of the application.


The **Inter-Blockchain Communication Protocol**(IBC) is a common framework for exchanging information between blockchains. it enables seamless interaction between blockchains that want to **transfer value (token transfers)** and exchange information. It enables communication between applications that run on separate application-specific blockchains.



The application, consensus, and network layers are contained within the custom blockchain node that forms the foundation of the custom blockchain.

CometBFT passes confirmed transactions to the application layer through the Application Blockchain Interface (ABCI). The application layer must implement ABCI, which is a **socket protocol**. CometBFT is unconcerned with the interpretation of transactions, and the application layer can be unconcerned with propagation, broadcast, confirmation, network formation, and other lower-level concerns that CometBFT attends to (unless it wants to inspect such properties).

Developers are free to create blockchains in **any language that supports sockets since the ABCI** is a socket protocol, provided their application implements ABCI. ABCI defines the boundary between replication concerns and the application, which is a state machine.


-   [Auth](https://docs.cosmos.network/main/build/modules/auth) \- Authentication of accounts and transactions for Cosmos SDK applications.
-   [Authz](https://docs.cosmos.network/main/build/modules/authz) \- Authorization for accounts to perform actions on behalf of other accounts.
-   [Bank](https://docs.cosmos.network/main/build/modules/bank) \- Token transfer functionalities.
-   [Distribution](https://docs.cosmos.network/main/build/modules/distribution) \- Fee distribution, and staking token provision distribution.
-   [Epochs](https://docs.cosmos.network/main/build/modules/epochs) \- Allow other modules to set that they would like to be signaled once every period
-   [Evidence](https://docs.cosmos.network/main/build/modules/evidence) \- Evidence handling for double signing, misbehaviour, etc.
-   [Feegrant](https://docs.cosmos.network/main/build/modules/feegrant) \- Grant fee allowances for executing transactions.
-   [Governance](https://docs.cosmos.network/main/build/modules/gov) \- On-chain proposals and voting.
-   [Mint](https://docs.cosmos.network/main/build/modules/mint) \- Creation of new units of staking token.
-   [Params](https://docs.cosmos.network/main/build/modules/params) \- Globally available parameter store.
-   [Protocolpool](https://docs.cosmos.network/main/build/modules/protocolpool) \- Functionalities handling community pool funds.
-   [Slashing](https://docs.cosmos.network/main/build/modules/slashing) \- Validator punishment mechanisms.
-   [Staking](https://docs.cosmos.network/main/build/modules/staking) \- Proof-of-Stake layer for public blockchains.
-   [Upgrade](https://docs.cosmos.network/main/build/modules/upgrade) \- Software upgrades handling and coordination.
-   [NFT](https://docs.cosmos.network/main/build/modules/nft) \- NFT module implemented based on [ADR43](https://docs.cosmos.network/main/build/architecture/adr-043-nft-module).
-   [Consensus](https://docs.cosmos.network/main/build/modules/consensus) \- Consensus module for modifying CometBFT's ABCI consensus params.
-   [Circuit](https://docs.cosmos.network/main/build/modules/circuit) \- Circuit breaker module for pausing messages.
-   [Genutil](https://docs.cosmos.network/main/build/modules/genutil) \- Genesis utilities for the Cosmos SDK.