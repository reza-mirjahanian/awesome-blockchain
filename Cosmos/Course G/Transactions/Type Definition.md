```
// Tx defines an interface a transaction must fulfill.
Tx interface {
	transaction.Tx

	HasMsgs

	// GetReflectMessages gets a reflected version of the transaction's messages
	// that can be used by dynamic APIs. These messages should not be used for actual
	// processing as they cannot be guaranteed to be what handlers are expecting, but
	// they can be used for dynamically reading specific fields from the message such
	// as signers or validation data. Message processors should ALWAYS use the messages
	// returned by GetMsgs.
	GetReflectMessages() ([]protoreflect.Message, error)
}


```
It contains the following methods:

-   **GetMsgs:** unwraps the transaction and returns a list of contained `sdk.Msg`s - one transaction may have one or multiple messages, which are defined by module developers.
-   **ValidateBasic:** lightweight, [*stateless*](https://docs.cosmos.network/v0.52/learn/beginner/tx-lifecycle#types-of-checks) checks used by ABCI messages [`CheckTx`](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#checktx) and [`RunTx`](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#runtx) to make sure transactions are not invalid. For example, the [`auth`](https://github.com/cosmos/cosmos-sdk/tree/main/x/auth) module's `ValidateBasic` function checks that its transactions are signed by the correct number of signers and that the fees do not exceed the user's maximum. When [`runTx`](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#runtx) is checking a transaction created from the [`auth`](https://github.com/cosmos/cosmos-sdk/tree/main/x/auth/) module, it first runs `ValidateBasic` on each message, then runs the `auth` module AnteHandler which calls `ValidateBasic` for the transaction itself.
-   **Hash()**: returns the unique identifier for the Tx.
-   **GetMessages:** returns the list of `sdk.Msg`s contained in the transaction.
-   **GetSenders:** returns the addresses of the signers who signed the transaction.
-   **GetGasLimit:** returns the gas limit for the transaction. Returns `math.MaxUint64` for transactions with unlimited gas.
-   **Bytes:** returns the encoded bytes of the transaction. This is typically cached after the first decoding of the transaction.


### Signing Transactions[​](https://docs.cosmos.network/v0.52/learn/advanced/transactions#signing-transactions "Direct link to Signing Transactions")

Every message in a transaction must be signed by the addresses specified by its `GetSigners`. The Cosmos SDK currently allows signing transactions in two different ways.

#### `SIGN_MODE_DIRECT` (preferred)[​](https://docs.cosmos.network/v0.52/learn/advanced/transactions#sign_mode_direct-preferred "Direct link to sign_mode_direct-preferred")

The most used implementation of the `Tx` interface is the Protobuf `Tx` message, which is used in `SIGN_MODE_DIRECT`:

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

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.52.0-beta.2/proto/cosmos/tx/v1beta1/tx.proto#L15-L28)

Because Protobuf serialization is not deterministic, the Cosmos SDK uses an additional `TxRaw` type to denote the pinned bytes over which a transaction is signed. Any user can generate a valid `body` and `auth_info` for a transaction, and serialize these two messages using Protobuf. `TxRaw` then pins the user's exact binary representation of `body` and `auth_info`, called respectively `body_bytes` and `auth_info_bytes`. The document that is signed by all signers of the transaction is `SignDoc` (deterministically serialized using [ADR-027](https://docs.cosmos.network/v0.52/architecture/adr-027-deterministic-protobuf-serialization.md)):

proto/cosmos/tx/v1beta1/tx.proto
```
// SignDoc is the type used for generating sign bytes for SIGN_MODE_DIRECT.
messageSignDoc{
// body_bytes is protobuf serialization of a TxBody that matches the
// representation in TxRaw.
bytes body_bytes =1;
// auth_info_bytes is a protobuf serialization of an AuthInfo that matches the
// representation in TxRaw.
bytes auth_info_bytes =2;
// chain_id is the unique identifier of the chain this transaction targets.
// It prevents signed transactions from being used on another chain by an
// attacker
string chain_id =3;
// account_number is the account number of the account in state
uint64 account_number =4;
}

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.52.0-beta.2/proto/cosmos/tx/v1beta1/tx.proto#L50-L67)

Once signed by all signers, the `body_bytes`, `auth_info_bytes` and `signatures` are gathered into `TxRaw`, whose serialized bytes are broadcasted over the network.