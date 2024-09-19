CLI
-----------------------------------------------------------------------------------------------------------

One of the main interfaces for an application is the [command-line interface](https://docs.cosmos.network/v0.50/learn/advanced/cli). This entrypoint adds commands from the application's modules enabling end-users to create [**messages**](https://docs.cosmos.network/v0.50/build/building-modules/messages-and-queries#messages) wrapped in transactions and [**queries**](https://docs.cosmos.network/v0.50/build/building-modules/messages-and-queries#queries). The CLI files are typically found in the module's `./client/cli` folder.


### Transaction Commands

In order to create messages that trigger state changes, end-users must create [transactions](https://docs.cosmos.network/v0.50/learn/advanced/transactions) that wrap and deliver the messages. A transaction command creates a transaction that includes one or more messages.

Transaction commands typically have their own `tx.go` file that lives within the module's `./client/cli` folder. The commands are specified in getter functions and the name of the function should include the name of the command.



### gRPC

[gRPC](https://grpc.io/) is a Remote Procedure Call (RPC) framework. RPC is the preferred way for external clients like wallets and exchanges to interact with a blockchain.

In addition to providing an ABCI query pathway, the Cosmos SDK provides a gRPC proxy server that routes gRPC query requests to ABCI query requests.

In order to do that, modules must implement `RegisterGRPCGatewayRoutes(clientCtx client.Context, mux *runtime.ServeMux)` on `AppModuleBasic` to wire the client gRPC requests to the correct handler inside the module.


gRPC-gateway REST
-----------------------------------------------------------------------------------------------------------------------------------------------------

Applications need to support web services that use HTTP requests (e.g. a web wallet like [Keplr](https://keplr.app/)). [grpc-gateway](https://github.com/grpc-ecosystem/grpc-gateway) translates REST calls into gRPC calls, which might be useful for clients that do not use gRPC.

Modules that want to expose REST queries should add `google.api.http` annotations to their `rpc` methods, such as in the example below from the `x/auth` module:

proto/cosmos/auth/v1beta1/query.proto


gRPC gateway is started in-process along with the application and CometBFT. It can be enabled or disabled by setting gRPC Configuration `enable` in [`app.toml`](https://docs.cosmos.network/v0.50/build/run-node/01-run-node.md#configuring-the-node-using-apptoml-and-configtoml).

The Cosmos SDK provides a command for generating [Swagger](https://swagger.io/) documentation (`protoc-gen-swagger`). Setting `swagger` in [`app.toml`](https://docs.cosmos.network/v0.50/build/run-node/01-run-node.md#configuring-the-node-using-apptoml-and-configtoml) defines if swagger documentation should be automatically registered.