
#### **Introduction**   
- **Topic**: Axelar’s tech stack and cross-chain communication.  

#### **What is Axelar?**  
- **Purpose**: Provides **secure cross-chain communication** between blockchains.  
- **Vision**: Unify ecosystems so users can:  
  - Access any application with any asset.  
  - Enjoy a **one-click experience** (no wrapped tokens or multiple wallets).  
- **Core Challenge**: Blockchains currently **don’t communicate** natively.  

---  

### **Axelar’s Tech Stack (3 Layers)**  

#### **1. Transport Layer (Bottom Layer)**  
- **Function**: Secure, scalable **cross-chain transaction processing**.  
- **Components**:  
  - **Decentralized Consensus Protocol** (Tendermint-based).  
  - **Validators**:  
    - Run Axelar nodes.  
    - Perform **three core functions**:  
      1. **Consensus mechanism** (agree on transaction validity).  
      2. **Vote on cross-chain events** (monitor chains for transactions to relay).  
      3. **Multi-party signing** (co-sign messages for destination chains).  

#### **2. Gateway Layer (Middle Layer)**  
- **Function**: Acts as a **router** between blockchains.  
- **How it Works**:  
  - **Gateways** (smart contracts on each chain) receive messages from local chains.  
  - Axelar’s network routes messages to other gateways.  
- **Capabilities**:  
  - **Lock & mint assets** (e.g., lock BTC on Ethereum, mint axlBTC on Polygon).  
  - **Send arbitrary messages** (e.g., cross-chain swaps, NFT transfers).  
- **Security**:  
  - Validators **collectively control gateway permissions** (no single entity can authorize messages).  

#### **3. API/SDK Layer (Top Layer)**  
- **Purpose**: Simplifies developer experience.  
- **Key Features**:  
  - **Stripe-like API** for cross-chain messaging.  
  - **Parameters needed**:  
    - Destination chain.  
    - Contract address.  
    - Message payload.  
  - **Automatic translation** between chains (e.g., EVM ↔ Cosmos formats).  
- **Additional Services**:  
  - **Gas Services**: Pay fees in one token (no need for destination-chain gas).  
  - **Monitoring**: Track cross-chain transactions.  

---  

### **Key Innovations**  

#### **General Message Passing (GMP)**  
- **Problem**: Traditional bridges require **wrapped assets** (e.g., wBTC).  
- **Solution**: GMP enables **native cross-chain interactions** without wrapping.  
  - Example: Swap AVAX for ETH directly, no intermediate tokens.  

#### **Permissionless Relaying**  
- **Liveness**: Anyone can relay transactions (no single point of failure).  
  - If one relayer fails, another can take over.  
- **Safety**: Validators ensure **message validity** (not delivery).  

#### **Dynamic Validator Support**  
- Validators **opt-in** to support specific chains.  
- **Incentives**:  
  - Extra inflation rewards for supporting less-secured chains.  
  - Ensures **adequate security** for all connected chains.  

---  

### **Q&A Highlights**  

#### **Is Axelar a DEX?**  
- **No**. Axelar is a **cross-chain communication platform**.  
- DEXs can be **built on top** (e.g., native cross-chain swaps).  

#### **Why Wrapped Tokens?**  
- Current limitation: Most dApps **don’t natively support cross-chain minting**.  
- Future: Axelar aims for **canonical asset representation** (e.g., DAI authorizing mints directly).  

#### **Validator Security**  
- **Why Proof-of-Stake?**  
  - Decentralization + diversity = **stronger security**.  
  - Prevents single points of failure (vs. centralized bridges).  
- **Criticism Response**:  
  - Large, decentralized validator sets **mirror Layer 1 security**.  

#### **Relayers vs. Validators**  
- **Validators**: Vote on cross-chain events (read-only).  
- **Relayers**: Deliver messages (permissionless, anyone can run).  

---  

