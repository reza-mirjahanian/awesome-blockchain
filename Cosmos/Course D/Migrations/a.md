Upgrading a live chain without software support for upgrades is risky, because all the validators need to pause their state machines at the same block height and apply the upgrade before resuming. If this is not done correctly, there can be state inconsistencies, which are hard to recover from.

The `Info` in a plan kicks off the `SideCar` process:

Copytype Plan struct { Name string Height int64 Info string }

### [#](https://ida.interchain.io/academy/2-cosmos-concepts/16-migrations.html#sidecar-process)`Sidecar` process

A `SideCar` is a binary which nodes can run to attend to processes outside of Cosmos binaries. This can include steps such as downloading and compiling software from a certain commit in a repo.

### [#](https://ida.interchain.io/academy/2-cosmos-concepts/16-migrations.html#upgradehandler)`UpgradeHandler`

An `UpgradeHandler` may be executed after the `SideCar` process is finished and the binary has been upgraded. It attends to on-chain activities that may be necessary before normal processing resumes. An upgrade handler may trigger a `StoreLoader`.

### [#](https://ida.interchain.io/academy/2-cosmos-concepts/16-migrations.html#storeloader)`StoreLoader`

A `StoreLoader` prepares the on-chain state for use by the new binary. This can include reorganizing existing data. The node does not resume normal operation until the store loader has returned and the handler has completed its work.

### [#](https://ida.interchain.io/academy/2-cosmos-concepts/16-migrations.html#proposal)Proposal

Governance uses proposals that are voted on and adopted or rejected. An upgrade proposal takes the form of accepting or rejecting a plan that is prepared and submitted through governance. Proposals can be withdrawn before execution with cancellation proposals.

[#](https://ida.interchain.io/academy/2-cosmos-concepts/16-migrations.html#advantages)Advantages
------------------------------------------------------------------------------------------------

Coordinated upgrades attend to the challenging process of upgrading blockchain applications and blockchain platforms.

The main advantages of this form of coordinated upgrades are:

-   **Avoidance of forks:** all validators move together at a pre-determined block height.
-   **Smooth upgrade of binaries:** the new software is adopted in an automated fashion.
-   **Reorganizing data stores:** data at rest can be reorganized as needed by processes that are not limited by factors such as a block gas limit.

[#](https://ida.interchain.io/academy/2-cosmos-concepts/16-migrations.html#effect-of-upgrades)Effect of upgrades
----------------------------------------------------------------------------------------------------------------

Blockchains are paused at the block height of an adopted plan. This initiates the upgrade process. The upgrade process itself may include switching to a new binary that is relatively small to download and install, or it may include an extensive data reorganization process. The validator stops processing blocks until it completes the process in either case.

The validator resumes processing blocks when the handler is satisfied with the completeness degree of the upgrade. From a user perspective, this appears as a pause which resumes with the new version.

[#](https://ida.interchain.io/academy/2-cosmos-concepts/16-migrations.html#application-specific)Application-specific
--------------------------------------------------------------------------------------------------------------------

The `SideCar`, handler, and store loader are application-specific. At each block, the Cosmos SDK checks for a plan that should be executed before processing block transactions. If none exists, then processing continues as usual. If a plan is scheduled to run, then the Cosmos SDK pauses normal processing and loads the `SideCar`. When the SideCar is finished, it loads the handler and optionally the store loader.

Application developers build implementations of those components that are tailored to their application and use case.


