# Geth Under the Hood: A Deep Dive into Transaction & Block Processing

---

## Transaction Lifecycle: From Submission to Propagation

### Submitting a Transaction

* When you send a transaction (e.g., via MetaMask) to your local Geth node, you're always sending a **raw, signed transaction**.
* The signing process happens *outside* of your Geth node, typically in your wallet.
* This signed transaction enters Geth through its **RPC API**.

### Internal Geth API (`internal/ethapi`)

* Geth's internal API defines many familiar methods:
    * `eth_sendRawTransaction`
    * `eth_getTransaction`
    * `eth_getBlock`
    * ...and many more.
* These methods serve as the entry points for external interactions with the node.

### Initial Checks & Transaction Pool Insertion

* Upon receiving a raw transaction, Geth performs basic validation before inserting it into the **transaction pool**.
* **Fee Checks**:
    * Ensuring sufficient fees are provided.
    * Preventing excessively high gas prices (a common user error where `value` and `gasPrice` are confused).

### `SendTransaction` in `api_backend.go`

* The `SendTransaction` function is part of the `api_backend` interface, which provides a collection of utility methods for RPC APIs.
* This function essentially adds the transaction as a *local transaction* to the transaction pool.

### The Transaction Pool (`txpool`)

* The transaction pool (`txpool`) is where all unconfirmed transactions reside.
* **Sorting**:
    * Transactions are sorted primarily by **gas price**.
    * With EIP-1559, sorting becomes more complex due to the **two-dimensional gas price** (base fee + tip).
    * The pool must continuously update valid transactions as the base fee changes, impacting transaction validity.

#### The "Pending Block" - A Fading Concept

* The **pending block** is a local representation of the next block, built by piling pending transactions.
* Its utility is diminishing with the rise of **MEV (Maximal Extractable Value)**, and efforts are underway to deprecate it.
* **Why it's still used**: For nonce management, allowing you to track unconfirmed transactions and increment your nonce for new submissions.

#### Transaction Replacement Logic

* Sending a new transaction with the *same nonce* as an existing one in the pool **replaces** it, but only under specific conditions:
    * `maxBaseFeePerGas` must be at least **10% higher** than the transaction to be replaced.
    * `maxTipPerGas` must also be at least **10% higher**.
* This mechanism prevents spamming the transaction pool by constantly re-sending the same nonce without increasing the fee.

### Adding to the Transaction Pool: Detailed Steps

* **Sender Extraction**: The sender's address is recovered from the transaction's signature using `ecRecover`.
* **Validation**:
    * Checks for duplicates (if the transaction is already in the pool).
    * Validation against current network rules.
    * Underpriced transactions are discarded.
* Finally, the transaction is **enqueued**.

---

## Transaction Propagation: Sharing Transactions with Peers

### The `eth` Protocol (`eth/ethprotocol`)

* The `eth` package defines the **`eth` protocol**, which governs network messages and peer interactions.
* It includes a **handler** with a `syncTransactions` function.

### Periodically Syncing Transactions

* The `syncTransactions` function periodically queries the transaction pool for **pending transactions**.
* **Pending Transactions**: Transactions that are currently executable.
* **Gapped Transactions**: Transactions with nonces far in the future (e.g., nonce 9000 when 8999 transactions are missing). These are stored to allow for eventual re-validation if the preceding transactions arrive.

### Broadcasting Transaction Hashes

* Geth sends **transaction hashes** to a subset of its peers.
* To all peers, only hashes are sent.
* **Full transactions** are sent only to a limited subset of peers.
* This strategy significantly reduces network bandwidth and churn, preventing nodes from receiving all transactions from every peer at all times.

### The P2P Layer (`p2p`)

* The P2P package is responsible for writing messages (including transaction hashes) onto the network wire.

### Receiving Transactions: The Transaction Fetcher

* On the receiving end, the **transaction fetcher** (within the `eth` package) handles incoming transaction announcements.
* When a node receives a transaction hash it hasn't seen before, it:
    * Checks if it already has the transaction.
    * If not, it requests the full transaction from the peer that announced it.
    * `ScheduleFetches` organizes these requests for various peers.
* Upon receiving the full transaction, the `AddTransaction` function enqueues it back into the transaction pool.

### Local vs. Remote Transactions in the Pool

