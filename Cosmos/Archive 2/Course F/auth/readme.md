-   `auth` \- authentication of accounts and transactions for Cosmos SDK applications and is responsible for specifying the base transaction and account types.
-   `authz` \- authorization for accounts to perform actions on behalf of other accounts and enables a granter to grant authorizations to a grantee that allows the grantee to execute messages on behalf of the granter.


Fees are determined by the gas limits and gas prices transactions provide, where `fees = ceil(gasLimit * gasPrices)`. Txs incur gas costs for all state reads/writes, signature verification, as well as costs proportional to the tx size. Operators should set minimum gas prices when starting their nodes. They must set the unit costs of gas in each token denomination they wish to support:

`simd start ... --minimum-gas-prices=0.00001stake;0.05photinos`


CometBFT does not currently provide fee based mempool prioritization, and fee based mempool filtering is local to node and not part of consensus. But with minimum gas prices set, such a mechanism could be implemented by node operators.

Because the market value for tokens will fluctuate, validators are expected to dynamically adjust their minimum gas prices to a level that would encourage the use of the network.


#### [Account Interface ](https://docs.cosmos.network/v0.52/build/modules/auth#account-interface)


##### Base Account[​](https://docs.cosmos.network/v0.52/build/modules/auth#base-account "Direct link to Base Account")

A base account is the simplest and most common account type, which just stores all requisite fields directly in a struct.

```
// BaseAccount defines a base account type. It contains all the necessary fields
// for basic account functionality. Any custom account type should extend this
// type for additional functionality (e.g. vesting).
messageBaseAccount{
string address =1;
google.protobuf.Any pub_key =2;
uint64 account_number =3;
uint64 sequence       =4;
}
```


Vesting accounts are deprecated in favor of `x/accounts`. The creation of vesting account, using `x/auth/vesting`, is not possible since v0.52. For existing chains, importing the `x/auth/vesting module` is still required for backward compatibility purposes.



AnteHandlers[​](https://docs.cosmos.network/v0.52/build/modules/auth#antehandlers "Direct link to AnteHandlers")
----------------------------------------------------------------------------------------------------------------

The `x/auth` module presently has no transaction handlers of its own, but does expose the special `AnteHandler`, used for performing basic validity checks on a transaction, such that it could be thrown out of the mempool. The `AnteHandler` can be seen as a set of decorators that check transactions within the current context, per [ADR 010](https://github.com/cosmos/cosmos-sdk/blob/main/docs/architecture/adr-010-modular-antehandler.md).

Note that the `AnteHandler` is called on both `CheckTx` and `DeliverTx`, as CometBFT proposers presently have the ability to include in their proposed block transactions which fail `CheckTx`.


### Decorators[​](https://docs.cosmos.network/v0.52/build/modules/auth#decorators "Direct link to Decorators")

The auth module provides `AnteDecorator`s that are recursively chained together into a single `AnteHandler` in the following order:

-   `SetUpContextDecorator`: Sets the `GasMeter` in the `Context` and wraps the next `AnteHandler` with a defer clause to recover from any downstream `OutOfGas` panics in the `AnteHandler` chain to return an error with information on gas provided and gas used.

-   `RejectExtensionOptionsDecorator`: Rejects all extension options which can optionally be included in protobuf transactions.

-   `MempoolFeeDecorator`: Checks if the `tx` fee is above local mempool `minFee` parameter during `CheckTx`.

-   `ValidateBasicDecorator`: Calls `tx.ValidateBasic` and returns any non-nil error.

-   `TxTimeoutHeightDecorator`: Check for a `tx` height timeout.

-   `ValidateMemoDecorator`: Validates `tx` memo with application parameters and returns any non-nil error.

-   `ConsumeGasTxSizeDecorator`: Consumes gas proportional to the `tx` size based on application parameters.

-   `DeductFeeDecorator`: Deducts the `FeeAmount` from first signer of the `tx`. If the `x/feegrant` module is enabled and a fee granter is set, it deducts fees from the fee granter account.

-   `SetPubKeyDecorator`: Sets the pubkey from a `tx`'s signers that does not already have its corresponding pubkey saved in the state machine and in the current context.

-   `ValidateSigCountDecorator`: Validates the number of signatures in `tx` based on app-parameters.

-   `SigGasConsumeDecorator`: Consumes parameter-defined amount of gas for each signature. This requires pubkeys to be set in context for all signers as part of `SetPubKeyDecorator`.

-   `SigVerificationDecorator`: Verifies all signatures are valid. This requires pubkeys to be set in context for all signers as part of `SetPubKeyDecorator`.

-   `IncrementSequenceDecorator`: Increments the account sequence for each signer to prevent replay attacks


Keepers[​](https://docs.cosmos.network/v0.52/build/modules/auth#keepers "Direct link to Keepers")
-------------------------------------------------------------------------------------------------

The auth module only exposes one keeper, the account keeper, which can be used to read and write accounts.

```
// AccountKeeperI is the interface contract that x/auth's keeper implements.
type AccountKeeperI interface {
    // Return a new account with the next account number and the specified address. Does not save the new account to the store.
    NewAccountWithAddress(sdk.Context, sdk.AccAddress) types.AccountI

    // Return a new account with the next account number. Does not save the new account to the store.
    NewAccount(sdk.Context, types.AccountI) types.AccountI

    // Check if an account exists in the store.
    HasAccount(sdk.Context, sdk.AccAddress) bool

    // Retrieve an account from the store.
    GetAccount(sdk.Context, sdk.AccAddress) types.AccountI

    // Set an account in the store.
    SetAccount(sdk.Context, types.AccountI)

    // Remove an account from the store.
    RemoveAccount(sdk.Context, types.AccountI)

    // Iterate over all accounts, calling the provided function. Stop iteration when it returns true.
    IterateAccounts(sdk.Context, func(types.AccountI) bool)

    // Fetch the public key of an account at a specified address
    GetPubKey(sdk.Context, sdk.AccAddress) (crypto.PubKey, error)

    // Fetch the sequence of an account at a specified address.
    GetSequence(sdk.Context, sdk.AccAddress) (uint64, error)

    // Fetch the next account number, and increment the internal counter.
    NextAccountNumber(sdk.Context) uint64
}
```


The `accounts` command allow users to query all the available accounts.

```
simd query auth accounts [flags]
```

```bat
sign
The sign command allows users to sign transactions that was generated offline.

simd tx sign tx.json --from $ALICE > tx.signed.json
```
