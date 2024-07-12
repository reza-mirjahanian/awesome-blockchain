### What is EVM?

**EVM**:

-   A decentralized program running on multiple computers
-   Allows Ethereum to be different from Bitcoin
-   Executes smart contracts
-   Smart contracts compile decentralized applications (dApps)

**Smart Contracts**:

-   Developed by developers for blockchain applications
-   Require state transitions (changes in balances)

### State Transition

-   Example: One address sends tokens to another
    -   Sender's balance decreases
    -   Receiver's balance increases
-   EVM is like a state machine, managing these transitions

### Adding Zero Knowledge (ZK) to EVM

**ZK EVM**:

-   Includes zero-knowledge proofs to ensure process correctness
-   Offloads computational costs to off-chain
-   Generates zero-knowledge proofs published on-chain
-   Reduces computational resources required

### Benefits of ZK EVM

**Layer 2 ZK EVM**:

-   Off-chain execution of transactions
-   Aggregates transactions into one zero-knowledge proof
-   Publishes one transaction proof on the main Ethereum chain
-   Inherits Ethereum's security with reduced fees

### Overview of ZK Roll-Ups

**ZK Roll-Ups**:

-   Use zero-knowledge proofs for transactions
-   More secure compared to optimistic rollups (e.g., Arbitrum, Optimism)

### EVM and ZK EVM Opcodes

**EVM Opcodes**:

-   Pieces of code for smart contracts to interact with EVM
-   ZK EVMs have different opcodes

**Recent Developments**:

-   Emerging ZK EVMs now use the same opcodes as EVM, making them EVM-equivalent

### Key ZK EVM Projects

#### Polygon ZK EVM

-   **Funding**: $450 million from Sequoia Capital and Dragonfly Capital
-   **Origin**: Acquired Hermes in 2022
-   **TPS**: Approximately 2,000 transactions per second
-   **Status**: In beta

#### ZK Sync Era (Version 2)

-   **Company**: Matter Labs
-   **Funding**: $258 million from a16z and Dragonfly Capital
-   **TPS**: Promises over 1,000 transactions per second
-   **Status**: Mainnet with restricted access

#### Scroll

-   **Start Year**: 2021
-   **Funding**: Investment from Polychain and Sequoia Capital
-   **TPS**: Approximately 1,000 to 2,000 transactions per second (unofficial)
-   **Status**: Aiming for mainnet in Q2 2024

#### Taiko

-   **Start Year**: 2022
-   **Status**: Aiming for mainnet in early 2024
-   **TPS**: Similar to Scroll, estimated around 2,000 transactions per second

### Summary

**Ethereum's Scalability**:

-   Ethereum needs layer 2 solutions for scalability
-   ZK EVMs provide a promising solution by being more developer-friendly and reducing computational costs

**Recent Developments**:

-   Enhanced usability of ZK EVMs
-   Old and new projects are emerging, making ZK EVMs nearly EVM-equivalent
-   This development might position ZK EVMs favorably against optimistic rollups like Arbitrum and Optimism