Consider the following simple constructor:

```
func NewBaseApp(
  name string, logger log.Logger, db dbm.DB, txDecoder sdk.TxDecoder, options ...func(*BaseApp),
) *BaseApp {

  // ...
}
```
The `BaseApp` constructor function is pretty straightforward. Notice the possibility of providing additional `options` to the `BaseApp`, which executes them in order. These options are generally setter functions for important parameters, like `SetPruning()` to set pruning options, or `SetMinGasPrices()` to set the node's min-gas-prices.

Developers can add additional options based on their application's needs.