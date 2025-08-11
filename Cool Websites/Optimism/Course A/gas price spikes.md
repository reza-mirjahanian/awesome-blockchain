# What happens if the L1 gas price spikes while a transaction is in process?

For an L2 transaction the normal process is:

1.  The wallet figures the cost of the transaction
2.  The user submits the transaction
3.  The sequencer processes the transaction
    -   Process the transaction
    -   [Deduct the gas cost](https://help.optimism.io/hc/en-us/articles/4411895794715), based on the L1 and L2 gas prices at that time
4.  The transaction is written to L1

Changes after the transaction is processed on the sequencer do not affect the cost the user pays. After step 3 the transaction is Optimism's responsibility. If the L1 gas price spikes, Optimism pays the new cost. Also, which the cost of L1 gas could increase between steps 1 and 3, it is only updated every five minutes and it does not change by more than 25%. So at most the user will pay 25% more than expected.

For an L2 transaction the normal process is:

1.  The wallet figures the cost of the transaction
2.  The user submits the transaction
3.  The sequencer processes the transaction
    -   Process the transaction
    -   [Deduct the gas cost](https://help.optimism.io/hc/en-us/articles/4411895794715), based on the L1 and L2 gas prices at that time
4.  The transaction is written to L1

Changes after the transaction is processed on the sequencer do not affect the cost the user pays. After step 3 the transaction is Optimism's responsibility. If the L1 gas price spikes, Optimism pays the new cost. Also, which the cost of L1 gas could increase between steps 1 and 3, it is only updated every five minutes and it does not change by more than 25%. So at most the user will pay 25% more than expected.


# Why not have more transaction per batch?
There are multiple reasons:

1.  A transaction is only 100% guaranteed, regardless of what any sequencer will do in the future, once it is written to L1. So batch frequency is a trade off between speed and cost. This is not a major consideration yet, but it will be once we decentralize.
2.  Clients drop transactions above a certain size from the mempool.
3.  Transactions have to fit in the gas limits for the block. The maximum gas limit is post EIP-1559 is 30,000,000. The average calldata byte costs 15.95 gas (4 gas if the byte is zero, 16 otherwise). So the theoretical maximum size is about 1.8 MB.