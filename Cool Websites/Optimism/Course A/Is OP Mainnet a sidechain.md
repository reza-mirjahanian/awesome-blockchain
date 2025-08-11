
# What is a rollup?
A  [rollup](https://ethereum.org/en/developers/docs/scaling/layer-2-rollups)  is a mechanism that allows transactions to be executed on a separate blockchain while inheriting the availability and integrity guarantees of Ethereum itself (also called L1 Ethereum). Rollups achieve this goal by writing all transactions into L1 Ethereum as  [calldata](https://ethresear.ch/t/clarification-on-how-calldata-persists-on-the-blockchain-and-how-optimistic-rollups-use-it/8136), and then writing a hash of the transaction result (the state of their blockchain after the transaction).

Rollups use one of two mechanisms to convince the rest of the network that the result is correct:

1.  Optimistic rollups rely on verifiers that challenge incorrect results.
2.  Zero Knowledge rollups rely on mathematical proofs, which are difficult (computationally intensive) to create, but relatively easy to verify.


# Is OP Mainnet a sidechain?

No.  [Sidechains](https://ethereum.org/en/developers/docs/scaling/sidechains)  are their own blockchain systems with entirely separate consensus mechanisms. OP Mainnet is verified by a series of smart contracts on the Ethereum mainnet. Because all the necessary data is written to the blockchain, we inherit Ethereum's availability, and as long as there is at least one honest verifier, Ethereum's integrity.

Unfortunately. There is a tradeoff between security and cost, with Ethereum being very expensive and very secure, sidechains being a lot cheaper and usually a lot more centralized and therefore less secure, and **rollups sitting in the middle with security that is about as good as Ethereum's and costs that are somewhat higher than those of sidechains.** using Ethereum also means we have to pay  [some Ethereum gas](https://help.optimism.io/hc/en-us/articles/4411895794715), which causes our transactions to be more expensive than those on a sidechain.