https://epf.wiki/#/wiki/protocol/architecture



# Diving Deep into the Go Ethereum Codebase

## Introduction to Go Ethereum (Geth)

* **Go Ethereum (Geth)**: One of the primary **execution layer clients** for Ethereum.
* Assumes **basic knowledge of Ethereum**.
* **Ethereum's Two-Layer Architecture**:
    * **Execution Layer (EL)**: Handles transaction processing, state management, and EVM operations.
    * **Consensus Layer (CL)**: Responsible for blockchain consensus, including fork choice, P2P communication, and validator coordination.

## Ethereum's Architecture: A Quick Recap

* **EPF.wiki**: A valuable resource for understanding Ethereum and its client implementations.
* **Consensus Layer Responsibilities**:
    * **Fork Choice**: Determining the canonical chain.
    * **P2P**: Peer-to-peer networking for block and attestation propagation.
    * **Validators**: Propose and attest to blocks.
* **Execution Layer Responsibilities**:
    * **State Storage**: Maintaining the current state of the Ethereum blockchain.
    * **EVM Operations**: Executing smart contract code via the *Ethereum Virtual Machine*.
    * **Transaction Processing**: Validating and including transactions.
    * **Mempool**: Storing pending transactions.
    * **P2P**: Communicating with other execution layer clients.
* **Engine API**: The interface through which the Execution Layer and Consensus Layer clients communicate.
* **Historical Context**:
    * Initially, Geth included both Consensus and Execution layer functionalities.
    * **The Merge**: A significant Ethereum upgrade that *separated* the Consensus and Execution layers.
    * Now, dedicated Consensus Layer clients (e.g., **Prism**, written in Go) handle consensus, and Execution Layer clients (like Geth) handle execution.

## Exploring the Geth Codebase: Entry Point

* **Finding the Entry Point**: For any Go codebase, the first step is locating the `main.go` file.
* **Geth Repository Structure**:
    * Cloned from GitHub.
    * `cmd` folder: Contains subfolders for various executables.
    * `cmd/geth`: The primary Geth executable.
        * `main.go`: The main entry point for the Geth client.

* **`main.go` Analysis**:
    * `init()` function: Performs initial setup and assignments.
    * `main()` function:
        * Creates a new command-line interface (CLI) application using `cli.NewApp()`.
        * The `app.Action` field is assigned to the `geth` function.
        * `app.Run()` executes the CLI application.
    * **`geth` function**: The core entry point when no specific subcommand is provided.
        * By default, it creates and runs a default Ethereum node in blocking mode.

## Node Initialization: `makeFullNode` and `startNode`

* The `geth` function calls two crucial functions:
    * **`makeFullNode()`**: Responsible for creating the Ethereum node instance.
    * **`startNode()`**: Initiates the node and its services.

### `makeFullNode()`

* Returns an object of type `*node.Node`.
* **`node.Node`**: A container within the `node` package where various services can be registered.
    * Key components of `node.Node`:
        * `P2PServer`: Handles peer-to-peer communication.
        * `Config`: Stores configuration settings.
        * `HTTP` / `WebSocket` servers: For RPC communication.
        * `Databases`: For storing blockchain data.
* **Initialization Steps within `makeFullNode`**:
    1.  `makeConfigNode()`: Loads Geth configuration and creates a blank `Node` instance.
    2.  **`registerEthService()`**: Crucial step for registering the Ethereum client.
        * Uses `ethconfig.Config` (from the `eth` package) to create a new Ethereum client instance.
        * Adds an "Ethereum client" to the `Node` (also referred to as the "stack").

### Ethereum Object (`eth.Ethereum`)

* `eth.New()`: Creates a new `Ethereum` object, initializing common Ethereum components.
* **`eth.Ethereum` struct (defined in `eth/backend.go`):** Implements the "Ethereum full node service."
    * Requires various components to run a full node, including:
        * **`TxPool`**: Manages transactions.
        * `Downloader`: For synchronizing blockchain data.
        * `Fetcher`: For retrieving specific data.
        * `GasPrice` oracle: For estimating gas prices.

## Transaction Pool (`txpool.TxPool`)

