Then, parameters used to define [volatile states](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#state-updates) (i.e. cached states):

-   `checkState`: This state is updated during [`CheckTx`](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#checktx), and reset on [`Commit`](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#commit).
-   `finalizeBlockState`: This state is updated during [`FinalizeBlock`](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#finalizeblock), and set to `nil` on [`Commit`](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#commit) and gets re-initialized on `FinalizeBlock`.
-   `processProposalState`: This state is updated during [`ProcessProposal`](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#process-proposal).
-   `prepareProposalState`: This state is updated during [`PrepareProposal`](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#prepare-proposal).