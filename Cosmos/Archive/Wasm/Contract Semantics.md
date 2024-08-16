
# Contract Semantics

This document explains how CosmWasm contracts interact with their environment. There are two main types of actions: **mutating actions** that modify the blockchain state and **query actions** that read data on a single node. The execution of a contract is handled by the Cosmos SDK, which processes transactions sequentially and isolates each one within a context.

The Cosmos SDK verifies signatures, deducts gas fees, and limits execution to the amount of gas paid. It then executes the transaction in an isolated environment, allowing read access to the chain's current state but writing only to a cache. Transactions can consist of multiple messages, and all messages must succeed for the transaction to be committed.

CosmWasm's `x/wasm` module handles smart contract interactions, accepting signed `MsgExecuteContract` messages and routing them to the appropriate contract's `execute` function. The contract's `execute` function can read and write to the blockchain's storage, validate addresses, and query the state of other contracts or modules. It returns either a `Response` object or an `Error`.

The `Response` object can contain submessages (which are executed before the main messages), messages that are executed on the host blockchain, attributes for events, and optional data. Submessages are executed sequentially, with replies handled by the calling contract.

The `Querier` object enables synchronous, read-only calls to other contracts, preventing reentrancy attacks. Queries are serialized into `QueryRequest` structs and sent to the x/wasm SDK module for processing.

CosmWasm's design prioritizes security and atomicity. By using the actor model and preventing nested function calls, reentrancy attacks are mitigated. Submessages provide a mechanism for communication and error handling between contracts while maintaining transaction integrity. The strict separation of read-only queries from mutating actions ensures that data is consistent and secure.