* **`txpool.TxPool` struct (defined in `core/txpool/txpool.go`):** An aggregator for various transaction-specific pools.
    * **Purpose**: Tracks all transactions deemed "interesting" by the node.
    * **Transaction Lifecycle**:
        * **Entry**: Transactions enter the pool from the network (P2P) or are submitted locally.
        * **In-memory storage**: Generally, transactions in the pool are held in memory.
        * **Exit**: Transactions exit the pool when:
            * Included in a block.
            * Evicted due to resource constraints (e.g., pool full, underpriced transactions).

### `txpool.TxPool` Subpools

* `txpool.TxPool` manages different `Subpool` interfaces.
* A `Subpool` is a specialized, independent transaction pool.
* **`Subpool` Interface Functions**:
    * `Filter()`: To filter transactions.
    * `Init()`: Initialization.
    * `Close()`: Cleanup.
    * `Reset()`: Resetting the pool.
    * `SetGasTip()`: Related to gas tip management.
    * `Has()`, `Get()`: To check for and retrieve transactions.
    * `GetRLP()`, `GetBlobs()`: For retrieving RLP-encoded transactions and blob data.

### Blob Transactions and EIP-4844 (Proto-Danksharding)

* **Blob Transactions**: A new transaction type introduced with EIP-4844.
    * **Purpose**: Ethereum's scaling effort to support Layer 2 (L2) solutions.
    * **L2 Data Storage**: L2s often need to store proof data or other information on Layer 1 (Ethereum).
    * **Call Data vs. Blobs**: Historically, this data was stored in `calldata`, which was expensive. Blobs provide a more efficient, transient data storage solution.
    * **Blob Size**: Each blob has a specific size (e.g., 128 KB).
    * L2s submit their data through blob transactions.

### `txpool.TxPool` and Blockchain Interaction

* `txpool.TxPool` holds a reference to the **`blockchain.Blockchain`** interface.
* **`blockchain.Blockchain` Interface**: Defines the minimal methods required to back a transaction pool with chain data.
    * Needs information about:
        * Current block.
        * State at a given block.
        * Chain's fork configuration.

### `core.Blockchain` Struct (defined in `core/blockchain.go`)

* Represents the canonical chain, managing:
    * Chain imports.
    * Block rewards.
    * Chain reorganizations (reorgs).
* **Key Components of `core.Blockchain`**:
    * **`ChainConfig`**: Contains blockchain-specific settings.
        * `ChainID`: Unique identifier for the network.
        * **Hard Forks**: Information about past and future hard fork block numbers or timestamps (e.g., Homestead, Byzantium, Istanbul, Berlin, London, Shanghai).
        * *Note*: Earlier hard forks were block-based; more recent ones (Shanghai, Capella, etc.) are time-based.
        * **Historical Consensus Mechanisms**: References to `Ethash`, `Clique` (now mostly handled by the Consensus Layer).
    * **`DB` (EDB Database)**: The low-level persistent database for storing final content (key-value data).
    * `Snapshots`, `TryDB`: For specific database and trie management.
    * `StateDB`: Manages the current state of the blockchain.
    * `CurrentBlock`, `CurrentSnapBlock`: Track the head of the canonical chain and the sync process.
    * `Caches`, `Channels`: For coordination.
    * `ConsensusEngine`, `Validators`: Although primarily handled by the CL now, these fields exist.

### `txpool.New()` and its Initialization

* `txpool.New()` is called during the `registerEthService()` -> `eth.New()` -> `makeFullNode()` -> `geth()` startup flow.
* For each subpool, `Init()` is called, followed by `pool.Loop()`.
* **`pool.Loop()`**: The main event loop for the transaction pool.
    * Listens for **new head events** (blockchain progression).
    * If a new head is detected, it triggers a `Reset()` on subpools to update their head information.
    * Runs in a separate goroutine to allow continuous consumption of chain head events.

### Subpools: `blobpool.go` and `legacypool.go`

* Currently, two main subpools:
    * **`blobpool.go`**: For blob transactions.
    * **`legacypool.go`**: For "normal" Ethereum transactions (e.g., ETH transfers).
* **`legacypool.go` (`Init()` function)**:
    * Sets gas price thresholds.
    * Starts an internal goroutine that runs an infinite `for` loop.
    * This loop continuously listens for:
        * Shutdown events.
        * Statistics events.
        * Transaction eviction events.

### Adding Transactions to the Pool (`Add` function)

