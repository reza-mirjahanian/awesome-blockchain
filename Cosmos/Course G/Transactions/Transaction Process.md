Transaction Process[​](https://docs.cosmos.network/v0.52/learn/advanced/transactions#transaction-process "Direct link to Transaction Process")
----------------------------------------------------------------------------------------------------------------------------------------------

The process of an end-user sending a transaction is:

-   decide on the messages to put into the transaction,
-   generate the transaction using the Cosmos SDK's `TxBuilder`,
-   broadcast the transaction using one of the available interfaces.

Module `sdk.Msg`s are not to be confused with [ABCI Messages](https://docs.cometbft.com/v1.0/spec/abci/) which define interactions between the CometBFT and application layers.



**Messages** (or `sdk.Msg`s) are module-specific objects that trigger state transitions within the scope of the module they belong to. Module developers define the messages for their module by adding methods to the Protobuf [`Msg` service](https://docs.cosmos.network/v0.52/build/building-modules/msg-services), and also implement the corresponding `MsgServer`.

Each `sdk.Msg`s is related to exactly one Protobuf [`Msg` service](https://docs.cosmos.network/v0.52/build/building-modules/msg-services) RPC, defined inside each module's `tx.proto` file. A SDK app router automatically maps every `sdk.Msg` to a corresponding RPC. Protobuf generates a `MsgServer` interface for each module `Msg` service, and the module developer needs to implement this interface. This design puts more responsibility on module developers, allowing application developers to reuse common functionalities without having to implement state transition logic repetitively.

To learn more about Protobuf `Msg` services and how to implement `MsgServer`, click [here](https://docs.cosmos.network/v0.52/build/building-modules/msg-services).

While messages contain the information for state transition logic, a transaction's other metadata and relevant information are stored in the `TxBuilder` and `Context`.


### Transaction Generation[​](https://docs.cosmos.network/v0.52/learn/advanced/transactions#transaction-generation "Direct link to Transaction Generation")

The `TxBuilder` interface contains data closely related with the generation of transactions, which an end-user can freely set to generate the desired transaction:

client/tx\_config.go
```
// TxBuilder defines an interface which an application-defined concrete transaction
// type must implement. Namely, it must be able to set messages, generate
// signatures, and provide canonical bytes to sign over. The transaction must
// also know how to encode itself.
TxBuilder interface{
GetTx() signing.Tx
SetMsgs(msgs ...sdk.Msg)error
SetSignatures(signatures ...signingtypes.SignatureV2)error
SetMemo(memo string)
SetFeeAmount(amount sdk.Coins)
SetFeePayer(feePayer sdk.AccAddress)
SetGasLimit(limit uint64)
SetTimeoutHeight(height uint64)
SetTimeoutTimestamp(timestamp time.Time)
SetUnordered(v bool)
SetFeeGranter(feeGranter sdk.AccAddress)
AddAuxSignerData(tx.AuxSignerData)error
}

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.52.0-beta.2/client/tx_config.go#L39-L57)

-   `Msg`s, the array of [messages](https://docs.cosmos.network/v0.52/learn/advanced/transactions#messages) included in the transaction.
-   `GasLimit`, option chosen by the users for how to calculate how much gas they will need to pay.
-   `Memo`, a note or comment to send with the transaction.
-   `FeeAmount`, the maximum amount the user is willing to pay in fees.
-   `TimeoutHeight`, block height until which the transaction is valid.
-   `Signatures`, the array of signatures from all signers of the transaction.

As there are currently two sign modes for signing transactions, there are also two implementations of `TxBuilder`:

-   [builder](https://github.com/cosmos/cosmos-sdk/blob/v0.52.0-beta.2/x/auth/tx/builder.go#L79-L98) for creating transactions for `SIGN_MODE_DIRECT`,
-   [StdTxBuilder](https://github.com/cosmos/cosmos-sdk/blob/v0.52.0-beta.2/x/auth/migrations/legacytx/stdtx_builder.go#L11-L17) for `SIGN_MODE_LEGACY_AMINO_JSON`.

However, the two implementations of `TxBuilder` should be hidden away from end-users, as they should prefer using the overarching `TxConfig` interface:

client/tx\_config.go
```
// TxConfig defines an interface a client can utilize to generate an
// application-defined concrete transaction type. The type returned must
// implement TxBuilder.
TxConfig interface{
	TxEncodingConfig
NewTxBuilder() TxBuilder
WrapTxBuilder(sdk.Tx)(TxBuilder,error)
SignModeHandler()*txsigning.HandlerMap
SigningContext()*txsigning.Context
}

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.52.0-beta.2/client/tx_config.go#L27-L37)

`TxConfig` is an app-wide configuration for managing transactions. Most importantly, it holds the information about whether to sign each transaction with `SIGN_MODE_DIRECT` or `SIGN_MODE_LEGACY_AMINO_JSON`. By calling `txBuilder := txConfig.NewTxBuilder()`, a new `TxBuilder` will be created with the appropriate sign mode.

Once `TxBuilder` is correctly populated with the setters exposed above, `TxConfig` will also take care of correctly encoding the bytes (again, either using `SIGN_MODE_DIRECT` or `SIGN_MODE_LEGACY_AMINO_JSON`). Here's a pseudo-code snippet of how to generate and encode a transaction, using the `TxEncoder()` method:

```
txBuilder := txConfig.NewTxBuilder()
txBuilder.SetMsgs(...)// and other setters on txBuilder
bz, err := txConfig.TxEncoder()(txBuilder.GetTx())
// bz are bytes to be broadcasted over the network
```

### Broadcasting the Transaction[​](https://docs.cosmos.network/v0.52/learn/advanced/transactions#broadcasting-the-transaction "Direct link to Broadcasting the Transaction")

Once the transaction bytes are generated, there are currently three ways of broadcasting it.

#### CLI[​](https://docs.cosmos.network/v0.52/learn/advanced/transactions#cli "Direct link to CLI")

Application developers create entry points to the application by creating a [command-line interface](https://docs.cosmos.network/v0.52/learn/advanced/cli), [gRPC and/or REST interface](https://docs.cosmos.network/v0.52/learn/advanced/grpc_rest), typically found in the application's `./cmd` folder. These interfaces allow users to interact with the application through command-line.

For the [command-line interface](https://docs.cosmos.network/v0.52/build/building-modules/module-interfaces#cli), module developers create subcommands to add as children to the application top-level transaction command `TxCmd`. CLI commands actually bundle all the steps of transaction processing into one simple command: creating messages, generating transactions and broadcasting. For concrete examples, see the [Interacting with a Node](https://docs.cosmos.network/main/user/run-node/interact-node) section. An example transaction made using CLI looks like:

```
simd tx send $MY_VALIDATOR_ADDRESS$RECIPIENT 1000stake

```

#### gRPC[​](https://docs.cosmos.network/v0.52/learn/advanced/transactions#grpc "Direct link to gRPC")

[gRPC](https://grpc.io/) is the main component for the Cosmos SDK's RPC layer. Its principal usage is in the context of modules' [`Query` services](https://docs.cosmos.network/v0.52/build/building-modules/query-services). However, the Cosmos SDK also exposes a few other module-agnostic gRPC services, one of them being the `Tx` service:

<https://github.com/cosmos/cosmos-sdk/blob/v0.52.0-beta.2/proto/cosmos/tx/v1beta1/service.proto>

The `Tx` service exposes a handful of utility functions, such as simulating a transaction or querying a transaction, and also one method to broadcast transactions.

Examples of broadcasting and simulating a transaction are shown [here](https://docs.cosmos.network/main/user/run-node/txs#programmatically-with-go).

#### REST[​](https://docs.cosmos.network/v0.52/learn/advanced/transactions#rest "Direct link to REST")

Each gRPC method has its corresponding REST endpoint, generated using [gRPC-gateway](https://github.com/grpc-ecosystem/grpc-gateway). Therefore, instead of using gRPC, you can also use HTTP to broadcast the same transaction, on the `POST /cosmos/tx/v1beta1/txs` endpoint.

An example can be seen [here](https://docs.cosmos.network/main/user/run-node/txs#using-rest)

#### CometBFT RPC[​](https://docs.cosmos.network/v0.52/learn/advanced/transactions#cometbft-rpc "Direct link to CometBFT RPC")

The three methods presented above are actually higher abstractions over the CometBFT RPC `/broadcast_tx_{async,sync,commit}` endpoints, documented [here](https://docs.cometbft.com/v1.0/explanation/core/rpc). This means that you can use the CometBFT RPC endpoints directly to broadcast the transaction, if you wish so.