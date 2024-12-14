### InitChain[​](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#initchain "Direct link to InitChain")

The [`InitChain` ABCI message](https://docs.cometbft.com/v1.0/spec/abci/abci++_methods#initchain) is sent from the underlying CometBFT engine when the chain is first started. It is mainly used to **initialize** parameters and state like:

-   [Consensus Parameters](https://docs.cometbft.com/v1.0/spec/abci/abci++_app_requirements#consensus-parameters) via `setConsensusParams`.
-   [`checkState` and `finalizeBlockState`](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#state-updates) via `setState`.
-   The [block gas meter](https://docs.cosmos.network/v0.52/learn/beginner/gas-fees#block-gas-meter), with infinite gas to process genesis transactions.

Finally, the `InitChain(req abci.RequestInitChain)` method of `BaseApp` calls the [`initChainer()`](https://docs.cosmos.network/v0.52/learn/beginner/app-anatomy#initchainer) of the application in order to initialize the main state of the application from the `genesis file` and, if defined, call the [`InitGenesis`](https://docs.cosmos.network/v0.52/build/building-modules/genesis#initgenesis) function of each of the application's modules.

### FinalizeBlock[​](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#finalizeblock "Direct link to FinalizeBlock")

The [`FinalizeBlock` ABCI message](https://docs.cometbft.com/v1.0/spec/abci/abci++_methods#finalizeblock) is sent from the underlying CometBFT engine when a block proposal created by the correct proposer is received. The previous `BeginBlock, DeliverTx and Endblock` calls are private methods on the BaseApp struct.

#### BeginBlock[​](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#beginblock "Direct link to BeginBlock")

-   Initialize [`finalizeBlockState`](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#state-updates) with the latest header using the `req abci.RequestFinalizeBlock` passed as parameter via the `setState` function.

```
baseapp/baseapp.go
func (app *BaseApp) beginBlock(_ *abci.FinalizeBlockRequest) (sdk.BeginBlock, error) {
	var (
		resp sdk.BeginBlock
		err  error
	)

	if app.beginBlocker != nil {
		resp, err = app.beginBlocker(app.finalizeBlockState.Context())
		if err != nil {
			return resp, err
		}

		// append BeginBlock attributes to all events in the BeginBlock response
		for i, event := range resp.Events {
			resp.Events[i].Attributes = append(
				event.Attributes,
				abci.EventAttribute{Key: "mode", Value: "BeginBlock"},
				abci.EventAttribute{Key: "event_index", Value: strconv.Itoa(i)},
			)
		}

		resp.Events = sdk.MarkEventsToIndex(resp.Events, app.indexEvents)
	}

	return resp, nil
}
```
-   This function also resets the [main gas meter](https://docs.cosmos.network/v0.52/learn/beginner/gas-fees#main-gas-meter).

-   Initialize the [block gas meter](https://docs.cosmos.network/v0.52/learn/beginner/gas-fees#block-gas-meter) with the `maxGas` limit. The `gas` consumed within the block cannot go above `maxGas`. This parameter is defined in the application's consensus parameters.

-   Run the application's [`beginBlocker()`](https://docs.cosmos.network/v0.52/learn/beginner/app-anatomy#beginblocker-and-endblocker), which mainly runs the [`BeginBlocker()`](https://docs.cosmos.network/v0.52/build/building-modules/preblock-beginblock-endblock#beginblocker-and-endblocker) method of each of the modules.

-   Set the [`VoteInfos`](https://docs.cometbft.com/v1.0/spec/abci/abci++_methods#voteinfo) of the application, i.e. the list of validators whose *precommit* for the previous block was included by the proposer of the current block. This information is carried into the [`Context`](https://docs.cosmos.network/v0.52/learn/advanced/context) so that it can be used during transaction execution and EndBlock.



#### EndBlock[​](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#endblock "Direct link to EndBlock")

EndBlock is run after transaction execution completes. It allows developers to have logic be executed at the end of each block. In the Cosmos SDK, the bulk EndBlock() method is to run the application's EndBlocker(), which mainly runs the EndBlocker() method of each of the application's modules.



```

baseapp/baseapp.go
func (app *BaseApp) beginBlock(_ *abci.FinalizeBlockRequest) (sdk.BeginBlock, error) {
	var (
		resp sdk.BeginBlock
		err  error
	)

	if app.beginBlocker != nil {
		resp, err = app.beginBlocker(app.finalizeBlockState.Context())
		if err != nil {
			return resp, err
		}

		// append BeginBlock attributes to all events in the BeginBlock response
		for i, event := range resp.Events {
			resp.Events[i].Attributes = append(
				event.Attributes,
				abci.EventAttribute{Key: "mode", Value: "BeginBlock"},
				abci.EventAttribute{Key: "event_index", Value: strconv.Itoa(i)},
			)
		}

		resp.Events = sdk.MarkEventsToIndex(resp.Events, app.indexEvents)
	}

	return resp, nil
}
```

### Commit[​](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#commit "Direct link to Commit")

The [`Commit` ABCI message](https://docs.cometbft.com/v1.0/spec/abci/abci++_methods#commit) is sent from the underlying CometBFT engine after the full-node has received *precommits* from 2/3+ of validators (weighted by voting power). This is the final step where nodes commit the block and state changes. Validator nodes perform the previous step of executing state transitions to validate the transactions, then sign the block to confirm it. Full nodes that are not validators do not participate in consensus but listen for votes to understand whether or not they should commit the state changes.

On the `BaseApp` end, the `Commit(res abci.CommitResponse)` function is implemented to commit all the valid state transitions that occurred during `FinalizeBlock` and to reset state for the next block. It syncs all the state transitions by writing the `finalizeBlockState.ms` into the application's internal state. `finalizeBlockState.ms` is a branched multistore of the main store `app.cms`. The `Commit` function calls the `Write()` function on `finalizeBlockState.ms`, effectively committing the state transitions. Then, the `Commit` function sets `checkState` to the latest header (obtained from `finalizeBlockState.ctx.BlockHeader`) and `finalizeBlockState` to `nil`.

Finally, `Commit` returns the hash of the commitment of `app.cms` back to the underlying consensus engine. This hash is used as a reference in the header of the next block. As soon as the state changes are committed, `checkState` starts afresh from the most recently committed state and `finalizeBlockState` resets to `nil` in order to be consistent and reflect the changes.

Note that not all blocks have the same number of transactions and it is possible for consensus to result in a `nil` block or one with none at all. In a public blockchain network, it is also possible for validators to be byzantine, or malicious, which may prevent a `Tx` from being committed in the blockchain. Possible malicious behaviors include the proposer deciding to censor a `Tx` by excluding it from the block or a validator voting against the block.

At this point, the transaction lifecycle of a `Tx` is over: nodes have verified its validity, delivered it by executing its state changes, and committed those changes. The `Tx` itself, in `[]byte` form, is stored in a block and appended to the blockchain.


### Info[​](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#info "Direct link to Info")

The [`Info` ABCI message](https://docs.cometbft.com/v1.0/spec/abci/abci++_methods#info) is a simple query from the underlying consensus engine, notably used to sync the latter with the application during a handshake that happens on startup. When called, the `Info(res abci.InfoResponse)` function from `BaseApp` will return the application's name, version and the hash of the last commit of `app.cms`.

### Query[​](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#query "Direct link to Query")

The [`Query` ABCI message](https://docs.cometbft.com/v1.0/spec/abci/abci++_methods#query) is used to serve queries received from the underlying consensus engine, including queries received via RPC like CometBFT RPC. It used to be the main entrypoint to build interfaces with the application, but with the introduction of [gRPC queries](https://docs.cosmos.network/v0.52/build/building-modules/query-services) in Cosmos SDK v0.40, its usage is more limited. The application must respect a few rules when implementing the `Query` method, which are outlined [here](https://docs.cometbft.com/v1.0/spec/abci/abci++_methods#query).

Each CometBFT `query` comes with a `path`, which is a `string` which denotes what to query. If the `path` matches a gRPC fully-qualified service method, then `BaseApp` will defer the query to the `grpcQueryRouter` and let it handle it like explained [above](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#grpc-query-router). Otherwise, the `path` represents a query that is not (yet) handled by the gRPC router. `BaseApp` splits the `path` string with the `/` delimiter. By convention, the first element of the split string (`split[0]`) contains the category of `query` (`app`, `p2p`, `store` or `custom` ). The `BaseApp` implementation of the `Query(req abci.QueryRequest)` method is a simple dispatcher serving these 4 main categories of queries:

-   Application-related queries like querying the application's version, which are served via the `handleQueryApp` method.
-   Direct queries to the multistore, which are served by the `handlerQueryStore` method. These direct queries are different from custom queries which go through `app.queryRouter`, and are mainly used by third-party service provider like block explorers.
-   P2P queries, which are served via the `handleQueryP2P` method. These queries return either `app.addrPeerFilter` or `app.ipPeerFilter` that contain the list of peers filtered by address or IP respectively. These lists are first initialized via `options` in `BaseApp`'s [constructor](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#constructor).

### ExtendVote[​](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#extendvote "Direct link to ExtendVote")

`ExtendVote` allows an application to extend a pre-commit vote with arbitrary data. This process does NOT have to be deterministic and the data returned can be unique to the validator process.

In the Cosmos-SDK this is implemented as a NoOp:

baseapp/abci\_utils.go
```
// NoOpExtendVote defines a no-op ExtendVote handler. It will always return an
// empty byte slice as the vote extension.
func NoOpExtendVote() sdk.ExtendVoteHandler {
	return func(_ sdk.Context, _ *abci.ExtendVoteRequest) (*abci.ExtendVoteResponse, error) {
		return &abci.ExtendVoteResponse{VoteExtension: []byte{}}, nil
	}
}

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.52.0-beta.2/baseapp/abci_utils.go#L456-L462)

### VerifyVoteExtension[​](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#verifyvoteextension "Direct link to VerifyVoteExtension")

`VerifyVoteExtension` allows an application to verify that the data returned by `ExtendVote` is valid. This process MUST be deterministic. Moreover, the value of ResponseVerifyVoteExtension.status MUST exclusively depend on the parameters passed in the call to RequestVerifyVoteExtension, and the last committed Application state.

In the Cosmos-SDK this is implemented as a NoOp:

baseapp/abci\_utils.go
```
// NoOpVerifyVoteExtensionHandler defines a no-op VerifyVoteExtension handler. It
// will always return an ACCEPT status with no error.
func NoOpVerifyVoteExtensionHandler() sdk.VerifyVoteExtensionHandler {
	return func(_ sdk.Context, _ *abci.RequestVerifyVoteExtension) (*abci.ResponseVerifyVoteExtension, error) {
		return &abci.ResponseVerifyVoteExtension{Status: abci.ResponseVerifyVoteExtension_ACCEPT}, nil
	}
}
```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/baseapp/abci_utils.go#L282-L288)