The `PrepareProposal` function is part of the new methods introduced in Application Blockchain Interface (ABCI++) in CometBFT and is an important part of the application's overall governance system. In the Cosmos SDK, it allows the application to have more fine-grained control over the transactions that are processed, and ensures that only valid transactions are committed to the blockchain.

Here is how the `PrepareProposal` function can be implemented:

1.  Extract the `sdk.Msg`s from the transaction.
2.  Perform *stateful* checks by calling `Validate()` on each of the `sdk.Msg`'s. This is done after *stateless* checks as *stateful* checks are more computationally expensive. If `Validate()` fails, `PrepareProposal` returns before running further checks, which saves resources.
3.  Perform any additional checks that are specific to the application, such as checking account balances, or ensuring that certain conditions are met before a transaction is proposed. They are processed by the consensus engine, if necessary.
4.  Return the updated transactions to be processed by the consensus engine

Note that, unlike `CheckTx()`, `PrepareProposal` process `sdk.Msg`s, so it can directly update the state. However, unlike `FinalizeBlock()`, it does not commit the state updates. It's important to exercise caution when using `PrepareProposal` as incorrect coding could affect the overall liveness of the network.

It's important to note that `PrepareProposal` complements the `ProcessProposal` method which is executed after this method. The combination of these two methods means that it is possible to guarantee that no invalid transactions are ever committed. Furthermore, such a setup can give rise to other interesting use cases such as Oracles, threshold decryption and more.

`PrepareProposal` returns a response to the underlying consensus engine of type [`abci.PrepareProposalResponse`](https://docs.cometbft.com/v1.0/spec/abci/abci++_methods#prepareproposal). The response contains:

-   `Txs ([][]byte)`: List of transactions which will form a block