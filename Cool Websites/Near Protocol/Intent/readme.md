Intents
=======

In NEAR, an `intent` is a high level declaration of what a user wants to achieve. Think of it as telling the blockchain "what" you want to do, not "how" to do it. For example, instead of manually:

-   Finding the best DEX for a token swap
-   Calculating optimal routes
-   Executing multiple transactions

You simply express: "I want to swap Token A for Token B at the best price."

[NEAR Intents](https://near.org/blog/introducing-near-intents/) is a revolutionary transaction framework that simplifies blockchain interactions for:

-   Users
-   Services
-   AI agents

The key innovation is that users & developers no longer need to handle complex cross-chain transactions themselves. Instead, they declare their desired outcome, and a specialized network of solvers (including both AI agents and traditional market participants) competes to execute that intent in the most optimal way possible.



How It Works[​](https://docs.near.org/chain-abstraction/intents/overview#how-it-works "Direct link to How It Works")
--------------------------------------------------------------------------------------------------------------------

1.  [**Intent Creation**:](https://docs.near.org/chain-abstraction/intents/overview#intent-creation) A user or AI agent expresses a desired outcome *(ex: Swap Token A for Token B)* and broadcasts the intent to a Solver Network of their choice.

2.  [**Solvers Compete**:](https://docs.near.org/chain-abstraction/intents/overview#solvers) A off-chain decentralized network of solvers compete to fulfill the request in the most optimal way. When the solver network finds the best solution, it presents it as a quote to the originating user/agent for approval.

3.  [**Intent Execution**:](https://docs.near.org/chain-abstraction/intents/overview#intent-execution) If the quote from the Solver Network is accepted, the intent begins execution. This is done by the solver performing a contract call (`execute_intents`) to the Intents smart contract on NEAR ([`intents.near`](https://nearblocks.io/address/intents.near)) and passing the intent details. This contract then fulfills the request and (if needed) uses a [cross-chain bridge](https://docs.near.org/chain-abstraction/intents/intents-bridge) to broadcast the intent to the destination chain. The NEAR Intent smart contract also verifies state changes and ensures the intent is settled correctly, reporting the outcome to the originating user/agent.