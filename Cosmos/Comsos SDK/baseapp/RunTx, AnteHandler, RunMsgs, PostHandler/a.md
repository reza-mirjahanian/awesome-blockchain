### RunTx[​](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#runtx "Direct link to RunTx")

`RunTx` is called from `CheckTx`/`Finalizeblock` to handle the transaction, with `execModeCheck` or `execModeFinalize` as parameter to differentiate between the two modes of execution. Note that when `RunTx` receives a transaction, it has already been decoded.

The first thing `RunTx` does upon being called is to retrieve the `context`'s `CacheMultiStore` by calling the `getContextForTx()` function with the appropriate mode (either `runTxModeCheck` or `execModeFinalize`). This `CacheMultiStore` is a branch of the main store, with cache functionality (for query requests), instantiated during `FinalizeBlock` for transaction execution and during the `Commit` of the previous block for `CheckTx`. After that, two `defer func()` are called for [`gas`](https://docs.cosmos.network/v0.50/learn/beginner/gas-fees) management. They are executed when `runTx` returns and make sure `gas` is actually consumed, and will throw errors, if any.

After that, `RunTx()` calls `ValidateBasic()`, when available and for backward compatibility, on each `sdk.Msg`in the `Tx`, which runs preliminary *stateless* validity checks. If any `sdk.Msg` fails to pass `ValidateBasic()`, `RunTx()` returns with an error.

Then, the [`anteHandler`](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#antehandler) of the application is run (if it exists). In preparation of this step, both the `checkState`/`finalizeBlockState`'s `context` and `context`'s `CacheMultiStore` are branched using the `cacheTxContext()` function.

baseapp/baseapp.go
```
// cacheTxContext returns a new context based off of the provided context with
// a branched multi-store.
func(app *BaseApp)cacheTxContext(ctx sdk.Context, txBytes []byte)(sdk.Context, storetypes.CacheMultiStore){
	ms := ctx.MultiStore()
// TODO: https://github.com/cosmos/cosmos-sdk/issues/2824
	msCache := ms.CacheMultiStore()
if msCache.TracingEnabled(){
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

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/baseapp/baseapp.go#L663-L680)

This allows `RunTx` not to commit the changes made to the state during the execution of `anteHandler` if it ends up failing. It also prevents the module implementing the `anteHandler` from writing to state, which is an important part of the [object-capabilities](https://docs.cosmos.network/v0.50/learn/advanced/ocap) of the Cosmos SDK.

Finally, the [`RunMsgs()`](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#runmsgs) function is called to process the `sdk.Msg`s in the `Tx`. In preparation of this step, just like with the `anteHandler`, both the `checkState`/`finalizeBlockState`'s `context` and `context`'s `CacheMultiStore` are branched using the `cacheTxContext()` function.

### AnteHandler[​](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#antehandler "Direct link to AnteHandler")

The `AnteHandler` is a special handler that implements the `AnteHandler` interface and is used to authenticate the transaction before the transaction's internal messages are processed.

types/handler.go
```
// AnteHandler authenticates transactions, before their internal messages are handled.
// If newCtx.IsZero(), ctx is used instead.
type AnteHandler func(ctx Context, tx Tx, simulate bool)(newCtx Context, err error)

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/types/handler.go#L6-L8)

The `AnteHandler` is theoretically optional, but still a very important component of public blockchain networks. It serves 3 primary purposes:

-   Be a primary line of defense against spam and second line of defense (the first one being the mempool) against transaction replay with fees deduction and [`sequence`](https://docs.cosmos.network/v0.50/learn/advanced/transactions#transaction-generation) checking.
-   Perform preliminary *stateful* validity checks like ensuring signatures are valid or that the sender has enough funds to pay for fees.
-   Play a role in the incentivisation of stakeholders via the collection of transaction fees.

`BaseApp` holds an `anteHandler` as parameter that is initialized in the [application's constructor](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#application-constructor). The most widely used `anteHandler` is the [`auth` module](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/x/auth/ante/ante.go).

Click [here](https://docs.cosmos.network/v0.50/learn/beginner/gas-fees#antehandler) for more on the `anteHandler`.

### RunMsgs[​](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#runmsgs "Direct link to RunMsgs")

`RunMsgs` is called from `RunTx` with `runTxModeCheck` as parameter to check the existence of a route for each message the transaction, and with `execModeFinalize` to actually process the `sdk.Msg`s.

First, it retrieves the `sdk.Msg`'s fully-qualified type name, by checking the `type_url` of the Protobuf `Any` representing the `sdk.Msg`. Then, using the application's [`msgServiceRouter`](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#msg-service-router), it checks for the existence of `Msg` service method related to that `type_url`. At this point, if `mode == runTxModeCheck`, `RunMsgs` returns. Otherwise, if `mode == execModeFinalize`, the [`Msg` service](https://docs.cosmos.network/v0.50/build/building-modules/msg-services) RPC is executed, before `RunMsgs` returns.

### PostHandler[​](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#posthandler "Direct link to PostHandler")

`PostHandler` is similar to `AnteHandler`, but it, as the name suggests, executes custom post tx processing logic after [`RunMsgs`](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#runmsgs) is called. `PostHandler` receives the `Result` of the `RunMsgs` in order to enable this customizable behavior.

Like `AnteHandler`s, `PostHandler`s are theoretically optional.

Other use cases like unused gas refund can also be enabled by `PostHandler`s.

x/auth/posthandler/post.go
```
package posthandler
import(
	sdk "github.com/cosmos/cosmos-sdk/types"
)
// HandlerOptions are the options required for constructing a default SDK PostHandler.
type HandlerOptions struct{}
// NewPostHandler returns an empty PostHandler chain.
funcNewPostHandler(_ HandlerOptions)(sdk.PostHandler,error){
	postDecorators :=[]sdk.PostDecorator{}
return sdk.ChainPostDecorators(postDecorators...),nil
}

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/x/auth/posthandler/post.go#L1-L15)

Note, when `PostHandler`s fail, the state from `runMsgs` is also reverted, effectively making the transaction fail.