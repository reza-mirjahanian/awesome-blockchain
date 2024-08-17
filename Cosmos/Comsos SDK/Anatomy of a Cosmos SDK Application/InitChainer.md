### InitChainer[​](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#initchainer "Direct link to InitChainer")

The `InitChainer` is a function that initializes the state of the application from a genesis file (i.e. token balances of genesis accounts). It is called when the application receives the `InitChain` message from the CometBFT engine, which happens when the node is started at `appBlockHeight == 0` (i.e. on genesis). The application must set the `InitChainer` in its [constructor](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#constructor-function) via the [`SetInitChainer`](https://pkg.go.dev/github.com/cosmos/cosmos-sdk/baseapp#BaseApp.SetInitChainer) method.

In general, the `InitChainer` is mostly composed of the [`InitGenesis`](https://docs.cosmos.network/v0.50/build/building-modules/genesis#initgenesis) function of each of the application's modules. This is done by calling the `InitGenesis` function of the module manager, which in turn calls the `InitGenesis` function of each of the modules it contains. Note that the order in which the modules' `InitGenesis` functions must be called has to be set in the module manager using the [module manager's](https://docs.cosmos.network/v0.50/build/building-modules/module-manager) `SetOrderInitGenesis` method. This is done in the [application's constructor](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#application-constructor), and the `SetOrderInitGenesis` has to be called before the `SetInitChainer`.

See an example of an `InitChainer` from `simapp`


https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/simapp/app.go#L626-L634