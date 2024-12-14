The `BaseApp` maintains **four** primary volatile states and a root or main state. The main state is the canonical state of the application and the volatile states, `checkState`, `prepareProposalState`, `processProposalState` and `finalizeBlockState` are used to handle state transitions in-between the main state made during [`Commit`](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#commit).

Internally, there is only a single `CommitMultiStore` which we refer to as the main or root state. From this root state, we derive four volatile states by using a mechanism called *store branching* (performed by `CacheWrap` function). The types can be illustrated as follows:

![alt text](image.png)


### InitChain State Updates[​](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#initchain-state-updates "Direct link to InitChain State Updates")

During `InitChain`, the four volatile states, `checkState`, `prepareProposalState`, `processProposalState` and `finalizeBlockState` are set by branching the root `CommitMultiStore`. Any subsequent reads and writes happen on branched versions of the `CommitMultiStore`. To avoid unnecessary roundtrip to the main state, all reads to the branched store are cached.

![alt text](image-1.png)


### CheckTx State Updates[​](https://docs.cosmos.network/v0.52/learn/advanced/baseapp#checktx-state-updates "Direct link to CheckTx State Updates")

During `CheckTx`, the `checkState`, which is based on the last committed state from the root store, is used for any reads and writes. Here we only execute the `AnteHandler` and verify a service router exists for every message in the transaction. Note, when we execute the `AnteHandler`, we branch the already branched `checkState`. This has the side effect that if the `AnteHandler` fails, the state transitions won't be reflected in the `checkState` \-- i.e. `checkState` is only updated on success.


![alt text](image-2.png)