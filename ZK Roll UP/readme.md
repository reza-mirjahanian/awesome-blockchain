
# Rollups and Ethereum Scaling

## Introduction

**Questions to Address:**
- What are rollups?
- Difference between optimistic and ZK rollups?
- How is Arbitrum different from Optimism?
- Why are rollups considered the holy grail for scaling Ethereum?



## Ethereum Scaling Overview

### High Demand and Gas Fees

- **High Network Activity**: During CryptoKitties craze (2017), DeFi Summer (2020), and the crypto bull market (2021).
- **Issue**: High gas fees making transactions expensive for everyday users.

### Scaling Solutions

1. **Layer 1 Scaling**: Enhancements to the blockchain itself.
   - Example: Eth2 (Proof-of-Stake, sharding).

2. **Layer 2 Scaling**: Building on top of the existing blockchain.
   - Example: Channels, Rollups.

3. **Sidechains**: Building alongside the main blockchain.
   - Example: EVM-compatible sidechains.

## Eth2 and Sharding

- **Eth2**: Migration to Proof-of-Stake (PoS) and incorporating sharding.
- **Sharding**: Increases Ethereum's throughput, especially when combined with rollups.

## Layer 2 Scaling Solutions

### Channels vs. Sidechains

- **Channels**: Secure but application-specific.
- **Sidechains**: General-purpose but less secure due to independent consensus models.

### Rollups: The Best of Both Worlds

- **Security**: Relies on Ethereum's security.
- **General-Purpose**: Deploy existing smart contracts with minimal changes.

## Understanding Rollups

### What Are Rollups?

- **Definition**: Execute transactions off-chain, post transaction data on Layer 1.
- **Benefits**: Scales network while maintaining Ethereum's security.

### How Rollups Work

1. **Execute Transactions**: On a separate chain.
2. **Batch Transactions**: Group transactions and post them on Ethereum.
3. **Compress Data**: Roll up data to fit into Ethereum blocks.

### Ensuring Data Validity

- **Smart Contracts**: Deployed on Layer 1 to process deposits, withdrawals, and verify proofs.
- **Proof Types**:
  - **Optimistic Rollups**: Use fraud proofs.
  - **ZK Rollups**: Use validity proofs (ZK-SNARK).

## Optimistic Rollups

### Key Features

- **Assume Validity**: Data is assumed correct unless challenged.
- **Dispute Resolution**: Verifies fraud proofs and penalizes fraudulent transactions.
- **Bonding**: Requires bond in ETH for batch submitters and fraud proof submitters.

### Advantages and Challenges

- **Time for Fraud Proofs**: Requires a waiting period (up to weeks) for fraud proofs.
- **Fast Withdrawals**: Solutions like Hop protocol and Connext offer faster withdrawals.

## ZK Rollups

### Key Features

- **Validity Proofs**: Uses ZK-SNARKs to verify transactions.
- **Immediate Withdrawals**: Funds available as soon as batch and proof are submitted.

### Advantages and Challenges

- **EVM Compatibility**: Harder to achieve but projects like ZKSync are progressing.
- **Computation-Heavy**: Requires high-spec machines for ZK proofs.

## Comparing Rollups

### Transaction Speed

- **Both Types**: Can scale Ethereum from 15-45 TPS to 1000-4000 TPS.
- **Eth2 Synergy**: Sharding increases data availability, boosting transaction speed to 100k TPS.

## Projects and Ecosystem

### Optimistic Rollups

- **Popular Options**: Optimism, Arbitrum.
- **Notable Projects**: Uniswap, Sushi, Bancor, Aave.
- **Reddit Partnership**: Arbitrum working with Reddit on a reward system rollup.

### Differences Between Arbitrum and Optimism

- **Dispute Resolution**: Arbitrum's multi-round model vs. Optimism's full transaction rerun.
- **Transaction Ordering**: Different approaches to MEV (Miner Extractable Value).

### Other Optimistic Rollups

- **Examples**: Fuel, OMGX, Cartesi.

### ZK Rollups

- **Examples**: Loopring, Hermez, ZKTube, Aztec, StarkWare, ZKSync.
- **Focus Areas**: Scaling payments, privacy features, EVM compatibility.

## Impact on DeFi

### Benefits

- **Lower Fees**: Enables more users to transact during high network activity.
- **New Applications**: Allows for cheaper transactions and faster confirmations.

### Challenges

- **Composability**: Requires all protocols to be deployed on the same rollup.
- **Fractured Liquidity**: Liquidity split between Layer 1 and multiple rollups.
- **Winners and Losers**: Some rollups may become ghost towns.

### Future Outlook

- **User Behavior**: Users may stay within one rollup ecosystem.
- **Centralized Exchanges**: May offer direct deposits/withdrawals to/from rollups.

## Rollups vs. Sidechains

- **Coexistence**: Sidechains may still be relevant for certain applications.
- **Cost**: Rollup fees may still be high enough to price out some applications.

## Conclusion

### Ethereum's Rollup-Centric Strategy

- **Vitalik Buterin's Post**: Recommended reading on rollup-centric Ethereum roadmap.

