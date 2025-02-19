`CheckTx` is sent by the underlying consensus engine when a new unconfirmed (i.e. not yet included in a valid block) transaction is received by a full-node. The role of `CheckTx` is to guard the full-node's mempool (where unconfirmed transactions are stored until they are included in a block) from spam transactions. Unconfirmed transactions are relayed to peers only if they pass `CheckTx`.

`CheckTx()` can perform both *stateful* and *stateless* checks, but developers should strive to make the checks **lightweight** because gas fees are not charged for the resources (CPU, data load...) used during the `CheckTx`.

The [`Context`](https://docs.cosmos.network/v0.52/learn/advanced/02-context.md), which includes a `GasMeter` that tracks how much gas is used during the execution of `Tx`, is initialized at the beginning of `CheckTx`. The user-provided amount of gas for `Tx` is referred to as `GasWanted`. If `GasConsumed`, the amount of gas used during execution, exceeds `GasWanted`, the execution is halted and the changes made to the cached copy of the state are not committed. Otherwise, `CheckTx` sets `GasUsed` equal to `GasConsumed` and returns it in the result. After calculating the gas and fee values, validator-nodes ensure that the user-specified `gas-prices` exceed their locally defined `min-gas-prices`.

In the Cosmos SDK, after [decoding transactions](https://docs.cosmos.network/v0.52/learn/advanced/encoding), `CheckTx()` is implemented to do the following checks:

1.  Extract the `sdk.Msg`s from the transaction.
2.  **Optionally** perform *stateless* checks by calling `ValidateBasic()` on each of the `sdk.Msg`s. This is done first, as *stateless* checks are less computationally expensive than *stateful* checks. If `ValidateBasic()` fail, `CheckTx` returns before running *stateful* checks, which saves resources. This check is still performed for messages that have not yet migrated to the new message validation mechanism defined in [RFC 001](https://docs.cosmos.network/main/rfc/rfc-001-tx-validation) and still have a `ValidateBasic()` method.
3.  Perform non-module related *stateful* checks on the [account](https://docs.cosmos.network/v0.52/learn/beginner/accounts). This step is mainly about checking that the `sdk.Msg` signatures are valid, that enough fees are provided and that the sending account has enough funds to pay for said fees. Note that no precise [`gas`](https://docs.cosmos.network/v0.52/learn/beginner/gas-fees) counting occurs here, as `sdk.Msg`s are not processed. Usually, the [`AnteHandler`](https://docs.cosmos.network/v0.52/learn/beginner/gas-fees#antehandler) will check that the `gas` provided with the transaction is superior to a minimum reference gas amount based on the raw transaction size, in order to avoid spam with transactions that provide 0 gas.

`CheckTx` does **not** process `sdk.Msg`s - they only need to be processed when the canonical state needs to be updated, which happens during `FinalizeBlock`.

Steps 2. and 3. are performed by the [`AnteHandler`](https://docs.cosmos.network/v0.52/learn/beginner/gas-fees#antehandler) in the [`RunTx()`](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#runtx) function, which `CheckTx()` calls with the `runTxModeCheck` mode. During each step of `CheckTx()`, a special [volatile state](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#state-updates) called `checkState` is updated. This state is used to keep track of the temporary changes triggered by the `CheckTx()` calls of each transaction without modifying the [main canonical state](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#state-updates). For example, when a transaction goes through `CheckTx()`, the transaction's fees are deducted from the sender's account in `checkState`. If a second transaction is received from the same account before the first is processed, and the account has consumed all its funds in `checkState` during the first transaction, the second transaction will fail `CheckTx`() and be rejected. In any case, the sender's account will not actually pay the fees until the transaction is actually included in a block, because `checkState` never gets committed to the main state. The `checkState` is reset to the latest state of the main state each time a blocks gets [committed](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#commit).

`CheckTx` returns a response to the underlying consensus engine of type [`abci.CheckTxResponse`](https://docs.cometbft.com/v1.0/spec/abci/abci++_methods#checktx). The response contains:

-   `Code (uint32)`: Response Code. `0` if successful.
-   `Data ([]byte)`: Result bytes, if any.
-   `Log (string):` The output of the application's logger. May be non-deterministic.
-   `Info (string):` Additional information. May be non-deterministic.
-   `GasWanted (int64)`: Amount of gas requested for transaction. It is provided by users when they generate the transaction.
-   `GasUsed (int64)`: Amount of gas consumed by transaction. During `CheckTx`, this value is computed by multiplying the standard cost of a transaction byte by the size of the raw transaction. Next is an example:

x/auth/ante/basic.go
```
gasService := cgts.ak.GetEnvironment().GasService
if err := gasService.GasMeter(ctx).Consume(params.TxSizeCostPerByte*storetypes.Gas(len(tx.Bytes())),"txSize"); err !=nil{
return err
}

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.52.0-beta.2/x/auth/ante/basic.go#L141-L144)

-   `Events ([]cmn.KVPair)`: Key-Value tags for filtering and indexing transactions (eg. by account). See [`events`](https://docs.cosmos.network/v0.52/learn/advanced/events) for more.
-   `Codespace (string)`: Namespace for the Code.


#### RecheckTx[​](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#rechecktx "Direct link to RecheckTx")

After `Commit`, `CheckTx` is run again on all transactions that remain in the node's local mempool excluding the transactions that are included in the block. To prevent the mempool from rechecking all transactions every time a block is committed, the configuration option `mempool.recheck=false` can be set. As of Tendermint v0.32.1, an additional `Type` parameter is made available to the `CheckTx` function that indicates whether an incoming transaction is new (`CheckTxType_New`), or a recheck (`CheckTxType_Recheck`). This allows certain checks like signature verification can be skipped during `CheckTxType_Recheck`.