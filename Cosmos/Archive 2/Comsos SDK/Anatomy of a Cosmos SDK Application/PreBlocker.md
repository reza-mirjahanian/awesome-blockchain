There are two semantics around the new lifecycle method:

-   It runs before the `BeginBlocker` of all modules
-   It can modify consensus parameters in storage, and signal the caller through the return value.

When it returns `ConsensusParamsChanged=true`, the caller must refresh the consensus parameter in the finalize context:

```
ap.ctx = app.finalizeBlockState.ctx.WithConsensusParams(app.GetConsensusParams())

```

The new ctx must be passed to all the other lifecycle methods.