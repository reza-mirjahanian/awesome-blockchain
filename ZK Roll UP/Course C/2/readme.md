[](https://github.com/0xPolygon/polygon-docs/edit/main/docs/zkEVM/architecture/high-level/smart-contracts/overview.md "Edit this page")

## Polygon smart contract architecture[¶](https://docs.polygon.technology/zkEVM/architecture/high-level/smart-contracts/overview/#polygon-smart-contract-architecture "Link to this section")

Chain stacks at the node level direct transaction data to the L2 and L1 networks via smart contract calls. The system stores state in binary tree structures containing verifiable local and global exit roots.

The diagram below details the Polygon Solidity smart contract architecture minus the bridging contracts.

![Polygon Solidity smart contract architecture](Overview%20-%20Polygon%20Knowledge%20Layer/smart-contracts-full-view.png)

There are four key contract types built into the system design:

-   The [consensus contract set](https://docs.polygon.technology/zkEVM/architecture/high-level/smart-contracts/overview/#consensus-contracts).
-   The [rollup manager](https://docs.polygon.technology/zkEVM/architecture/high-level/smart-contracts/overview/#rollup-manager).
-   The [bridge contract](https://docs.polygon.technology/zkEVM/architecture/high-level/smart-contracts/overview/#bridge).
-   [Exit root management contracts](https://docs.polygon.technology/zkEVM/architecture/high-level/smart-contracts/overview/#global-exit-roots).

## Consensus contracts[¶](https://docs.polygon.technology/zkEVM/architecture/high-level/smart-contracts/overview/#consensus-contracts "Link to this section")

In the Ethereum realm, the set of consensus contracts and the functions they expose fuel the sequencing and verification mechanisms triggered by stack components, such as the sequencer and aggregator, at the node level.

![Polygon Solidity smart contract consensus contract set](Overview%20-%20Polygon%20Knowledge%20Layer/consensus-contracts.png)

These contracts define the type of chain, i.e. validium or non-validium, and there is usually a single contract per CDK chain that supplies custom functionality.

## Rollup manager[¶](https://docs.polygon.technology/zkEVM/architecture/high-level/smart-contracts/overview/#rollup-manager "Link to this section")

The [PolygonRollupManager.sol](https://github.com/0xPolygonHermez/zkevm-contracts/blob/main/contracts/v2/PolygonRollupManager.sol) contract is responsible for creating, updating, and verifying CDK rollup and validium chains.

![Polygon Solidity smart contract rollup manager](Overview%20-%20Polygon%20Knowledge%20Layer/rollup-manager.png)

## Bridge[¶](https://docs.polygon.technology/zkEVM/architecture/high-level/smart-contracts/overview/#bridge "Link to this section")

The class diagram below describes the unified bridge interactions.

![Polygon Solidity smart contract bridging architecture](Overview%20-%20Polygon%20Knowledge%20Layer/bridging-class-diagram.png)

The unified bridge contract [PolygonZkEVMBridgeV2.sol](https://github.com/0xPolygonHermez/zkevm-contracts/blob/main/contracts/v2/PolygonZkEVMBridgeV2.sol) is responsible for bridging and claiming activity across L1 and L2 chains.

![Polygon Solidity smart contract bridge](Overview%20-%20Polygon%20Knowledge%20Layer/bridge.png)

In the L1 network, the bridge also manages the complex exit root mechanism governing system state. In the L2 network, there is a lighter exit root mechanism that governs state at this layer.

## Global exit roots[¶](https://docs.polygon.technology/zkEVM/architecture/high-level/smart-contracts/overview/#global-exit-roots "Link to this section")

The class diagram below describes the exit root interactions.

![Polygon Solidity smart contract bridging architecture](Overview%20-%20Polygon%20Knowledge%20Layer/exit-root-class-diagram.png)

System state as a whole is stored on binary trees with data and/or exit roots written into their leaves.

Local updates at the leaf-level trigger exit root updates at the top of the trees which are then available to the global exit root trees and the consensus contracts via the L1 bridge contract.

The [PolygonZkEVMGlobalExitRootV2.sol](https://github.com/0xPolygonHermez/zkevm-contracts/blob/main/contracts/v2/PolygonZkEVMGlobalExitRootV2.sol) contract manages the exit roots across multiple networks at the Ethereum L1 level.

![Polygon Solidity smart contract exit root L1](Overview%20-%20Polygon%20Knowledge%20Layer/l1-exit-root.png)

The L2 exit root management contract, [PolygonZkEVMGlobalExitRootL2.sol](https://github.com/0xPolygon/cdk-validium-contracts/blob/main/contracts/PolygonZkEVMGlobalExitRootL2.sol), has a lighter-weight exit root mechanism.

![Polygon Solidity smart contract exit root L2](Overview%20-%20Polygon%20Knowledge%20Layer/l2-exit-root.png)

## Validium stacks[¶](https://docs.polygon.technology/zkEVM/architecture/high-level/smart-contracts/overview/#validium-stacks "Link to this section")

CDK validium stacks use the [cdk-validium-contracts](https://github.com/0xPolygon/cdk-validium-contracts/tree/main) which has slightly adjusted behavior to take account of validium components and CDK custom requirements.

The CDK repo is a fork of the zkEVM main contracts repo and all contracts, therefore, extend from common interfaces.

Important

-   A CDK validium stack starts life as a rollup stack.
-   It may interchangeably be referred to as such when discussing aspects shared by the two options.