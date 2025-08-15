

## 🔍 Introduction  
- Fetch latest blockhash via Solana RPC to prevent replay attacks  
- Blockhashes expire quickly—offline/hardware wallets risk invalid transactions  
- Durable nonces enable secure offline transactions  
- **Guide covers:**  
  - What a durable nonce is  
  - Why it’s needed  
  - How to implement it  

---

## ⚙️ Prerequisites  
- Basic JavaScript knowledge  
- Node.js installed  
- Solana CLI installed  
- Git installed  

---

## 🔧 Environment Setup  
1. Clone example repository containing utility scripts  
2. `npm install` to install dependencies  
3. Create and fund wallets:  
   - Generate paying wallet: `solana-keygen new -o ./wallets/wallet.json`  
   - Airdrop 2 SOL via Solana CLI  
   - Generate nonce authority wallet: `solana-keygen new -o ./wallets/nonceAuth.json`  
   - Airdrop 1 SOL via a public faucet  

---

## 🏦 What Is a Durable Nonce?  
- **Analogy:** A safety deposit box storing a unique, stable code  
- Typical nonces change every transaction; durable nonce remains constant  
- Offline transactions reference this stored nonce for validation  
- Solana checks incoming nonce against stored value before approving  

### 🛡️ Key Use Cases  
- Scheduled transactions at future timestamps  
- Coordination in multisig wallets  
- Programs requiring timed or repeated interactions  
- Cross-chain transaction validity  

---

## 🚀 Steps to Build  

### Step 1: Set Up Dependencies & Constants  
- Import from `@solana/web3.js`:  
  - `Connection`, `Keypair`, `LAMPORTS_PER_SOL`, `NonceAccount`, `NONCE_ACCOUNT_LENGTH`, `SystemProgram`, `Transaction`  
- Import utilities: `encodeAndWriteTransaction`, `loadWallet`, `readAndDecodeTransaction`  
- Define:  
  - `nonceAuthKeypair` via `loadWallet('./wallets/nonceAuth.json')`  
  - `nonceKeypair` with `Keypair.generate()`  
  - `senderKeypair` via `loadWallet('./wallets/wallet.json')`  
  - `connection` to devnet RPC endpoint  
  - `waitTime` = 120 000 ms  
  - `TransferAmount` = `LAMPORTS_PER_SOL * 0.01`  

---

### Step 2: Create sendTransaction Function  
- Logs start message  
- Sequence:  
  1. `nonceCreationTxSig = await nonce()`  
  2. Confirm creation; if successful:  
     - `nonce = await getNonce()`  
     - `await createTx(nonce)`  
     - `await signOffline(waitTime)`  
     - `await executeTx()`  
  3. Error branch logs failure  

---

### Step 3: Create nonce Function  
- **Purpose:** Create and initialize durable nonce account  
- **Process:**  
  1. Calculate rent exemption:  
     - `rent = await connection.getMinimumBalanceForRentExemption(NONCE_ACCOUNT_LENGTH)`  
  2. Fetch latest blockhash & height  
  3. Build `createAccount` transaction:  
     - From `nonceAuthKeypair.publicKey`  
     - New account `nonceKeypair.publicKey` with `rent` lamports  
  4. Sign & send; confirm status; log signature  
  5. Build `nonceInitialize` transaction:  
     - Set `noncePubkey = nonceKeypair.publicKey`  
     - `authorizedPubkey = nonceAuthKeypair.publicKey`  
  6. Sign & send; confirm status; log initialization signature  
- **Error Handling:** Logs “Failed in createNonce function” and rethrows  

---

### Step 4: Create getNonce Function  
- Fetches account info via `fetchNonceInfo()`  
- Returns `nonceAccount.nonce`  

---

### Step 5: Create createTx Function  
- Generates a new destination keypair  
- Constructs two instructions:  
  1. **Advance Nonce:**  
     - `noncePubkey = nonceKeypair.publicKey`  
     - `authorizedPubkey = nonceAuthKeypair.publicKey`  
  2. **Transfer:**  
     - From `senderKeypair.publicKey` to `destination.publicKey`  
     - Amount = `TransferAmount`  
- Builds `Transaction`:  
  - Adds both instructions  
  - Sets `recentBlockhash = nonce`  
  - Sets `feePayer = senderKeypair.publicKey`  
- Serializes and writes to `./unsignedTxn.json`  

---

### Step 6: Create signOffline Function  
- Simulates offline delay: `await new Promise(res => setTimeout(res, waitTime))`  
- Reads unsigned transaction from `./unsignedTxn.json`  
- Signs with `senderKeypair` and `nonceAuthKeypair`  
- Writes signed transaction to `./signedTxn.json`  

---

### Step 7: Create executeTx Function  
- Reads signed transaction from `./signedTxn.json`  
- Sends via `connection.sendRawTransaction(signedTx.serialize())`  
- Logs resulting transaction signature  

---

### Step 8: Create fetchNonceInfo Function  
- Retries up to 3 times:  
  - Fetch account data for `nonceKeypair.publicKey`  
  - If found: parse `NonceAccount.fromAccountData` and return  
  - Else: log retry message, wait 3 s, decrement retry count  
- Throws error if no info after retries  

---

### Step 9: Invoke sendTransaction  
- Run `sendTransaction()` to:  
  - Create, sign, and broadcast a transaction using a durable nonce  
  - Output: unsigned/signed JSON files and final transaction signature 🎉  

