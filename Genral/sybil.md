Now, very roughly, a consensus protocol in a blockchain can be broken down into two pieces.

*   `Chain Selection Algorithm`
    
*   `Sybil Resistance mechanism`
    

**Proof of Work/Sybil resistance mechanism**

That mining piece we're doing or the proof of work algorithm is what's known as a Sybil resistance mechanism, and this is what Ethereum and Bitcoin currently use. Proof of work is known as a Sybil resistance mechanism because it defines a way to figure out who is the block author: which node is going to be the node that did the work to find that mine and be the author of the block so all the other nodes can verify that it's accurate.

### **Sybil Resistance**

[](#sybil-resistance)

Sybil Resistance is a blockchain's ability to defend against users creating a large number of pseudo-anonymous identities to gain a disproportionately advantageous influence over said system, and in laymen's terms, it's a way for the blockchain to defend against somebody making fake nodes so that they can get more and more rewards.

There are two types of Sybil resistance that we're going to talk about here, namely POW (Proof of Work) and POS (Proof of Stake).

### **POW**

[](#pow)

In POW, this is Sybil resistant because a single node has to go through a very computationally expensive process called mining, which we demonstrated earlier to figure out the answer to the blockchain's riddle of finding that correct nonce.

In POW, this works because no matter how many pseudo-anonymous accounts you make, each one still has to undergo this very computationally expensive activity of finding the answer to the proof-of-work problem, which in our demonstration was finding a nonce with those first four zeros, but each blockchain might change the riddle or change the problem to be a little bit different.

**Nakamoto consensus**

Bitcoin and Ethereum both use a form of consensus called `Nakamoto Consensus`, which is a combination of POW and longest chain rule. The decentralized network decides that whichever blockchain has the longest chain or the most number of blocks on it is going to be the chain that they use. This makes a lot of sense because every additional block that a chain is behind is going to take more and more computation for it to come up. That's why we saw confirmations on our transactions.

**Block Confirmations**

The number of confirmations is the number of additional blocks added on after our transaction went through in a block. So if we see confirmation two, it means the block that our transactions were in has two blocks ahead of it in the longest chain. Now I do want to point out that a lot of people use POW as a consensus protocol, and I do want to say that this is a little bit inaccurate, but sometimes people use it interchangeably. In Bitcoin and Ethereum, POW is a piece of the overall consensus protocol, which in Bitcoin and Ethereum's current case is Nakamoto consensus.


**Sybil attack**

The Sybil Attack is when a user creates a whole bunch of pseudo-anonymous accounts to try to influence a network. Now, obviously on Bitcoin and Ethereum, this is really really difficult because the user needs to do all that work in POW or have a ton of collateral in proof of stake


**Sybil attack**

The Sybil Attack is when a user creates a whole bunch of pseudo-anonymous accounts to try to influence a network. Now, obviously on Bitcoin and Ethereum, this is really really difficult because the user needs to do all that work in POW or have a ton of collateral in proof of stake

### Longest Chain Rule

[](#longest-chain-rule)

Now you can see that blockchains are very democratic. Whichever blockchain has the most buy-in and is the longest is the blockchain that the whole system is going to corroborate. When nodes produce a new block and add it to the longest chain, the other nodes will follow the longest chain that the rest of the network is agreeing with and add those blocks to their chain and follow up. When a blockchain selects a block from a different longest chain, places it on, and then has to swap it out for another block to continue with the different blockchain, very small reorganizations are fairly common.

However, if a group of nodes had enough nodes or enough power, they could essentially be 51% of the network and influence the network in whatever direction they wanted. This is what is known as a 51% attack. This happened on blockchains like Ethereum Classic, which is not Ethereum. This is why the bigger a blockchain is, the more decentralized and secure it becomes.