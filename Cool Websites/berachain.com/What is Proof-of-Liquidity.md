![alt text](image.png)

Proof-of-Liquidity (PoL) is a new way to secure a blockchain by increasing the amount of liquidity (funds) on the network.

It's similar to Proof-of-Stake (PoS), where users secure the chain by staking the native token. However, PoL adds a special governance token that:

-   Decides how much reward a staker can earn, based on delegation from others.
-   Is given to those who provide liquidity to the network through reward vaults.

![alt text](image-1.png)


Proof-of-Liquidity Overview 📓[​](https://docs.berachain.com/learn/pol/#proof-of-liquidity-overview-%F0%9F%93%93)
=================================================================================================================

One of the main [shortcomings](https://docs.berachain.com/learn/what-is-proof-of-liquidity#shortcomings-of-pos) of Proof-of-Stake is the lack of incentive for different ecosystem players to collaborate. Validators have little reason to interact with the protocols and end-users for whom they are ultimately running the infrastructure, yet they receive the majority of the economic incentives. On the other hand, projects launch on this infrastructure but retain the majority of the project tokens for themselves.

A healthier equilibrium can be achieved between projects, validators, and the chain when all actors share in the network's growth.

Proof-of-Liquidity (PoL) involves the participation and influence of all the chain's stakeholders. PoL requires different stakeholders to work in sync to maximize liquidity on the chain, ensuring they receive the greatest benefit.

The following sequence diagram represents the different steps that Proof-of-Liquidity (PoL) undergoes from the perspective of a validator and delegator.

![alt text](image-2.png)

1.  A *Prospective Validator* will provide an initial gas token bond (`$BERA`) to secure the network and gain eligibility to produce blocks. All active validators have an equal chance to be selected to propose a block.
2.  An *Active Validator* is chosen at random and proposes a new block.
3.  For proposing a new block, the chain allots the *Active Validator* with the governance token (`$BGT`) for distribution
4.  With the rewarded governance token, the *Active Validator* distributes to various *Reward Vaults*, decided by the validator in the Berachef contract (A list of addresses and distribution percentages to different Reward Vaults).
5.  A *Liquidity Provider* may perform a liquidity action like depositing a certain token with a BEX pool. Ex: Providing `$HONEY` and `$BERA` to a liquidity pool
6.  For providing liquidity, the *Liquidity Provider* receives a receipt token. Ex: `$HONEY-WBERA`.
7.  The *Liquidity Provider* stakes the receipt token with the *Reward Vault* making them eligible to receive `$BGT` based on their contribution.
8.  The `$BGT` that was distributed to the reward vault is now eligible to be claimed by the *Liquidity Provider*, making them a *BGT Holder*
9.  A BGT Holder can now delegate their `$BGT` to an *Active Validator*, making that validator a *Boosted Validator*, and increases the rewards a validator is alloted to distribute when they propose a block

Aligning Protocols and Validators 🤝[​](https://docs.berachain.com/learn/pol/#aligning-protocols-and-validators-%F0%9F%A4%9D)
-----------------------------------------------------------------------------------------------------------------------------

Because validators are given the responsibility of distributing governance tokens to Reward Vaults, when chosen to propose a block, it introduces a new dynamic where rewards are essentially shared with the ecosystem protocols.

Validators will share a stronger relationship with protocols, as their reward weight is determined by the governance tokens delegated to them, creating a symbiotic relationship.

Protocols can also convince Validators to start directing rewards to them by offering *Incentives* in exchange for the `$BGT` rewards directed to their specific *Reward Vaults*.

Existing Reward Vault Implementations 🐻[​](https://docs.berachain.com/learn/pol/#existing-reward-vault-implementations-%F0%9F%90%BB)
-------------------------------------------------------------------------------------------------------------------------------------

The following are implementations of existing Reward Vaults:

1.  [BEX](https://docs.berachain.com/learn/dapps/bex) \- Specific BEX Pools
2.  [Berps](https://docs.berachain.com/learn/dapps/berps) \- Depositing `$HONEY` into Berps Honey Vault
3.  [Bend](https://docs.berachain.com/learn/dapps/bend) \- Borrow `$HONEY`