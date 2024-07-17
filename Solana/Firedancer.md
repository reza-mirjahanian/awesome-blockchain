What is Firedancer, and why is it big for Solana?
=================================================

The beginner's guide to Firedancer
----------------------------------

Firedancer is a new third-party validator client software for the [Solana](https://www.kraken.com/prices/solana) blockchain that aims to improve the network's efficiency and transaction processing capabilities.

To interact with the Solana blockchain, or indeed any [blockchain](https://www.kraken.com/learn/what-is-blockchain-technology), people wishing to connect to the blockchain must run client software. 

In the blockchain space, individuals that run client software on their computers are referred to as nodes. Nodes can perform a range of important roles, such as helping to secure and maintain blockchain networks. Client software acts like a bridge between a node's computer and a blockchain.

Open-source public blockchains allow developers to create their own applications, or implementations, of this software to perform various functions.

[Cryptocurrency wallets](https://www.kraken.com/learn/custodial-non-custodial-crypto-wallet) are one type of implementation of client software. Crypto wallets allow users to transact cryptocurrency directly between each other over a blockchain network.

Validator clients are another type of client software implementation that manage nodes participating in the [crypto staking process](https://www.kraken.com/learn/what-is-crypto-staking) (proposing new blocks).



[Buy / Sell SOL](https://www.kraken.com/sign-up)

### Solana's current challenges

* * * *

To promote blockchain decentralization and remove any single points of failure, projects like [Ethereum (ETH)](https://www.kraken.com/prices/ethereum) encourage external developers to create their own client implementations in various programming languages. This means there are several different options node operators have when deciding which client they'd like to use.

There are three main advantages to this:

-   Each client has its own unique codebase, reducing the network's overall vulnerability to bugs and attacks.
-   Reduces transaction finality risk (prevents a single client from manipulating blockchain transactions).
-   Developers can create applications using whichever programming languages they're most comfortable with.

Due to the importance of client diversity, the [Ethereum protocol](https://www.kraken.com/learn/what-is-ethereum-eth) has introduced penalties such as [inactivity leaks](https://eth2book.info/capella/part2/incentives/inactivity/) to dissuade nodes from all running the same client software.

Right now, the Solana blockchain has only three validator clients in operation: 

-   Solana Labs client (written in Rust programming language).
-   Jito-Solana client (forked from Solana Labs client, also written in Rust programming language).
-   Sig client (written in Zig programming language).

This means Solana is not as resilient to attacks as its main competitor, which currently [boasts](https://clientdiversity.org/) around six different consensus clients and eight execution clients.

Additionally, Solana's native client does not support sharding --- a system that improves blockchain scalability by spreading the transaction processing load across multiple smaller shard chains simultaneously.

But, there is a new client on the horizon.

### What is Firedancer and why is it important?

* * * *

Firedancer represents a new high-performance validator client for Solana, written in C++ programming language.

Developed by Jump Crypto, a Web3 infrastructure company, Firedancer has been designed to significantly increase Solana's transaction processing capabilities and provide support for sharding. 

In a [live demo](https://www.youtube.com/watch?v=YF-7duYCK54) broadcast in 2022, Firedancer demonstrated an ability to process over 1 million transactions per second. This figure is many times greater than Solana's current theoretical limit of 50,000 tps, and far exceeds the amount of transactions leading service providers like [Visa](https://news.bitcoin.com/no-visa-doesnt-handle-24000-tps-and-neither-does-your-pet-blockchain/) typically process.

This upgraded performance will likely boost Solana's appeal as a leading Layer 1 blockchain, and allow the network to keep pace with other top competitors in the crypto market. 

The project was first [announced](https://jumpcrypto.com/writing/jump-crypto-sets-out-to-build-new-validator-client-for-the-solana-blockchain-to-increase-the-throughput-and-reliability-of-the-network/) in 2022, with the live testnet [launching](https://twitter.com/solana/status/1719310929593057720) in October 2023.

Beyond its performance advantages, Firedancer will provide Solana validator nodes with a fourth software option to choose from; further reducing Solana's vulnerability to bugs, code exploits and attacks.