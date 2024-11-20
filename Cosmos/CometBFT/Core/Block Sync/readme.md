Block Sync
==========

*Formerly known as Fast Sync*

In a proof of work blockchain, syncing with the chain is the same process as staying up-to-date with the consensus: download blocks, and look for the one with the most total work. In proof-of-stake, the consensus process is more complex, as it involves rounds of communication between the nodes to determine what block should be committed next. Using this process to sync up with the blockchain from scratch can take a very long time. It's much faster to just download blocks and check the merkle tree of validators than to run the real-time consensus gossip protocol.

Using Block Sync
----------------

When starting from scratch, nodes will use the Block Sync mode. In this mode, the CometBFT daemon will sync hundreds of times faster than if it used the real-time consensus process. Once caught up, the daemon will switch out of Block Sync and into the normal consensus mode. After running for some time, the node is considered `caught up` if it has at least one peer and its height is at least as high as the max reported peer height. See [the IsCaughtUp method](https://github.com/cometbft/cometbft/blob/v0.38.x/blocksync/pool.go#L168).