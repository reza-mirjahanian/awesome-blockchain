Blockchain as a Replicated State Machine
========================================

Developers can create the state machine using the Cosmos SDK. This includes:

-   Storage organization: also known as *the state*.
-   State transition functions: these determine what is permissible and if adjustments to the state result from a transaction.

Core Concepts
-------------

### State Machine

-   A computer science concept
-   Multiple possible states, but only one at a time
-   State transitions follow defined processes

### Blockchain State Transition

-   **State transition = Transaction**
-   Initial state (S) → Transaction → New state (S')
-   Rules for interpretation defined at the application layer

Key Characteristics of Blockchains
----------------------------------

1.  **Deterministic**
    -   Only one correct interpretation of a transaction
    -   Results in a single, specific new state
2.  **Distributed**
    -   Transactions arrive in batches (blocks)
    -   State persists after each transaction in a block
    -   Each transaction executes in the context of the previous state
3.  **Genesis State**
    -   Initial "nothing has happened yet" state
    -   Current state achievable by applying all transactions to Genesis state

Cosmos SDK and State Machine Development
----------------------------------------

Developers can create the state machine using Cosmos SDK, including:

-   **Storage organization** (the state)
-   **State transition functions** (rules for permissible actions and state adjustments)

Consensus and State Agreement
-----------------------------

-   Establishes a canonical set of ordered blocks and transactions
-   All nodes agree on the canonical set
-   Only one correct interpretation due to determinism

CometBFT and Tendermint
-----------------------

-   CometBFT is agnostic to block interpretation
-   Tendermint consensus establishes the ordered transaction set
-   Nodes reach consensus on the application state