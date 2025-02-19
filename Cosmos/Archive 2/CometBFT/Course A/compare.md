CometBFT vs. X
--------------

CometBFT is broadly similar to two classes of software. The first class consists of distributed key-value stores, like Zookeeper, etcd, and consul, which use non-BFT consensus. The second class is known as "blockchain technology", and consists of both cryptocurrencies like Bitcoin and Ethereum, and alternative distributed ledger designs like Hyperledger's Burrow.


### Zookeeper, etcd, consul

Zookeeper, etcd, and consul are all implementations of key-value stores atop a classical, non-BFT consensus algorithm. Zookeeper uses an algorithm called Zookeeper Atomic Broadcast, while etcd and consul use the Raft log replication algorithm. A typical cluster contains 3-5 machines, and can tolerate crash failures in less than 1/2 of the machines (e.g., 1 out of 3 or 2 out of 5), but even a single Byzantine fault can jeopardize the whole system.

Each offering provides a slightly different implementation of a featureful key-value store, but all are generally focused around providing basic services to distributed systems, such as dynamic configuration, service discovery, locking, leader-election, and so on.

CometBFT is in essence similar software, but with two key differences:

-   It is Byzantine Fault Tolerant, meaning it can only tolerate less than 1/3 of machines failing, but those failures can include arbitrary behavior - including hacking and malicious attacks.
-   It does not specify a particular application, like a fancy key-value store. Instead, it focuses on arbitrary state machine replication, so developers can build the application logic that's right for them, from key-value store to cryptocurrency to e-voting platform and beyond.


-----------------

### Bitcoin, Ethereum, etc

[Tendermint consensus algorithm](https://arxiv.org/abs/1807.04938), adopted by CometBFT, emerged in the tradition of cryptocurrencies like Bitcoin, Ethereum, etc. with the goal of providing a more efficient and secure consensus algorithm than Bitcoin's Proof of Work. In the early days, Tendermint consensus-based blockchains had a simple currency built in, and to participate in consensus, users had to "bond" units of the currency into a security deposit which could be revoked if they misbehaved -this is what made Tendermint consensus a Proof-of-Stake algorithm.

Since then, CometBFT has evolved to be a general purpose blockchain consensus engine that can host arbitrary application states. That means it can be used as a plug-and-play replacement for the consensus engines of other blockchain software. So one can take the current Ethereum code base, whether in Rust, or Go, or Haskell, and run it as an ABCI application using CometBFT. Indeed, [we did that with Ethereum](https://github.com/cosmos/ethermint). And we plan to do the same for Bitcoin, ZCash, and various other deterministic applications as well.

Another example of a cryptocurrency application built on CometBFT is [the Cosmos network](http://cosmos.network/).



### Other Blockchain Projects

[Fabric](https://github.com/hyperledger/fabric) takes a similar approach to CometBFT, but is more opinionated about how the state is managed, and requires that all application behavior runs in potentially many docker containers, modules it calls "chaincode". It uses an implementation of [PBFT](http://pmg.csail.mit.edu/papers/osdi99.pdf). from a team at IBM that is [augmented to handle potentially non-deterministic chaincode](https://drops.dagstuhl.de/opus/volltexte/2017/7093/pdf/LIPIcs-OPODIS-2016-24.pdf). It is possible to implement this docker-based behavior as an ABCI app in CometBFT, though extending CometBFT to handle non-determinism remains for future work.

[Burrow](https://github.com/hyperledger/burrow) is an implementation of the Ethereum Virtual Machine and Ethereum transaction mechanics, with additional features for a name-registry, permissions, and native contracts, and an alternative blockchain API. It uses CometBFT as its consensus engine, and provides a particular application state.