* `Add()`: Enqueues a batch of valid transactions into the pool.
* Called when:
    * A node receives a new transaction directly.
    * A new transaction is received from a peer.
    * Blocks are imported (transactions from the block are added to the pool).
* **Transaction Validation**: Basic checks are performed.
* **`addTxLocked()`**: Handles the core logic for adding a single transaction, with locking for ACID compliance.
    * **Discarding Known Transactions**: Prevents re-adding already known transactions.
    * **Address Tracking**: Limits the number of transactions from a single address to prevent DDoS attacks.
    * **Full Pool Logic**: If the transaction pool is full, it may:
        * Discard an "underpriced" existing transaction to make space for a new, higher-value transaction.
        * **Slots**: Transactions are measured in "slots" (fixed-size units) rather than just number of transactions, as different transaction types have different sizes.
* **Transaction Pool Stages**:
    * **Queue (`Q`)**: Transactions are initially queued (non-processable).
    * **Pending (`Pending`)**: Transactions are promoted to pending, waiting to be included in a block.
* **`legacyPool` Struct**:
    * `Pending` (map from `address` to `List`): Contains currently processable transactions.
    * `Q` (map from `address` to `List`): Contains queued but non-processable transactions.
    * `List`: A sorted map storing transactions belonging to a specific account, indexed by nonce.

## Ethereum Virtual Machine (`core/vm/evm.go`)

* **`core.vm.EVM` struct**: The core Ethereum Virtual Machine object.
    * Provides tools to run smart contracts on a given state with a provided context.
    * **Error Handling**: Any EVM error results in a "revert state" and consumes all remaining gas. Faulty code is assumed.
* **`Call()` function**: The most important function for executing smart contract calls.
    * Takes caller address, input data, gas, and value as parameters.
    * Handles precompiled contracts.
    * Retrieves contract code.
    * **`EVM.interpreter.Run()`**: The heart of EVM execution.

### `interpreter.Run()`

* **`Interpreter`**: Represents an EVM interpreter.
* **Main Run Loop**: Continuously executes **opcodes** until:
    * An explicit `STOP` or `SELFDESTRUCT` instruction is executed.
    * An error occurs.
    * The `done` flag is set by the parent context.
* **Opcode Execution**:
    * Smart contract code is converted into a sequence of opcodes (e.g., `ADD`, `SUB`, `STOP`).
    * For each opcode, the corresponding `operation.Execute()` function is called.
    * Different opcodes have different execution functions.
* **Error Reversion**: If an error occurs during EVM execution:
    * The state is **reverted to a snapshot** (an earlier copy of the state).
    * Remaining gas is consumed. This prevents permanent state corruption.
    * `StateDB.RevertToSnapshot()` is used for this purpose.

## State Transition (`core/state_transition.go`)

* **`StateTransition` struct**: Represents a change in the world state when a transaction is applied.
* **`Execute()` function**: Transitions the state by applying the current message (transaction).
    * Returns EVM execution results (used gas, return data, error).
* **`Execute()` Call Chain**:
    * Called by `ApplyMessage()`.
    * Called by `StateProcessor.ApplyTransaction()`.
    * Called by `ProcessBlock()` (in `core/blockchain.go`).

### `ProcessBlock()` and `InsertChain()`

* **`ProcessBlock()`**: Executes and validates a given block.
* **`InsertChain()` (in `core/blockchain.go`)**: Internal implementation for inserting a batch of blocks.
    * Called when a node receives a batch of blocks from a peer.

## Downloader (`eth/downloader/downloader.go`)

* **Purpose**: Responsible for:
    * Listening to new block events.
    * Downloading blocks when syncing a new node or catching up.
* **`InsertBlockResults()`**: Called within `ProcessFullSync()` -> `SyncToHead()`.
* **`SyncToHead()`**: Synchronizes the node with a peer by selecting a peer and downloading data.
* **`Synchronize()`**: Called from `Resume()` (which starts the download or backfilling of state data).
* **`Skeleton.Startup()` (in `eth/downloader/skeleton.go`)**: An initial background loop that waits for events to start or tear down the syncer.
    * Continuously listens for `Head` events and performs corresponding syncing.
    * `NewSkeleton()` starts this process.
* **Connection to `makeFullNode()`**: `NewDownloader()` is called during `NewHandler()` -> `eth.New()` -> `registerEthService()` -> `makeFullNode()`, demonstrating the coordinated startup process.

