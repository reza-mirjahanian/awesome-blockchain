# Go Ethereum: The Life of a Transaction

### A Transaction's Journey: High-Level Overview

A transaction travels through several key components within a Go Ethereum (Geth) node before it is included in a block.

1.  **Submission:** A user submits a signed transaction, typically via **RPC** (Remote Procedure Call).
2.  **Transaction Pool:** The transaction is received by the node and placed into the **Transaction Pool** (`TxPool`) after initial validation.
3.  **Propagation:** The transaction is then gossiped to other nodes (**peers**) on the network using the **`eth` protocol**.
4.  **Reception:** Peers receive the transaction, validate it, and add it to their own transaction pools.

\<br\>

A visual representation of the flow:

```
[User/Metamask] --RPC--> [Geth Node]
                        |
                        v
                  [TxPool (Local)]
                        |
                        v
                  [ETH Protocol] --P2P--> [Peer Geth Node]
                                               |
                                               v
                                         [ETH Protocol]
                                               |
                                               v
                                         [TxPool (Remote)]
```

-----

### Part 1: Transaction Submission & The Pool

#### **Submitting a Transaction via RPC**

  * When you send a transaction from a wallet like MetaMask to your local node, you are sending a **signed, raw transaction**. The node itself does not hold your private keys and does not sign anything for you.
  * This transaction is received by the **RPC API**. A common method for this is `eth_sendRawTransaction`.
  * The call path within Geth typically flows through `internal/ethapi`, which defines methods like `SendRawTransaction`, `GetBlock`, etc.
  * From there, it reaches an **API backend**, which is a collection of helper methods for the RPC services.

#### **Initial Validation Checks**

Before being accepted into the transaction pool, the node performs several crucial checks:

  * **Fee Checks:**
      * Ensures that the transaction provides **enough fees** to be considered for inclusion.
      * It also checks that the fees **do not exceed a certain threshold**.
      * *This sanity check was added because users would occasionally mix up the `value` of a transaction with the `gas price`, accidentally specifying an astronomically high fee and losing significant funds.*
  * The validated transaction is then passed to the transaction pool via an interface method, `SendTransaction`.

<!-- end list -->

```go
// Simplified view of the API backend sending a tx to the pool
type APIBackend struct {
    txPool *TxPool
    // ... other fields
}

func (b *APIBackend) SendTransaction(signedTx *types.Transaction) error {
    // ... initial checks for fee caps, etc.

    // Add the transaction to the pool as a "local" transaction
    return b.txPool.AddLocal(signedTx)
}
```

-----

### The Transaction Pool (`TxPool`)

The `TxPool` is the waiting area for transactions that have been validated but not yet included in a block.

#### **Transaction Sorting & EIP-1559**

  * The primary job of the `TxPool` is to hold and sort transactions.
  * Traditionally, transactions were sorted simply by **gas price**. Higher gas price meant higher priority.
  * With the introduction of **EIP-1559**, sorting became more complex due to a *two-dimensional gas price*:
    1.  **Max Fee Per Gas:** The absolute maximum the sender is willing to pay per unit of gas (includes base fee + tip).
    2.  **Max Priority Fee Per Gas:** The maximum "tip" the sender is willing to pay directly to the miner.
  * A transaction's validity now depends on the current block's **`base_fee`**. If a transaction's `max_fee_per_gas` is lower than the current `base_fee`, it is not executable in the current block. This means the set of valid transactions must be continuously re-evaluated.

#### **The "Pending" Block**

  * The `TxPool` is used to construct a **pending block**.
  * *The pending block is an in-memory representation of what the next block would look like if it were mined immediately, containing all the currently executable transactions from the pool.*
  * **Historical Purpose:** Crucial for wallets and dapps to estimate the state of the chain for the next block (e.g., to determine the correct nonce).
  * **Modern Challenge:** The rise of **MEV (Maximal Extractable Value)** and services like Flashbots means that the actual block produced by a miner can be very different from the public pending block. This has made the pending block concept less reliable and "basically useless" for predicting block contents. Geth developers are actively working on reducing the reliance on it.

