### `BeginBlock` state updates

The `deliverState` is set for use in subsequent `DeliverTx` ABCI messages during `BeginBlock`. `deliverState` is based on the last committed state from the root store, and is branched.

Note the `deliverState` is set to nil on `Commit`.

### [#](https://ida.interchain.io/academy/2-cosmos-concepts/8-base-app.html#delivertx-state-updates)`DeliverTx` state updates

The state flow for `DeliverTx` is nearly identical to `CheckTx`, except state transitions occur on the `deliverState` and messages in a transaction are executed. Similarly to `CheckTx`, state transitions occur on a doubly branched state, `deliverState`. Successful message execution results in writes being committed to `deliverState`.

If message execution fails, state transitions from the `AnteHandler` are persisted.

### [#](https://ida.interchain.io/academy/2-cosmos-concepts/8-base-app.html#commit-state-updates)`Commit` state updates

All the state transitions that occurred in `deliverState` are finally written during `Commit` to the root `CommitMultiStore`, which in turn is committed to disk and results in a new application root hash. These state transitions are now considered final. The `checkState` is finally set to the newly committed state and `deliverState` is set to nil to be reset on `BeginBlock`.