
# Batching transactions

Batching transactions allows sending multiple transactions in a single user operation. This can be useful to save on fees, reduce number of user confimations or to ensure that multiple transactions are executed atomically.

A typical example is to do an approval and a transfer in a single userOperation. This way, the transfer will only happen if the approval is successful.


**Definition:** Batching transactions involves combining several individual transactions into one comprehensive transaction. This single transaction is then processed on the blockchain, allowing multiple operations to be executed at once.

**Purpose:** The main goal of batching transactions is to optimize performance and reduce the costs associated with executing multiple transactions separately.


-----------

Certainly! Batching transactions in smart accounts is a powerful technique that allows you to group multiple transactions together into a single transaction. Let’s explore how this works:

1.  **Smart Accounts and Batching Transactions:**
    
    -   **Smart accounts** (such as Biconomy, Kernel, and Sobajaswap) support batching transactions. Batching involves rolling multiple transactions into one, making it simpler for users to interact with Web3.
    -   [For example, instead of executing an `approve()` followed by a `transfer()` separately, users can perform both actions in a single transaction](https://docs.biconomy.io/Account/smartAccountv1/tutorials/nodejs/batchingTransactions)[1](https://docs.biconomy.io/Account/smartAccountv1/tutorials/nodejs/batchingTransactions)[2](https://docs.zerodev.app/sdk/core-api/batch-transactions)[3](https://sobajaswap.com/).
2.  **How to Batch Transactions:**
    
    -   To batch transactions, you typically create an array of transactions and pass it to a method provided by the smart account.
    -   For instance, in Biconomy’s Smart Accounts, you can use the `buildUserOp` method to construct an array of transactions. [By passing the same transaction multiple times in the array, you can mint multiple NFTs in a single transaction](https://docs.biconomy.io/Account/smartAccountv1/tutorials/nodejs/batchingTransactions)[1](https://docs.biconomy.io/Account/smartAccountv1/tutorials/nodejs/batchingTransactions).
    -   [This approach is useful for scenarios like ticketing systems (where you need to mint multiple tickets) or creating one-click experiences in DeFi by combining different transaction types](https://docs.biconomy.io/Account/smartAccountv1/tutorials/nodejs/batchingTransactions)[1](https://docs.biconomy.io/Account/smartAccountv1/tutorials/nodejs/batchingTransactions).
3.  **Atomicity:**
    
    -   Batching ensures that all transactions either execute successfully or revert entirely. This property is known as **atomicity**.
    -   [When batching, if any part of the transaction fails, the entire batch is reverted, maintaining consistency and integrity](https://docs.biconomy.io/Account/smartAccountv1/tutorials/nodejs/batchingTransactions)[3](https://sobajaswap.com/).




----------

### 2. Benefits of Batching Transactions

**Cost Efficiency:**

-   **Reduced Gas Fees:** By combining multiple transactions into one, users can save on transaction fees, as the overhead per transaction is minimized.
-   **Economies of Scale:** The more transactions batched together, the lower the average cost per transaction.

**Improved Throughput:**

-   **Network Efficiency:** Batching reduces the number of transactions broadcast to the network, decreasing congestion and increasing overall network efficiency.
-   **Faster Execution:** Fewer transactions mean faster processing times for individual transactions.

**Enhanced User Experience:**

-   **Simplified Management:** Users can manage fewer transactions, streamlining their interaction with the blockchain.
-   **Atomic Operations:** Batching ensures that all grouped transactions either succeed or fail together, maintaining consistency.

----------

### 3. How Batching Transactions Work

**Technical Process:**

1.  **Aggregation:** Collecting multiple transactions that need to be processed.
2.  **Packaging:** Combining these transactions into a single batch transaction.
3.  **Execution:** Submitting the batch transaction to the blockchain for processing.
4.  **Validation:** The blockchain validates the batch transaction, executing all included operations.

**Example Workflow:**

-   **Step 1:** A user wants to send tokens to multiple recipients.
-   **Step 2:** Instead of sending each transaction separately, the user groups them into a single batch.
-   **Step 3:** The batch transaction is created, containing all recipient addresses and amounts.
-   **Step 4:** The user signs and submits the batch transaction to the blockchain.
-   **Step 5:** The blockchain processes the batch, executing all transfers in one go.

----------

### 4. Implementation in Smart Accounts

**Smart Contract Logic:**

-   **Batch Function:** Smart accounts can include functions that accept an array of operations, processing them within a single transaction.
-   **Error Handling:** Incorporating mechanisms to handle partial failures, ensuring atomicity where necessary.