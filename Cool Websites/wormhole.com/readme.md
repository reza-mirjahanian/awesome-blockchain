# Wormhole Ecosystem Overview


## What is Wormhole?

**Wormhole** is a decentralized universal message-passing protocol for blockchains. While this is quite a mouthful, let's break it down and explore Wormhole's role in the DeFi ecosystem.

## The Role of Wormhole in the DeFi Ecosystem

### Current Blockchain Landscape
- **Blockchains** like Ethereum, Solana, and Terra have significant value and lively ecosystems.
- However, they are fundamentally incapable of communicating with each other, leading to several issues:
  - No straightforward way to move tokens between blockchains.
  - The primary solution so far has been using centralized bridges and exchanges, which involves trusting third parties with your funds.
  - Smart contracts and applications on different blockchains cannot communicate with each other.

### Wormhole Core Layer
The **Wormhole Core Layer** addresses these communication issues without centralization:
- **Core Contract**: Deployed on each blockchain, allowing contracts to send messages off-chain or verify received messages' authenticity.
- **Guardian Network**: 
  - Observes and signs messages from each chain via the core contract.
  - Comprises 19 guardians, selected from top infrastructure providers and validators.
  - A Wormhole message is valid when two-thirds of the guardians have signed it.
  - Core contracts verify these signatures to assess message authenticity.

### Running a Guardian
- Running a guardian is challenging as it requires observing every blockchain supported by Wormhole.
- The guardians were chosen from the most reputable and capable infrastructure providers globally.

## Wormhole Token Bridge

### Misconception
- Wormhole is often mistakenly seen only as a token bridge. However, it encompasses much more.

### Version 1 vs. Version 2
- **Version 1 Token Bridge**: Launched last year but now deprecated.
- **Version 2 Token Bridge**: 
  - Built on top of the Core Layer.
  - Facilitates token transfers across chains.
  - Is as decentralized as the Core Layer.
  - Primary token bridge for Solana and becoming so for Terra.
  - Holds over $1 billion in total value (TVL), making it one of the highest TVL bridges.

### Features
- Supports NFTs, which is relatively uncommon among token bridges.

## Applications Built on Wormhole

### Benefits of Wormhole
- **Arbitrary Message Passing**: Developers can design applications from a protocol-first perspective.
- **Chain Agnostic**: Applications can operate on multiple blockchains, leveraging the strengths of each.
  - For example, using Solana for transaction offloading due to lower fees and performance, while using Ethereum for final settlement.

### Potential Applications
- Multi-chain DEXs (Decentralized Exchanges)
- Chain-agnostic wallets
- Multi-chain DAOs (Decentralized Autonomous Organizations) with cross-chain governance
- Novel NFT projects

## Conclusion

### Goal
- Wormhole aims to unify the DeFi ecosystem by enhancing interoperability. Applications can be deployed across multiple blockchains instead of being limited to one.



### Resources
- **Open Source**: Wormhole provides TypeScript and Rust SDKs.
- **Example Projects**: Available in the official repository for developers to get started.