#### **Nonce Management & Transaction Replacement**

  * **Nonce:** *A sequential, incrementing counter for each transaction sent from a specific account. It prevents replay attacks.*
  * The pending block is essential for determining the next valid nonce for an account. If you have a pending transaction with nonce `5`, your next transaction must have nonce `6`.
  * **Replacing a Transaction:** You can replace a pending transaction by sending a new transaction with the **same nonce**.
  * This is only allowed if the new transaction pays a higher fee. The specific rules (a form of Replace-by-Fee) are:
      * The new `Max Fee Per Gas` must be at least **10% higher** than the old one.
      * The new `Max Priority Fee Per Gas` (tip) must also be at least **10% higher** than the old one.
  * *This 10% rule prevents network spam where users could continuously replace transactions at no extra cost, causing excessive churn in the transaction pool.*

-----

### Part 2: Network Propagation

#### **Broadcasting Transactions to Peers**

  * Once a transaction is in the pool, it must be propagated to the rest of the network.
  * This is handled within the `eth` package, which implements the Ethereum wire protocol.
  * A function, `syncTransaction`, periodically queries the `TxPool` for new pending transactions.

#### **Types of Transactions in the Pool**

The `TxPool` categorizes transactions to manage them effectively:

  * **Pending Transactions:** *These are immediately executable transactions where the nonce is correct relative to the sender's last confirmed transaction.*
  * **Queued (or Gapped) Transactions:** *These are valid transactions but have a future nonce. For example, a transaction with nonce `10` when the account's current nonce is only `8`.* The pool holds these in a separate queue, waiting for the missing transactions (`nonce 8`, `nonce 9`) to arrive.

#### **The Gossip Mechanism**

To avoid overwhelming the network, Geth uses a controlled gossiping strategy:

1.  The **full transaction data** is sent to a small subset of connected peers (specifically, the square root of the total number of peers, e.g., $\\sqrt{N}$).
2.  Only the **transaction hash** is announced to all other peers.

<!-- end list -->

  * This dramatically reduces bandwidth usage. A peer that receives a hash announcement can check if it already has that transaction. If not, it can explicitly request the full transaction data from the peer that sent the hash.

#### **Receiving Transactions from Peers**

  * On the other side, a node's **`Transaction Fetcher`** listens for these hash announcements.
  * When a new hash arrives, the fetcher schedules a request to retrieve the full transaction from the announcing peer.
  * Once the full transaction is received, it goes through the same validation process and is added to the local `TxPool`.

#### **Local vs. Remote Transactions**

The `TxPool` makes a critical distinction based on a transaction's origin:

  * **Local Transactions:**
      * Origin: Submitted directly to the node via its **RPC** endpoint.
      * Behavior: These are considered the node operator's own transactions. **They will never be dropped** from the pool to make way for a higher-paying transaction.
  * **Remote Transactions:**
      * Origin: Received from a **peer** over the P2P network.
      * Behavior: These can be dropped and replaced if another transaction with the same nonce arrives that meets the "10% higher fee" replacement criteria.

-----

### Part 3: Block Creation & EVM Execution

#### **The Miner: Creating a Block**

  * The process of building a new block is handled by the **`miner`** package.
  * The miner enters a work loop. When a new block arrives from the network, it begins the process of creating the *next* block to build on top of it.

#### **Step 1: Prepare the Work (`prepareWork`)**

The miner first assembles the block header with essential information:

  * **`ParentHash`**: The hash of the latest block on the chain.
  * **`Timestamp`**: The current time.
  * **`GasLimit`**: The gas limit for the new block. This is adjusted slightly up or down from the parent's limit to target a configured value (e.g., the default target is 30 million gas).
  * **`Coinbase`**: The miner's Ethereum address, which will receive the transaction tips.
  * **`BaseFee`**: For post-London blocks, this is calculated based on the parent block's gas usage and fullness.

#### **Step 2: Fill the Block with Transactions**

  * The miner fetches all executable (`pending`) transactions from the `TxPool`.
  * **Transaction Selection Strategy:**
    1.  It first includes all **local** transactions, ordered by their effective fee.
    2.  It then fills the remaining block space with the highest-paying **remote** transactions.
  * Each selected transaction is then executed to update the state and finalize the block.

#### **Step 3: Apply Transactions (`applyTransaction`)**

  * This is the most complex part of block creation and happens in the **`core`** package, which contains the logic for the **Ethereum Virtual Machine (EVM)**.
  * The process involves creating a `State Processor` and a new `EVM` instance for each transaction.

##### **EVM Pre-Checks**

Before bytecode execution, several pre-checks are performed:

