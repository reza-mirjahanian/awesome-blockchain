# NEAR Protocol

Here are the crucial terms associated with NEAR Protocol, along with brief, easy-to-understand definitions:

1. NEAR Protocol
   - A blockchain platform designed for scalability and user-friendliness

2. Sharding
   - A technique to split the blockchain into smaller parts for improved performance

3. Nightshade
   - NEAR's specific sharding implementation

4. NEAR token
   - The native cryptocurrency of the NEAR ecosystem

5. Proof of Stake (PoS)
   - The consensus mechanism used by NEAR to validate transactions

6. Validators
   - Participants who secure the network by staking NEAR tokens

7. Smart contracts
   - Self-executing programs stored on the blockchain

8. Rust
   - The primary programming language used for developing on NEAR

9. WebAssembly (WASM)
   - A binary instruction format used to run smart contracts on NEAR

10. Aurora
    - An Ethereum Virtual Machine (EVM) built on NEAR Protocol

11. Rainbow Bridge
    - A tool for transferring assets between NEAR and Ethereum

12. NEAR Wallet
    - A user-friendly interface for managing NEAR accounts and assets

13. Human-readable accounts
    - NEAR's system of using names instead of cryptographic addresses for accounts

14. Gas fees
    - The cost of computing power required to process transactions on NEAR

15. Epochal finality
    - NEAR's approach to transaction finality, occurring at regular intervals



--------
NEAR Protocol is a software that aims to incentivize a network of computers to operate a platform for developers to create and launch decentralized applications.

Central to NEAR Protocol’s design is the concept of sharding, a process that aims to split the network’s infrastructure into several segments in order for computers, also known as nodes, to only have to handle a fraction of the network’s transactions.

By distributing segments of the blockchain, rather than the complete blockchain across network participants, sharding is expected to create a more efficient way to retrieve network data and scale the platform.

NEAR operates in a similar manner to other centralized data storage systems like Amazon Web Services (AWS) that serve as the base layer on which applications are built. But rather than being run by a single entity, NEAR is operated and maintained by a distributed network of computers.

Just as AWS allows developers to deploy code in the cloud without needing to create their own infrastructure, NEAR Protocol facilitates a similar architecture built around a network of computers and its native cryptocurrency, the NEAR token.

------------

### 1. **Blockchain**
- **Definition**: A distributed ledger technology where transactions are recorded across many computers so that the record cannot be altered retroactively without the alteration of all subsequent blocks and the consensus of the network.

### 2. **Smart Contracts**
- **Definition**: Self-executing contracts with the terms of the agreement directly written into code, which automatically enforce and execute the terms of the contract when certain conditions are met.

### 3. **Sharding**
- **Definition**: A method of splitting a blockchain into smaller, more manageable pieces called shards, which allows the network to process many transactions simultaneously, thereby increasing its scalability.

### 4. **Proof of Stake (PoS)**
- **Definition**: A consensus mechanism where validators are chosen to create new blocks and validate transactions based on the amount of cryptocurrency they hold and are willing to "stake" as collateral.

### 5. **Validator**
- **Definition**: A participant in the network that is responsible for validating transactions and creating new blocks in the blockchain. Validators are chosen based on their stake in the network.

### 6. **Delegation**
- **Definition**: The process by which token holders assign their tokens to a validator, thereby allowing the validator to increase its stake and chances of being chosen to validate transactions. Delegators earn rewards based on the validator's performance.

### 7. **NEAR Token**
- **Definition**: The native cryptocurrency of the NEAR Protocol, used to pay for transaction fees, storage, and staking within the network.

### 8. **Epoch**
- **Definition**: A period during which the network's state remains unchanged, after which it undergoes a reconfiguration, such as reassigning validators and recalculating rewards.

### 9. **Finality**
- **Definition**: The guarantee that a transaction has been permanently recorded on the blockchain and cannot be reversed or altered.

### 10. **Aurora**
- **Definition**: A project built on the NEAR Protocol that provides a bridge for Ethereum applications to run on NEAR, offering developers the ability to utilize NEAR's scalability and low fees while maintaining compatibility with Ethereum.

### 11. **Rainbow Bridge**
- **Definition**: A bridge that allows for the transfer of assets and data between the NEAR Protocol and Ethereum, enabling interoperability between the two blockchain networks.

### 12. **Developer-Friendly**
- **Definition**: Refers to the ease with which developers can build and deploy applications on a blockchain platform. NEAR Protocol is designed to be user-friendly for developers, with tools and documentation to support application development.

### 13. **Economically Sustainable**
- **Definition**: Ensuring that the network remains viable and efficient over the long term, with mechanisms in place to balance rewards for validators and the costs of operating the network.

### 14. **Community-Driven Governance**
- **Definition**: A system where the community of token holders and network participants have a say in the decision-making processes of the network, such as proposing and voting on protocol upgrades and changes.
------------
## How Does NEAR Protocol Work?

----------

NEAR Protocol is a  [Proof of Stake (PoS)] blockchain that aims to compete with other platforms thanks to its sharding solution, which it calls ‘Nightshade.’

### Nightshade

Sharding is a blockchain architecture that allows each participating node in the blockchain to only store a small subset of the platform’s data. Sharding should allow the blockchain to scale more efficiently, while enabling a greater amount of transactions per second and lower transaction fees.

  
Nightshade allows NEAR Protocol to maintain a single chain of data, while distributing the computing required to maintain this data into “chunks.” These chunks are handled by nodes, who process the data and add the information to the main chain.

One of the main benefits of Nightshade is that its architecture allows for fewer potential points of failure when it comes to security, as participating nodes are only responsible for maintaining smaller sections of the chain.

### Rainbow Bridge

NEAR Protocol includes an application called the Rainbow Bridge that allows participants to easily transfer  [Ethereum] tokens back and forth between Ethereum and NEAR.

In order to move tokens from Ethereum to NEAR Protocol, a user would first deposit tokens in an Ethereum smart contract. These tokens are then locked, and new tokens would be created on NEAR’s platform representing the original ones.

Since the original funds are held in storage through the smart contract, the process can be reversed when the user wishes to retrieve their original tokens.

### Aurora

Aurora is a Layer 2 scaling solution built on NEAR Protocol intended for developers to launch their Ethereum decentralized applications on NEAR’s network.

Aurora is built using Ethereum’s coding technology, the Ethereum Virtual Machine (EVM), as well as a cross-chain bridge which enables developers to link their Ethereum smart contracts and assets seamlessly.

Developers can use Aurora to gain the low fee and high throughput advantages of NEAR Protocol, with the familiarity and network of applications of Ethereum.