The main challenge a consensus protocol aims to solve is building a reliable distributed system on top of unreliable infrastructure. Research on consensus protocols dates back to the 1970s, but the scale of what Ethereum is trying to achieve is far more ambitious.

In Ethereum's consensus layer, the goal is to ensure that tens of thousands of independent nodes around the world stay reasonably synchronized. Each node keeps a ledger with the state of every account, and all these ledgers must match exactly. There can be no differences; the nodes must agree and do so quickly. This is what we mean by "a reliable distributed system."


These nodes often use [consumer grade hardware](https://stakefromhome.com/) and communicate over internet connections that may be slow, lose data, or disconnect unexpectedly. Node operators might misconfigure their software or fail to update it. Additionally, there could be many bad actors running rogue nodes or tampering with communications for personal gain. This is what we mean by "unreliable infrastructure."

### [Byzantine Fault Tolerance (BFT) and Byzantine Generals' Problem](https://epf.wiki/#/wiki/CL/overview?id=byzantine-fault-tolerance-bft-and-byzantine-generals39-problem)

Byzantine Fault Tolerance (BFT) is a property of distributed systems that allows them to function correctly even when some components fail or act maliciously. BFT is crucial in decentralized networks, where trust among nodes cannot be assumed. In other words, a system exhibiting BFT can tolerate Byzantine faults, which are arbitrary failures that include malicious behavior. For a system to be Byzantine fault-tolerant, it must reach consensus despite these faults. For more on the problem and practical solutions read [this](https://hackmd.io/@kira50/SknuPZMIC).