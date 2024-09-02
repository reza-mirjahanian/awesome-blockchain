When messages and queries are received by the application, they must be routed as is appropriate to be processed. Routing is done via `BaseApp`, which holds a `msgServiceRouter` for messages and a `grpcQueryRouter` for queries.


### `Msg` service router

The main ABCI messages that `BaseApp` implements are [`CheckTx` (opens new window)](https://github.com/cosmos/cosmos-sdk-docs/blob/main/docs/learn/advanced/00-baseapp.md#checktx)and `DeliverTx`.

Other ABCI message handlers being implemented are:

-   `InitChain`
-   `BeginBlock`
-   `EndBlock`
-   `Commit`
-   `Info`
-   `Query`