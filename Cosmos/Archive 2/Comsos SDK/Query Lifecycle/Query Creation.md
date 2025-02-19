A [**query**](https://docs.cosmos.network/v0.50/build/building-modules/messages-and-queries#queries) is a request for information made by end-users of applications through an interface and processed by a full-node. Users can query information about the network, the application itself, and application state directly from the application's stores or modules. Note that queries are different from [transactions](https://docs.cosmos.network/v0.50/learn/advanced/transactions) (view the lifecycle [here](https://docs.cosmos.network/v0.50/learn/beginner/tx-lifecycle)), particularly in that they do not require consensus to be processed (as they do not trigger state-transitions); they can be fully handled by **one full-node**.

For the purpose of explaining the query lifecycle, let's say the query, `MyQuery`, is requesting a list of delegations made by a certain delegator address in the application called `simapp`. As is to be expected, the [`staking`](https://docs.cosmos.network/v0.50/build/modules/staking) module handles this query. But first, there are a few ways `MyQuery` can be created by users.

### CLI[​](https://docs.cosmos.network/v0.50/learn/beginner/query-lifecycle#cli "Direct link to CLI")

The main interface for an application is the command-line interface. Users connect to a full-node and run the CLI directly from their machines - the CLI interacts directly with the full-node. To create `MyQuery` from their terminal, users type the following command:

```
simd query staking delegations <delegatorAddress>

```

This query command was defined by the [`staking`](https://docs.cosmos.network/v0.50/build/modules/staking) module developer and added to the list of subcommands by the application developer when creating the CLI.

Note that the general format is as follows:

```
simd query [moduleName][command]<arguments> --flag <flagArg>

```

To provide values such as `--node` (the full-node the CLI connects to), the user can use the [`app.toml`](https://docs.cosmos.network/v0.50/user/run-node/run-node#configuring-the-node-using-apptoml-and-configtoml) config file to set them or provide them as flags.

The CLI understands a specific set of commands, defined in a hierarchical structure by the application developer: from the [root command](https://docs.cosmos.network/v0.50/learn/advanced/cli#root-command) (`simd`), the type of command (`Myquery`), the module that contains the command (`staking`), and command itself (`delegations`). Thus, the CLI knows exactly which module handles this command and directly passes the call there.

### gRPC[​](https://docs.cosmos.network/v0.50/learn/beginner/query-lifecycle#grpc "Direct link to gRPC")

Another interface through which users can make queries is [gRPC](https://grpc.io/) requests to a [gRPC server](https://docs.cosmos.network/v0.50/learn/advanced/grpc_rest#grpc-server). The endpoints are defined as [Protocol Buffers](https://developers.google.com/protocol-buffers) service methods inside `.proto` files, written in Protobuf's own language-agnostic interface definition language (IDL). The Protobuf ecosystem developed tools for code-generation from `*.proto` files into various languages. These tools allow to build gRPC clients easily.

One such tool is [grpcurl](https://github.com/fullstorydev/grpcurl), and a gRPC request for `MyQuery` using this client looks like:

```
grpcurl\
    -plaintext                                           # We want results in plain test
    -import-path ./proto \# Import these .proto files
    -proto ./proto/cosmos/staking/v1beta1/query.proto \# Look into this .proto file for the Query protobuf service
    -d '{"address":"$MY_DELEGATOR"}'\# Query arguments
    localhost:9090 \# gRPC server endpoint
    cosmos.staking.v1beta1.Query/Delegations             # Fully-qualified service method name

```

### REST[​](https://docs.cosmos.network/v0.50/learn/beginner/query-lifecycle#rest "Direct link to REST")

Another interface through which users can make queries is through HTTP Requests to a [REST server](https://docs.cosmos.network/v0.50/learn/advanced/grpc_rest#rest-server). The REST server is fully auto-generated from Protobuf services, using [gRPC-gateway](https://github.com/grpc-ecosystem/grpc-gateway).

An example HTTP request for `MyQuery` looks like:

```
GET http://localhost:1317/cosmos/staking/v1beta1/delegators/{delegatorAddr}/delegations

```