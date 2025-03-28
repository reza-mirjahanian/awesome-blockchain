When messages and queries are received by the application, they must be routed to the appropriate module in order to be processed. Routing is done via `BaseApp`, which holds a `msgServiceRouter` for messages, and a `grpcQueryRouter` for queries.


### `Msg` Service Router[​](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#msg-service-router "Direct link to msg-service-router")

[`sdk.Msg`s](https://docs.cosmos.network/v0.52/build/building-modules/messages-and-queries#messages) need to be routed after they are extracted from transactions, which are sent from the underlying CometBFT engine via the [`CheckTx`](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#checktx) and [`FinalizeBlock`](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#finalizeblock) ABCI messages. To do so, `BaseApp` holds a `msgServiceRouter` which maps fully-qualified service methods (`string`, defined in each module's Protobuf `Msg` service) to the appropriate module's `MsgServer` implementation.

The [default `msgServiceRouter` included in `BaseApp`](https://github.com/cosmos/cosmos-sdk/blob/v0.52.0-beta.2/baseapp/msg_service_router.go) is stateless. However, some applications may want to make use of more stateful routing mechanisms such as allowing governance to disable certain routes or point them to new modules for upgrade purposes. For this reason, the `sdk.Context` is also passed into each [route handler inside `msgServiceRouter`](https://github.com/cosmos/cosmos-sdk/blob/v0.52.0-beta.2/baseapp/msg_service_router.go#L42). For a stateless router that doesn't want to make use of this, you can just ignore the `ctx`.

The application's `msgServiceRouter` is initialized with all the routes using the application's [module manager](https://docs.cosmos.network/v0.52/build/building-modules/module-manager#manager) (via the `RegisterServices` method), which itself is initialized with all the application's modules in the application's [constructor](https://docs.cosmos.network/v0.52/learn/beginner/app-anatomy#constructor-function).



### gRPC Query Router[​](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#grpc-query-router "Direct link to gRPC Query Router")

Similar to `sdk.Msg`s, [`queries`](https://docs.cosmos.network/v0.52/build/building-modules/messages-and-queries#queries) need to be routed to the appropriate module's [`Query` service](https://docs.cosmos.network/v0.52/build/building-modules/query-services). To do so, `BaseApp` holds a `grpcQueryRouter`, which maps modules' fully-qualified service methods (`string`, defined in their Protobuf `Query` gRPC) to their `QueryServer` implementation. The `grpcQueryRouter` is called during the initial stages of query processing, which can be either by directly sending a gRPC query to the gRPC endpoint, or via the [`Query` ABCI message](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#query) on the CometBFT RPC endpoint.

Just like the `msgServiceRouter`, the `grpcQueryRouter` is initialized with all the query routes using the application's [module manager](https://docs.cosmos.network/v0.52/build/building-modules/module-manager) (via the `RegisterServices` method), which itself is initialized with all the application's modules in the application's [constructor](https://docs.cosmos.network/v0.52/learn/beginner/app-anatomy#constructor-function).