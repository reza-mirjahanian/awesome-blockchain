

## Scalability Issues with Ethereum
- Ethereum is great because it is decentralized, secure, and has a large community of developers building on top of it.
- However, Ethereum also has the problem of not being built to be scalable, i.e., it cannot handle thousands of transactions per second.
- This is due to the blockchain dilemma, where a single blockchain cannot achieve scalability, decentralization, and security at the same time.
- Decentralization means the chain can run without any trust dependencies on a small group of centralized actors.
- Security means the network can sustain attacks from a large proportion of the nodes.
- Scalability means the blockchain can process more transactions than a single node.
- Ethereum and other traditional blockchains prioritize decentralization and security, and thus suffer from scalability issues.

## The Rollup Solution
- About two and a half years ago, Ethereum proposed a solution to scale the network through Rollup chains.
- Rollups can process more transactions on Layer 2, while having efficient settlement and finalization on Layer 1, without sacrificing security.

## How Rollups Work
- There are two chains: Layer 1 (Ethereum) and Layer 2 (Rollup chain).
- Layer 2 is a less decentralized blockchain that can process transactions at a faster pace.
- Periodically, Layer 2 will batch all the transactions and send sufficient data to Layer 1 with some proof to finalize the transactions.
- The requirement for a Rollup is that users can recover the latest state on Layer 2 purely by relying on the data stored on Layer 1.

## Types of Rollups
1. **Optimistic Rollups**:
   - Optimistic Rollups will just submit the transaction data to Layer 1 and wait for a period of time for challenges.
   - If no one challenges the validation within the batch for a certain period, the batch will be considered finalized and cannot be reverted.
   - The drawback is that the waiting period is quite long, around 7 days, to ensure the finalization on Layer 1.

2. **ZK-Rollups**:
   - ZK-Rollups will not only submit the data to Layer 1 but also accompany it with a ZK-proof.
   - Once the ZK-proof is verified by a smart contract on Ethereum, the batch of transactions can be finalized quickly.
   - Building a ZK-Rollup system is non-trivial for several reasons:
     - Generating a proof for computations requires writing the program logic in the form of an arithmetic circuit, which is complicated.
     - There is overhead in handling the generation of the linkage proof for the ZK-Rollups.
     - Different programs and applications may have different circuits, leading to a lack of composability.

## ZK-EVM: A General-Purpose ZK-Rollup
- To solve the composability issue, Scroll is building a general-purpose ZK-Rollup called ZK-EVM.
- ZK-EVM puts the same virtual machine (EVM) into a ZK-circuit, allowing any application built on Solidity and the Ethereum ecosystem to run directly on the Layer 2 ZK-Rollup.
- This provides composability and developer-friendliness, as developers can use the same toolchains they use on Layer 1.
- However, building the ZK-EVM is challenging due to the incompatibility between the EVM and the finite field used in ZK-proofs, as well as the complexity of handling special operations like bitwise operations and ECDSA signature verification.

## Advancements Enabling ZK-EVM
- Recent advancements in ZK-proof systems, such as polynomial commitments, custom gates, lookup arguments, and recursive proofs, have made building the ZK-EVM more practically viable.
- Increased hardware acceleration, like GPUs and future FPGA or ASIC accelerators, can further reduce the cost of generating Z