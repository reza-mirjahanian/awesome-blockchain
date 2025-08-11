
# Sybil Attacks

While developing your smart contract, keep in mind that an individual can potentially create multiple NEAR accounts. This is especially relevant in ecosystems involving crowd decisions, such as  [DAOs]

Imagine that you open the voting to anyone in the community. If each account can cast a vote, a malicious actor could span multiple accounts and gain a disproportionately large influence on the result.


## Understanding Sybil Attacks

Sybil attacks aim to exploit and manipulate peer-to-peer networks—often open and decentralized [blockchain networks] through multiple fake identities. In the context of Sybil attacks, “identities” can refer to a blockchain node, a social media account, a pseudo anonymous wallet address, or any other entity.

A successfully executed Sybil attack requires creating and controlling a large number of pseudo anonymous entities to influence the network in a malicious manner. For blockchains, this can result in a 51% attack or transaction censorship. In a social media network, Sybil attacks can be used to disseminate misinformation and create a false sense of community consensus.

There are two main types of Sybil attacks:

-   **Direct Sybil Attacks:** Malicious nodes influence the network by directly communicating with honest nodes to take control of decision-making processes, voting procedures, or consensus mechanisms.
-   **Indirect Sybil Attacks:** Malicious nodes or participants do not directly interact with honest nodes, but instead aim to silently leverage malicious nodes to artificially increase the reputation of particular nodes, alter a network’s topology, or isolate certain parts of the network.


## Sybil Attack Prevention and Defense Mechanisms

Sybil attacks represent an interesting phenomenon for blockchain networks. While Sybil attacks are uniquely dangerous for blockchains (high-value target, pseudoanonymous participation), blockchains are also purpose-built to be resilient against them.


### Cryptoeconomic Security

One of the main innovations of the Bitcoin network, and the blockchain ecosystem at large, was the creation of _cryptoeconomic security_. Whether in a proof-of-work or proof-of-stake mechanism, requiring network participants to give proof of computational work or economic stake makes it economically or technically impractical for a single entity to directly control a majority of nodes, hashrate, or stake.

### Reputation Systems

Another countermeasure against Sybil attacks is to build reputation systems directly into the network. For example, delegated proof-of-stake networks rely on a group of known and reputable, but potentially pseudoanonymous, entities to perform the major functions of a blockchain. This limits, and often completely removes, the ability for a Sybil attacker to join and influence the network as multiple entities.  
  
Similarly, reputation systems that record a node’s trustworthiness based on its history and contributions make it difficult for a Sybil attacker to masquerade as multiple entities because the attacker must maintain and build up reputation and influence across many nodes _over time_.

### Identity Verification

All Sybil attacks depend on semi-permissionless and pseudoanonymous access to a network. The reason that the vast majority of networks don’t need to worry about Sybil attacks is because they are permissioned and participants are known.

Thus, a very effective, but often unviable, protection mechanism against Sybil attacks for peer-to-peer networks is to validate node identities before they enter the network. While this doesn’t work for public blockchain networks that have been specifically designed to be permissionless while staying tamper-proof against Sybil attacks, it is nonetheless an incredibly effective defense against any form of Sybil attack—and may be used in the future alongside [decentralized, privacy-preserving identity protocols]