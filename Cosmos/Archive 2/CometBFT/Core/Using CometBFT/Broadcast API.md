Broadcast API
-------------

Earlier, we used the `broadcast_tx_commit` endpoint to send a transaction. When a transaction is sent to a CometBFT node, it will run via `CheckTx` against the application. If it passes `CheckTx`, it will be included in the mempool, broadcasted to other peers, and eventually included in a block.

Since there are multiple phases to processing a transaction, we offer multiple endpoints to broadcast a transaction:

```
/broadcast_tx_async
/broadcast_tx_sync
/broadcast_tx_commit

```

These correspond to no-processing, processing through the mempool, and processing through a block, respectively. That is, `broadcast_tx_async`, will return right away without waiting to hear if the transaction is even valid, while `broadcast_tx_sync` will return with the result of running the transaction through `CheckTx`. Using `broadcast_tx_commit` will wait until the transaction is committed in a block or until some timeout is reached, but will return right away if the transaction does not pass `CheckTx`. The return value for `broadcast_tx_commit` includes two fields, `check_tx` and `deliver_tx`, pertaining to the result of running the transaction through those ABCI messages.

The benefit of using `broadcast_tx_commit` is that the request returns after the transaction is committed (i.e. included in a block), but that can take on the order of a second. For a quick result, use `broadcast_tx_sync`, but the transaction will not be committed until later, and by that point its effect on the state may change.

Note the mempool does not provide strong guarantees - just because a tx passed CheckTx (ie. was accepted into the mempool), doesn't mean it will be committed, as nodes with the tx in their mempool may crash before they get to propose. For more information, see the [mempool write-ahead-log](https://docs.cometbft.com/v0.38/core/running-in-production#mempool-wal)