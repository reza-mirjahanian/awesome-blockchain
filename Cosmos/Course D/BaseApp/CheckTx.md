### `CheckTx` state updates

The `checkState`, which is based on the last committed state from the root store, is used for any reads and writes during `CheckTx`. Here, you only execute the `AnteHandler` and verify a service router exists for every message in the transaction.

Note that you branch the already branched `checkState` when you execute the `AnteHandler`. This has the side effect that if the `AnteHandler` fails, the state transitions will not be reflected in the `checkState`. `checkState` is only updated on success.