* The transaction pool differentiates between:
    * **Local transactions**: Received via RPC (from your own applications).
    * **Remote transactions**: Received from peers on the network.
* **Handling Difference**: Local transactions are prioritized and are generally *not* dropped, while remote transactions can be replaced if a higher-paying transaction from the network arrives. This ensures your own submitted transactions are less likely to be discarded.

---

## Block Creation: How Miners Build Blocks

### The Miner Package (`miner`)

* The `miner` package is responsible for orchestrating the block creation process.
* It operates in a continuous loop, updating its "work package" whenever a new block is received.

### `CommitWork` and `PrepareWork`

* `CommitWork` is a central function in the miner's loop.
* `PrepareWork` is called to set up the new block based on the latest chain state:
    * **Parent Hash**: Set to the hash of the latest received block.
    * **Timestamp**: Current time.
    * **Gas Limit**: Calculated based on network rules (can be slightly shifted up or down, targeting 30 million by default).
    * **Coinbase**: The address where transaction fees will be collected.
    * **Base Fee**: Handled for EIP-1559 chains.
    * **Consensus Engine**: Configured (e.g., `ethash` for Proof-of-Work, or the Beacon chain's consensus engine for Proof-of-Stake).
    * **Environment Setup**: Preparing the execution environment for transactions.

### Filling the Block: Applying Transactions (`ApplyTransactions`)

* Once the block structure is prepared, the miner fetches all **pending transactions** from the transaction pool.
* Transactions are prioritized: local transactions first, then remote transactions, both ordered by fee.
* These transactions are then committed to the pending work package.

### `CommitTransaction` and the Core Package (`core`)

* `CommitTransaction` ultimately calls `ApplyTransaction`, which resides in the **`core` package**.
* The `core` package is the heart of Geth, handling most of the core logic, including EVM execution.

---

## Transaction Execution: Deep Dive into the EVM

### State Processor and EVM Context

* Within the `core` package, the `StateProcessor` is responsible for applying transactions.
* It creates a new **EVM block context** and a new **EVM instance**.
* `ApplyTransaction` then calls `ApplyMessage`.

### `ApplyMessage` and `StateTransition.TransactionDatabase`

* `ApplyMessage` is the entry point for EVM execution.
* It calls `StateTransition.TransactionDatabase`, which kicks off the EVM's execution.

### Pre-Checks Before Execution

* Before an EVM transaction is executed, several pre-checks are performed:
    * **Nonce Verification**: Ensures the nonce is correct.
    * **EOA Sender**: Confirms the sender is an Externally Owned Account (EOA), preventing transactions originating from smart contracts (a recent change to address potential edge cases with genesis-deployed code).
    * **Base Fee Verification**: For EIP-1559 transactions.
    * **Gas Purchase**: Calculates the maximum gas that can be used during execution.

### Tracing and Debugging

* Geth includes extensive tracing capabilities (e.g., `config.Debug` with `tracer` options). These allow you to inspect every opcode execution, its outcome, and other debugging information.

### Intrinsic Gas & Sender Balance Check

* Intrinsic gas (gas for the transaction itself, not the EVM execution) is checked.
* The sender's balance is verified to ensure they can cover the transaction cost.

### Access Lists (Post-Berlin Hard Fork)

* **EIP-2930 (Optional Access Lists)**: Introduced in the Berlin hard fork.
* **Purpose**: To make calls to contracts in the access list cheaper.
* **Mechanism**: Addresses and storage keys that are "warmed" (accessed frequently) are cheaper to interact with. Access lists allow pre-declaration of these.
* **Precompiles, Sender, Recipient**: Calls to precompiled contracts, the sender, and the recipient are also considered "warm" by default.
* **Stateless Ethereum**: Mandatory access lists are a concept that will be crucial for stateless clients, where the pre-state and post-state of transactions would be provided as a form of access list for verification without needing the full state.

### Contract Creation vs. Contract Call

* **Contract Creation**: If the transaction is sent to the zero address (`0x0`), and includes code, it's a contract creation. The `CREATE` or `CREATE2` opcode is triggered.
* **Contract Call**: If the transaction is sent to an existing address, the code at that address is executed.

#### Focusing on Contract Call

### `Call` Function Details

* **Value Transfer Check**: Verifies if the sender has enough balance to cover any attached value.
* **State Database Snapshot**: A snapshot of the state database is taken *before* executing the call. This allows for rollback if the transaction reverts.
* **Precompiled Contract Check**:
    * Geth first checks if the target address is a **precompiled contract**.
    * Precompiled contracts (e.g., `ecRecover`, `sha256`) are efficient, native implementations of common cryptographic operations.
    * They don't have EVM bytecode or storage; they are implemented directly in Go.
    * Each precompile has a `Run` function (executes the logic) and a `RequiredGas` function (returns the fixed gas cost).
    * A notable historical issue involved a precompile being self-destructed during a hard fork due to specific conditions, which is now explicitly checked against.

#### Example: `ecRecover` Precompile

```go
// core/vm/contracts.go
type PrecompiledContract interface {
	RequiredGas(input []byte) uint64
	Run(input []byte) ([]byte, error)
}

// core/vm/vm_test.go (simplified example of use)
// Function for ECRecover precompile
func (c *ecRecover) RequiredGas(input []byte) uint64 {
    return 3000 // Fixed gas cost
}

func (c *ecRecover) Run(input []byte) ([]byte, error) {
    // ... validation of signature values ...
    // ... perform ecRecover ...
    // ... return public key ...
}
```

* If it's not a precompile, the code at the target address is retrieved. If no code exists, the call finishes (unless it's a value transfer to an EOA).
* **Triggering Code**: If code is present, it's triggered using various call types:
    * `CALL`
    * `CALLCODE`
    * `DELEGATECALL`
    * `STATICCALL`

