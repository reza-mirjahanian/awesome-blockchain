# What is OP Mainnet's path to decentralization?

Currently the Optimism Foundation is running the sole sequencer because we don't have the fault challenges running yet. However, there are already many verifiers watching us to make sure this power is not abused.  [We are working hard at decentralization](https://medium.com/ethereum-optimism/our-pragmatic-path-to-decentralization-cb5805ca43c1).

1.  Release Bedrock, enabling a multi-client architecture (done, June 6th 2023).
2.  Support (directly or indirectly) the creation of alternative Optimism clients (see  [op-erigon](https://github.com/testinprod-io/op-erigon)).
3.  Ship the multi-client proof contracts.
4.  Either renounce the power to upgrade the contracts further, or transfer it to  [the most trusted address in Ethereum](https://etherscan.io/address/0x0000000000000000000000000000000000000000).


# What did Bedrock change?

Here are the most visible changes (from the user and dapp developer perspectives):

-   The  [L1 security fee](https://help.optimism.io/hc/en-us/articles/4411895794715)  is lower. Bedrock will use a much more efficient compression algorithm, and avoid calling a CTC chain contract.
-   Blocks are produced every two seconds, regardless of the number of transactions. Some blocks will be empty, others filled to the block gas limit.
-   OP Mainnet uses the same  [EIP 1559 mechanism](https://github.com/ethereum/EIPs/blob/master/EIPS/eip-1559.md)  as L1 Ethereum (for the  [L2 execution fee](https://help.optimism.io/hc/en-us/articles/4411895794715)).
-   Rather than accepting or rejecting transactions immediately, the sequencer will store them in a  _private_  mempool, and use priority fees to order them.
-   Deposits (transactions from L1 Ethereum to OP Mainnet) will be processed by verifiers even when the sequencer is down, so the sequencer will be unable to delay transactions submitted on L1.

Note that this  **was not a regenesis**. The transaction history since November 11th, 2021, will stay available.