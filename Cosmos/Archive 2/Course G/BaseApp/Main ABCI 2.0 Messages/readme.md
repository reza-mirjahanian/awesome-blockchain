Main ABCI 2.0 Messages[​](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#main-abci-20-messages "Direct link to Main ABCI 2.0 Messages")
-------------------------------------------------------------------------------------------------------------------------------------------------

The [Application-Blockchain Interface](https://docs.cometbft.com/v1.0/spec/abci/abci++_basic_concepts#overview-and-basic-concepts) (ABCI) is a generic interface that connects a state-machine with a consensus engine to form a functional full-node. It can be wrapped in any language, and needs to be implemented by each application-specific blockchain built on top of an ABCI-compatible consensus engine like CometBFT.

The consensus engine handles two main tasks:

-   The networking logic, which mainly consists in gossiping block parts, transactions and consensus votes.
-   The consensus logic, which results in the deterministic ordering of transactions in the form of blocks.

It is **not** the role of the consensus engine to define the state or the validity of transactions. Generally, transactions are handled by the consensus engine in the form of `[]bytes`, and relayed to the application via the ABCI to be decoded and processed. At keys moments in the networking and consensus processes (e.g. beginning of a block, commit of a block, reception of an unconfirmed transaction, ...), the consensus engine emits ABCI messages for the state-machine to act on.

Developers building on top of the Cosmos SDK don't need to implement the ABCI themselves, as `BaseApp` comes with a built-in implementation of the interface. Let us go through the main ABCI messages that `BaseApp` implements:

-   [`Prepare Proposal`](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#prepare-proposal)
-   [`Process Proposal`](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#process-proposal)
-   [`CheckTx`](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#checktx)
-   [`FinalizeBlock`](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#finalizeblock)
-   [`ExtendVote`](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#extendvote)
-   [`VerifyVoteExtension`](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#verifyvoteextension)