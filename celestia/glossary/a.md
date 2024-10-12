
##   Composability
    
    The ability for applications on a blockchain to read and write state to each other.
    
##    Consensus
    
    A function of modular blockchains where the ordering of transactions is agreed upon by a set of validators.



## Consensus algorithm

	A consensus algorithm is a set of rules that blockchains use to determine how nodes produce new blocks and agree to finalize them.


DA stands for Data Availability.

DAC stands for Data Availability Committee.

DAS stands for Data Availability Sampling.


## Data availability sampling
A technique in which nodes can verify that data is available for a block without having to download the entire block, formerly known as data availability proofs.


## Data availability
The condition of whether or not transaction data was made available for nodes to download, when a block was proposed.

## Data availability committee
A data availability committee (DAC) is a permissioned group of nodes responsible for providing data availability to a blockchain.


## Data availability layer
A blockchain that provides data availability for other types of chains, like rollups.

## Data withholding attack
A type of attack that occurs when a block producer proposes a new block but does not share the underlying transaction data that was used to create the block.


## Dispute resolution
The handling and resolution of disputes, commonly used as a term in relation to optimistic rollups and their fraud proof mechanism.


## Execution
A function of modular blockchains where transactions are executed and the new state of the chain is determined.

## Execution environment
The virtual environment within a blockchain where transactions are processed, and accounts and smart contracts live.


## Execution layer
A type of modular blockchain whose primary job is hosting smart contracts and executing transactions.


## Fork choice rule
An algorithm that nodes use to correctly identify and follow the canonical chain.

## ISR stands for Intermediate State Root.


## Light client
A type of node that only downloads and verifies block headers, relying on an honest majority assumption that the state of the chain indicated by the block header is valid.


## Light node
In Celestia, a light node is a type of node that verifies block headers and does data availability sampling.

## Liveness
Liveness is a property of blockchains where validators produce new blocks and successfully finalize transactions.


## Modular blockchain
A type of blockchain that specializes in one or two tasks, rather than all of them.


## Namespaced Merkle Tree
A Namespaced Merkle Tree (NMT) is a type of binary Merkle tree where each node in the tree is tagged by the minimum and maximum namespace of their children.


## Off-chain data availability
Off-chain data availability occurs when an L2 publishes its transaction data somewhere separate from the L1 it settles on.


## On-chain data availability
On-chain data availability occurs when an L2 publishes its transaction data to its designated L1.


## Optimistic rollup
A type of rollup that posts its blocks to a separate chain without any cryptographic proofs that attest to their validity.


## Safety
Safety is a property of blockchains that a chain will not fork.

## Scalability
Scalability is the ability of a blockchain to increase its capacity without an equal increase in the cost to run a node that verifies the chain.


## Settlement
A function of modular blockchains where transaction proofs from rollups are verified and disputes are resolved.

## Settlement layer
A modular blockchain whose primary role is to provide proof verification and dispute resolution for rollups.


## Sequencer
A sequencer is a type of rollup node that is responsible for collecting transactions and producing new blocks.


## Sharding
The process of separating a blockchain from a single chain into multiple chains (shards).


## Shared security
Security that a blockchain inherits from an external source.


## Sovereign blockchain
A blockchain that has independent control over itself and its applications via social consensus.Social consensus
The process by which individuals come to an agreement on a change that will be made to a blockchain.


## Sovereign rollup
A type of rollup that does not use a settlement layer to determine its canonical chain and validity rules.

## State transition fraud proof
A method for proving that a state transition is invalid.


## Synchrony assumption
An assumption that the network is synchronous such that when a message is sent it will be received within a certain amount of time.