## Peer-to-Peer (P2P) Communication (`p2p/server.go`)

* **P2P Package**: Responsible for:
    * **Peer Discovery**: Finding other nodes on the network.
    * **Message Exchange**: Sending and receiving information (blocks, transactions, etc.) with peers.
    * **Handshaking**: Initial communication protocol between peers.
* **`p2p/server.go`**: Contains the `Start()` function, which initiates the P2P server.

## Handler (`eth/handler.go`)

* **`eth.Handler` struct**: A crucial component that combines several key functionalities:
    * `Downloader`: For block synchronization.
    * `TxFetcher`: For retrieving new transactions.
    * `TxPool`: Reference to the transaction pool.
    * `Blockchain`: Reference to the blockchain struct.
* **`TxFetcher`**: Responsible for retrieving new transactions based on announcements from peers.
    * **`Notify()`**: Notifies announcers (peers) about the availability of new transaction batches.
    * **`Enqueue()` (`NQ`)**: Imports a batch of received transactions into the transaction pool.
        * Calls `AddTransactions()` (a callback function) which ultimately adds transactions to the `txpool`.
* **`NewHandler()`**: Function to create a new `Handler` instance.
    * `AddTransactions` is a callback function passed to the `TxFetcher`, allowing the fetcher to add transactions to the `txpool` without directly holding a reference to the entire pool.

## Execution Layer and Consensus Layer Interaction: Engine API

* **Engine API**: The bridge for communication between CL and EL clients.
* **Consensus Layer Request for Execution Payload**:
    * When a validator needs to create a new block, the Consensus Layer client (e.g., Prism) requests the "execution payload" from the Execution Layer client (Geth).
    * This request is made through the Engine API.
* **`engine.GetPayloadV4()` (Geth side)**: A function within Geth that returns the execution payload.
    * Returns an `engine.ExecutionPayloadEnvelope`, containing:
        * **Executable Data**: Standard block fields (parent hash, state root, gas limit, transactions, base fee per gas, timestamp, etc.).
        * **Blobs**: `BlobsBundle` containing commitments, proofs, and the actual blob data.
* **Prism (Consensus Layer Client)**:
    * `engine.NewPayloadMethodV4`: Used by Prism to call the Geth's Engine API (e.g., via JSON RPC).
    * `NewPayload()`: Called when it's time for a validator to create a new block.

```go
// Example: Geth's geth function entry point (simplified)
package main

import (
	"fmt"
	"os"
	"path/filepath"

	"github.com/ethereum/go-ethereum/cmd/geth/cmd" // Assuming 'geth' is a cmd subpackage
	"github.com/ethereum/go-ethereum/node"
	"github.com/urfave/cli/v2" // Common CLI library used in Go projects
)

func main() {
	// Create a new CLI application
	app := cli.NewApp()
	app.Name = "geth"
	app.Usage = "the go-ethereum command line interface"
	app.Version = "1.x.x" // Placeholder

	// Define commands (e.g., export, init)
	app.Commands = []*cli.Command{
		// ... other commands like 'export', 'init', etc.
		// For simplicity, we'll focus on the default action
	}

	// The default action when no subcommand is given
	app.Action = func(c *cli.Context) error {
		// This is where the core 'geth' logic starts
		fmt.Println("Running default Geth node...")

		// In a real scenario, this would involve calling makeFullNode and startNode
		// For demonstration, let's simulate a simplified node startup.
		
		// Simulate node configuration
		nodeConfig := &node.Config{
			DataDir: filepath.Join(os.Getenv("HOME"), ".ethereum"),
			// ... other node configurations
		}

		// Simulate creating a full node (simplified)
		fmt.Println("Creating full node...")
		// In actual Geth, this involves makeFullNode()
		// For demo, just print
		
		// Simulate starting the node
		fmt.Println("Starting node...")
		// In actual Geth, this involves startNode()
		// For demo, just print
		
		fmt.Println("Geth node running in blocking mode. Waiting for shutdown...")
		// In a real application, this would block until os.Interrupt or similar
		select {} // Block indefinitely for demonstration

		return nil
	}

	if err := app.Run(os.Args); err != nil {
		fmt.Fprintf(os.Stderr, "Error running Geth: %v\n", err)
		os.Exit(1)
	}
}
```

