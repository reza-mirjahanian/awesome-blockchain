### `InitChain` state updates

The two volatile states `checkState` and `deliverState` are set by branching the root `CommitMultiStore` during `InitChain`. Any subsequent reads and writes happen on branched versions of the `CommitMultiStore`. All reads to the branched store are cached to avoid unnecessary roundtrips to the main state.