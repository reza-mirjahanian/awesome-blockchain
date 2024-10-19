-   **Problem Statement**
    -   **Monolithic Blockchains**:
        -   Lack of scalability
        -   Endless cycle of new monolithic Layer 1s (L1s) that fail to deliver on promises

**Modular Blockchains Overview**
--------------------------------

-   **Definition & Benefits**
    -   **Modular Blockchains**: Decouple consensus and execution layers
    -   **Advantages**:
        -   Enhanced scalability
        -   Increased flexibility for experimenting with different execution environments

**Evolution from Monolithic to Modular**
----------------------------------------

-   **Bitcoin White Paper (2008)**
    -   Introduction of the monolithic blockchain model
    -   **Monolithic Model Characteristics**:
        -   Coupled consensus and execution
        -   Every user executes every transaction
        -   Scalability and flexibility limitations
-   **Lazy Ledger Proposal (2019)**
    -   Separation of consensus and data availability
    -   **Roll-Up Centric Model**:
        -   **Data & Consensus Layer**: Handles ordering and availability
        -   **Execution Layer**: Processes transactions (e.g., Roll-Ups)
    -   Result: **Modular Blockchain Ecosystem**

**Layers in a Modular Stack**
-----------------------------

1.  ### **Consensus Layer**

    -   **Function**:
        -   Orders messages/transactions
        -   Ensures data availability
    -   **Key Concepts**:
        -   Timestamping server as per Bitcoin's original solution
        -   **Data Availability Sampling**:
            -   Ensures >99% data availability by downloading a small data portion
2.  ### **Execution Layer**

    -   **Function**:
        -   Processes transactions
        -   Outputs state commitments (e.g., account balances)
    -   **Examples**:
        -   **Roll-Ups**: Provide execution environments for Layer 2
        -   **Layer 2**: Inherits consensus and security from the data layer
3.  ### **Settlement Layer**

    -   **Function**:
        -   Bridges different execution layers/Roll-Ups
    -   **Examples**:
        -   **Ethereum**: Acts as a settlement layer with on-chain bridges for Roll-Ups

**What is a Modular Blockchain?**
---------------------------------

-   **Definition**
    -   A blockchain that outsources at least one of the four components:
        -   Consensus
        -   Data Availability
        -   Settlement
        -   Execution

**Benefits of Modularity**
--------------------------

1.  ### **Scalability**

    -   **Dedicated Execution Environments**:
        -   Users don't execute every transaction
        -   Roll-Ups have their own computation resources
    -   **Data Availability Sampling**:
        -   More light nodes increase secure block space
2.  ### **Freedom of Choice for Developers**

    -   **Flexible Execution Environments**:
        -   Ability to modify or replace execution layers without deploying new L1s
    -   **Variety of Roll-Ups**:
        -   Sovereign Roll-Ups
        -   Settled Roll-Ups
        -   Palladiums and Celestiums

**Current State of the Modular Stack (2023)**
---------------------------------------------

-   **Progress Since 2022**

    -   Transition from theoretical to developed ecosystem
-   **Infrastructure Developments**

    -   Block explorers, analytics providers
    -   Sequencing providers (shared sequencing)
    -   Roll-Up frameworks (e.g., Obstacles Labs, Sovereign)
    -   Roll-Up as a service providers
-   **Notable Projects & Highlights**

    -   **Upstack**: Ethereum-focused Roll-Up framework with Celestia DA layer
    -   **Manta**: Deploying OpStack Roll-Up
    -   **Calderas**: Testnet using OpStack interface
    -   **Sovereign Roll-Ups**:
        -   Sovereign SDK (alpha release)
        -   Eclipse: Roll-Up service provider
        -   Roll Kit: Deployed first Sovereign Roll-Up on Bitcoin
    -   **Dimension**: Released first IBC-enabled Roll-Up using EVM
-   **Ecosystem Growth**

    -   Expansion of Celestia ecosystem
    -   Applications including gaming (e.g., Curio's real-time strategy game on Celestia)

**Open Problems in the Modular Stack**
--------------------------------------

1.  ### **User Experience (UX) for Bridging**

    -   Simplifying multi-token bridging across chains
    -   Example: **IBC.fun** demo by Skip
2.  ### **Tooling for Custody and Payment Systems**

    -   Facilitating interactions between different stack layers
    -   Managing multiple tokens and pricing mechanisms
3.  ### **Developer Education**

    -   Explaining trade-offs between different execution environments and layers
    -   Enhancing understanding of stack components
4.  ### **Dependency Management**

    -   Addressing dependencies across stack layers
    -   Proposing common interfaces to prevent breaking changes
5.  ### **Proof Systems Development**

    -   **Fault Proofing Systems**:
        -   Limited implementations (e.g., Fraud Proofs V1)
    -   **ZK Proving Systems**:
        -   Need for speed optimizations (hardware acceleration, FPGAs)
6.  ### **Privacy Enhancements**

    -   Enabling privacy through modular execution environments without deploying new L1s

**Values and Goals of Modularism**
----------------------------------

1.  ### **User-Centric Network**

    -   **First-Class Citizens**:
        -   Emphasis on light nodes
        -   Reducing reliance on centralized APIs
    -   **Core Ideal**:
        -   Trustless interactions without middlemen
2.  ### **Modularity Over Maximalism**

    -   **Positive Sum Mindset**:
        -   Encouraging ecosystem collaboration instead of zero-sum competition
    -   **Sustainability**:
        -   Avoiding the cycle of failed Layer 1s
        -   Promoting incremental, stack-wide improvements
3.  ### **Community Sovereignty**

    -   **Freedom to Fork**:
        -   Allowing communities to modify or replace layers as needed
    -   **Decentralized Governance**:
        -   Enabling self-organization and collective action without centralized enforcement

**Recap of Three Core Values**
------------------------------

1.  **User-Centricity**

    -   Users as first-class citizens through light node support
2.  **Modularity**

    -   Escaping the monolithic blockchain loop for sustainable growth
3.  **Sovereignty**

    -   Empowering communities with the right to fork and customize