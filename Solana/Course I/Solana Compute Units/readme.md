In Ethereum, the price of a transaction is computed as gasUsed×gasPrice. This tells us how much Ether will be spent to include the transaction on the blockchain. Before a transaction is sent, a gasLimit is specified and paid upfront. If the transaction runs out of gas, it reverts.

Unlike on EVM chains, Solana opcodes/instructions consume "compute units" (arguably a better name) not gas, and each transaction is soft-capped at 200,000 compute units. If the transaction costs more than 200,000 compute units, it reverts.

In Ethereum, gas costs for computing are treated the same as gas costs associated with storage. In Solana, storage is handled differently, so the pricing of persistent data in Solana is a different topic of discussion.

From the perspective of pricing running op codes however, Ethereum and Solana behave similarly.

Both chains execute compiled bytecode and charge a fee for each instruction executed. Ethereum uses EVM bytecode, but Solana runs a modified version of [berkeley packet filter](https://en.wikipedia.org/wiki/Berkeley_Packet_Filter) called Solana packet filter.

Ethereum charges different prices for different op codes depending on how long they take to execute, ranging from one gas to thousands of gas. In Solana, each opcode costs one compute unit.

What to do when you don't have enough compute units
---------------------------------------------------

When performing heavy computational operations that cannot be done below the limit, the traditional strategy is to "save your work" and do it in multiple transactions.

The "save your work" part needs to be put into permanent storage, which is not something we've covered yet. This is similar to if you were trying to iterate over a massive loop in Ethereum; you'd have a storage variable for the index you left off at, and a storage variable saving the computation done up to that point.