```go
// Example: Simplified Transaction Pool (core/txpool/txpool.go)
package txpool

import (
	"fmt"
	"sync"
	"time"

	"github.com/ethereum/go-ethereum/core"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/log"
)

// Subpool interface (simplified)
type Subpool interface {
	Init(chain core.Blockchain, config interface{}) error
	Add(txs types.Transactions) []error // Add a batch of transactions
	// ... other subpool methods
}

// TxPool is an aggregator for various transaction-specific pools.
type TxPool struct {
	mu sync.RWMutex // Mutex for concurrent access

	chain    core.Blockchain // Reference to the blockchain
	legacyTx Subpool         // Subpool for legacy transactions
	blobTx   Subpool         // Subpool for blob transactions

	// ... other fields like event channels, caches
}

// New creates a new transaction pool.
func New(chain core.Blockchain, config interface{}) *TxPool {
	pool := &TxPool{
		chain:    chain,
		legacyTx: NewLegacyPool(), // Assume this creates a concrete implementation
		blobTx:   NewBlobPool(),   // Assume this creates a concrete implementation
	}

	// Initialize subpools
	if err := pool.legacyTx.Init(chain, config); err != nil {
		log.Error("Failed to initialize legacy transaction pool", "err", err)
	}
	if err := pool.blobTx.Init(chain, config); err != nil {
		log.Error("Failed to initialize blob transaction pool", "err", err)
	}

	// Start the main event loop in a goroutine
	go pool.loop()
	return pool
}

// loop is the transaction pool's main event loop.
func (p *TxPool) loop() {
	ticker := time.NewTicker(5 * time.Second) // Simulate regular checks
	defer ticker.Stop()

	for {
		select {
		case <-ticker.C:
			// Periodically check for new chain heads, evictions, etc.
			// In a real scenario, this would listen to chain head events
			fmt.Println("TxPool: Checking for new chain heads or evictions...")
			// Simulate a reset if chain head changes
			// p.legacyTx.Reset(oldHead, newHead)
			// p.blobTx.Reset(oldHead, newHead)
		case <-time.After(10 * time.Minute): // Simulate a shutdown signal
			fmt.Println("TxPool: Shutting down...")
			return
		}
	}
}

// Add adds a batch of transactions to the pool.
func (p *TxPool) Add(txs types.Transactions) []error {
	p.mu.Lock()
	defer p.mu.Unlock()

	var errors []error
	for _, tx := range txs {
		// Basic validation (simplified)
		if tx.Gas() == 0 {
			errors = append(errors, fmt.Errorf("transaction gas cannot be zero"))
			continue
		}

		// Determine transaction type and add to appropriate subpool
		if tx.Type() == types.BlobTxType {
			if errs := p.blobTx.Add(types.Transactions{tx}); len(errs) > 0 {
				errors = append(errors, errs...)
			}
		} else {
			if errs := p.legacyTx.Add(types.Transactions{tx}); len(errs) > 0 {
				errors = append(errors, errs...)
			}
		}
	}
	return errors
}

// Simplified LegacyPool implementation for demonstration
type LegacyPool struct{}

func NewLegacyPool() *LegacyPool { return &LegacyPool{} }
func (p *LegacyPool) Init(chain core.Blockchain, config interface{}) error {
	fmt.Println("LegacyPool: Initialized.")
	return nil
}
func (p *LegacyPool) Add(txs types.Transactions) []error {
	fmt.Printf("LegacyPool: Adding %d legacy transactions.\n", len(txs))
	// In a real implementation, this would involve nonce tracking,
	// gas price sorting, and potential eviction logic.
	return nil
}

// Simplified BlobPool implementation for demonstration
type BlobPool struct{}

func NewBlobPool() *BlobPool { return &BlobPool{} }
func (p *BlobPool) Init(chain core.Blockchain, config interface{}) error {
	fmt.Println("BlobPool: Initialized.")
	return nil
}
func (p *BlobPool) Add(txs types.Transactions) []error {
	fmt.Printf("BlobPool: Adding %d blob transactions.\n", len(txs))
	// In a real implementation, this would handle blob-specific validation.
	return nil
}

```

