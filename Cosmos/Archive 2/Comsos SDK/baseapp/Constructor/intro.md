Constructor[​](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#constructor "Direct link to Constructor")
-----------------------------------------------------------------------------------------------------------------

```
funcNewBaseApp(
  name string, logger log.Logger, db dbm.DB, txDecoder sdk.TxDecoder, options ...func(*BaseApp),
)*BaseApp {
// ...
}

```

The `BaseApp` constructor function is pretty straightforward. The only thing worth noting is the possibility to provide additional [`options`](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/baseapp/options.go) to the `BaseApp`, which will execute them in order. The `options` are generally `setter` functions for important parameters, like `SetPruning()` to set pruning options or `SetMinGasPrices()` to set the node's `min-gas-prices`.

Naturally, developers can add additional `options` based on their application's needs.