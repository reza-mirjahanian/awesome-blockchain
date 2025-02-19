Transaction process from an end-user perspective
------------------------------------------------

**Decide** on the messages to put into the transaction. This is normally done with the assistance of a wallet or application and a user interface.

**Generate** the transaction using the Cosmos SDK's `TxBuilder`. `TxBuilder` is the preferred way to generate a transaction.


**Sign** the transaction. Transactions must be signed before a validator includes them in a block

**Broadcast** the signed transaction using one of the available interfaces so that it eventually reaches validators.


--------------


Transaction objects
-------------------

Transaction objects are Cosmos SDK types that implement the [`Tx` ](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/types/tx_msg.go#L39-L46)interface. They contain the following methods:

-   [`GetMsgs` ](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/types/tx_msg.go#L41): unwraps the transaction and returns a list of contained [`sdk.Msg` ](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/types/tx_msg.go#L11-L22). One transaction may have one or multiple messages.
-   [`ValidateBasic` ](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/types/tx_msg.go#L45): includes lightweight, stateless checks used by the ABCI messages' `CheckTx` and `DeliverTx` to make sure transactions are not invalid. For example, the [`Tx` ](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/types/tx/tx.pb.go#L32-L42)[`ValidateBasic` ](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/types/tx/types.go#L38)function checks that its transactions are signed by the correct number of signers and that the fees do not exceed the user's maximum.

![](https://ida.interchain.io/hi-tip-icon.svg)

This function is different from the [`ValidateBasic` ](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/types/tx_msg.go#L16)functions for `sdk.Msg`, which perform basic validity checks on messages only. For example, `runTX` first runs `ValidateBasic` on each message when it checks a transaction created from the auth module. Then it runs the auth module's [`AnteHandler` ](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/types/handler.go#L8), which calls `ValidateBasic` for the transaction itself.

You should rarely manipulate a `Tx` object directly. It is an intermediate type used for transaction generation. Developers usually use the [`TxBuilder` ](https://github.com/cosmos/cosmos-sdk/blob/v0.45.4/client/tx_config.go#L36-L46)interface.


