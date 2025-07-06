# AutoNAT in libp2p

## Overview
**AutoNAT** is a libp2p protocol that helps nodes determine if their addresses are publicly reachable, addressing NAT traversal challenges. It uses the `/libp2p/autonat/1.0.0` protocol to verify *observed addresses* provided by other peers.

## How It Works
1. **Address Verification**:
   - A node (e.g., QmAlice) sends a `DialMe` message with its presumed public address to a trusted peer (e.g., QmBob).
   - QmBob attempts to dial QmAlice’s address and responds with a `DialResponse` indicating success or failure.
2. **Confidence Scoring**:
   - Successful dials increase confidence in the address’s reachability.
   - Failed attempts decrease confidence, prompting the node to try alternative addresses.
3. **Dial-Back**:
   - Nodes periodically request dial-backs from multiple peers to confirm address reachability.
   - AutoNAT uses *exponential backoff* for retries to avoid overwhelming the network.

## Key Features
- **No Centralized Server**: Relies on other libp2p peers for validation.
- **Integration**: Works with protocols like Circuit Relay and DCUtR for robust NAT traversal.
- **Security**:
   - Encrypts messages to prevent address spoofing.
   - Limits dial-back requests to trusted peers to avoid abuse.

## Use Case
> Enables nodes behind NATs to confirm their public reachability, improving connectivity in P2P networks without manual configuration.

## Resources
- [AutoNAT Specification](https://github.com/libp2p/specs/blob/master/autonat/README.md) for detailed protocol information.