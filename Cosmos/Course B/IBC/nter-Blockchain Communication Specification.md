

# IBC (Inter-Blockchain Communication) Overview

## ICS (Inter-Blockchain Communication Specification) Components

### ICS 23: Vector Commitments
- Focuses on Merkle proofs
- Two main functions:
  1. **Verify membership**: Proves a value is included at a specific path with a given consensus root
  2. **Verify non-membership**: Proves there is no value at a particular path for a given root

### ICS 24: Host State Machine Requirements
- Not discussed in detail

## Packet Handling

- **Sending chain**: 
  - Stores the commitment (hash) of the packet in the IBC module state
  - Key path includes port, channel, and sequence number
- **Full packet data**: 
  - Emitted in events
  - Relayers pick up the full packet data from events
  - Relayers submit the pre-image (full packet) to the counterparty chain

## Client Types and Pruning

### Consensus States
- Multiple consensus states can exist for different heights
- Only consensus states less than two weeks old (within trusting period) are considered valid

### Pruning Logic
- Basic pruning implemented
- Occurs during client updates
- Only prunes the oldest expired consensus state per update
- Reason: To maintain an upper bound on gas costs for relayers

### Future Improvements
- Potential for a separate message to prune all expired states

## Documentation and Technical Details

### Merkle Trees and Storage
- SDK uses IAVL (Immutable AVL) trees for module storage
- Each module has its own IAVL store
- Overall structure is part of the commit multi-store in the SDK

### Documentation Resources
- IAVL repository: github.com/cosmos/iavl
- Limited documentation on multi-store construction
- Suggestion to improve documentation, especially for the multi-store diagram

## Cross-Chain Communication with Non-Cosmos SDK Chains

### Light Client Requirements
- For chains using Tendermint consensus, no additional light client needed
- For new consensus mechanisms (e.g., Solana, NEAR), separate client implementations required

### Wasm Client
- Being developed for IBC-go
- Acts as an intermediary between ICS2 (client handler) and individual smart contracts implementing client interfaces
- Allows adding new client implementations without upgrading the hub
- Smart contracts can implement specific light clients (e.g., Solana, NEAR)

# Key Takeaways

1. IBC uses light clients for efficient cross-chain communication
2. Packet commitments are stored on-chain, with full data in events
3. Pruning helps manage consensus state storage
4. IAVL trees are used for efficient state management and proofs
5. Wasm client development aims to increase flexibility in supporting new chains

This overview provides insights into the technical aspects of IBC, focusing on client types, packet handling, and cross-chain compatibility.