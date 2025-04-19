What are Byzantine vaults?
==========================

The Byzantine protocol gives access to a wide variety of restaking opportunities. It's adapters link into a large set of staking operators as well as restaking protocols and AVS/network operators.

The Byzantine vault (ByzVault) is the standardised access point into this ecosystem. It is a self-contained, non-custodial smart contract that connects all these opportunities together and allows the implementation of a user's custom restaking strategy. Depending on the chosen strategy, vaults can be exposed to multiple restaking protocols at a time and have a specific portfolio of AVSs/networks.

Every ByzVault can have a custom strategy. ByzVaults are fully fund-segregated and risk-isolated, meaning a slashing event in one (**on the Ethereum beacon chain or the restaking layer**) has no impact on other vaults.

ByzVault strategies can be built in an entirely flexible way. For example, users can define them according to specific risk-profiles, yield expectations, or AVS/network types (ZK services, oracles, intents, data availabilities, interoperability layers, ...).

All Byzantine vaults are [ERC4626](https://eips.ethereum.org/EIPS/eip-4626) **compatible**, except for **asynchronous withdrawals**, which are executed in 2 steps spaced apart by a maximum 14 days. This is a necessary protection against instant withdrawals after a malicious action from an Operator.

When depositing into a vault, the user receives vaultshares representing their ownership in the vault. Through the vaultshares mechanism, deposited assets can always be withdrawn and are never under the control/custody of the curator, vault owner, or even the staking or restaking operator.

Vaultshares are non-rebasing ERC20 tokens. As rewards accrue in native restaking vaults, the value of the vaultshares increases (they represent the same share of ownership in a growing pot).

In the case of ERC20 ByzVaults, i.e. ByzVaults in which the collateral asset is an ERC20 token and there are thus only restaking rewards, but no staking rewards, the value accrual of AVS/network reward tokens occurs off-chain