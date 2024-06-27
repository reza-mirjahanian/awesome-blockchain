# Querying Contract State

We will first cover the two types of queries - raw and custom - then look at the semantics of querying via an _external client_, as well as an _internal client_ (another contract).

## Raw Queries[​](https://docs.cosmwasm.com/docs/architecture/query#raw-queries "Direct link to heading")

The simplest query to implement is just raw read access to the key-value store. If the caller (either an external client or another contract) passes in the raw binary key that is used in the contract's storage, we can easily return the raw binary value. The benefit of this approach is that it is very easy to implement and universal. The downside is that it links the caller to the  _implementation_  of the storage and requires knowledge of the exact contract being executed.

This is implemented inside the  `wasmd`  runtime and circumvents the VM. As a consequence, it requires no support from the CosmWasm contract and all contract state is visible. Such a  `query_raw`  function is exposed to all callers (external and internal).


## Introduction

Querying the state of a contract is essential both for external clients (using CLI) and for other contracts during execution. This document discusses two main types of queries—raw and custom—along with their semantics when performed by external clients and internal clients (other contracts). The focus includes practical implementation, design, and security issues.

## Types of Queries

### Raw Queries

-   **Description**: Direct access to the key-value store using a binary key.
-   **Advantages**: Simple and universal.
-   **Disadvantages**: Ties the caller to the specific storage implementation, requiring knowledge of the exact contract.

### Custom Queries

-   **Description**: Allows contracts to expose a read-only query function, which can load and process any required data.
-   **Advantages**: More flexible and decoupled from the storage implementation.
-   **Usage**: Common for standardized interfaces like "ERC20" for querying balances, allowances, and token info.
-   **Gas Limit**: Unlike raw queries, custom queries may consume significant gas, necessitating enforced gas limits to prevent abuse.

## Query Execution

### External Queries

-   **Process**: Utilizes Tendermint RPC and Cosmos SDK's `abci_query`.
-   **Gas Limit**: Typically has an infinite gas limit since it executes on one node, but custom queries require a configurable gas limit to prevent abuse (e.g., DoS attacks).
-   **State Snapshot**: Uses a read-only snapshot of the state after the last committed block.

### Internal Queries

-   **Process**: Contracts query other modules synchronously without altering their state, essential for tasks like resolving names or checking KYC status.
-   **Design Challenges**:
    -   **Actor Model Violation**: Internal state access risks concurrency and reentrancy issues.
    -   **Read-Only Access**: Ensured by providing a snapshot of the state before current message execution, maintaining safety.
-   **Reentrancy Prevention**: Queries only have read-only access and are part of a transaction with a strong gas limit, reducing risks of reentrancy issues.
-   **Future Work**: Potential addition of explicit guards against reentrancy and maximum query depth.