Processing L2 blocks
====================

In this document we discuss the differences between the Dragonfruit upgrade, which comes with the executor fork-ID 5, and the [Etrog upgrade](https://docs.polygon.technology/zkEVM/architecture/protocol/etrog-upgrade/) associated with fork-ID 6.

The key differences between the two Polygon zkEVM upgrades are mainly related to the definition of the L2 block and timestamps.

In the Dragonfruit upgrade,

-   An L2 block is defined to contain only one transaction, resulting in as many blocks per batch as there are transactions.

-   Timestamps are not assigned to blocks but to batches, which means each batch typically contains more than one block.

Since the timestamp is part of batch data instead of block data, it is shared among all the blocks within the batch.

Although the Dragonfruit approach minimizes delay, it has the following drawbacks:

-   It leads to a bloated database due to the large number of L2 blocks created.
-   It causes breaks in dApps that are configured with block-per-timestamp settings, as they rely on timestamps for proper timing of smart contract actions.

The Etrog upgrade addresses these two issues by allowing multiple transactions per block and assigning a unique timestamp to each block rather than to each batch.

It also introduces a timeout of a few seconds or milliseconds, during which the sequencer waits for transactions while creating a block.

To change the timestamp from one block to the next, the sequencer uses a special transaction as a new block marker, called `changeL2Block`.


## The 0x5ca1ab1e smart contract

The answer is: This information is included within the L2 state.

Specifically, the data is held in the storage slot 0 of an L2 system smart contract, which is deployed at the address `0x5ca1ab1e`.

After processing a transaction, the ROM writes the current block number into this specific storage location.

As depicted in the figure below, the L2 system smart contract deployed at address `0x5ca1ab1e` stores the number of the last processed block at slot 0.

Henceforth, during each batch processing, the system records all block numbers it contains.

![alt text](image-16.png)


