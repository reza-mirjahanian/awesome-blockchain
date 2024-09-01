For more details see the [Cosmos SDK documentation on errors when building modules ](https://docs.cosmos.network/v0.45/building-modules/errors.html).

### [#](https://ida.interchain.io/academy/2-cosmos-concepts/5-modules.html#registration)Registration

Modules should define and register their custom errors in `x/{module}/errors.go`. Registration of errors is handled via the `types/errors` package.

Each custom module error must provide the codespace, which is typically the module name (for example, "distribution") and is unique per module, and a `uint32` code. The codespace and code together provide a globally unique Cosmos SDK error.

The only restrictions on error codes are the following:

-   They must be greater than one, as a code value of one is reserved for internal errors.
-   They must be unique within the module.



The Cosmos SDK provides a core set of common errors. These errors are defined in [`types/errors/errors.go` ](https://github.com/cosmos/cosmos-sdk/blob/master/types/errors/errors.go).


### Wrapping

The custom module errors can be returned as their concrete type, as they already fulfill the error interface. Module errors can be wrapped to provide further context and meaning to failed executions.

Regardless of whether an error is wrapped or not, the Cosmos SDK's errors package provides an API to determine if an error is of a particular kind via `Is`.

### [#](https://ida.interchain.io/academy/2-cosmos-concepts/5-modules.html#abci)ABCI

If a module error is registered, the Cosmos SDK errors package allows ABCI information to be extracted through the `ABCIInfo` API. The package also provides `ResponseCheckTx` and `ResponseDeliverTx` as auxiliary APIs to automatically get `CheckTx` and `DeliverTx` responses from an error.