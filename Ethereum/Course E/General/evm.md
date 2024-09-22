Understanding the Ethereum Virtual Machine (EVM)
================================================

The Ethereum World State
------------------------

-   **World State**: Current state of all important elements in Ethereum
    -   Includes: NFTs, tokens, DeFi, account balances
-   Changes occur when new blocks are mined
-   Transitions from one state to another through transactions

Purpose of the EVM
------------------

1.  Compute state transitions
2.  Process transactions
3.  Execute smart contract code

Components of the World State
-----------------------------

1.  **Externally Owned Accounts**
    -   Controlled by private keys
    -   Contains:
        -   Nonce
        -   Balance
2.  **Contract Accounts**
    -   Contains:
        -   Nonce
        -   Balance
        -   Storage (mutable)
        -   Code (immutable)

Data Structure: Merkle Patricia Tree
------------------------------------

-   Used to store and organize account data
-   Allows for quick access and verification of data

EVM Architecture
----------------

-   **Virtual ROM**: Stores contract code (immutable)
-   **Machine State** (volatile):
    -   Program counter
    -   Gas tracker
    -   Stack
    -   Memory
-   **Account Storage**: Persistent storage for contracts

EVM as a Stack Machine
----------------------

-   Uses a stack-based architecture (unlike register-based CPUs)
-   Stack: 256 bits wide, 1024 elements deep
-   Memory: Byte-addressable linear memory
-   Account Storage: Key-value mapping (256-bit to 256-bit)

EVM Execution Process
---------------------

1.  Load contract code into virtual ROM
2.  Set program counter to zero
3.  Load contract storage
4.  Initialize memory to zero
5.  Load block and environment variables

EVM Opcodes
-----------

-   Similar to CPU assembly instructions
-   Examples:
    -   ADD (3 gas)
    -   MULTIPLY (5 gas)
    -   SSTORE (100 gas, writes to contract storage)
    -   CREATE (32,000 gas, creates a new account)

Gas Costs and Pricing
---------------------

-   Gas cost per opcode: Fixed, reflects computational complexity
-   Gas price: Variable, depends on network demand
-   Total transaction cost = Gas used \* Gas price

EVM Implementation
------------------

-   Implemented in client software (e.g., Go Ethereum)
-   Opcodes are regular functions in the client code

EVM Limitations and Security
----------------------------

-   Sandboxed environment
-   Cannot access external APIs or websites
-   Limited view of the Ethereum network
-   Ensures deterministic execution across all nodes

Smart Contract Interactions
---------------------------

-   Contracts can call other contracts
-   Creates nested EVM instances
-   Passes potentially modified world state to child instances
-   Transaction succeeds only if all nested calls complete successfully