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