### EVM Execution Loop (`Run` Function)

* The EVM's `Run` function (in `core/vm/evm.go`) executes the contract's bytecode.
* It continuously fetches the **next opcode** in a loop.
* The **jump table** (a mapping of opcodes to their corresponding functions) determines the action.
* **Opcode Structure**: Each opcode (e.g., `ADD`, `SHIFTLEFT`, `CREATE2`) has:
    * **Stack Requirements**: What it expects on the EVM stack.
    * **Gas Function**: How its gas cost is calculated.
    * **Static Gas**: Fixed cost for operations like `ADD` or `SUB`.
    * **Dynamic Gas**: Cost varies based on input (e.g., `CREATE` where gas depends on code length).

#### Example: `ADD` Opcode

```go
// core/vm/instructions.go
func opAdd(pc *int, evm *EVM, contract *Contract, memory *Memory, stack *Stack) ([]byte, error) {
	x, y := stack.pop(), stack.pop() // Take two values from the stack
	stack.push(math.Add(x, y))     // Add them and push the result
	return nil, nil
}
```

---

## Post-Execution: Refunds, Receipts, and the Bloom Filter

### Reverting Transactions

* If an EVM call reverts (e.g., due to an error in the smart contract), Geth uses the **snapshot** taken earlier to revert all state modifications made by that transaction.

### Gas Refunds & Coinbase Payment

* After execution, **gas refunds** are calculated (e.g., for clearing storage or using `SELFDESTRUCT`).
* The **coinbase** (miner's address) is paid the transaction fees.

### Transaction Receipts

* The result of the transaction (including gas used, success/failure, and logs) is captured in a **transaction receipt**.
* **Pre-Byzantium**: Intermediate state roots were calculated after every transaction, a very expensive operation. This is no longer done.
* **Logs and Bloom Filter**:
    * When a smart contract emits an **event**, it's recorded in the **logs** section of the receipt.
    * The **bloom filter** is a probabilistic data structure used to efficiently query for specific events across many blocks.
    * All logs are added to the bloom filter.
    * **How it works**: You hash the event data and set specific bits in the bloom filter. Later, you can check if those bits are set to quickly determine if an event *might* be present.
    * **False Positives**: Bloom filters can have false positives (indicating an event might be present when it isn't), but no false negatives (if the event was present, the bloom filter will reflect it).
    * **Usage**: Clients can quickly scan block headers for their desired events using the bloom filter. If a match is found, they then fetch the full block and receipt to verify the exact event.
    * The bloom filter has a fixed length (e.g., 2048 bytes). While still present, its value has somewhat diminished, and there's a desire to move away from it.

### Worker & Finalizing the Block

* After a transaction is applied, the worker returns to its `CommitTransaction` context.
* The worker has now successfully built a pending block, having applied all necessary transactions.
* The next step is to **finalize** the block by creating the **proof of work** (for PoW chains).