---

## 🌐 Using Helius RPCs  
- Helius acts as intermediary for Solana RPC calls  
- Simplifies fetching blockhash lifecycle for durable nonces  
- Enhances reliability for offline or delayed transactions  
- Streamlines access to blockhashes, reducing expiry errors 👍

---



## Durable Nonces  ⏳

* **Durable Nonce**
  *A consistent reference that acts like a “safety deposit box” nonce account, avoiding rapid blockhash expiration.*

* **Use Case**
  *Ideal for offline or delayed transactions (e.g., via hardware wallets), where the usual quick-expiring recent blockhash would otherwise invalidate the transaction.* ([Helius][1])

---

## Transaction Failure Causes & Best Practices  🚫➡️📈

### Common Failure Reasons:

* **Network Drops**
  *Packets may be lost or overflow queue (especially when rebroadcast queue > 10,000), dropping transactions before leader processes them.* ([Helius][2])

* **Stale or Incorrect Blockhash**

  * *If a blockhash is too old (> \~150 slots) or not recognized due to stale RPC pool nodes, validators reject the transaction.* ([Helius][2])

* **Temporary Network Forks**
  *If the referenced blockhash belongs to a minority fork that gets dropped, associated transactions are also dropped.* ([Helius][2])

### Best Practices to Land Transactions:

1. **Fetch Latest Blockhash**
   *Use RPC methods like `getLatestBlockhash` with commitment levels “confirmed” or “finalized” to avoid minority forks.* ([Helius][2])
2. **Skip Preflight Settings**
   *Beginners: use `skipPreflight = false` for safety.*
   *Advanced: set `skipPreflight = true` for minimal latency at your own risk.* ([Helius][2])
3. **Optimize Compute Units**
   *Adjust compute usage to increase success probability.* ([Helius][2])
4. **Set & Calculate Priority Fees Dynamically**
   *Use priority fees to incentivize block producers during high congestion.* ([Helius][2])
5. **Custom Retry Logic**
   *Set `maxRetries = 0` and implement your own resend logic at \~2-second intervals until confirmation.* ([Helius][2])
6. **Leverage Staked Connections**
   *Routes through priority lanes directly to block leaders, bypassing public queues—recommended for production use.* ([Helius][3])
7. **Use Durable Nonces for Non-Time-Sensitive Transactions**
   *Circumvents rapid blockhash expiry by reusing the stored nonce until consumed.* ([Helius][2])

---

## Commitment Levels & Their Trade-offs  ⚖️

* **Processed**

  * *Fastest (\~sub-second) confirmation.*
  * *Lowest security—subject to forks, not for critical use.* ([Helius][4])

* **Confirmed**

  * *Moderate delay (0.5–1 second).*
  * *High reliability; ideal balance for most apps.* ([Helius][4])

* **Finalized**

  * *Longest delay (\~10–20 seconds).*
  * *Highest security; ensures ledger permanence.* ([Helius][4])

* **Insight:**
  *Using ‘Finalized’ for blockhash may reduce the effective transaction window due to its lag; using ‘Confirmed’ is often optimal to reduce expiration risk.* ([Helius][4])

---

## Staked Connections & Sender Service  ⚡

* **Staked RPC Connections**
  *Routes transactions directly to block leaders via stake-weighted quality-of-service (SWQoS), improving inclusion chances.* ([Helius][3])

* **Sender (Ultra-Low Latency Service)**
  *Simultaneously routes through Helius staked infrastructure and Jito MEV auctions for strongest inclusion guarantees.*
  *Useful for high-frequency trading, arb bots, wallet UX, memecoin sniping.* ([Helius][5])

---

## Transaction APIs & Parsing Tools  📊

* **Priority Fee API**
  *Provides dynamic fee suggestions based on local/global demand; allows selecting priority levels to avoid overspending.* ([Helius][6])

* **Enhanced Transactions API**
  *Transforms raw Solana instructions into human-readable structured data — includes amounts, accounts, action types (e.g., SWAP, NFT\_SALE).*
  *Supports batch parsing and transaction history retrieval (e.g., `/v0/transactions`, `/v0/addresses/{address}/transactions`).* ([Helius][7])

* **Streaming APIs (Webhooks, WebSockets, gRPC)**
  *Enable real-time transaction monitoring at ultra-low latency; supports push updates via event listening.* ([Helius][6])

---

## Transaction Optimization & Polling  ⏱

* **Optimize & Serialize Transactions**
  *Minimize compute use, encode efficiently, and set appropriate priority fees for cost-efficient fast processing.* ([Helius][8])

* **Confirmation Polling Pattern**

  ```js
  // Poll every 5s up to 15s
  async pollTransactionConfirmation(txtSig) { … }
  ```

  *Tips: retry often, timeout to handle delays gracefully.* ([Helius][8])

---

## Summary Icons at a Glance

* ⏳ *Durable Nonce* = safe offline transactions
* 🚫 *Network/Blockhash* = failure causes
* 🔄 *Custom Retry & Preflight* = better delivery
* 🛣 *Staked Connections / Sender* = priority routing
* 💸 *Priority Fees API* = adaptive costing
* 📰 *Enhanced Parsing API* = clear transaction insights
* ⏱ *Polling Strategy* = efficient confirmation

---

