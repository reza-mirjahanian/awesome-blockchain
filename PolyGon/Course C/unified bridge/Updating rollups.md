
### Updating a rollup

It is often necessary to enable upgradeability of rollups.

More specifically, a user with appropriate rights can change the consensus implementation and the type of a certain rollup. Such a user can therefore modify the sequencing or verification procedures of a rollup.

In order to change the consensus, the function  UpdateRollup()  needs to change the transparent proxy implementation.

In the upgrading procedure, the  rollupCompatibilityID  comes into play:

-   In order to avoid errors, we can only upgrade to a rollup type having the same compatibility identifier as the original one.

If this is not the case, the transaction is reverted, raising the  UpdateNotCompatible  error.

### Adding existing rollups

Rollups that are already deployed and working, do not follow any rollup type.

Such an existing rollup can be added to the  RollupManager  via the  addExistingRollup()  function, by specifying its current address.

When the verifier implements the  IVerifierRollup  interface, it requests only for the raw consensus contract address, as it will not be used directly but through a proxy to allow upgradeability options.

As mentioned before, rollups that are deployed and already in operation can be added to the  RollupManager  in order to allow unified management.

In this case, the  [addExistingRollup()](https://github.com/0xPolygonHermez/zkevm-contracts/blob/8fc03b0e83cbb143fdc6c1ecfaafa5c294c25509/contracts/v2/PolygonRollupManager.sol#L640C14-L640C31)  function is called.

Since the rollup has previously been initialized, the following information needs to be provided:

-   The consensus contract, implementing the  [IPolygonRollupBase](https://github.com/0xPolygonHermez/zkevm-contracts/blob/develop/contracts/v2/interfaces/IPolygonRollupBase.sol)  interface.
-   The verifier contract, implementing the  [IVerifierRollup](https://github.com/0xPolygonHermez/zkevm-contracts/blob/develop/contracts/interfaces/IVerifierRollup.sol)  interface.
-   The  forkID  of the existent rollup.
-   The  chainID  of the existent rollup.
-   The genesis block of the rollup.
-   The  rollupCompatibilityID.

Observe that most of these parameters were actually provided by the  RollupType, but  RollupData  of already existing rollups is constructed by hand, since they do not follow any rollup type as yet.