```go
// Example: Simplified EVM Execution (core/vm/evm.go)
package vm

import (
	"fmt"

	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/params"
)

// EVM is the Ethereum virtual machine base object.
type EVM struct {
	interpreter *Interpreter // The EVM interpreter
	// ... other fields like state, gas pool, etc.
}

// Interpreter represents an EVM interpreter.
type Interpreter struct {
	// ... fields like contract, memory, stack
}

// NewEVM creates a new EVM instance (simplified).
func NewEVM() *EVM {
	return &EVM{
		interpreter: &Interpreter{},
	}
}

// Call executes the contract associated with the address.
func (evm *EVM) Call(caller Address, contractAddr Address, input []byte, gas uint64, value *big.Int) ([]byte, uint64, error) {
	fmt.Printf("EVM: Calling contract %s with input %x\n", contractAddr.Hex(), input)

	// Simulate precompile check
	if contractAddr == params.PrecompiledContractsHomestead[0] { // Example precompile address
		fmt.Println("EVM: Executing precompiled contract.")
		return []byte("precompile_result"), 0, nil // Simulate result
	}

	// Simulate contract code retrieval
	contractCode := []byte{ /* bytecode representing contract logic */ }
	if len(contractCode) == 0 {
		return nil, gas, fmt.Errorf("contract code not found")
	}

	// Execute the contract via the interpreter
	result, err := evm.interpreter.Run(nil, contractCode, input, gas, value) // Simplified args
	if err != nil {
		fmt.Println("EVM: Execution error, reverting state.")
		// In a real EVM, this would trigger a state revert and gas consumption
		return nil, gas, err
	}

	return result, 0, nil // Return result and remaining gas
}

// Run executes the EVM interpreter's main loop.
func (i *Interpreter) Run(contract *Contract, code, input []byte, gas uint64, value *big.Int) ([]byte, error) {
	fmt.Println("Interpreter: Starting main run loop.")

	pc := uint64(0) // Program Counter

	// Simulate opcode execution
	for pc < uint64(len(code)) {
		opcode := OpCode(code[pc])
		fmt.Printf("Interpreter: Executing opcode %s (0x%x) at PC %d\n", opcode.String(), opcode, pc)

		// Get the operation function for the opcode
		op := GetOp(opcode) // Assuming a function to get operation by opcode

		// Execute the operation
		_, err := op.Execute(nil, i, contract, &gas) // Simplified args
		if err != nil {
			return nil, err
		}

		pc++ // Move to the next instruction

		// Simulate stop condition
		if opcode == STOP {
			fmt.Println("Interpreter: STOP opcode encountered.")
			return []byte("execution_successful"), nil
		}
	}

	fmt.Println("Interpreter: Loop finished.")
	return []byte("execution_successful"), nil
}

// Simplified Opcode and Operation types for demonstration
type OpCode byte

const (
	STOP OpCode = 0x00
	ADD  OpCode = 0x01
	// ... other opcodes
)

func (op OpCode) String() string {
	switch op {
	case STOP:
		return "STOP"
	case ADD:
		return "ADD"
	default:
		return fmt.Sprintf("UNKNOWN_OPCODE(0x%x)", byte(op))
	}
}

type ExecutionFunc func(stack *Stack, mem *Memory, contract *Contract, gas *uint64) ([]byte, error)

type Operation struct {
	execute ExecutionFunc
	// ... other fields
}

// GetOp returns the Operation for a given OpCode (simplified).
func GetOp(opcode OpCode) *Operation {
	switch opcode {
	case STOP:
		return &Operation{
			execute: func(stack *Stack, mem *Memory, contract *Contract, gas *uint64) ([]byte, error) {
				fmt.Println("Operation: Executing STOP.")
				return nil, nil // No error, stop execution
			},
		}
	case ADD:
		return &Operation{
			execute: func(stack *Stack, mem *Memory, contract *Contract, gas *uint64) ([]byte, error) {
				fmt.Println("Operation: Executing ADD.")
				// Simulate some stack operations for ADD
				return nil, nil
			},
		}
	default:
		return &Operation{
			execute: func(stack *Stack, mem *Memory, contract *Contract, gas *uint64) ([]byte, error) {
				return nil, fmt.Errorf("unimplemented opcode: %s", opcode.String())
			},
		}
	}
}

// Simplified Stack and Memory (placeholders)
type Stack struct{}
type Memory struct{}
type Contract struct{}
type Address [20]byte // Placeholder for address type
```