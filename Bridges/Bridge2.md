


# Crypto Bridges: Connecting the Multichain Ecosystem

## Introduction
The crypto space has evolved into a sprawling, multichain ecosystem. Bridges have become increasingly important for connecting and moving between different networks.

## Types of Crypto Bridges

Crypto bridges can be broadly categorized into three types:

### 1. External Validators and Federations

**Mechanism:**
- Uses a group of external validators
- Monitors an address (like a "mailbox") on the source chain
- Locks input tokens and mints output tokens on the destination chain

**Pros:**
- Simple and easy to implement

**Cons:**
- Reliance on trusting external validators
- Raises security questions
- Often issues IOU derivatives instead of native assets

**Examples:** Wormhole, Multichain, Poly Network, Synapse

### 2. Liquidity-Based Bridges

**Mechanism:**
- Nodes on each blockchain hold an inventory or pool of tokens
- Functions similarly to liquidity pools in AMMs
- Users sell tokens to the pool on Blockchain A and receive corresponding assets from a pool on Blockchain B

**Pros:**
- Trustless (relies on smart contracts)
- Issues native assets, which are more fungible

**Cons:**
- Requires deep liquidity on both source and destination chains
- Imbalances may limit transfers in certain directions

**Examples:** Celer, Connext, Hop Protocol

### 3. Light Clients and Relays

**Mechanism:**
- Actors monitor events on the source chain
- Generate and relay actions via proofs of block headers to light clients on the destination chain
- Light clients verify status changes

**Pros:**
- Relatively safe design

**Cons:**
- Slow and resource-intensive
- Requires deployment of clients, oracles, and relayers on every chain

**Examples:** Near Rainbow Bridge, Cosmos IBC, Stargate

## Challenges and Considerations

- No perfect solution for bridges yet
- Trade-offs between extensibility, integration ease, and security
- Users should DYOR (Do Your Own Research) and understand the risks involved

