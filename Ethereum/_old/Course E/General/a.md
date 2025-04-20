**Main Components**
-------------------

### **The Spec**

-   **Blueprint**: The spec is like a blueprint that defines the Ethereum system.
-   **Ethereum Research Forum**: A place where the spec is updated, refined, and new changes are proposed and debated.
-   **Implementation by Clients**: The spec is implemented by client software run by various computers (nodes) around the world.

### **Ethereum World Computer System**

-   **Nodes**: Computers around the world that run the client code, forming the decentralized Ethereum network.

**Detailed Structure**
----------------------

### **Spec and Features**

-   **Source of Truth**: The spec defines how Ethereum should function, ensuring all important features and aspects are clear.
-   **Examples**:
    -   **Blocks**: Fields like difficulty, block number, transactions, and transaction root are all defined in the spec.
    -   **Accounts**: Ethereum accounts (whether regular accounts or contracts) have:
        -   **Balance**
        -   **Nonce**
        -   **Storage hash**
        -   **Code hash** (for contract accounts)
-   **Proof of Work**:
    -   **Difficulty Algorithm**: Defined in the spec, ensuring all nodes calculate difficulty consistently.
    -   **Block Validation**: Nonce is permuted to produce a block hash with enough leading zeros for validation.

### **Ethereum Virtual Machine (EVM)**

-   **State Machine**: Computes state changes for every transaction.
-   **Operations**: Basic opcodes like add, multiply, subtract, and hash.
-   **Contract Code**: Contracts are compiled into EVM bytecode, which is like assembly language for the EVM.
-   **Consistency Across Nodes**: All nodes must compute the same result, or the system forks, disrupting consensus.

### **EVM Implementation**

-   **Op Codes**: Simple operations like addition are defined in Go or other languages to comply with Ethereum's rules (e.g., using 256-bit numbers).
-   **Example**:
    -   **Addition**: Adding two numbers constrained to 256 bits can result in overflow, which is a behavior that must be handled consistently across implementations.

**Client Implementations**
--------------------------

-   **Multiple Implementations**: Ethereum has several client implementations (e.g., Geth, Besu, OpenEthereum).
-   **Languages**: Written in various languages like Go, Rust, Java, C#.
-   **Diverse Implementations**: Each client might have optimizations for different hardware setups (e.g., low-power hardware or high-end servers).
-   **Importance of Multiple Clients**:
    -   **Resilience**: If one client fails, the others continue running, preventing a total system failure.
    -   **Geth Popularity**: As the most popular client, issues in Geth are more impactful.

### **Client Update Process**

-   **Eth Research to Spec**: Proposed changes (e.g., EIP-1559) go through research, refinement, and are integrated into the spec.
-   **Forks and Releases**: Changes are bundled into forks like Constantinople or Byzantium. Clients update their code and push new releases.
-   **Node Updates**: Nodes update their software before the fork, ensuring they're ready to switch to new rules when the predetermined block is reached.

**Node Operations**
-------------------

-   **Global Synchronization**: All nodes run the same implementation and follow the same rules, leading to consistent transaction processing.
-   **Full Copy of Ethereum System**: Each node maintains a complete copy of the Ethereum system, ensuring redundancy and reliability.

### **Client Independence**

-   **Different Implementations**: Though clients are written in different languages and have unique architectures, as long as they follow the spec, they can function as Ethereum nodes.

**Ethereum as a Holographic System**
------------------------------------

-   **Distributed System**: All nodes working together, running the same code, ensure Ethereum's decentralized and resilient nature.


Ethereum Clients
----------------

Multiple client implementations exist, including:

-   Bezu
-   Open Ethereum
-   Geth (most popular, written in Go)
-   Nethermind
-   Aragon

