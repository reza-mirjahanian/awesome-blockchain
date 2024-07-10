
# What is OP Mainnet?

OP Mainnet is an [EVM equivalent](https://medium.com/ethereum-optimism/introducing-evm-equivalence-5c2021deb306)  [Optimistic Rollup](https://research.paradigm.xyz/rollups) chain. It's designed to be fast, simple, and secure.

# Why use OP Mainnet?
OP Mainnet lets you send transactions, similar to Ethereum, but with two important advantages, and one feature that stays exactly the same.

-   **Faster transactions.**  OP Mainnet produces blocks every 2 seconds (as opposed to every 12 seconds on Ethereum).
    
-   **Much lower transaction fees.**  Typically between a 1% and 10% of the cost on the Ethereum mainnet (also known as layer 1 or L1).  [For the current gas price and a few sample transactions' costs see here](https://public-grafana.optimism.io/d/9hkhMxn7z/public-dashboard?orgId=1&refresh=5m).
    
-   **Decentralization:** All transactions are posted to L1 Ethereum, inheriting the strong security guarantees of Ethereum.


----
# How does OP Mainnet work?
## Summary

OP Mainnet is an  [optimistic rollup](https://ethereum.org/en/developers/docs/scaling/layer-2-rollups/#optimistic-rollups). When you submit a transaction, the transaction's data is written to the Ethereum mainnet, but the **transaction itself** is executed by a single computer, the **sequencer** (instead of thousands of computers as Ethereum does it). This significantly reduces the transaction cost, while a challenge mechanism ensures the sequencer is honest.

## Go deeper

[Ethereum has limited capacity](https://ethereum.org/en/layer-2/) which can generally be understood due to constraints of the  [blockchain trilemma](https://medium.com/certik/the-blockchain-trilemma-decentralized-scalable-and-secure-e9d8c41a87b3). To have decentralization we need to enable as many people as possible to run a node, which means that the number of transactions that can be processed per minute is limited by the capacity of standard hardware. This limits our ability to scale, unless we accept the security implication of not having every node process every transaction.  
  

Solutions that build on top on Ethereum are called layer 2 or L2 (with Ethereum itself called layer 1 or L1). OP Mainnet uses a type of solution called an  [optimistic rollup](https://ethereum.org/en/developers/docs/scaling/optimistic-rollups/).  _Rollups_ post all transactions on layer 1, so **data integrity and availability are provided by Ethereum**.  
  

_Optimistic Rollups_ use economic incentives to ensure the data processing, done off chain, is done correctly. The  _sequencer_ node posts the **merkle root** of the blockchain state on L1 (called the  _state root_). Other nodes, called  _verifiers_, can issue  _fault challenges_if they believe the state root is incorrect. In the case of a fault challenge part of the transaction is executed on L1 to verify which is the correct state root.  
  

Sequencers that post correct state roots, and verifiers that challenge incorrect ones, are rewarded for their honesty. Sequencers that post incorrect state roots are penalized for dishonesty. **Verifiers** that challenge correct results, which could be used as a denial of service attack, are penalized. If a state is not challenged for the challenge period (seven days on OP Mainnet), it is assumed to be correct. As long as there is at least one honest verifier, the state will end up being the correct one - and the economic incentives are aligned with honesty.  
  

[We have a bridge](https://app.optimism.io/bridge)  that allows users to deposit into OP Mainnet and withdraw from it using this mechanism. A **withdrawal** requires you to **waiting the challenge** period (until the blockchain state becomes indisputable), but **faster withdrawals are** available from  [third party bridges](https://www.optimism.io/apps/bridges)  that run their own verifiers so they  **know** the state submitted is correct.  
  

------------------

## Breakdown of Optimism for Non-Techies 

### Agenda
1. **Why Roll-ups?**
   - Importance of improving Ethereum.
   - Current performance issues of Ethereum.
2. **Roll-ups as the Solution**
   - Explanation of roll-ups.
   - Employee perspective from Optimism on why roll-ups are effective.
3. **Bridges**
   - Mechanisms to transfer messages and assets between Layer 1 (Ethereum) and roll-ups.
4. **Additional Resources**
   - Guidance on where to learn more about the topic.

### Why Not Just Use Ethereum?
- **Ethereum's Greatness and Limitations**: 
  - Limited capacity leads to congestion.
  - High gas costs due to every transaction being executed by numerous nodes.
- **Scalability Issues**:
  - High transaction costs make it unaffordable for small transactions.

### Need for a Scalable System
- **Analogy**: Comparing Ethereum's role to a court system for dispute resolution, which is slow and expensive but necessary for enforcing rules.
- **Goal**: Use Ethereum primarily for dispute resolution while handling most transactions off-chain to reduce load and costs.

### Background Information: Blockchain Trilemma
- **Security**: Ensuring trustworthy information.
- **Decentralization**: Avoiding central authority to maintain trust.
- **Scalability**: Ability to handle numerous transactions per second.
- **Challenge**: Achieving all three simultaneously is difficult.

### Ethereum Features
- **Data Integrity**: Transactions are signed and can't be faked.
- **Data Availability**: Every node has the data to verify the blockchain state.
- **Secure Computation and Storage**: Thousands of nodes ensure accuracy and security.

### Layer 1 vs. Layer 2
- **Layer 1 (Ethereum)**: Core blockchain.
- **Layer 2 (Optimism, Arbitrum, etc.)**: Built on top of Ethereum, offloading tasks that don't need to be on the main chain to improve scalability.

### Roll-ups
- **Definition**: Roll-ups write all transactions to Ethereum but process them off-chain.
- **Types**:
  - **Zero-Knowledge Roll-ups**: Use complex mathematics to prove transactions are correct.
  - **Optimistic Roll-ups**: Post results on Ethereum with a dispute window for challenges.

### Node Software
- **Node Software**: Runs the network and processes transactions.
- **Virtual Machines**: Software that simulates a computer within another computer, like the Ethereum Virtual Machine (EVM).

### Optimistic Roll-ups
- **Mechanism**: Uses economic incentives rather than complex math.
- **Dispute Process**: 
  - **Proposer**: Suggests post-transaction state.
  - **Verifier**: Challenges incorrect states.
  - **Arbiter**: Smart contract decides the outcome by running disputed steps.

### Fault Challenges
- **Process**: Identify the step where disagreement occurs through bisection and execute that step on-chain.
- **Entities Involved**:
  - **Proposer**: Suggests state.
  - **Verifier**: Challenges incorrect proposals.
  - **Arbiter**: Runs the disputed step to determine correctness.

### How It Works
- **Bisection Method**: Identify where disagreement starts and narrow it down to a specific step.
- **Virtual Machines**: Translate and execute disputed steps on Ethereum.

### Bridging Assets
- **Trustless Bridges**: Use the same mechanisms as roll-ups to transfer assets between Layer 1 and Layer 2.
- **Trusted Bridges**: Faster but require trust in the bridge operator.

### Trustless Bridge Deposits and Withdrawals
- **Deposits**: User sends assets to Layer 1 bridge, which locks them and notifies Layer 2 bridge to mint equivalent assets.
- **Withdrawals**: User sends assets to Layer 2 bridge, which burns them and notifies Layer 1 bridge to release equivalent assets after a dispute window.

### Trusted Bridges
- **Faster Transactions**: Directly handle assets but require trust in the bridge operator.
- **Liquidity Providers**: Provide assets for the bridge, balancing asset pools between Layer 1 and Layer 2.

### Additional Resources
- **Help Center**: [help.optimism.io](https://help.optimism.io)
- **Optimism Website**: [optimism.io](https://optimism.io)
- **Discord**: [discord.gateway.optimism.io](https://discord.gateway.optimism.io)
- **Getting Started**: [app.optimism.io/get-started](https://app.optimism.io/get-started)
- **Playful Learning**: [op.op.atlantis.world](https://op.op.atlantis.world)

### Conclusion
- **Optimism's Promise**: Lower costs and improved scalability for Ethereum.
- **Engagement**: Encouragement to explore resources and join the community.

