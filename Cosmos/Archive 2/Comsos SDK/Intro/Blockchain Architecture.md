State machine

At its core, a blockchain is a [replicated deterministic state machine](https://en.wikipedia.org/wiki/State_machine_replication)


### CometBFT

```
                ^  +-------------------------------+  ^
                |  |                               |  |   Built with Cosmos SDK
                |  |  State-machine = Application  |  |
                |  |                               |  v
                |  +-------------------------------+
                |  |                               |  ^
Blockchain node |  |           Consensus           |  |
                |  |                               |  |
                |  +-------------------------------+  |   CometBFT
                |  |                               |  |
                |  |           Networking          |  |
                |  |                               |  |
                v  +-------------------------------+  v
```

CometBFT is an application-agnostic engine that is responsible for handling the networking and consensus layers of a blockchain. In practice, this means that CometBFT is responsible for **propagating and ordering transaction bytes**. CometBFT relies on an eponymous Byzantine-Fault-Tolerant (BFT) algorithm to reach consensus on the order of transactions.

The CometBFT consensus algorithm works with a set of special nodes **called Validators**. Validators are responsible for adding blocks of transactions to the blockchain. At any given block, there is a **validator set V**. A validator in V is chosen by the algorithm to be the **proposer of the next block**. This block is considered valid if more than two thirds of V signed a **prevote** and a **precommit** on it, and if all the transactions that it contains are valid. The validator set can be changed by rules written in the state-machine.



-   **CometBFT Overview:**

    -   CometBFT is an engine that handles the networking and consensus layers of a blockchain.
    -   It is application-agnostic, meaning it can be used with different types of blockchain applications.
-   **Core Responsibilities:**

    -   Responsible for propagating and ordering transaction bytes.
    -   Uses a Byzantine-Fault-Tolerant (BFT) algorithm to achieve consensus on transaction order.
-   **Validators Role:**

    -   Validators are special nodes responsible for adding transaction blocks to the blockchain.
    -   A validator set, referred to as V, exists at any given block.
-   **Consensus Process:**

    -   A validator from the set V is chosen by the algorithm to propose the next block.
    -   The proposed block is valid if:
        -   More than two-thirds of the validator set signs a prevote and a precommit on it.
        -   All transactions in the block are valid.
-   **Validator Set Management:**

    -   The validator set can be modified based on rules defined in the state-machine.

ABCI
------------------------------------------------------------------------------------------------------

CometBFT passes transactions to the application through an interface called the [ABCI](https://docs.cometbft.com/v0.37/spec/abci/), which the application must implement.


Note that **CometBFT only handles transaction bytes**. It has no knowledge of what these bytes mean. All CometBFT does is order these transaction bytes deterministically. CometBFT passes the bytes to the application via the ABCI, and expects a return code to inform it if the messages contained in the transactions were successfully processed or not.

Here are the most important messages of the ABCI:

-   `CheckTx`: When a transaction is received by CometBFT, it is passed to the application to check if a few basic requirements are met. `CheckTx` is used to protect the mempool of full-nodes against spam transactions. . A special handler called the [`AnteHandler`](https://docs.cosmos.network/v0.50/learn/beginner/gas-fees#antehandler) is used to execute a series of validation steps such as checking for sufficient fees and validating the signatures. If the checks are valid, the transaction is added to the [mempool](https://docs.cometbft.com/v0.37/spec/p2p/messages/mempool) and relayed to peer nodes. Note that transactions are not processed (i.e. no modification of the state occurs) with `CheckTx` since they have not been included in a block yet.
-   `DeliverTx`: When a [valid block](https://docs.cometbft.com/v0.37/spec/core/data_structures#block) is received by CometBFT, each transaction in the block is passed to the application via `DeliverTx` in order to be processed. It is during this stage that the state transitions occur. The `AnteHandler` executes again, along with the actual [`Msg` service](https://docs.cosmos.network/v0.50/build/building-modules/msg-services) RPC for each message in the transaction.
-   `BeginBlock`/`EndBlock`: These messages are executed at the beginning and the end of each block, whether the block contains transactions or not. It is useful to trigger automatic execution of logic. Proceed with caution though, as computationally expensive loops could slow down your blockchain, or even freeze it if the loop is infinite.

Find a more detailed view of the ABCI methods from the [CometBFT docs](https://docs.cometbft.com/v0.37/spec/abci/).

Any application built on CometBFT needs to implement the ABCI interface in order to communicate with the underlying local CometBFT engine. Fortunately, you do not have to implement the ABCI interface. The Cosmos SDK provides a boilerplate implementation of it in the form of [baseapp](https://docs.cosmos.network/v0.50/learn/intro/sdk-design#baseapp).

-   **ABCI Interface Overview:**

    -   CometBFT uses the ABCI (Application Blockchain Interface) to pass transactions to the application.
    -   The application must implement the ABCI interface to interact with CometBFT.
-   **CometBFT's Role:**

    -   CometBFT handles only the transaction bytes, not their content or meaning.
    -   It orders transaction bytes deterministically and passes them to the application via the ABCI.
-   **Transaction Processing Workflow:**

    -   **CheckTx:**
        -   Used to validate transactions when they are received by CometBFT.
        -   Ensures transactions meet basic requirements, protecting the mempool from spam.
        -   Utilizes the AnteHandler for validation checks like fees and signatures.
        -   Valid transactions are added to the mempool but not yet processed or included in a block.
    -   **DeliverTx:**
        -   Processes transactions when they are included in a valid block.
        -   Executes state transitions and runs the AnteHandler again, along with the Msg service RPC.
    -   **BeginBlock/EndBlock:**
        -   Executed at the start and end of each block, regardless of whether it contains transactions.
        -   Used for triggering automatic logic but should be handled carefully to avoid performance issues.
-   **ABCI Implementation:**

    -   Applications built on CometBFT need to implement the ABCI interface for communication with CometBFT.
    -   The Cosmos SDK provides a boilerplate ABCI implementation called `baseapp`, simplifying development.
