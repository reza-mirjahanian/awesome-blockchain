### RunTx[​](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#runtx "Direct link to RunTx")

`RunTx` is called from `CheckTx`/`Finalizeblock` to handle the transaction, with `execModeCheck` or `execModeFinalize` as parameter to differentiate between the two modes of execution. Note that when `RunTx` receives a transaction, it has already been decoded.

The first thing `RunTx` does upon being called is to retrieve the `context`'s `CacheMultiStore` by calling the `getContextForTx()` function with the appropriate mode (either `runTxModeCheck` or `execModeFinalize`). This `CacheMultiStore` is a branch of the main store, with cache functionality (for query requests), instantiated during `FinalizeBlock` for transaction execution and during the `Commit` of the previous block for `CheckTx`. After that, two `defer func()` are called for [`gas`](https://docs.cosmos.network/v0.52/learn/beginner/gas-fees) management. They are executed when `runTx` returns and make sure `gas` is actually consumed, and will throw errors, if any.

After that, `RunTx()` calls `ValidateBasic()`, when available and for backward compatibility, on each `sdk.Msg`in the `Tx`, which runs preliminary *stateless* validity checks. If any `sdk.Msg` fails to pass `ValidateBasic()`, `RunTx()` returns with an error.

Then, the [`anteHandler`](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#antehandler) of the application is run (if it exists). In preparation of this step, both the `checkState`/`finalizeBlockState`'s `context` and `context`'s `CacheMultiStore` are branched using the `cacheTxContext()` function.

```
// cacheTxContext returns a new context based off of the provided context with
// a branched multi-store.
func (app *BaseApp) cacheTxContext(ctx sdk.Context, txBytes []byte) (sdk.Context, storetypes.CacheMultiStore) {
	ms := ctx.MultiStore()
	msCache := ms.CacheMultiStore()
	if msCache.TracingEnabled() {
		msCache = msCache.SetTracingContext(
			storetypes.TraceContext(
				map[string]interface{}{
					"txHash": fmt.Sprintf("%X", tmhash.Sum(txBytes)),
				},
			),
		).(storetypes.CacheMultiStore)
	}

	return ctx.WithMultiStore(msCache), msCache
}
```

This allows `RunTx` not to commit the changes made to the state during the execution of `anteHandler` if it ends up failing. It also prevents the module implementing the `anteHandler` from writing to state, which is an important part of the [object-capabilities](https://docs.cosmos.network/v0.52/learn/advanced/ocap) of the Cosmos SDK.

Finally, the [`RunMsgs()`](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#runmsgs) function is called to process the `sdk.Msg`s in the `Tx`. In preparation of this step, just like with the `anteHandler`, both the `checkState`/`finalizeBlockState`'s `context` and `context`'s `CacheMultiStore` are branched using the `cacheTxContext()` function.


### AnteHandler
The `AnteHandler` is a special handler that implements the `AnteHandler` interface and is used to authenticate the transaction before the transaction's internal messages are processed. It is theoretically optional but serves as a crucial component in most public blockchain networks.

The `AnteHandler` operates on a copy of the cached context, allowing it to perform stateful checks such as signature verification, sequence number incrementation, and fee deduction without altering the last committed state. If the execution fails, it can revert to the original state, ensuring that only successful transactions modify the blockchain state.

Key operations performed by the `AnteHandler` include:

-   **Signature Verification**: Ensures that the transaction's signatures are valid.
-   **Sequence Checking**: Verifies and increments the sequence numbers to prevent replay attacks.
-   **Fee Deduction**: Deducts the transaction fees from the accounts involved, typically starting with the first signer.

These operations are crucial for maintaining the security and integrity of transactions on the blockchain.

For more detailed examples, see the [`auth` module's `AnteHandler`](https://github.com/cosmos/cosmos-sdk/tree/main/x/auth) which is widely used for these purposes.

⚠️ danger

Ante handlers typically operate at the transaction level. By default, they process only the outermost message of a transaction. However, transactions that embed multiple messages, such as those found in modules like x/authz or x/gov, may have inner messages that are not automatically processed by these default ante handlers. These inner messages are generally routed directly to the [message router](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#msg-service-router) bypassing the ante handlers. To ensure comprehensive processing, custom ante handlers can be designed to recursively inspect and apply necessary checks to all embedded messages within a transaction. This capability must be explicitly implemented to extend the awareness of ante handlers to inner messages.


The `AnteHandler` is a primary line of defense against spam and a second line of defense (the first one being the mempool) against transaction replay with fees deduction and [`sequence`](https://docs.cosmos.network/v0.52/learn/advanced/transactions#transaction-generation) checking. It also performs preliminary *stateful* validity checks like ensuring signatures are valid or that the sender has enough funds to pay for fees, and plays a role in the incentivisation of stakeholders via the collection of transaction fees.

`BaseApp` holds an `anteHandler` as parameter that is initialized in the [application's constructor](https://docs.cosmos.network/v0.52/learn/beginner/app-anatomy#constructor-function). The most widely used `anteHandler` is the [`auth` module](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/x/auth/ante/ante.go).

Click [here](https://docs.cosmos.network/v0.52/learn/beginner/gas-fees#antehandler) for more on the `anteHandler`.


### RunMsgs[​](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#runmsgs "Direct link to RunMsgs")

`RunMsgs` is called from `RunTx` with `runTxModeCheck` as parameter to check the existence of a route for each message the transaction, and with `execModeFinalize` to actually process the `sdk.Msg`s.

`FinalizeBlock`, calls [`runMsgs`](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#runtx-antehandler-runmsgs-posthandler) to fully execute each `Msg` within the transaction. Since the transaction may have messages from different modules, `BaseApp` needs to know which module to find the appropriate handler. This is achieved using `BaseApp`'s `MsgServiceRouter` so that it can be processed by the module's Protobuf [`Msg` service](https://docs.cosmos.network/v0.52/build/building-modules/msg-services).

For `LegacyMsg` routing, the `Route` function is called via the [module manager](https://docs.cosmos.network/v0.52/build/building-modules/module-manager) to retrieve the route name and find the legacy [`Handler`](https://docs.cosmos.network/v0.52/build/building-modules/msg-services#handler-type) within the module.

First, it retrieves the `sdk.Msg`'s fully-qualified type name, by checking the `type_url` of the Protobuf `Any` representing the `sdk.Msg`. Then, using the application's [`msgServiceRouter`](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#msg-service-router), it checks for the existence of `Msg` service method related to that `type_url`. At this point, if `mode == runTxModeCheck`, `RunMsgs` returns. Otherwise, if `mode == execModeFinalize`, the [`Msg` service](https://docs.cosmos.network/v0.52/build/building-modules/msg-services) RPC is executed, before `RunMsgs` returns.


### PostHandler[​](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#posthandler "Direct link to PostHandler")

`PostHandler` is similar to `AnteHandler`, but it, as the name suggests, executes custom post tx processing logic after [`RunMsgs`](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#runmsgs) is called. `PostHandler` receives the `Result` of the `RunMsgs` in order to enable this customizable behavior.

Like `AnteHandler`s, `PostHandler`s are theoretically optional.

Other use cases like unused gas refund can also be enabled by `PostHandler`s.

Note, when `PostHandler`s fail, the state from `runMsgs` is also reverted, effectively making the transaction fail.