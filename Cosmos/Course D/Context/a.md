`Context` is the setting in which transactions execute, and is the sum of all pertinent information at runtime. Here you will find out what transaction context means in detail and learn more about the important elements that together form the execution context.


Transactions execute in a context. The context includes information about the current state of the application, the block, and the transaction.

Context is represented as data structures that carry information about the current state of the application and are intended to be passed from function to function. Context provides access to branched storage, that is a safe branch of the entire state, as well as useful objects and information, like `gasMeter`, block height, and consensus parameters.


The Cosmos SDK context is a custom data structure that contains Go's stdlib context as its base. It has many additional types within its definition that are specific to the Cosmos SDK.


Context properties
------------------

The context has the following properties:

-   **Context:** the base type is a Go Context.
-   **Multistore:** every application's `BaseApp` contains a `CommitMultiStore`, which is provided when a context is created. Calling the `KVStore()` and `TransientStore()` methods allows modules to fetch their respective `KVStore`s using their unique `StoreKey`s.
-   **ABCI Header:** the header is an ABCI type. It carries important information about the state of the blockchain, such as block height and the proposer of the current block.
-   **Chain ID:** the unique identification number of the blockchain a block pertains to.
-   **Transaction bytes:** the \[\]byte representation of a transaction is processed using the context.


Every transaction is processed by various parts of the Cosmos SDK and consensus engine (for example CometBFT) throughout its lifecycle, some of which do not have any understanding of transaction types. Thus, transactions are marshaled into a generic `[]byte` type using some kind of encoding format such as Amino.

-   **Logger:** a logger from the Tendermint libraries. [Learn more about logs here ](https://github.com/tendermint/tendermint/blob/master/libs/log/logger.go). Modules call this method to create their unique module-specific logger.
-   **`VoteInfo`:** a list of the ABCI type `VoteInfo`, which includes the name of a validator and a boolean indicating whether they have signed the block.
-   **Gas meters:** specifically, a `gasMeter` for the transaction currently being processed, using the context and a `blockGasMeter` for the entire block it belongs to.