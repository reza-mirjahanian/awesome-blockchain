sysvars are read-only system accounts that give Solana programs access to the blockchain state as well as network information. They are similar to Ethereum global variables, which also enable smart contracts to access network or blockchain state information, but they have unique public addresses like the Ethereum precompiles.

In Anchor programs, you can access sysvars in two ways: either by using the anchor's get method wrapper, or by treating it as an account in your `#[Derive(Accounts)]`, using its public address.

Not all sysvars support the `get` method, and some are deprecated (information on deprecation will be specified in this guide). For those sysvars that don't have a `get` method, we will access them using their public address.

-   **Clock:** Used for time-related operations like getting the current time or slot number.
-   **EpochSchedule:** Contains information about epoch scheduling, including the epoch for a particular slot.
-   **Rent:** Contains the rental rate and information like the minimum balance requirements to keep an account rent exempt.
-   **Fees:** Contains the fee calculator for the current slot. The fee calculator provides information on how many lamports are paid per signature in a Solana transaction.
-   **EpochRewards:** The EpochRewards sysvar holds a record of epoch rewards distribution in Solana, including block rewards and staking rewards.
-   **RecentBlockhashes:** Contains the active recent block hashes.
-   **SlotHashes:** Contains history of recent slot hashes.
-   **SlotHistory:** Holds an array of slots available during the most recent epoch in Solana, and it is updated every time a new slot is processed.
-   **StakeHistory:** maintains a record of stake activations and deactivations for the entire network on a per-epoch basis, which is updated at the beginning of each epoch.
-   **Instructions:** To get access to the serialized instructions that are part of the current transaction.
-   **LastRestartSlot:** Contains the slot number of the last restart (the last time Solana restarted ) or zero if none ever happened. If the Solana blockchain were to crash and restart, an application can use this information to determine if it should wait until things stabilize.

Differentiating between Solana slots and blocks.
------------------------------------------------

A slot is a window of time (about 400ms) where a designated leader can produce a block. A slot contains a block (the same kind of block on Ethereum, i.e a list of transactions). However, a slot might not contain a block if the block leader failed to produce a block during that slot. Their relationship is illustrated below:

![alt text](image.png)

Although every block maps to exactly one slot, the block hash is not the same as the slot hash. This distinction is evident when clicking on a slot number in an explorer, it opens up the details of a block with a different hash.

Let's take an example from the image below from the Solana block explorer:

![alt text](image-1.png)
![alt text](image-2.png)

The highlighted green number in the image is the slot number **237240962**, and the highlighted yellow text is the slot hash **DYFtWxEdLbos9E6SjZQCMq8z242Yv2bVoj6dzwskd5vZ**. The block hash highlighted in red below is **FzHwFHDAXJBc55rpjShznGCBnC7DsTCjxf3KKAk6hk9T**.

We can distinguish between a block and a slot by their unique hashes, even though they have the same numbers.

As a test, click on any slot number in the explorer [here](https://explorer.solana.com/address/SysvarS1otHashes111111111111111111111111111/slot-hashes?cluster=testnet) and you will notice that a block page will open. This block will have a different hash from the slot hash.


Solana Sysvars in Anchor, using the get method
-----------------------------------------------

As mentioned earlier, not all sysvars can be accessed using Anchor's `get` method. Sysvars such as Clock, EpochSchedule, and Rent can be accessed using this method.

While the Solana documentation includes Fees and EpochRewards as sysvars that can be accessed with the `get` method, these are deprecated in the latest version of Anchor. Therefore, they cannot be called using the `get` method in Anchor.