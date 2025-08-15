# **Key Points: Uploading Files to Shadow Drive on Solana**  

## **🔍 Core Concepts**  
- **Shadow Drive**: *A decentralized storage solution built on Solana, enabling secure and scalable file storage.*  
- **Decentralized Storage**: *Files are stored across a distributed network, reducing reliance on centralized servers.*  
- **Solana Blockchain**: *Used for transaction verification and metadata storage, ensuring fast and low-cost operations.*  

## **🚀 Key Steps for Uploading Files**  
1. **Set Up Environment**  
   - Install required dependencies (`@shadow-drive/sdk`).  
   - Configure Solana wallet connection.  

2. **Initialize Shadow Drive Client**  
   - Create a client instance using a Solana keypair.  

3. **Create Storage Account**  
   - *A dedicated account to manage stored files.*  
   - Specify storage size (e.g., 1GB).  

4. **Upload Files**  
   - Use `uploadFile()` method to send files to Shadow Drive.  
   - Supports various file types (images, docs, etc.).  

5. **Retrieve File References**  
   - Files are stored with unique identifiers (URIs) for access.  

## **💡 Important Insights**  
- **Cost Efficiency**: *Storage costs are paid in SOL, with fees based on size and duration.*  
- **Immutable Storage**: *Once uploaded, files cannot be modified (ensures data integrity).*  
- **Fast Retrieval**: *Leverages Solana’s high throughput for quick file access.*  

## **⚠️ Considerations**  
- **Wallet Requirements**: *Need a funded Solana wallet for transactions.*  
- **Storage Limits**: *Account size must be pre-allocated (cannot be dynamically resized).*  
- **Persistence**: *Files remain stored as long as the storage account is active and funded.*  

## **📌 Best Practices**  
- **Optimize File Size**: *Compress files to reduce storage costs.*  
- **Secure Wallet**: *Use a dedicated wallet for storage operations.*  
- **Monitor Storage**: *Regularly check account balance to avoid service interruptions.*


---------


## 🚀 Overview  
- **Solana Storage Limitations** 🪙 – *On-chain storage is costly and limited for large data.*  
- **Shadow Drive (SHDW)** – *Off-chain, decentralized storage by GenesysGo; cost-efficient and performant.*  
- **Token Requirement** – *Needs SHDW token (acquired via SOL → SHDW swap) to create storage accounts.*  
- **Mainnet Only** – *No devnet option for SHDW currently.*  

---

## 🛠 Prerequisites  
- **Backpack Wallet Extension** – *For managing Solana wallets.*  
- **Node.js & ts-node** – *For running TypeScript scripts.*  
- **Solana CLI** – *For wallet generation and blockchain interaction.*  

---

## 📂 Environment Setup  
1. **Create Project Folder** – *e.g., `Shadow` with `upload.ts` file.*  
2. **Install Packages** 📦 – `@shadow-drive/sdk` & `@project-serum/anchor`.  
3. **Generate Wallet** 🔑 – `solana-keygen grind` to create a keypair.  
4. **Prepare Test File** – *Example: `helius.txt` with sample content.*  
5. **Import Wallet to Backpack** – *Use private key from generated wallet.*  
6. **Fund Wallet** 💰 – *At least 0.03 SOL for testing.*  

---

## 💱 Getting SHDW Token  
- **Swap via Jupiter** – *Exchange small amount of SOL for SHDW (e.g., 0.01 SOL ≈ <1 SHDW).*  
- **Wallet Balance Check** – *Ensure ≥ 1 SHDW + 0.01 SOL before proceeding.*  

---

## 🧩 Core Steps to Upload Files  

### 1️⃣ Connect to SHDW Network  
- **Import Modules** – `@project-serum/anchor`, `@solana/web3.js`, `@shadow-drive/sdk`, `fs`.  
- **Initialize Drive** – *Create connection with RPC endpoint & wallet.*  

### 2️⃣ Create Storage Account  
- **Function** – `drive.createStorageAccount(name, size, version)`.  
- **Output** – *Returns storage account ID & transaction signature.*  
- **Requirement** – *Wallet must have required SOL & SHDW balance.*  

### 3️⃣ Upload File  
- **Retrieve Accounts** – `drive.getStorageAccounts(version)`.  
- **Read File** – *Load file into buffer with `fs.readFileSync`.*  
- **Prepare ShadowFile Object** – *Includes `name` & `file` buffer.*  
- **Upload** – `drive.uploadFile(accountPublicKey, fileObject)`.  
- **Result** – *Returns finalized file URL & transaction details.*  

---

## 📌 Key Insights & Takeaways  
- **Cost Efficiency** 💲 – *Uploading small files costs fractions of SHDW (e.g., 0.00244 SHDW).*  
- **URL Structure** 🌐 – `https://shdw-drive.genesysgo.net/[account_address]/[file_name]`.  
- **Scalability** 📈 – *Multiple storage accounts can be created for organization.*  
- **Decentralization** 🔗 – *Files are stored off-chain but linked to Solana ecosystem.*  