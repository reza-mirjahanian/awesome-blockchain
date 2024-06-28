
## Contract Composition in CosmWasm

CosmWasm v0.8 introduces the capability to compose contracts with both other contracts and native modules. This allows for complex interactions and functionality within the blockchain ecosystem. This document delves into the technical details of contract composition, highlighting its features, benefits, and design considerations.

### Terminology

-   **Contracts**  : CosmWasm code dynamically uploaded to the blockchain at a specific address.
-   **Native Modules**  : Go Cosmos SDK modules compiled into the blockchain binary.

### Message Composition

Contracts can send messages to other contracts or native modules. These messages are grouped into three categories:

-   **Contract Messages**  : Call a specific contract address with a serialized message. Requires understanding the contract's API format.
-   **Module Interfaces**  : Standardized interfaces for accessing native modules. Provides portability across different CosmWasm chains.
-   **Custom Messages**  : Chain-dependent extensions for calling custom native modules. Offers no portability guarantees.

### Query Composition

Similar to messages, queries allow contracts to interact with other contracts or native modules. The three categories are:

-   **Contract Queries**  : Query a specific contract address with a serialized message. Requires understanding the contract's API format.
-   **Module Interfaces**  : Standardized interfaces for querying native modules. Provides portability across different CosmWasm chains.
-   **Custom Queries**  : Chain-dependent extensions for querying custom native modules. Offers no portability guarantees.

### Module Interfaces

To enhance interoperability, CosmWasm provides standardized module interfaces. These interfaces allow contracts to interact with common blockchain functionalities in a consistent manner, regardless of the underlying blockchain implementation. Examples include:

-   **Bank Module:**  Provides access to native tokens with functions like sending tokens and querying balances.
-   **Staking Module:**  Enables standardized messages for delegating, undelegating, and redelegating tokens, as well as querying validator information.

### Customization

While module interfaces promote portability, some blockchains require custom integrations. This is achieved through "Custom" variants for both messages and queries. This approach offers immutability but sacrifices portability.

### Design Considerations

The contract composition design prioritizes the following principles:

-   **Portability**  : Contracts should run on different blockchains with minimal modification. This is achieved through standardized module interfaces and avoiding custom messages.
-   **Immutability**  : Contracts are immutable and their formats are encoded in their bytecode. This ensures that changes in native message formats do not break existing contracts.
-   **Extensibility**  : New interfaces can be added to contracts and blockchains without requiring changes in core components.
-   **Usability**  : The design aims for a user-friendly experience for both contract authors and blockchain developers. This includes using JSON encoding for messages and providing developer tools like CosmJS.