Application Blockchain Interface (ABCI)
---------------------------------------

[CometBFT](https://docs.cometbft.com/v0.37/)packages the networking and consensus layers of a blockchain and presents an interface to the application layer, the **Application Blockchain Interface (ABCI)**. Developers can focus on higher-order concerns and delegate peer-discovery, validator selection, staking, upgrades, and consensus to CometBFT. The consensus engine runs in one process and controls the state machine, while the application runs in another process. The architecture is equally appropriate for **private or public blockchains**.

CometBFT is connected to the application by a socket protocol. ABCI provides a socket for applications written in other languages. If the application is written in the same language as the CometBFT implementation, the socket is not used.



CometBFT provides security guarantees, including the following:

-   **Forks** are never created, provided that at least half the validators are honest.
-   **Strict accountability** for fork creation allows determination of liability.
-   Transactions are **finalized** as soon as a block is created.

CometBFT is not concerned with the interpretation of transactions. That occurs at the application layer. CometBFT presents confirmed, well-formed transactions and blocks of transactions agnostically. CometBFT is un-opinionated about the meaning any transactions have.

The *block time* is approximately seven seconds, and blocks may contain thousands of transactions. Transactions are finalized and cannot be overturned as soon as they appear in a block.


There are at least two broad approaches to **application-level concerns** using blockchains:

1.  Create an application-specific blockchain where everything that can be done is defined in the protocol.
2.  Create a programmable state machine and push application concerns to a higher level, such as bytecode created by compilers interpreting higher-level languages.

Ethereum-like blockchains are part of the second category: only the state machine is defined in the on-chain protocol, which defines the rules of contract creation, interaction, execution, and little else.

This method is not without its limitations:

-   Very little is universally defined: standards for basic concerns such as tokens emerge organically through voluntary participation.
-   Contracts can and do contain repetitive code that may or may not correctly implement the developer's intentions.
-   This inherent flexibility makes it challenging to reason about what is correct, or even what is friendly.
-   There are practical limits to the complexity of operations, which are very low compared to what is possible in other settings.

These limitations make it especially difficult to perform analysis or reorganize data, and developers are forced to adapt to the constraints.

A **purpose-built or application-specific blockchain** is different. There is no need to present a "Turing-complete" language or a general-purpose, programmable state machine because application concerns are addressed by the protocol the developers create.

Developers who have worked with blockchains based on the Ethereum Virtual Machine (EVM) will recognize a shift in the way concerns are addressed. Authorization and access control, the layout of storage and the state, and application governance are not implemented as contracts on a state machine. They instead become properties of a unique blockchain that is built for a singular purpose.