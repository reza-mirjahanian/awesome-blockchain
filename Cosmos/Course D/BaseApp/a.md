ABCI itself includes methods such as `DeliverTx`, which delivers a transaction. The interpretation of the transaction is an application-level responsibility. Since a typical application supports more than one type of transaction, interpretation implies the need for a service router that will send the transaction to different interpreters based on the transaction type. `BaseApp` includes a service router implementation.

`BaseApp` also provides a state machine implementation. The implementation of a state machine is an application-level concern because the CometBFT consensus is application-agnostic. The Cosmos SDK state machine implementation contains an overall state that is subdivided into various substates. Subdivisions include module states, persistent states, and transient states. These are all implemented in `BaseApp`.


#### Bootstrapping

Important parameters that are initialized during the bootstrapping of the application are:

-   **`CommitMultiStore`:** this is the main store of the application, which holds the canonical state that is committed at the end of each block. This store is not cached, meaning it is not used to update the application's volatile (un-committed) states.

    The `CommitMultiStore` is a store of stores. Each module of the application uses one or multiple `KVStores` in the multistore to persist their subset of the state.

-   **Database:** the database is used by the `CommitMultiStore` to handle data persistence.

-   **`Msg` service router:** the `msgServiceRouter` facilitates the routing of `sdk.Msg` requests to the appropriate module `Msg` service for processing.

    An `sdk.Msg` here refers to the transaction component that needs to be processed by a service to update the application state, and not to the ABCI message, which implements the interface between the application and the underlying consensus engine.

-   **gRPC Query Router:** the `grpcQueryRouter` facilitates the routing of gRPC queries to the appropriate module that will process them. These queries are not ABCI messages themselves. They are relayed to the relevant module's gRPC query service.

-   **`TxDecoder`:** this is used to decode raw transaction bytes relayed by the CometBFT.

-   **`ParamStore`:** this is the parameter store used to get and set application consensus parameters.

-   **`AnteHandler`:** this is used to handle signature verification, fee payment, and other pre-message execution checks when a transaction is received. It is executed during `CheckTx/RecheckTx` and `DeliverTx`.

-   **`InitChainer`, `BeginBlocker`, and `EndBlocker`:** these are the functions executed when the application receives the `InitChain`, `BeginBlock`, and `EndBlock` ABCI messages from CometBFT.