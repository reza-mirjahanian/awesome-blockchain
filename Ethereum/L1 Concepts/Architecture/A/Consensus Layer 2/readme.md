The main challenge a consensus protocol aims to solve is building a reliable distributed system on top of unreliable infrastructure. Research on consensus protocols dates back to the 1970s, but the scale of what Ethereum is trying to achieve is far more ambitious.

In Ethereum's consensus layer, the goal is to ensure that tens of thousands of independent nodes around the world stay reasonably synchronized. Each node keeps a ledger with the state of every account, and all these ledgers must match exactly. There can be no differences; the nodes must agree and do so quickly. This is what we mean by "a reliable distributed system."


These nodes often use [consumer grade hardware](https://stakefromhome.com/) and communicate over internet connections that may be slow, lose data, or disconnect unexpectedly. Node operators might misconfigure their software or fail to update it. Additionally, there could be many bad actors running rogue nodes or tampering with communications for personal gain. This is what we mean by "unreliable infrastructure."

### [Byzantine Fault Tolerance (BFT) and Byzantine Generals' Problem](https://epf.wiki/#/wiki/CL/overview?id=byzantine-fault-tolerance-bft-and-byzantine-generals39-problem)

Byzantine Fault Tolerance (BFT) is a property of distributed systems that allows them to function correctly even when some components fail or act maliciously. BFT is crucial in decentralized networks, where trust among nodes cannot be assumed. In other words, a system exhibiting BFT can tolerate Byzantine faults, which are arbitrary failures that include malicious behavior. For a system to be Byzantine fault-tolerant, it must reach consensus despite these faults. For more on the problem and practical solutions read [this](https://hackmd.io/@kira50/SknuPZMIC).

----

Consensus is a way to build reliable distributed systems with unreliable components. Blockchain-based distributed systems aim to agree on a single history of transactions.

Proof-of-work and Proof-of-stake are not consensus protocols, but enable consensus protocols. In Ethereum, Nodes and validators are the actors of the consensus system. Slots and epochs regulate consensus time. Blocks and attestations are the currency of consensus.

------

The Consensus Layer (CL) is a fundamental component that ensures the network's security, reliability, and efficiency. Originally, Ethereum utilized Proof-of-work (PoW) as its consensus mechanism, similar to Bitcoin. PoW, while effective in maintaining decentralization and security, has significant drawbacks, including high energy consumption and limited scalability. To address these issues, Ethereum has transitioned to Proof-of-Stake (PoS), a more sustainable and scalable consensus mechanism.

The Ethereum network consists of many individual nodes. Each node operates independently and communicates over the Internet, which is often unreliable and asynchronous.

Users send transactions to this network of nodes, and the consensus protocol ensures that all honest nodes eventually agree on a single, consistent transaction history. This means they agree on the order of transactions and their outcomes. For example, if I have 1 ETH and tell the network to send it to both Alice and Bob at the same time, the network must eventually agree on whether I sent it to Alice or Bob. It would be a failure if both Alice and Bob received the Ether, or if neither did. A consensus protocol is the process that enables this agreement on transaction order.

Ethereum's consensus protocol actually *bolts together* two different consensus protocols. 
- One is called [LMD GHOST](https://epf.wiki/#/wiki/CL/gasper?lmd-ghost), 
- the other [Casper FFG](https://epf.wiki/#/wiki/CL/gasper?casper-ffg). The combination has become known as [Gasper](https://epf.wiki/#/wiki/CL/gasper). In subsequent sections we will be looking at these both separately and in combination.


[Proof-of-work and Proof-of-stake](https://epf.wiki/#/wiki/CL/overview?id=proof-of-work-and-proof-of-stake)
-----------------------------------------------------------------------------------------------------------

This is a good point to clarify that neither Proof-of-work (PoW) nor Proof-of-Stake (PoS) are consensus protocols by themselves. They are often incorrectly referred to as such, but they are actually mechanisms that enable consensus protocols. Both PoW and PoS primarily serve as Sybil resistance mechanisms, placing a cost on participating in the protocol. This prevents attackers from overwhelming the system cheaply.

However, both PoW and PoS are closely linked to the consensus mechanisms they support through [fork choice](https://epf.wiki/#/wiki/CL/cl-architecture?id=fork-choice-rules) rules. They help assign a weight or score to a chain of blocks: in PoW, it's the total computational work done; in PoS, it's the total value staked that supports a particular chain. Beyond these basics, PoW and PoS can support various consensus protocols, each with its own dynamics and trade-offs.
