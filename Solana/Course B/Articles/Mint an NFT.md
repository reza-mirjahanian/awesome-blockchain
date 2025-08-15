# 🔧 Prerequisites & Setup

- 📦 **Node.js** & **npm**  
  *runtime and package manager for JavaScript projects*

- 🔑 **Solana CLI**  
  *used to generate wallets and manage keypairs*

- 🌐 **Helius RPC** account (optional)  
  *boost performance over public RPC endpoints*


# 📥 Installation & Imports

1. Install packages:  
   ```bash
   npm install @solana/web3.js @metaplex-foundation/js
   ```  
   *installs Solana API and Metaplex SDK*

2. Import in `mintNFTs.js`:  
   ```js
   const { Keypair, Connection } = require('@solana/web3.js');
   const { Metaplex, keypairIdentity, bundlrStorage, toMetaplexFile } = require('@metaplex-foundation/js');
   const fs = require('fs');
   ```  
   *sets up blockchain connection, identity, storage, and file handling*


# 🔑 Wallet & Connection

- Generate wallet JSON:  
  ```bash
  solana-keygen new
  ```  
  *stores seed phrase; import into Phantom or Backpack*

- Initialize blockchain connection:  
  ```js
  const connection = new Connection('https://api.mainnet-beta.solana.com');
  ```

- Create `Metaplex` instance:  
  ```js
  const metaplex = Metaplex.make(connection)
    .use(keypairIdentity(walletKeypair))
    .use(bundlrStorage());
  ```  
  *configures identity and Bundlr for metadata storage*


# 🎨 Creating NFT Metadata

- Function signature:  
  ```js
  async function createMetadata(imageName) { … }
  ```

- Steps inside `createMetadata`:
  - 🖼️ Read image file:  
    `fs.readFileSync(imageName)`
  - 📝 Convert to Metaplex file:  
    `toMetaplexFile(buffer, 'fileName')`
  - 📡 Upload metadata:  
    ```js
    const uri = await metaplex.nfts().uploadMetadata({
      name: 'Helius NFT',
      description: 'Helius NFT created in the SolanaDev 101 course',
      image: metaplexFile,
      attributes: [
        { trait_type: 'Test', value: 'Yes' },
        { trait_type: 'Logo', value: 'Helius' },
      ],
    });
    ```
  - 🌍 *Returns metadata URI pointing to on-chain JSON*


# 🛠️ Minting the NFT

1. Call metadata creation:  
   ```js
   const metadata = await createMetadata('./heliusLogo.png');
   ```

2. Mint NFT via Metaplex:  
   ```js
   const nft = await metaplex.nfts().create({
     uri: metadata.uri,
     name: 'Helius NFT',
     seller_fee_basis_points: 500,     // 5% royalties
     creators: [
       { address: wallet.publicKey, verified: true, share: 100 }
     ],
   });
   ```

3. Retrieve mint address:  
   ```js
   console.log('NFT Mint Address:', nft.mintAddress.toBase58());
   ```  
   *use address to view on a block explorer*


# ✨ Core Concepts & Tips

- **Metaplex SDK**  
  central toolkit for minting, uploading, and managing NFTs on Solana

- **Bundlr Storage**  
  decentralized provider for storing NFT assets and metadata

- **seller_fee_basis_points**  
  determines royalty percentage (e.g., 500 = 5%)

- **Creators array**  
  defines royalty distribution and verification per address

- Keep private keys secure 🔐 and never expose them publicly  

- Use fast RPC endpoints (Helius) ⚡ for reliable performance and archival data