The next step of consensus is to execute the transactions to fully validate them. All full-nodes that receive a block proposal from the correct proposer execute the transactions by calling the ABCI function `FinalizeBlock`. As mentioned throughout the documentation `BeginBlock`, `ExecuteTx` and `EndBlock` are called within FinalizeBlock. Although every full-node operates individually and locally, the outcome is always consistent and unequivocal. This is because the state changes brought about by the messages are predictable, and the transactions are specifically sequenced in the proposed block.

```
        --------------------------
        | Receive Block Proposal |
        --------------------------
                    |
                    v
        -------------------------
        |     FinalizeBlock      |
        -------------------------
                    |
                    v
            -------------------
            |   BeginBlock     | 
            -------------------
                    |
                    v
            --------------------
            | ExecuteTx(tx0)   |
            | ExecuteTx(tx1)   |
            | ExecuteTx(tx2)   |
            | ExecuteTx(tx3)   |
            |       .          |
            |       .          |
            |       .          |
            -------------------
                    |
                    v
            --------------------
            |    EndBlock      |
            --------------------
                    |
                    v
        -------------------------
        |       Consensus        |
        -------------------------
                    |
                    v
        -------------------------
        |         Commit         |
        -------------------------
```

### Transaction Execution

The `FinalizeBlock` ABCI function defined in [`BaseApp`](https://docs.cosmos.network/v0.50/learn/advanced/baseapp) does the bulk of the state transitions: it is run for each transaction in the block in sequential order as committed to during consensus. Under the hood, transaction execution is almost identical to `CheckTx` but calls the [`runTx`](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#runtx) function in deliver mode instead of check mode. Instead of using their `checkState`, full-nodes use `finalizeblock`:

-   **Decoding:** Since `FinalizeBlock` is an ABCI call, `Tx` is received in the encoded `[]byte` form. Nodes first unmarshal the transaction, using the [`TxConfig`](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#register-codec) defined in the app, then call `runTx` in `execModeFinalize`, which is very similar to `CheckTx` but also executes and writes state changes.

-   **Checks and `AnteHandler`:** Full-nodes call `validateBasicMsgs` and `AnteHandler` again. This second check happens because they may not have seen the same transactions during the addition to Mempool stage and a malicious proposer may have included invalid ones. One difference here is that the `AnteHandler` does not compare `gas-prices` to the node's `min-gas-prices` since that value is local to each node - differing values across nodes yield nondeterministic results.

-   **`MsgServiceRouter`:** After `CheckTx` exits, `FinalizeBlock` continues to run [`runMsgs`](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#runtx-antehandler-runmsgs-posthandler) to fully execute each `Msg` within the transaction. Since the transaction may have messages from different modules, `BaseApp` needs to know which module to find the appropriate handler. This is achieved using `BaseApp`'s `MsgServiceRouter` so that it can be processed by the module's Protobuf [`Msg` service](https://docs.cosmos.network/v0.50/build/building-modules/msg-services). For `LegacyMsg` routing, the `Route` function is called via the [module manager](https://docs.cosmos.network/v0.50/build/building-modules/module-manager) to retrieve the route name and find the legacy [`Handler`](https://docs.cosmos.network/v0.50/build/building-modules/msg-services#handler-type) within the module.

-   **`Msg` service:** Protobuf `Msg` service is responsible for executing each message in the `Tx` and causes state transitions to persist in `finalizeBlockState`.

-   **PostHandlers:** [`PostHandler`](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#posthandler)s run after the execution of the message. If they fail, the state change of `runMsgs`, as well of `PostHandlers`, are both reverted.

-   **Gas:** While a `Tx` is being delivered, a `GasMeter` is used to keep track of how much gas is being used; if execution completes, `GasUsed` is set and returned in the `abci.ExecTxResult`. If execution halts because `BlockGasMeter` or `GasMeter` has run out or something else goes wrong, a deferred function at the end appropriately errors or panics.

If there are any failed state changes resulting from a `Tx` being invalid or `GasMeter` running out, the transaction processing terminates and any state changes are reverted. Invalid transactions in a block proposal cause validator nodes to reject the block and vote for a `nil` block instead.

### Commit[​](https://docs.cosmos.network/v0.50/learn/beginner/tx-lifecycle#commit "Direct link to Commit")

The final step is for nodes to commit the block and state changes. Validator nodes perform the previous step of executing state transitions in order to validate the transactions, then sign the block to confirm it. Full nodes that are not validators do not participate in consensus - i.e. they cannot vote - but listen for votes to understand whether or not they should commit the state changes.

When they receive enough validator votes (2/3+ *precommits* weighted by voting power), full nodes commit to a new block to be added to the blockchain and finalize the state transitions in the application layer. A new state root is generated to serve as a merkle proof for the state transitions. Applications use the [`Commit`](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#commit) ABCI method inherited from [Baseapp](https://docs.cosmos.network/v0.50/learn/advanced/baseapp); it syncs all the state transitions by writing the `deliverState` into the application's internal state. As soon as the state changes are committed, `checkState` starts afresh from the most recently committed state and `deliverState` resets to `nil` in order to be consistent and reflect the changes.

Note that not all blocks have the same number of transactions and it is possible for consensus to result in a `nil` block or one with none at all. In a public blockchain network, it is also possible for validators to be **byzantine**, or malicious, which may prevent a `Tx` from being committed in the blockchain. Possible malicious behaviors include the proposer deciding to censor a `Tx` by excluding it from the block or a validator voting against the block.

At this point, the transaction lifecycle of a `Tx` is over: nodes have verified its validity, delivered it by executing its state changes, and committed those changes. The `Tx` itself, in `[]byte` form, is stored in a block and appended to the blockchain.