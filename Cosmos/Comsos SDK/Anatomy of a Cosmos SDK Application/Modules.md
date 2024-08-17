[Modules](https://docs.cosmos.network/v0.50/build/building-modules/intro) are the heart and soul of Cosmos SDK applications. They can be considered as state-machines nested within the state-machine. When a transaction is relayed from the underlying CometBFT engine via the ABCI to the application, it is routed by [`baseapp`](https://docs.cosmos.network/v0.50/learn/advanced/baseapp) to the appropriate module in order to be processed. This paradigm enables developers to easily build complex state-machines, as most of the modules they need often already exist. **For developers, most of the work involved in building a Cosmos SDK application revolves around building custom modules required by their application that do not exist yet, and integrating them with modules that do already exist into one coherent application**. In the application directory, the standard practice is to store modules in the `x/` folder (not to be confused with the Cosmos SDK's `x/` folder, which contains already-built modules).

### Application Module Interface[​](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#application-module-interface "Direct link to Application Module Interface")

Modules must implement [interfaces](https://docs.cosmos.network/v0.50/build/building-modules/module-manager#application-module-interfaces) defined in the Cosmos SDK, [`AppModuleBasic`](https://docs.cosmos.network/v0.50/build/building-modules/module-manager#appmodulebasic) and [`AppModule`](https://docs.cosmos.network/v0.50/build/building-modules/module-manager#appmodule). The former implements basic non-dependent elements of the module, such as the `codec`, while the latter handles the bulk of the module methods (including methods that require references to other modules' `keeper`s). Both the `AppModule` and `AppModuleBasic` types are, by convention, defined in a file called `module.go`.

`AppModule` exposes a collection of useful methods on the module that facilitates the composition of modules into a coherent application. These methods are called from the [`module manager`](https://docs.cosmos.network/v0.50/build/building-modules/module-manager#manager), which manages the application's collection of modules.



### `Msg` Services

Each application module defines two [Protobuf services](https://developers.google.com/protocol-buffers/docs/proto#services): one `Msg` service to handle messages, and one gRPC `Query` service to handle queries. If we consider the module as a state-machine, then a `Msg` service is a set of state transition RPC methods. Each Protobuf `Msg` service method is 1:1 related to a Protobuf request type, which must implement `sdk.Msg` interface. Note that `sdk.Msg`s are bundled in [transactions](https://docs.cosmos.network/v0.50/learn/advanced/transactions), and each transaction contains one or multiple messages.

When a valid block of transactions is received by the full-node, CometBFT relays each one to the application via [`DeliverTx`](https://docs.cometbft.com/v0.37/spec/abci/abci++_app_requirements#specifics-of-responsedelivertx). Then, the application handles the transaction:

1.  Upon receiving the transaction, the application first unmarshalls it from `[]byte`.
2.  Then, it verifies a few things about the transaction like [fee payment and signatures](https://docs.cosmos.network/v0.50/learn/beginner/gas-fees#antehandler) before extracting the `Msg`(s) contained in the transaction.
3.  `sdk.Msg`s are encoded using Protobuf [`Any`s](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#register-codec). By analyzing each `Any`'s `type_url`, baseapp's `msgServiceRouter` routes the `sdk.Msg` to the corresponding module's `Msg` service.
4.  If the message is successfully processed, the state is updated.

For more details, see [transaction lifecycle](https://docs.cosmos.network/v0.50/learn/beginner/tx-lifecycle).

Module developers create custom `Msg` services when they build their own module. The general practice is to define the `Msg` Protobuf service in a `tx.proto` file. For example, the `x/bank` module defines a service with two methods to transfer tokens:


https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/proto/cosmos/bank/v1beta1/tx.proto#L13-L36


Service methods use `keeper` in order to update the module state.

Each module should also implement the `RegisterServices` method as part of the [`AppModule` interface](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#application-module-interface). This method should call the `RegisterMsgServer` function provided by the generated Protobuf code.


## gRPC Query Services

gRPC `Query` services allow users to query the state using [gRPC](https://grpc.io/). They are enabled by default, and can be configured under the `grpc.enable` and `grpc.address` fields inside [`app.toml`](https://docs.cosmos.network/v0.50/user/run-node/run-node#configuring-the-node-using-apptoml-and-configtoml).

gRPC `Query` services are defined in the module's Protobuf definition files, specifically inside `query.proto`. The `query.proto` definition file exposes a single `Query` [Protobuf service](https://developers.google.com/protocol-buffers/docs/proto#services). Each gRPC query endpoint corresponds to a service method, starting with the `rpc` keyword, inside the `Query` service.

Protobuf generates a `QueryServer` interface for each module, containing all the service methods. A module's [`keeper`](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#keeper) then needs to implement this `QueryServer` interface, by providing the concrete implementation of each service method. This concrete implementation is the handler of the corresponding gRPC query endpoint.

Finally, each module should also implement the `RegisterServices` method as part of the [`AppModule` interface](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#application-module-interface). This method should call the `RegisterQueryServer` function provided by the generated Protobuf code.


### Keeper

[`Keepers`](https://docs.cosmos.network/v0.50/build/building-modules/keeper) are the gatekeepers of their module's store(s). To read or write in a module's store, it is mandatory to go through one of its `keeper`'s methods. This is ensured by the [object-capabilities](https://docs.cosmos.network/v0.50/learn/advanced/ocap) model of the Cosmos SDK. Only objects that hold the key to a store can access it, and only the module's `keeper` should hold the key(s) to the module's store(s).

`Keepers` are generally defined in a file called `keeper.go`. It contains the `keeper`'s type definition and methods.

The `keeper` type definition generally consists of the following:

-   **Key(s)** to the module's store(s) in the multistore.
-   Reference to **other module's `keepers`**. Only needed if the `keeper` needs to access other module's store(s) (either to read or write from them).
-   A reference to the application's **codec**. The `keeper` needs it to marshal structs before storing them, or to unmarshal them when it retrieves them, because stores only accept `[]bytes` as value.

Along with the type definition, the next important component of the `keeper.go` file is the `keeper`'s constructor function, `NewKeeper`. This function instantiates a new `keeper` of the type defined above with a `codec`, stores `keys` and potentially references other modules' `keeper`s as parameters. The `NewKeeper` function is called from the [application's constructor](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#constructor-function). The rest of the file defines the `keeper`'s methods, which are primarily getters and setters.

### Command-Line, gRPC Services and REST Interfaces[​](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#command-line-grpc-services-and-rest-interfaces "Direct link to Command-Line, gRPC Services and REST Interfaces")

Each module defines command-line commands, gRPC services, and REST routes to be exposed to the end-user via the [application's interfaces](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#application-interfaces). This enables end-users to create messages of the types defined in the module, or to query the subset of the state managed by the module.

#### CLI[​](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#cli "Direct link to CLI")

Generally, the [commands related to a module](https://docs.cosmos.network/v0.50/build/building-modules/module-interfaces#cli) are defined in a folder called `client/cli` in the module's folder. The CLI divides commands into two categories, transactions and queries, defined in `client/cli/tx.go` and `client/cli/query.go`, respectively. Both commands are built on top of the [Cobra Library](https://github.com/spf13/cobra):

-   Transactions commands let users generate new transactions so that they can be included in a block and eventually update the state. One command should be created for each [message type](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#message-types) defined in the module. The command calls the constructor of the message with the parameters provided by the end-user, and wraps it into a transaction. The Cosmos SDK handles signing and the addition of other transaction metadata.
-   Queries let users query the subset of the state defined by the module. Query commands forward queries to the [application's query router](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#query-routing), which routes them to the appropriate [querier](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#querier) the `queryRoute` parameter supplied.

#### gRPC[​](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#grpc "Direct link to gRPC")

[gRPC](https://grpc.io/) is a modern open-source high performance RPC framework that has support in multiple languages. It is the recommended way for external clients (such as wallets, browsers and other backend services) to interact with a node.

Each module can expose gRPC endpoints called [service methods](https://grpc.io/docs/what-is-grpc/core-concepts/#service-definition), which are defined in the [module's Protobuf `query.proto` file](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#grpc-query-services). A service method is defined by its name, input arguments, and output response. The module then needs to perform the following actions:

-   Define a `RegisterGRPCGatewayRoutes` method on `AppModuleBasic` to wire the client gRPC requests to the correct handler inside the module.
-   For each service method, define a corresponding handler. The handler implements the core logic necessary to serve the gRPC request, and is located in the `keeper/grpc_query.go` file.

#### gRPC-gateway REST Endpoints[​](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#grpc-gateway-rest-endpoints "Direct link to gRPC-gateway REST Endpoints")

Some external clients may not wish to use gRPC. In this case, the Cosmos SDK provides a gRPC gateway service, which exposes each gRPC service as a corresponding REST endpoint. Please refer to the [grpc-gateway](https://grpc-ecosystem.github.io/grpc-gateway/) documentation to learn more.

The REST endpoints are defined in the Protobuf files, along with the gRPC services, using Protobuf annotations. Modules that want to expose REST queries should add `google.api.http` annotations to their `rpc` methods. By default, all REST endpoints defined in the SDK have a URL starting with the `/cosmos/` prefix.

The Cosmos SDK also provides a development endpoint to generate [Swagger](https://swagger.io/) definition files for these REST endpoints. This endpoint can be enabled inside the [`app.toml`](https://docs.cosmos.network/v0.50/user/run-node/run-node#configuring-the-node-using-apptoml-and-configtoml) config file, under the `api.swagger` key.