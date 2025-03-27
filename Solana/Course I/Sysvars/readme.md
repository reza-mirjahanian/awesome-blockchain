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