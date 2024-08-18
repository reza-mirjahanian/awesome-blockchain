**Cross-Chain Token Transfers**
-------------------------------

-   **Token Movement:**
    -   Lock token on one blockchain.
    -   Represent the token on another blockchain.
    -   Ensure the second blockchain knows the token is locked to avoid token creation out of thin air.

**Proof Mechanisms**
--------------------

-   **Challenge:** Proving to the second blockchain that the token is locked on the first blockchain.
-   **Solution:**
    -   Use **light clients** for proof.
    -   **Light Clients:** Sparse representations of a blockchain, skipping blocks to reduce space requirements.

**IBC (Inter-Blockchain Communication) Overview**
-------------------------------------------------

-   **Application Logic:** Needs proof and consensus information about other blockchains.
-   **Generalization:**
    -   **IBC Model:** Based on TCP, to handle various applications.
    -   **Structure:** Bottom-up approach (as opposed to top-down in sharding).

**IBC Structure Breakdown**
---------------------------

-   **IBC Clients:**

    -   **Purpose:** Verify and keep track of blockchain states.
    -   **Components:**
        -   **Client State:** Encodes verification parameters and security guarantees.
        -   **Consensus State:** Lighter version of a block, containing only essential information (validator hash and root hash).
-   **Supported Blockchains:**

    -   Initially, **Tendermint clients** and **Solo Machine** (a proof of authority client).
    -   Plans to support other blockchains like Ethereum and Polkadot with different consensus algorithms.

**Light Clients**
-----------------

-   **Functionality:**
    -   Verify blocks and transactions.
    -   Allow a blockchain (e.g., Hub) to prove a transaction from another blockchain (e.g., Ethereum).

**IBC Connections**
-------------------

-   **Connections Layer:**
    -   **Purpose:** Establishes communication between blockchains.
    -   **Process:**
        -   Uses a **connection handshake** modeled after TCP.
        -   Four Steps: **Init, Try, Ack, and Confirm**.
    -   **Verification:** Ensures that both blockchains are accurately representing each other.

**Channels and Packets**
------------------------

-   **Channels:**
    -   **Purpose:** Facilitate application communication on top of established connections.
    -   **Components:**
        -   **Channel ID:** Uniquely identifies the channel.
        -   **Port ID:** Specifies the application in use.
-   **Packets:**
    -   **Purpose:** Data transfer between blockchains.
    -   **Content:** Includes channel ID, port ID, and application data.
    -   **Processing:**
        -   Handled by the module (e.g., transfer module checks user balance).
        -   Creation of a **packet commitment** (a hash of packet data, timestamp, and sequence).
    -   **Minimalistic Design:** Uses a hash instead of storing the entire packet, ensuring efficient state management.

**Sending and Receiving Packets**
---------------------------------

-   **Sending Process:**
    -   User initiates a transaction.
    -   Packet is created, hashed, and stored on the originating blockchain.
    -   The hash is committed to the blockchain's state.
-   **Receiving Process:**
    -   The receiving blockchain processes the packet and validates it based on the commitment from the sending blockchain.
-   


----


# Interblockchain Communication (IBC) Overview

## Cross-Chain Token Transfers

- **Current Application**: Cross-chain token transfers
- **Process**:
  1. Lock token on one chain
  2. Represent it on another chain
  3. Prove the lock-up occurred

## Light Clients

- **Purpose**: Provide sparse representations of blockchains
- **Advantage**: Reduced space requirements compared to full nodes
- **Example**:
  - Full node: Block 1 → Block 2 → Block 3 → ...
  - Light client: Block 1 → (skip 12) → Block 13 → (skip 20) → Block 33

## IBC Structure

1. **Application Logic**
2. **Proof System**
3. **Chain-Specific Information**

## IBC Layers (Bottom-Up Approach)

### 1. IBC Clients (Light Client Algorithms)

- **Components**:
  1. **Client State**:
     - Contains parameters for verification
     - Encodes security guarantees
  2. **Consensus State**:
     - Mapped to a specific height
     - Contains:
       - Hash of the validator set
       - Root hash for Tendermint

- **Implementations**:
  - Tendermint clients
  - Solo machine (public/private key pair)

### 2. Connections

- **Purpose**: Establish and maintain communication between chains
- **Connection Handshake** (4 steps):
  1. **Init**: Initial "Hello"
  2. **Try**: Response to "Hello"
  3. **Ack**: Acknowledgment of response
  4. **Confirm**: Confirmation of acknowledgment

- **Handshake Benefits**:
  - Ensures both chains are on the same page
  - Verifies correct client representation

### 3. Channels

- **Components**:
  - Channel ID (unique identifier)
  - Port ID (specifies the application)

- **Function**: Facilitate packet communication

### 4. Packets

- **Purpose**: Transfer data between chains
- **Contents**:
  - Channel and port IDs (source and destination)
  - Application data (opaque byte stream)

- **Packet Commitment**:
  - Hash of specific packet parts (data, timestamp, sequence)
  - Stored in chain state under a specific key

## IBC Design Philosophy

- **Minimalistic Approach**:
  1. Use of light clients instead of full nodes
  2. Packet commitments (hashes) instead of full packets
