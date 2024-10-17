https://celestia.org/learn/sovereign-rollups/misconceptions//

#### Summary

-   Sovereign rollups publish transactions to another blockchain for ordering and data availability, leaving its nodes to determine the correct chain.
-   The main difference between a smart contract rollup and sovereign rollup is that transactions are verified by a smart contract on Ethereum and the sovereign rollup nodes, respectively.
-   Contrary to popular belief, sovereign rollups do inherit multiple aspects of security and they can have a bridge to the DA layer.
-   

![alt text](image-3.png)

As their name implies, smart contract rollups rely on a set of smart contracts on a settlement layer, like Ethereum, to verify its blocks. The smart contracts on the settlement layer become the source of truth for determining the correct rollup chain



#### What is a sovereign rollup?

A sovereign rollup is a type of blockchain that publishes its transactions to another blockchain, typically for ordering and [data availability](https://celestia.org/glossary/data-availability/), but handles its own [settlement](https://celestia.org/learn/modular-settlement-layers/settlement-in-the-modular-stack/).

In the context of the modular stack, sovereign rollups are responsible for execution and settlement, while the DA layer handles consensus and data availability.

![alt text](image-4.png)

Uniquely, DA layers don’t verify whether sovereign rollup transactions are correct. Nodes verifying the sovereign rollup are responsible for verifying whether new transactions are correct.

![alt text](image-5.png)

Sovereign rollups typically do not have a native trust-minimized bridge with the DA layer. Although sovereign rollups can have a general bridge with the DA layer, it just isn’t used for settling the chain.


#### The differences

We now have an idea of what sovereign rollups are and how they work. Now, how are they different from smart contract rollups?

The most notable difference between the two is where transactions are verified as correct.

-   **Smart contract rollup**: transactions are verified by smart contracts on the settlement layer.
-   **Sovereign rollup**: transactions are verified by nodes of the sovereign rollup.

##### Upgrades and sovereignty

With a smart contract rollup, upgrades depend on the smart contracts on the settlement layer. Upgrading the rollup requires changing the smart contracts. A multisig may be needed to control who can initiate updates to the smart contracts. While it is common for teams to control the upgrade multisig, it may be possible to make the multisig controllable via governance. Since the smart contracts live on the settlement layer, they are also bound by the [social consensus](https://celestia.org/glossary/social-consensus/) of the settlement layer.

Sovereign rollups upgrade through [forks](https://celestia.org/glossary/fork/) like a layer 1 blockchain. New software versions are published, and nodes can choose to update their software to the latest version. If nodes disagree with the upgrade, they can stay on the old software. Providing a choice lets the community, those that run nodes, decide whether they agree with the new changes. They can't be forced into accepting upgrades, even if most nodes upgrade. This feature, compared to smart contract rollups, is what makes sovereign rollups 'sovereign'.

The nature of upgrading an smart contract rollup poses a difficult set of tradeoffs. With a sovereign rollup, upgrades through forks are standard and allow users (nodes) to opt in or out of upgrades.

##### Bridges

The native bridges of smart contract rollups have [trust-minimized security](https://celestia.org/glossary/trust-minimized-bridge/) because the settlement layer verifies the entire block. While the native smart contract rollup bridge has ideal trust assumptions in theory, they may still require upgradeability through a multisig. As long as the multisig exists, the trust assumptions for the bridge are weaker.

As for sovereign rollups, they can optionally have a trust-minimized bridge to the DA layer, but it does not determine the correct chain for the sovereign rollup. If the DA layer lacks execution, like Celestia, then a bridge to the DA layer would require [stronger trust assumptions](https://celestia.org/glossary/honest-majority-assumption/) for security.

For DA layers that don't have smart contracts, bridges with sovereign rollups would mainly facilitate the movement of the DA layer token. This would let the DA layer token get used in applications (e.g. DeFi & NFTs) and moved throughout the rest of the blockchain ecosystem.


----------------

#### Summary

-   Sovereign rollups inherit multiple aspects of security from the DA layer, such as [liveness](https://celestia.org/glossary/liveness/), [safety](https://celestia.org/glossary/safety/), re-org resistance, and censorship resistance.
-   Sovereign rollups can have a bridge to the DA layer. The design space for bridging between sovereign rollups is wide.
-   Sovereign rollups don't need a settlement layer because they [do their own settlement](https://celestia.org/learn/sovereign-rollups/an-introduction/#what-is-a-sovereign-rollup).
-   



