Introduction to `Gas` and `Fees`[​](https://docs.cosmos.network/v0.50/learn/beginner/gas-fees#introduction-to-gas-and-fees "Direct link to introduction-to-gas-and-fees")
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------

In the Cosmos SDK, `gas` is a special unit that is used to track the consumption of resources during execution. `gas` is typically consumed whenever read and writes are made to the store, but it can also be consumed if expensive computation needs to be done. It serves two main purposes:

-   Make sure blocks are not consuming too many resources and are finalized. This is implemented by default in the Cosmos SDK via the [block gas meter](https://docs.cosmos.network/v0.50/learn/beginner/gas-fees#block-gas-meter).
-   Prevent spam and abuse from end-user. To this end, `gas` consumed during [`message`](https://docs.cosmos.network/v0.50/build/building-modules/messages-and-queries#messages) execution is typically priced, resulting in a `fee` (`fees = gas * gas-prices`). `fees` generally have to be paid by the sender of the `message`. Note that the Cosmos SDK does not enforce `gas` pricing by default, as there may be other ways to prevent spam (e.g. bandwidth schemes). Still, most applications implement `fee` mechanisms to prevent spam by using the [`AnteHandler`](https://docs.cosmos.network/v0.50/learn/beginner/gas-fees#antehandler).

Gas Meter[​](https://docs.cosmos.network/v0.50/learn/beginner/gas-fees#gas-meter "Direct link to Gas Meter")
------------------------------------------------------------------------------------------------------------

In the Cosmos SDK, `gas` is a simple alias for `uint64`, and is managed by an object called a *gas meter*. Gas meters implement the `GasMeter` interface

store/types/gas.go
```
// GasMeter interface to track gas consumption
type GasMeter interface{
GasConsumed() Gas
GasConsumedToLimit() Gas
GasRemaining() Gas
Limit() Gas
ConsumeGas(amount Gas, descriptor string)
RefundGas(amount Gas, descriptor string)
IsPastLimit()bool
IsOutOfGas()bool
String()string
}

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/store/types/gas.go#L40-L51)

where:

-   `GasConsumed()` returns the amount of gas that was consumed by the gas meter instance.
-   `GasConsumedToLimit()` returns the amount of gas that was consumed by gas meter instance, or the limit if it is reached.
-   `GasRemaining()` returns the gas left in the GasMeter.
-   `Limit()` returns the limit of the gas meter instance. `0` if the gas meter is infinite.
-   `ConsumeGas(amount Gas, descriptor string)` consumes the amount of `gas` provided. If the `gas` overflows, it panics with the `descriptor` message. If the gas meter is not infinite, it panics if `gas` consumed goes above the limit.
-   `RefundGas()` deducts the given amount from the gas consumed. This functionality enables refunding gas to the transaction or block gas pools so that EVM-compatible chains can fully support the go-ethereum StateDB interface.
-   `IsPastLimit()` returns `true` if the amount of gas consumed by the gas meter instance is strictly above the limit, `false` otherwise.
-   `IsOutOfGas()` returns `true` if the amount of gas consumed by the gas meter instance is above or equal to the limit, `false` otherwise.

The gas meter is generally held in [`ctx`](https://docs.cosmos.network/v0.50/learn/advanced/context), and consuming gas is done with the following pattern:

```
ctx.GasMeter().ConsumeGas(amount,"description")

```

By default, the Cosmos SDK makes use of two different gas meters, the [main gas meter](https://docs.cosmos.network/v0.50/learn/beginner/gas-fees#main-gas-metter) and the [block gas meter](https://docs.cosmos.network/v0.50/learn/beginner/gas-fees#block-gas-meter).

### Main Gas Meter[​](https://docs.cosmos.network/v0.50/learn/beginner/gas-fees#main-gas-meter "Direct link to Main Gas Meter")

`ctx.GasMeter()` is the main gas meter of the application. The main gas meter is initialized in `FinalizeBlock` via `setFinalizeBlockState`, and then tracks gas consumption during execution sequences that lead to state-transitions, i.e. those originally triggered by [`FinalizeBlock`](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#finalizeblock). At the beginning of each transaction execution, the main gas meter **must be set to 0** in the [`AnteHandler`](https://docs.cosmos.network/v0.50/learn/beginner/gas-fees#antehandler), so that it can track gas consumption per-transaction.

Gas consumption can be done manually, generally by the module developer in the [`BeginBlocker`, `EndBlocker`](https://docs.cosmos.network/v0.50/build/building-modules/beginblock-endblock) or [`Msg` service](https://docs.cosmos.network/v0.50/build/building-modules/msg-services), but most of the time it is done automatically whenever there is a read or write to the store. This automatic gas consumption logic is implemented in a special store called [`GasKv`](https://docs.cosmos.network/v0.50/learn/advanced/store#gaskv-store).

### Block Gas Meter[​](https://docs.cosmos.network/v0.50/learn/beginner/gas-fees#block-gas-meter "Direct link to Block Gas Meter")

`ctx.BlockGasMeter()` is the gas meter used to track gas consumption per block and make sure it does not go above a certain limit.

During the genesis phase, gas consumption is unlimited to accommodate initialisation transactions.

```
app.finalizeBlockState.SetContext(app.finalizeBlockState.Context().WithBlockGasMeter(storetypes.NewInfiniteGasMeter()))

```

Following the genesis block, the block gas meter is set to a finite value by the SDK. This transition is facilitated by the consensus engine (e.g., CometBFT) calling the `RequestFinalizeBlock` function, which in turn triggers the SDK's `FinalizeBlock` method. Within `FinalizeBlock`, `internalFinalizeBlock` is executed, performing necessary state updates and function executions. The block gas meter, initialised each with a finite limit, is then incorporated into the context for transaction execution, ensuring gas consumption does not exceed the block's gas limit and is reset at the end of each block.

Modules within the Cosmos SDK can consume block gas at any point during their execution by utilising the `ctx`. This gas consumption primarily occurs during state read/write operations and transaction processing. The block gas meter, accessible via `ctx.BlockGasMeter()`, monitors the total gas usage within a block, enforcing the gas limit to prevent excessive computation. This ensures that gas limits are adhered to on a per-block basis, starting from the first block post-genesis.

```
gasMeter := app.getBlockGasMeter(app.finalizeBlockState.Context())
app.finalizeBlockState.SetContext(app.finalizeBlockState.Context().WithBlockGasMeter(gasMeter))

```

This above shows the general mechanism for setting the block gas meter with a finite limit based on the block's consensus parameters.

AnteHandler[​](https://docs.cosmos.network/v0.50/learn/beginner/gas-fees#antehandler "Direct link to AnteHandler")
------------------------------------------------------------------------------------------------------------------

The `AnteHandler` is run for every transaction during `CheckTx` and `FinalizeBlock`, before a Protobuf `Msg` service method for each `sdk.Msg` in the transaction.

The anteHandler is not implemented in the core Cosmos SDK but in a module. That said, most applications today use the default implementation defined in the [`auth` module](https://github.com/cosmos/cosmos-sdk/tree/main/x/auth). Here is what the `anteHandler` is intended to do in a normal Cosmos SDK application:

-   Verify that the transactions are of the correct type. Transaction types are defined in the module that implements the `anteHandler`, and they follow the transaction interface:

types/tx\_msg.go
```
Tx interface{
	HasMsgs
// GetMsgsV2 gets the transaction's messages as google.golang.org/protobuf/proto.Message's.
GetMsgsV2()([]protov2.Message,error)
}

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/types/tx_msg.go#L51-L56)

This enables developers to play with various types for the transaction of their application. In the default `auth` module, the default transaction type is `Tx`:

proto/cosmos/tx/v1beta1/tx.proto
```
// Tx is the standard type used for broadcasting transactions.
messageTx{
// body is the processable content of the transaction
TxBody body =1;
// auth_info is the authorization related content of the transaction,
// specifically signers, signer modes and fee
AuthInfo auth_info =2;
// signatures is a list of signatures that matches the length and order of
// AuthInfo's signer_infos to allow connecting signature meta information like
// public key and signing mode by position.
repeatedbytes signatures =3;
}

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/proto/cosmos/tx/v1beta1/tx.proto#L14-L27)

-   Verify signatures for each [`message`](https://docs.cosmos.network/v0.50/build/building-modules/messages-and-queries#messages) contained in the transaction. Each `message` should be signed by one or multiple sender(s), and these signatures must be verified in the `anteHandler`.
-   During `CheckTx`, verify that the gas prices provided with the transaction is greater than the local `min-gas-prices` (as a reminder, gas-prices can be deducted from the following equation: `fees = gas * gas-prices`). `min-gas-prices` is a parameter local to each full-node and used during `CheckTx` to discard transactions that do not provide a minimum amount of fees. This ensures that the mempool cannot be spammed with garbage transactions.
-   Verify that the sender of the transaction has enough funds to cover for the `fees`. When the end-user generates a transaction, they must indicate 2 of the 3 following parameters (the third one being implicit): `fees`, `gas` and `gas-prices`. This signals how much they are willing to pay for nodes to execute their transaction. The provided `gas` value is stored in a parameter called `GasWanted` for later use.
-   Set `newCtx.GasMeter` to 0, with a limit of `GasWanted`. **This step is crucial**, as it not only makes sure the transaction cannot consume infinite gas, but also that `ctx.GasMeter` is reset in-between each transaction (`ctx` is set to `newCtx` after `anteHandler` is run, and the `anteHandler` is run each time a transactions executes).

As explained above, the `anteHandler` returns a maximum limit of `gas` the transaction can consume during execution called `GasWanted`. The actual amount consumed in the end is denominated `GasUsed`, and we must therefore have `GasUsed =< GasWanted`. Both `GasWanted` and `GasUsed` are relayed to the underlying consensus engine when [`FinalizeBlock`](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#finalizeblock) returns.