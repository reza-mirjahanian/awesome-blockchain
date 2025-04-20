**Ethereum Tokens and Checking Balances**
-----------------------------------------

This explanation delves into how Ethereum tokens work, why they sometimes don't show up in wallets like MetaMask or Ledger Live, and how to properly check token balances. The tutorial emphasizes the concept of the **Ethereum State Machine** and its role in managing accounts and transactions.

### **1\. Ethereum as a State Machine**

-   Ethereum can be understood as a **state machine**. The **world state** includes all account balances, smart contracts, and their storage.
-   **State transitions** occur when transactions are processed in a block, changing account balances or contract states.
-   The **world state** contains externally owned accounts (EOAs) and contract accounts. EOAs are controlled by private keys, while contract accounts are controlled by smart contract code and cannot initiate their own transactions.

### **2\. Accounts in Ethereum**

-   **Externally owned accounts (EOAs):** These are the regular Ethereum accounts controlled by users with private keys. Each EOA contains a **balance** (in ETH) and a **nonce** (transaction counter).
-   **Contract accounts:** These are controlled by smart contract code and include a **storage hash** and **code hash**, pointing to where the contract data and code are stored.

### **3\. Token Storage**

-   **Tokens (ERC20/NFTs) are not stored in user accounts**. Instead, they are stored in the **contract's state**. The contract keeps track of who owns what through key-value mappings.
    -   For example, the **contract storage** maintains a list of addresses and their token balances.
-   When transferring tokens, the contract's storage is updated to reflect changes in ownership.

### **4\. How Token Transfers Work**

-   Token transfers happen inside the contract, **not from the user's account**.
    -   Users call the **transfer** function in the contract, which updates the token balances in the contract's storage.
-   For example, transferring tokens involves the contract decreasing the sender's balance and increasing the recipient's balance.

### **5\. Why Tokens Might Not Show in MetaMask**

-   Tokens don't appear in MetaMask or similar wallets because these wallets don't automatically scan for all token contracts. They may not detect newly deployed contracts.
-   To verify token ownership:
    -   **Find the token contract address** on platforms like CoinGecko.
    -   **Check EtherScan** for your account by entering the contract address to see your token balance.

### **6\. ERC20 Tokens**

-   **ERC20** is a widely used standard for Ethereum tokens. It defines basic functions like `totalSupply`, `balanceOf`, and `transfer`.
-   **Approval Mechanism:** Before interacting with decentralized exchanges (like Uniswap), users must **approve** the exchange contract to spend their tokens, ensuring the contract has permission to transfer tokens on behalf of the user.

### **7\. Checking Token Balances and Contracts**

-   Use EtherScan to check a token contract's storage and see if your address holds any tokens by visiting the **Token Tracker Page**.
    -   You can search your Ethereum address under the **Holders** section to confirm your token balance.