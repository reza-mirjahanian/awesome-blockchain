Subscribing to events
----------------------

You can use Tendermint's [WebSocket (opens new window)](https://docs.tendermint.com/v0.34/tendermint-core/subscription.html)to subscribe to events by calling the `subscribe` RPC method.

The main `eventCategories` you can subscribe to are:

-   **`NewBlock`:** contains events triggered during `BeginBlock` and `EndBlock`.
-   **`Tx`:** contains events triggered during `DeliverTx`, the transaction processing.
-   **`ValidatorSetUpdates`:** contains updates about the set of validators for the block.

![](https://ida.interchain.io/hi-tip-icon.svg)

You can find a full list of event categories in the [Tendermint Go documentation (opens new window)](https://godoc.org/github.com/tendermint/tendermint/types#pkg-constants).

You can filter for event types and attribute values. For example, a transfer transaction triggers an event of type `Transfer` and has `Recipient` and `Sender` as attributes, as defined in the `events.go` file of the `bank` module