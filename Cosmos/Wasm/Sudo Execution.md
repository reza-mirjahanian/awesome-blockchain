# Sudo Execution

One of the wonders of the Cosmos SDK is  [governance](https://docs.cosmos.network/v0.44/modules/gov/). Network participants can vote on proposals to decide the future of the network. Proposals can contain messages that will be executed based on the result of the voting.

We can define a smart contract entry point that can only be called by trusted native Cosmos modules. This entry point is  `sudo`. It can not be called by users or other smart contracts but only by Cosmos modules. This means that  `sudo`  is useful for more than just governance.