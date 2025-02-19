Parameters that define volatile states, cached states, include:

-   **checkState:** this state is updated during `CheckTx` and resets on `Commit`.
-   **deliverState:** this state is updated during `DeliverTx` and set to nil on `Commit`. It gets re-initialized on `BeginBlock`.