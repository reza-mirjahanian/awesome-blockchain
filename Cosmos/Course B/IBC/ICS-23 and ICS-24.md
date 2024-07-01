

## IBC Communication and Proof Verification

### Overview

-   **Tracking Headers and Root Hashes**:
    -   Light clients track headers and the root hash of each block.
    -   This is used to communicate between two blockchain chains.

### Key Components

1.  **ICS 24**:
    
    -   Provides a standardized keypad for storing information.
    -   Defines where specific information is stored in blockchains for IBC (Inter-Blockchain Communication).
2.  **ICS 23**:
    
    -   Handles Merkle proofs verification.
    -   Ensures that the data is correctly stored and verified between chains.

### Communication Between Chains

-   **Key Paths and Packet Storage**:
    -   Chains use a predefined path to store packets in their state machines.
    -   Chain A stores the packet at a predefined path, and Chain B verifies it using the same path.
    -   Example: To prove a packet is stored, the counterparty chain stores it under a specific packet path (port ID, channel ID, sequence).

### Verification Process

-   **Verifying Packets**:
    -   When Chain B sends a packet, it includes the height, proof height, proof, port ID, channel ID, sequence, and commitment bytes.
    -   The relayer provides the proof and the consensus state.
    -   The Merkle proof verifies that the commitment bytes are included in the consensus state's root, using the predefined key path.

### Merkle Trees and Proofs

-   **Merkle Tree Structure**:
    
    -   Each header contains an app hash, which is a Merkleized commitment of the entire state.
    -   Data blocks are stored as leaves, continually hashed up to a root hash.
    -   This allows for succinct proofs that data was included in the state.
-   **Merkle Proofs in IBC**:
    
    -   The proof consists of neighboring hashes that, when hashed together, return to the root hash.
    -   Client state specifies the structure of the Merkle proof.
    -   Different clients/chains can have different proof specs depending on their state Merkleization.

### Practical Example

-   **Commitment Bytes**:
    
    -   Instead of storing the entire packet, only the hash of the packet is stored to save state.
    -   The relayer provides the packet and its Merkle proof, which is verified against the stored root hash.
-   **Root Hash and Proofs**:
    
    -   The root hash represents the state of the entire chain, not just the IBC part.
    -   Proofs show that a packet was included in the IBC store, which in turn is included in the final root hash.

### Verification Functions

-   **Various Verifications**:
    -   **Packet-related**: Packet commitments, acknowledgments, non-membership for timeouts.
    -   **Handshake-related**: Channel state, connection state, client state, client consensus state.

### Additional Information

-   **Full Node and Light Client Interaction**:
    -   Light clients query full nodes for data and Merkle proofs.
    -   Full nodes provide the necessary proofs, which light clients verify against the root hash.

### Questions and Clarifications

-   **Common Questions**:
    -   **Root Hash**: It's the hash of the entire chain's state, including IBC-related data.
    -   **Proof Structure**: Proofs include a standardized key path and Merkle proof that shows data inclusion in the state.

----------

