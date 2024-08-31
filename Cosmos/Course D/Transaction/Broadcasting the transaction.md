Once the transaction bytes are generated and signed, there are **three primary ways of broadcasting** the transaction:

-   Using the command-line interface (CLI).
-   Using gRPC.
-   Using REST endpoints.

Application developers create entrypoints to the application by creating a command-line interface typically found in the application's `./cmd` folder, gRPC, and/or REST interface. These interfaces allow users to interact with the application.

### [#](https://ida.interchain.io/academy/2-cosmos-concepts/3-transactions.html#cli)CLI

The command-line interface (CLI) client is a versatile way for users to create transactions.

For the CLI, module developers create subcommands to add as children to the application top-level transaction command [`TxCmd` ](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/x/bank/client/cli/tx.go#L29-L60).

CLI commands bundle all the steps of transaction processing into one simple command:

-   Creating messages.
-   Generating transactions.
-   Signing.
-   Broadcasting.

### [#](https://ida.interchain.io/academy/2-cosmos-concepts/3-transactions.html#grpc)gRPC

The principal usage of gRPC is in the context of module query services. The Cosmos SDK also exposes other module-agnostic gRPC services. One of these is the `Tx` service, which exposes a handful of utility functions such as simulating a transaction or querying a transaction, and also one method to [broadcast transactions ](https://github.com/cosmos/cosmos-sdk-docs/blob/main/docs/user/run-node/03-txs.md#broadcasting-a-transaction).

![](https://ida.interchain.io/hi-tip-icon.svg)

See this [code example ](https://github.com/cosmos/cosmos-sdk-docs/blob/main/docs/user/run-node/03-txs.md#programmatically-with-go)for more insight.

### [#](https://ida.interchain.io/academy/2-cosmos-concepts/3-transactions.html#rest)REST

Each gRPC method has its corresponding REST endpoint generated using gRPC-gateway. Rather than using gRPC, you can also use HTTP to broadcast the same transaction on the `POST` `/cosmos/tx/v1beta1/txs` endpoint.

![](https://ida.interchain.io/hi-tip-icon.svg)

See this [code example ](https://github.com/cosmos/cosmos-sdk-docs/blob/main/docs/user/run-node/03-txs.md#using-rest)for more details.

### [#](https://ida.interchain.io/academy/2-cosmos-concepts/3-transactions.html#cometbft-rpc)CometBFT RPC

The three methods presented previously are higher abstractions on the CometBFT RPC `/broadcast_tx_{async,sync,commit}` endpoints. You can use the [Tendermint RPC endpoints ](https://docs.tendermint.com/v0.34/tendermint-core/rpc.html)to directly broadcast the transaction through CometBFT if you wish to.

![](https://ida.interchain.io/hi-info-icon.svg)

CometBFT supports the following RPC protocols:

-   URI over HTTP.
-   JSONRPC over HTTP.
-   JSONRPC over WebSockets.

For more information on broadcasting with CometBFT RPC, see the documentation on [Tendermint RPC transactions broadcast APIs ](https://docs.tendermint.com/v0.34/tendermint-core/rpc.html).

[#](https://ida.interchain.io/academy/2-cosmos-concepts/3-transactions.html#code-example)Code example
-----------------------------------------------------------------------------------------------------

Show me some code for my checkers blockchain

synopsis

To summarize, this section has explored:

-   How transactions are objects created by end-users to trigger state changes in an application module through that module's Protobuf message service.
-   How transaction messages are not to be confused with ABCI messages, which define interactions between CometBFT and application layers.
-   How *deciding* and *signing* transactions are the main interactions of a user, whereas *generating* and *broadcasting* transactions are attended to by the user interface and other automation.
-   How the modular nature of the Cosmos SDK places more responsibility on *module* developers to effectively code transaction processes, so *application* developers can reuse common functionalities without having to repetitively implement state transition logic.