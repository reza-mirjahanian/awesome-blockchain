
```
type App struct {
  // reference to a BaseApp
  *baseapp.BaseApp

  // list of application store keys

  // list of application keepers

  // module manager
}

```
First, the important parameters that are initialized during the bootstrapping of the application:

-   [`CommitMultiStore`](https://docs.cosmos.network/v0.52/learn/advanced/store#commitmultistore): This is the **main store** of the application, which holds the canonical state that is committed at the [end of each block](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#commit). This store is **not** cached, meaning it is not used to update the application's volatile (un-committed) states. The `CommitMultiStore` is a multi-store, meaning a store of stores. Each module of the application uses one or multiple `KVStores` in the multi-store to persist their subset of the state.
-   Database: The `db` is used by the `CommitMultiStore` to handle data persistence.
-   [`Msg` Service Router](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#msg-service-router): The `msgServiceRouter` facilitates the routing of `sdk.Msg` requests to the appropriate module `Msg` service for processing. Here a `sdk.Msg` refers to the transaction component that needs to be processed by a service in order to update the application state, and not to ABCI message which implements the interface between the application and the underlying consensus engine.
-   [gRPC Query Router](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#grpc-query-router): The `grpcQueryRouter` facilitates the routing of gRPC queries to the appropriate module for it to be processed. These queries are not ABCI messages themselves, but they are relayed to the relevant module's gRPC `Query` service.
-   [`TxDecoder`](https://pkg.go.dev/github.com/cosmos/cosmos-sdk@v0.52.0-beta.2/types#TxDecoder): It is used to decode raw transaction bytes relayed by the underlying CometBFT engine.
-   [`AnteHandler`](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#antehandler): This handler is used to handle signature verification, fee payment, and other pre-message execution checks when a transaction is received. It's executed during [`CheckTx/RecheckTx`](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#checktx) and [`FinalizeBlock`](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#finalizeblock).
-   [`InitChainer`](https://docs.cosmos.network/v0.52/learn/beginner/app-anatomy#initchainer), [`PreBlocker`](https://docs.cosmos.network/v0.52/learn/beginner/app-anatomy#preblocker), [`BeginBlocker` and `EndBlocker`](https://docs.cosmos.network/v0.52/learn/beginner/app-anatomy#beginblocker-and-endblocker): These are the functions executed when the application receives the `InitChain` and `FinalizeBlock` ABCI messages from the underlying CometBFT engine.


Then, parameters used to define [volatile states](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#state-updates) (i.e. cached states):

-   `checkState`: This state is updated during [`CheckTx`](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#checktx), and reset on [`Commit`](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#commit).
-   `finalizeBlockState`: This state is updated during [`FinalizeBlock`](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#finalizeblock), and set to `nil` on [`Commit`](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#commit) and gets re-initialized on `FinalizeBlock`.
-   `processProposalState`: This state is updated during [`ProcessProposal`](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#process-proposal).
-   `prepareProposalState`: This state is updated during [`PrepareProposal`](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#prepare-proposal).

Finally, a few more important parameters:

-   `voteInfos`: This parameter carries the list of validators whose precommit is missing, either because they did not vote or because the proposer did not include their vote. This information is carried by the [Context](https://docs.cosmos.network/v0.52/learn/advanced/context) and can be used by the application for various things like punishing absent validators.
-   `minGasPrices`: This parameter defines the minimum gas prices accepted by the node. This is a **local** parameter, meaning each full-node can set a different `minGasPrices`. It is used in the `AnteHandler` during [`CheckTx`](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#checktx), mainly as a spam protection mechanism. The transaction enters the [mempool](https://docs.cometbft.com/v1.0/explanation/core/mempool) only if the gas prices of the transaction are greater than one of the minimum gas price in `minGasPrices` (e.g. if `minGasPrices == 1uatom,1photon`, the `gas-price` of the transaction must be greater than `1uatom` OR `1photon`).
-   `appVersion`: Version of the application. It is set in the [application's constructor function](https://docs.cosmos.network/v0.52/learn/beginner/app-anatomy#constructor-function).


ParamStore[​](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#paramstore "Direct link to ParamStore")
--------------------------------------------------------------------------------------------------------------

During `InitChain`, the `RequestInitChain` provides `ConsensusParams` which contains parameters related to block execution such as maximum gas and size in addition to evidence parameters. If these parameters are non-nil, they are set in the BaseApp's `ParamStore`. Behind the scenes, the `ParamStore` is managed by an `x/consensus` module. This allows the parameters to be tweaked via on-chain governance.