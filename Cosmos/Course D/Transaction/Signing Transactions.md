Every message in a transaction must be signed by the addresses specified by its [`GetSigners` ](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/types/tx_msg.go#L21). The Cosmos SDK currently allows signing transactions in two different ways:

-   [`SIGN_MODE_DIRECT` ](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/types/tx/signing/signing.pb.go#L36)(preferred): the most used implementation of the `Tx` interface is the [Protobuf `Tx` ](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/types/tx/tx.pb.go#L32-L42)message, which is used in `SIGN_MODE_DIRECT`. Once signed by all signers, the `BodyBytes`, `AuthInfoBytes`, and [`Signatures` ](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/types/tx/tx.pb.go#L113)are gathered into [`TxRaw` ](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/types/tx/tx.pb.go#L103-L114), whose [serialized bytes ](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/types/tx/tx.pb.go#L125-L136)are broadcast over the network.
-   [`SIGN_MODE_LEGACY_AMINO_JSON` ](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/types/tx/signing/signing.pb.go#L43): the legacy implementation of the `Tx` interface is the [`StdTx` ](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/x/auth/legacy/legacytx/stdtx.go#L77-L83)struct from `x/auth`. The document signed by all signers is [`StdSignDoc` ](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/x/auth/legacy/legacytx/stdsign.go#L42-L50), which is encoded into [bytes ](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/x/auth/legacy/legacytx/stdsign.go#L53-L78)using Amino JSON. Once all signatures are gathered into `StdTx`, `StdTx` is serialized using Amino JSON and these bytes are broadcast over the network. **This method is being deprecated**.

[#](https://ida.interchain.io/academy/2-cosmos-concepts/3-transactions.html#generating-transactions)Generating transactions
---------------------------------------------------------------------------------------------------------------------------

The `TxBuilder` interface contains metadata closely related to the generation of transactions. The end-user can freely set these parameters for the transaction to be generated:

-   [`Msgs` ](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/client/tx_config.go#L39): the array of messages included in the transaction.
-   [`GasLimit` ](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/client/tx_config.go#L43): an option chosen by the users for how to calculate the gas amount they are willing to spend.
-   [`Memo` ](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/client/tx_config.go#L41): a note or comment to send with the transaction.
-   [`FeeAmount` ](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/client/tx_config.go#L42): the maximum amount the user is willing to pay in fees.
-   [`TimeoutHeight` ](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/client/tx_config.go#L44): the block height until which the transaction is valid.
-   [`Signatures` ](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/client/tx_config.go#L40): the array of signatures from all signers of the transaction.

As there are currently two modes for signing transactions, there are also two implementations of `TxBuilder`. There is a wrapper for `SIGN_MODE_DIRECT` and the [`StdTxBuilder` ](https://github.com/cosmos/cosmos-sdk/blob/8fc9f76329dd2433d9b258a867500de419522619/x/auth/migrations/legacytx/stdtx_builder.go#L18-L21)for `SIGN_MODE_LEGACY_AMINO_JSON`. The two possibilities should normally be hidden because end-users should prefer the overarching [`TxConfig` ](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/client/tx_config.go#L24-L30)interface. `TxConfig` is an [app-wide ](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/client/context.go#L50)configuration for managing transactions accessible from the context. Most importantly, it holds the information about whether to sign each transaction with `SIGN_MODE_DIRECT` or `SIGN_MODE_LEGACY_AMINO_JSON`.

A new `TxBuilder` will be instantiated with the appropriate sign mode by calling `txBuilder := txConfig.NewTxBuilder()`. `TxConfig` will correctly encode the bytes either using `SIGN_MODE_DIRECT` or `SIGN_MODE_LEGACY_AMINO_JSON` once `TxBuilder` is correctly populated with the setters of the fields described previously.

This is a pseudo-code snippet of how to generate and encode a transaction using the `TxEncoder()` method:

CopytxBuilder := txConfig.NewTxBuilder() txBuilder.SetMsgs(...) // and other setters on txBuilder