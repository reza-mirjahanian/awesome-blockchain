# What is the difference between EVM equivalence and Ethereum equivalence

-   **EVM equivalent**: The rollup is nearly identical to L1 Ethereum from the  _dapp developer perspective_. OP Mainnet has been EVM equivalent since the OVM 2.0 upgrade on November 11th, 2021.
-   **Ethereum equivalent**: The rollup is nearly identical to L1 Ethereum from the  _protocol developer perspective_. With the Bedrock upgrade (June 6th, 2023),  [the difference is very small](https://op-geth.optimism.io/), mostly dealing with communication between layers. The execution-layer functionality is nearly identical. The consensus layer has to be very different in a rollup, but we are isolating that functionality into a component called op-node.


# Where are tokens stored during the fault proof window?

The tokens are held by the bridge on the L1 Ethereum network from the time they are bridged over to OP Mainnet, until they are claimed from the bridge after the fault proof window (seven days at present) passes.

So tokens that are “in limbo” during withdrawal are on the L1 Ethereum network. Their OP Mainnet equivalents have already been burned, but they can’t be claimed yet because of the fault challenge window.

# Is transaction front running possible on OP Mainnet?
Right now  [front running](https://medium.com/degate/an-analysis-of-ethereum-front-running-and-its-defense-solutions-34ef81ba8456)  is  _difficult_. There is a transaction pool, but it is  _private_  so it is hard to know what transaction to front run.

Once we decentralize the sequencer, whoever runs the sequencer would be able to determine the order of transactions, so some front running might be possible. Also, we may make the transaction pool public in the future.

