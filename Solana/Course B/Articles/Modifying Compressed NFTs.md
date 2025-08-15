## 🧠 Core Concepts of Compressed NFTs on Solana

### 🌳 **Compressed NFTs (cNFTs)**
- *Efficient, scalable NFTs stored off-chain using Merkle trees.*
- *Require Merkle proofs for any modification (e.g., transfer, burn).*

### 🧪 **Merkle Proofs**
- *Set of hashes proving a leaf (NFT) belongs to a Merkle tree.*
- *Essential for validating ownership and enabling changes.*

### 🧰 **Digital Asset Standard (DAS) API**
- *Provides access to asset data and Merkle proofs.*
- *Used to fetch creator hash, data hash, leaf nonce, and ownership info.*

---

## 🔧 Available Modifications via Bubblegum Program

- **Mint**
- **Transfer**
- **Burn**
- **Delegate / CancelDelegate**
- **Redeem / CancelRedeem**
- **Decompress**
- **VerifyCreator / SetAndVerifyCreator**
- **VerifyCollection / SetAndVerifyCollection**

---

## 🔄 Transfer Compressed NFT

### 📝 Required Inputs
- **Connection** – *Solana RPC connection.*
- **Current Owner** – *Public key of current NFT holder.*
- **New Owner** – *Public key of recipient.*
- **Asset ID** – *Unique identifier of the NFT.*

### 🔍 Steps
1. **Fetch Merkle Proof** – `getAssetProof(assetId)`
2. **Compute Proof Path** – *Convert proof nodes to public keys.*
3. **Fetch Asset Data** – `getAsset(assetId)` to get:
   - *Leaf nonce*
   - *Owner/delegate*
   - *Data hash*
   - *Creator hash*
4. **Get Tree Authority** – `getBubblegumAuthorityPDA(tree_id)`
5. **Create Transfer Instruction** – *Includes all fetched data and program IDs.*
6. **Submit Transaction** – *Using `sendAndConfirmTransaction`.*

---

## 🔥 Burn Compressed NFT

### 📝 Required Inputs
- **Connection** – *Solana RPC connection.*
- **Owner** – *Public key of NFT holder.*
- **Asset ID** – *Identifier of NFT to burn.*

### 🔍 Steps
1. **Fetch Merkle Proof** – `getAssetProof(assetId)`
2. **Fetch Asset Data** – `getAsset(assetId)` to get:
   - *Leaf nonce*
   - *Owner/delegate*
   - *Data hash*
   - *Creator hash*
3. **Get Tree Authority** – `getBubblegumAuthorityPDA(tree_id)`
4. **Create Burn Instruction** – *Similar to transfer, but purges NFT from tree.*
5. **Submit Transaction** – *Using `sendAndConfirmTransaction`.*

### 🧨 Effects
- *NFT is removed from Merkle tree.*
- *Flagged as `burnt=true` in DAS API responses.*

---

## 💡 Developer Insights

- **Modifications require Merkle proofs and asset metadata.**
- **DAS API is central to interacting with compressed NFTs.**
- **Transfer and burn operations follow similar structure.**
- **Compressed NFTs enable scalable, low-cost NFT interactions.**