# All You Need to Know About Compression on Solana  

## **State vs. Ledger Compression**  

### **State Compression**  
- **Definition**: Reduces the size of on-chain account data while maintaining accessibility.  
- **Purpose**: Lowers storage costs and improves efficiency.  
- **Key Techniques**:  
  - **Merkle Trees**: Hashes data into a compact structure for verification.  
  - **Sparse Merkle Trees**: Optimizes storage by only tracking changes.  
- **Use Cases**:  
  - NFT collections (e.g., compressed NFTs).  
  - Dynamic on-chain data (e.g., game states).  

### **Ledger Compression**  
- **Definition**: Minimizes the size of the blockchain's transaction history.  
- **Purpose**: Reduces node storage requirements and sync times.  
- **Key Techniques**:  
  - **Snapshotting**: Stores periodic checkpoints instead of full history.  
  - **Pruning**: Removes old, non-essential data.  
- **Use Cases**:  
  - Light clients (e.g., mobile wallets).  
  - Historical data archiving.  

## **How Compression Works on Solana**  

### **1. State Compression (Merkle Trees)**  
- **Step 1**: Data is hashed and stored in a Merkle tree.  
- **Step 2**: Only the root hash is stored on-chain.  
- **Step 3**: Proofs validate data integrity off-chain.  

```rust  
// Example: Compressed NFT metadata stored off-chain  
let root_hash = compute_merkle_root(metadata_leaves);  
```  

### **2. Ledger Compression (Snapshotting)**  
- **Step 1**: Validators agree on a snapshot at a specific slot.  
- **Step 2**: Old transactions are pruned or archived.  
- **Step 3**: New nodes sync from the latest snapshot.  

> **Note**: Ledger compression does not affect consensus—only storage efficiency.  

## **Benefits of Compression**  
- **Cost Reduction**: Lower storage costs for developers.  
- **Scalability**: Enables more data on-chain without bloating the ledger.  
- **Performance**: Faster sync times for new nodes.  

## **Trade-offs**  
- **State Compression**: Requires off-chain infrastructure for proofs.  
- **Ledger Compression**: Limits access to full historical data.



----


# 🌲 State Compression & Compressed NFTs on Solana

> *Mint 10 million NFTs for ≈ 50 SOL instead of 1 million USD.*  
> This is made possible by **state compression**, a technique that stores the *cryptographic proof* of data on-chain while the actual data lives in the immutable ledger.

---

## 1. Myth-busting

| Misconception | Reality |
|---------------|---------|
| **“It’s like zipping a file.”** | ❌ It’s **not** lossless/lossy compression. It’s a *Merkle-root* + *ledger* trick. |
| **“Off-chain = unsafe.”** | ❌ Data is still *on-chain* (ledger); the *root* is in state for fast verification. |
| **“Lose my indexer → lose my tree.”** | ❌ The entire Merkle tree can be **rebuilt from the ledger**. |
| **“Concurrent = parallel execution.”** | ❌ Validators still process updates **sequentially** within a slot; the tree’s *changelog* just keeps proofs valid. |
| **“Tree == Collection.”** | ❌ One collection can span **many trees**. One tree can hold **millions** of NFTs. |

---

## 2. State vs Ledger

| Ledger | State |
|--------|-------|
| **Append-only** history. | **Mutable** snapshot. |
| Stored by **archival nodes**. | Held in **validator RAM**. |
| Used for **verification**. | Used for **execution**. |

> Alice → Bob 10 SOL  
> *Ledger entry* records the transfer for eternity.  
> *State* updates balances instantly.

---

## 3. Compressed NFT (cNFT) Essentials

### 🔑 Key Differences
- cNFT ➞ uncompressed NFT **possible**, reverse **not**.
- No token/mint/metadata accounts; instead:
  - **Asset ID** (stable identifier).
  - **Merkle tree account** (one tree → millions of NFTs).
- All changes via **Bubblegum program**.
- Read via **DAS API** (indexers).

---

## 4. Concurrent Merkle Tree Deep-dive

### 4.1 Anatomy
- **Max depth** `d` → `2^d` leaves.
- **Max buffer size** → how many *leaf replacements* per slot keep root valid.
- **Canopy depth** → cached proofs for smaller tx size.

> Rule of thumb: `maxDepth - canopyDepth ≤ 10` for marketplaces.

### 4.2 Cost Snapshot
| NFTs to store | Optimal depth | Rent (SOL) |
|---------------|---------------|------------|
| 100 | 7 | 0.006 |
| 1 M | 24 | 3.8 |
| 10 M | 27 | 7.7 |

---

## 5. Creating a Tree (Bubblegum SDK)

