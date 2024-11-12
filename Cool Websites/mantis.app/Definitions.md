**Block Builder:** An entity that constructs blocks from transaction order flow.

**Coincidence of Wants (CoWs):** A phenomenon where two (or more) parties coincidentally hold an item or asset that the other wants, and thus are able to exchange directly without the need for an intermediary exchange. In the case of intents, this means that one user's intent can coincidentally be the opposite of another user's intents. For example, one intent to swap asset A for asset B and another to swap B for A form a CoW.

**Cross-Domain Maximal Extractable Value (MEV):** The maximum value that can be captured from arbitrage transactions executed in a specified order across multiple domains. See: Maximal Extractable Value.

**Cross-Domain Slots:** Blocks that are created across multiple chains simultaneously.

**Crypto-Economic Security:** A model for securing a network via economic incentives and cryptography.

**Intent:** An expression of what a user wants to achieve whenever they interact with a blockchain protocol, for instance "transfer X asset from blockchain A to blockchain B" or "trade X asset for Y asset." Practically, an intent is an off-chain signed message that encodes which state transitions a user wants to achieve. Unlike transactions, intents are partial. Thus, one can think of intents as parts of transactions that require other direct or indirect parts as complements in order to form a final balanced transaction that satisfies all of a user's constraints.

**Inter-Blockchain Communication (IBC) Protocol:** A protocol for trust-minimized communication between different blockchain networks/ecosystems. IBC is leveraged on Mantis via the [Picasso Network](https://www.picasso.network/).

**Mantis Virtual Machine (MVM):** An orchestration language and execution runtime for cross-chain program execution and intents settlement.

**Maximal Extractable Value (MEV):** The maximal value extractable between one or more blocks, given any arbitrary re-ordering, insertion or censorship of pending or existing transactions (as defined by [Obadia et al., 2021](https://arxiv.org/pdf/2112.01472.pdf)).

**Multichain-Agnostic Normalized Trust-Minimized Intent Settlement (Mantis):** A vertically integrated, optimized intents settlement framework. Mantis is complete with with expression, execution, and settlement.

**Multi-Domain Auction:** A novel system being introduced by Mantis that allows block builders to pre-reserve cross-chain blockspace and sell this to searchers

**The Picasso Layer 1 (L1):** A [Cosmos SDK](https://v1.cosmos.network/sdk) blockchain that acts as an [Inter-Blockchain Communication (IBC) Protocol](https://www.ibcprotocol.dev/) hub between [Cosmos](https://cosmos.network/) and non-Cosmos IBC-enabled chains. Picasso serves as the cross-ecosystem IBC Hub due to its ongoing efforts to implement IBC on Ethereum, Solana, Polkadot, and other chains anticipated in the future. Moreover, Picasso delivers restaking to these ecosystems through its Restaking Layer. Picasso documentation is available [here](https://docs.picasso.network/).

**Proposers:** Entities that propose which transactions to include in the next block by looking at which transactions in the mempool pay the highest priority fee. This means proposers are able to take advantage of MEV opportunities. See: Maximal Extractable Value.

**Searchers:** Entities that extract MEV by running complex algorithms. On Mantis, searchers extract MEV from the presence of transactions in pre-processed blocks proposed by proposers. See: Maximal Extractable Value.

**Solvers:** Entities that compete to determine an optimal solution (in the form of a transaction execution pathway) for a user's intent. See: Intent