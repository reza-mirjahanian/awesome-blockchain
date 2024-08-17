The Cosmos SDK offers developers the possibility to implement automatic execution of code as part of their application. This is implemented through two functions called `BeginBlocker` and `EndBlocker`. They are called when the application receives the `FinalizeBlock` messages from the CometBFT consensus engine, which happens respectively at the beginning and at the end of each block. The application must set the `BeginBlocker` and `EndBlocker` in its [constructor](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#constructor-function) via the [`SetBeginBlocker`](https://pkg.go.dev/github.com/cosmos/cosmos-sdk/baseapp#BaseApp.SetBeginBlocker) and [`SetEndBlocker`](https://pkg.go.dev/github.com/cosmos/cosmos-sdk/baseapp#BaseApp.SetEndBlocker) methods.

In general, the `BeginBlocker` and `EndBlocker` functions are mostly composed of the [`BeginBlock` and `EndBlock`](https://docs.cosmos.network/v0.50/build/building-modules/beginblock-endblock) functions of each of the application's modules. This is done by calling the `BeginBlock` and `EndBlock` functions of the module manager, which in turn calls the `BeginBlock` and `EndBlock` functions of each of the modules it contains. Note that the order in which the modules' `BeginBlock` and `EndBlock` functions must be called has to be set in the module manager using the `SetOrderBeginBlockers` and `SetOrderEndBlockers` methods, respectively. This is done via the [module manager](https://docs.cosmos.network/v0.50/build/building-modules/module-manager) in the [application's constructor](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#application-constructor), and the `SetOrderBeginBlockers` and `SetOrderEndBlockers` methods have to be called before the `SetBeginBlocker` and `SetEndBlocker` functions.

As a sidenote, it is important to remember that application-specific blockchains are deterministic. Developers must be careful not to introduce non-determinism in `BeginBlocker` or `EndBlocker`, and must also be careful not to make them too computationally expensive, as [gas](https://docs.cosmos.network/v0.50/learn/beginner/gas-fees) does not constrain the cost of `BeginBlocker` and `EndBlocker` execution.

See an example of `BeginBlocker` and `EndBlocker` functions from `simapp`


https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/simapp/app.go#L613-L620

```
simapp/app.go
func (app *SimApp) BeginBlocker(ctx sdk.Context) (sdk.BeginBlock, error) {
	return app.ModuleManager.BeginBlock(ctx)
}

// EndBlocker application updates every end block
func (app *SimApp) EndBlocker(ctx sdk.Context) (sdk.EndBlock, error) {
	return app.ModuleManager.EndBlock(ctx)
}
```