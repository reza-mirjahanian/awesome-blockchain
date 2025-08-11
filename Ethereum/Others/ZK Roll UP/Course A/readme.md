
### Understanding ZK Rollups and Layer 2 Scaling on Ethereum

  

In previous videos, I explained why  **Ethereum**  is scaling on  **Layer 2**  and not Layer 1, and how the  **modular vision**  has emerged to support this Layer 2 rollup-centric future. In this video, I'm going to:

  

1.  **Zoom in on ZK Rollups**:
    -   Explain how they work.
    -   How they achieve scale.
    -   Look at the underlying compute hardware powering ZK Rollups.
    -   Discuss the scaling dynamics and bottlenecks.

  

### High-Level Understanding of ZK Rollups

  

I have a detailed diagram that provides a high-level understanding of how ZK roll-ups work. Let's break it down:

  

#### **1. Overview of Ethereum’s World State**

  

-   **Ethereum World State**: A set of all Ethereum accounts and contracts at a given point in time.
    
    -   Each Ethereum account has an associated nonce and balance.
    -   Each Ethereum contract has some associated code and storage.
    -   The world state is organized into a  **Merkle tree**.
    -   Changes in the data are reflected in a new  **Merkle root**.
-   **Role of EVM**: The Ethereum Virtual Machine (EVM) computes changes to the world state based on transactions.
    
    -   Transactions include simple ETH transfers and contract calls with call data.

  

#### **2. How ZK Rollups Differ**

  

-   **State Transition Proofs**: Unlike Ethereum, ZK Rollups create proofs to validate state transitions.
    
    -   The virtual machine is designed for efficient proof generation, often using polynomial math (e.g., in StarkNet).
-   **Sequencer Role**: ZK Rollup sequencers, unlike numerous Ethereum nodes, are fewer but more powerful machines managing the state changes.
    

  

### ZK Rollups and their Scaling Mechanics

  

#### **3. Sequencers**

  

-   **Workload**: Sequencers process transactions and compute state changes.
    -   The computational workload is  **linear**  concerning the number of transactions, assuming state expiry.

  

#### **4. Provers**

  

-   **Proof Generation**: The amount of computational work is  **quasi-linear**.
    -   Provers can be powerful machines like FPGAs, potentially providing specialized hardware for proof generation.
    -   Proof generation involves transitioning from an initial state to a resultant state, considering all relevant transactions.
    -   Each proof is verified by a verifier (often a smart contract on Ethereum).

  

### Proofs and Data Availability

  

-   **Proof Verification**: The process is poly-logarithmic, ensuring efficient computation.
-   **State Diff Data**: The minimal set of data reflecting the difference in world states, ensuring that participants can keep their copies of the world state up-to-date.

  

### Efficiency and Bottlenecks

  

-   **Sequencers**: Limited by computational power and state size.
-   **Provers**: Efficient due to quasi-linear complexity but can benefit from hardware acceleration.
-   **Verifiers**: Efficient due to poly-logarithmic complexity.
-   **Data Availability**: Ensuring the state diff data is accessible for L2 participants, posting it on Ethereum L1.

  

### Exploring Different ZK Proof Systems

  

-   **Properties**: Looks at transparency, universality, and post-quantum security.
    -   Current popular proof systems include  **Starks**,  **Snarks**, and emerging technologies like  **Plonky II**.

  

### Contextualizing ZK Rollups in the Modular Chain Vision

  

-   Discusses the modular approach, separating  **data availability**,  **settlement**, and  **execution**.
-   Potential for layer 3 systems, application-specific rollups on top of platforms like  **StarkNet**.

  

### Future of ZK Rollups

  

-   **Scaling Potential**: The long-term vision of achieving high throughput, potentially reaching over 14 million TPS.
-   **Community Input**: Welcoming ideas for future content, with upcoming topics like  **Cairo**  and  **Optimistic Rollups**.