1.  **Nonce Verification:** Confirms the transaction nonce matches the account's state nonce.
2.  **Sufficient Funds:** Checks that the sender's balance can cover the maximum possible cost: `value + (gasLimit * maxFeePerGas)`.
3.  **Intrinsic Gas:** Deducts the base cost of the transaction before any code runs.
4.  **Origin Check:** A recent change ensures the sender is an **EOA (Externally Owned Account)** and not a contract. This prevents theoretical attacks where a contract could initiate a transaction.

##### **Access Lists (EIP-2930)**

  * Introduced in the Berlin hard fork, access lists allow a transaction to specify which addresses and storage slots it intends to touch.
  * Accessing a "pre-warmed" address or slot from the list is cheaper than a "cold" access.
  * This was a forward-looking feature to help with gas cost stability and is a stepping stone toward future **stateless Ethereum** concepts.

-----

### Inside the EVM: The `CALL` Operation

When a transaction calls a smart contract, the `CALL` opcode is invoked. Here is the sequence of events:

1.  **Value Check:** If the call is sending ETH (`value > 0`), it first checks if the caller's account has sufficient balance.
2.  **Create State Snapshot:** The EVM takes a snapshot of the current state database. *This is critical. If the contract execution fails for any reason (e.g., `REVERT` opcode, out of gas), the state can be instantly rolled back to this snapshot.*
3.  **Check for Precompiles:** The EVM checks if the destination address is one of the **precompiled contracts**.
      * *Precompiles are complex cryptographic functions (like `ecrecover` or hash functions) implemented directly in the Go client for performance, as they would be too expensive to run as EVM bytecode.*
      * If it's a precompile, the native Go function is run instead of the EVM.
4.  **Transfer Value:** The specified ETH value is transferred to the recipient contract.
5.  **Execute Code:** If the destination address contains code and is not a precompile, the EVM begins executing the bytecode.

#### **The EVM Execution Loop**

The EVM runs in a simple but powerful loop:

1.  Fetch the next **opcode** from the contract's bytecode.
2.  Look up the opcode in a **jump table** to find its corresponding function.
3.  Execute the function, which manipulates the **stack**, **memory**, or **storage**.
4.  Repeat until an opcode like `STOP`, `RETURN`, or `REVERT` is reached.

<!-- end list -->

```go
// Simplified view of an opcode function in the EVM
// This is the function for the ADD opcode.
func opAdd(pc *uint64, interpreter *EVMInterpreter, scope *ScopeContext) ([]byte, error) {
    // Pop two 256-bit integers from the stack
    x, y := scope.Stack.Pop(), scope.Stack.Pop()
    // Add them together
    y.Add(&x, &y)
    // Push the result back onto the stack
    scope.Stack.Push(&y)
    return nil, nil
}
```

#### **Handling Reverts and Failures**

  * If the contract execution reverts or runs out of gas, the operation fails.
  * The EVM discards all state changes made during the execution by restoring the **state snapshot** taken at the beginning of the `CALL`.
  * **Important:** Even though the state change is reverted, the sender **still pays the gas fees** consumed up to the point of failure.

-----

### Finalizing the Transaction

After the EVM execution is complete (either successfully or not):

1.  **Refund Gas:** Any unused gas from the initial `gasLimit` is refunded to the sender. Additional refunds can be granted for certain operations that clean up the state, such as clearing a storage slot or using `SELFDESTRUCT`.
2.  **Pay the Miner:** The final transaction fee (`gasUsed * effectiveGasPrice`) is transferred to the miner's **`coinbase`** address.
3.  **Generate Receipt:** A transaction receipt is created. This contains:
      * **Status:** A flag indicating success (`1`) or failure (`0`).
      * **Logs:** Any events emitted by the contract.
      * **Gas Used:** The total gas consumed by the transaction.
      * **Bloom Filter:** A data structure to make searching for logs efficient.

#### **The Bloom Filter**

  * *A 2048-bit (256-byte) probabilistic data structure included in every block header.*
  * **Purpose:** To allow for very fast searching of event logs without having to download and inspect every transaction in every block.
  * **How it works:** When a contract emits an event, its address and topics are hashed. These hashes are used to flip specific bits in the bloom filter from 0 to 1.
  * An application can check the bloom filter of a block. If the bits corresponding to its contract of interest are set, it knows the block *might* contain relevant logs and can proceed to download the full receipts. If the bits are not set, it can safely skip the block entirely.