### 5.1 Ingredients
- `maxDepth`, `maxBufferSize` (from `ALL_DEPTH_SIZE_PAIRS`)
- `canopyDepth`
- Payer keypair
- New keypair for the tree account

### 5.2 Recipe (TypeScript)

```ts
import { Connection, Keypair, Transaction } from "@solana/web3.js";
import { createAllocTreeIx, createCreateTreeInstruction } from "@solana/spl-account-compression";
import { PROGRAM_ID } from "@metaplex-foundation/mpl-bubblegum";

async function createTree(
  connection: Connection,
  payer: Keypair,
  treeKeypair: Keypair,
  maxDepth: number,
  maxBufferSize: number,
  canopyDepth: number
) {
  const alloc = await createAllocTreeIx(connection, treeKeypair.publicKey, payer.publicKey, { maxDepth, maxBufferSize }, canopyDepth);
  const [treeAuth] = PublicKey.findProgramAddressSync([treeKeypair.publicKey.toBuffer()], PROGRAM_ID);
  const create = createCreateTreeInstruction(
    { payer: payer.publicKey, treeCreator: payer.publicKey, treeAuthority: treeAuth, merkleTree: treeKeypair.publicKey },
    { maxBufferSize, maxDepth, public: false },
    PROGRAM_ID
  );
  const tx = new Transaction().add(alloc, create);
  await sendAndConfirmTransaction(connection, tx, [payer, treeKeypair], { skipPreflight: true });
}
```

> 🪄 Umi one-liner:  
> `await createTree(umi, { merkleTree, maxDepth: 14, maxBufferSize: 64 }).sendAndConfirm(umi);`

---

## 6. Minting cNFTs

### 6.1 Prerequisites
- Tree address
- Collection mint + metadata + master-edition accounts
- Metadata JSON (on-chain or Arweave)

### 6.2 Bubblegum SDK (with collection)

```ts
createMintToCollectionV1Instruction(
  {
    payer, merkleTree, treeAuthority, leafOwner, collectionMint, collectionMetadata, editionAccount, compressionProgram, ...
  },
  { metadataArgs }
)
```

### 6.3 Umi (no collection)

```ts
await mintV1(umi, {
  leafOwner,
  merkleTree,
  metadata: {
    name: "My cNFT",
    uri: "https://arweave.net/abc123.json",
    sellerFeeBasisPoints: 500,
    creators: [{ address: umi.identity.publicKey, share: 100, verified: false }],
    collection: none()
  }
}).sendAndConfirm(umi);
```

### 6.4 Helius Mint API (hands-off)

```json5
POST https://mainnet.helius-rpc.com/?api-key=***
{
  "jsonrpc": "2.0",
  "id": "helius-test",
  "method": "mintCompressedNft",
  "params": {
    "name": "Exodia",
    "symbol": "ETFO",
    "owner": "DCQnfUH6mHA333mzkU22b4hMvyqcejUBociodq8bB5HF",
    "description": "Legendary creature...",
    "imageUrl": "https://...jpg",
    "attributes": [ { "trait_type": "Type", "value": "Legendary" } ],
    "sellerFeeBasisPoints": 6900
  }
}
```

---

## 7. Transferring cNFTs

### 7.1 Workflow
1. **DAS API** → `getAsset` (owner, delegate, leaf_id, hashes)  
2. **DAS API** → `getAssetProof` (root, proof, tree_id)  
3. Build **transfer instruction** with proof path.  
4. Submit transaction.

### 7.2 Raw web3.js snippet

```ts
const proofPath: AccountMeta[] = proof
  .slice(0, proof.length - canopyDepth)
  .map(p => ({ pubkey: new PublicKey(p), isSigner: false, isWritable: false }));

const ix = createTransferInstruction(
  {
    merkleTree, treeAuthority, leafOwner, leafDelegate, newLeafOwner,
    anchorRemainingAccounts: proofPath,
  },
  {
    root: [...new PublicKey(root).toBytes()],
    dataHash: [...new PublicKey(dataHash).toBytes()],
    creatorHash: [...new PublicKey(creatorHash).toBytes()],
    nonce: leafId,
    index: leafId,
  }
);
```

### 7.3 Umi shortcut

```ts
const assetWithProof = await getAssetWithProof(umi, assetId);
await transfer(umi, {
  ...assetWithProof,
  leafOwner,
  newLeafOwner: recipient
}).sendAndConfirm(umi);
```

---

## 8. Toolbox

- **@solana/spl-account-compression** – tree math & account sizing  
- **@metaplex-foundation/mpl-bubblegum** – mint, transfer, delegate  
- **Umi framework** – zero-dependency, composable SDK  
- **DAS API endpoints** – `getAsset`, `getAssetProof`, `getAssetsByOwner`  
- **Helius Mint API** – fully managed cNFT creation + metadata upload