**What is a Solana Transaction?**  


[Deep dive into versioned transactions (with fancy graphics) - Anvit Mangal](https://anvit.hashnode.dev/versioned-transactions)

[Jito - Solana Validator 101 transaction processing](https://www.jito.wtf/blog/solana-validator-101-transaction-processing/)

[Umbra Research - Lifecycle of a Solana transaction](https://www.umbraresearch.xyz/writings/lifecycle-of-a-solana-transaction)




- A **verifiable message format** proving a specific identity sent it.  
- Contains:  
  - **Instructions** (e.g., swaps, sends, lending actions).  
  - **Signatures** to authenticate the sender.  

---

**Transaction Lifecycle**  
1. **User Action**: Initiates via an app (e.g., clicking "Swap" or "Mint NFT").  
2. **Transaction Formation**:  
   - Developer assembles:  
     - Addresses  
     - Instructions  
     - Block hash  
3. **Simulation** (Optional):  
   - Checks if the transaction will fail *before* sending.  
   - Reduces network load from invalid transactions.  
4. **User Signing**:  
   - Requires a signature (usually one) to authorize.  
5. **Broadcasting**:  
   - Sent to an **RPC node** (entry point to the Solana network).  
6. **Validation**:  
   - Validators process and finalize the transaction.  
7. **Confirmation**:  
   - Success/failure notification sent to the developer.  

**Developer Work Distribution**:  
- **80%**: Transaction formation (instructions, addresses).  
- **15%**: Broadcasting (RPC handling, pre-flight checks).  
- **5%**: Confirmation handling.  

---

**Transaction Structure**  
- **Signatures**: Array proving sender authorization.  
- **Message**:  
  - **Header**: Metadata (signature count, read/write accounts).  
  - **Account Addresses**: List of accounts read/written during execution.  
  - **Block Hash**: Recent block ID to prevent outdated transactions.  
  - **Instructions**: Code to execute (e.g., token transfers, program calls).  

---

**Transaction Fees**  
1. **Static Signature Fee**:  
   - Cost per signature (fixed per signature).  
2. **Dynamic Compute Fee**:  
   - Based on computational resources used.  
3. **Prioritization Fee** (Optional):  
   - Extra fee to expedite processing.  

---

**Transaction Versions**  
1. **Legacy**:  
   - Original format (deprecated).  
   - Limited to ~35 addresses per transaction.  
2. **v0**:  
   - Supports **address lookup tables** (up to 256 addresses per transaction).  
   - Reduces message size overhead.  

---

**Transaction Failures**  
- **Dropped Transactions**:  
  - Never reach validators due to:  
    - Client/RPC failures (e.g., scaling issues, load balancing).  
    - Expired block hashes.  
- **Failed Transactions**:  
  - Processed but rejected due to:  
    - Unmet conditions (e.g., price changes in swaps).  
    - Insufficient fees or compute units.  
    - Missing/invalid signatures.  

**Common Failure Scenarios**:  
- **Arbitrage Spam**: 98% fail but cost minimal fees (high-volume attempts).  
- **Improper Pre-Checks**: Not validating smart contract conditions upfront.  

---

**Building a Transaction from Scratch**  
- **Key Components**:  
  - Key pairs (ed25519 cryptography).  
  - Recent block hash.  
  - Instructions (e.g., memo program interaction).  
- **Steps**:  
  1. Generate key pair.  
  2. Request block hash.  
  3. Assemble message (header, accounts, instructions).  
  4. Sign the message.  
  5. Broadcast via RPC.  

---

