
a)	How users send L2 transactions and who receives them?
•	The zkEVM uses unicast to let user send their transaction (calls to an RPC).
•	The zkEVM also enables posting L2 transactions via a method in a smart contract as an **anti-censorship** measure (called "forced batches”).

b)	How L2 transactions are made publicly available (if so)?
• The zkEVM is a rollup, the L2 data is available in L1.

c)	Who processes the L2 transactions and how, and, when it is publicly considered that a new state is correctly computed?
•	In the zkEVM, currently, there is a centralized aggregator node that proves the processing of the L2 transactions.
•	However, this node cannot cheat because there is a succinct computation verification (using zero-knowledge technology).

d)	What type of applications the L2 supports? simple or rich processing?
• zkEVM is **rich processing** since it is an EVM.