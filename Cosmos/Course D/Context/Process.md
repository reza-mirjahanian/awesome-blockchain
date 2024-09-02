Prior to calling `runMsgs` on any messages in the transaction, `app.cacheTxContext()` is used to branch and cache the context and multistore:

-   For `runMsgCtx`, the context with the branched store is used in `runMsgs` to return a result.
-   If the process is running in `checkTxMode`, there is no need to write the changes. The result is returned immediately.
-   If the process is running in `deliverTxMode` and the result indicates a successful run over all the messages, the branched multistore is written back to the original.