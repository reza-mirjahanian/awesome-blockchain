### **Transaction Propagation in Go Ethereum**

#### **1. Transaction Submission via RPC**
- **User submits a signed transaction** (e.g., via MetaMask) to the node’s RPC endpoint.
- **Internal `eth` API** handles methods like `eth_sendRawTransaction`, `eth_getBlock`, etc.
- Basic validation checks:
  - **Sufficient fees** (gas price within bounds to prevent user errors).
  - **Signature recovery** (`ecrecover`) to derive the sender address.

#### **2. Transaction Pool Handling**
- **Local transactions** (submitted via RPC) are prioritized and **never dropped**.
- **Remote transactions** (from peers) may be replaced if a higher-fee transaction exists.
- **Structure**:
  - **Pending transactions**: Executable (valid nonce, sufficient balance).
  - **Queued transactions**: Non-executable (e.g., nonce gaps); stored temporarily for potential future validity.
- **Sorting**:
  - Transactions ordered by **gas price** (pre-EIP-1559) or **two-dimensional fee market** (post-EIP-1559).
  - Replacement requires:
    - `maxBaseFeePerGas` ≥ 110% of the original.
    - `maxPriorityFeePerGas` ≥ 110% of the original.

```go
// Example: Adding a transaction to the pool
func (pool *TxPool) addTx(tx *types.Transaction, local bool) error {
    // Validate sender, nonce, fees
    if err := pool.validateTx(tx, local); err != nil {
        return err
    }
    // Enqueue transaction
    pool.enqueueTx(tx.Hash(), tx, local)
    return nil
}
```

#### **3. Gossip Protocol**
- **Announcement phase**:
  - Transaction **hashes** broadcasted to **all peers**.
  - Full transactions sent to a **subset of peers** (√N peers) to reduce bandwidth.
- **Fetching phase**:
  - Peers request full transactions via `GetPooledTransactions` if they don’t have them.
  - Received transactions are re-validated and added to the receiver’s transaction pool.

```go
// Example: Broadcasting transactions
func (h *handler) broadcastTransactions(txs []*types.Transaction) {
    for _, peer := range h.peers.SamplePeersToBroadcast() {
        peer.SendTransactions(txs) // Full tx to subset
    }
    hashAnnouncement := make([]common.Hash, len(txs))
    for i, tx := range txs {
        hashAnnouncement[i] = tx.Hash()
    }
    h.broadcast(hashAnnouncement) // Hashes to all peers
}
```

---

### **Block Creation**

#### **1. Miner Workflow**
- Triggered on **new block arrival** or **timer expiration**.
- **Steps**:
  1. **Prepare block**:
     - Set `parentHash`, `timestamp`, `coinbase` (fee recipient).
     - Calculate **dynamic gas limit** (adjusted ±1/1024 of parent’s gas limit).
     - Apply **EIP-1559 rules**: Compute `baseFee` for the next block.
  2. **Fetch transactions**:
     - Fetch **pending transactions** from the pool (prioritizing locals).
     - Apply **gas limit constraints** (default target: 30M gas).

#### **2. Transaction Execution**
- **State preparation**:
  - Initialize **state database** snapshot.
  - Set up **EVM context** (block number, timestamp, etc.).
- **Apply transactions sequentially**:
  - **Pre-checks**:
    - Validate nonce, sender balance, intrinsic gas.
    - **Access list handling** (post-Berlin): Warm storage slots/addresses to reduce gas costs.
  - **Execution**:
    - **Contract creation** (if `to` address is empty).
    - **Contract call** (otherwise).

```go
// Example: Applying a transaction
func ApplyTransaction(config *params.ChainConfig, bc ChainContext, author *common.Address, gp *GasPool, statedb *state.StateDB, header *types.Header, tx *types.Transaction, usedGas *uint64, cfg vm.Config) (*types.Receipt, error) {
    msg, _ := tx.AsMessage(types.MakeSigner(config, header.Number), header.BaseFee)
    // Create EVM instance
    evm := vm.NewEVM(vm.BlockContext{...}, vm.TxContext{...}, statedb, config, cfg)
    // Execute
    result, err := ApplyMessage(evm, msg, gp)
    // Handle result (update state, gas used, etc.)
}
```

---

### **EVM Execution**

#### **1. Core Execution Loop**
- **Opcode processing**:
  - **Fetch next opcode** from contract bytecode.
  - **Jump table** maps opcodes to functions (e.g., `ADD`, `SSTORE`).
  - **Gas consumption**: Each opcode deducts gas (static or dynamic cost).
- **Precompiled contracts**:
  - Optimized native implementations (e.g., `ecrecover`, `sha256`).
  - Defined in `core/vm/contracts.go`.

```go
// Example: EVM opcode execution
func (in *EVMInterpreter) Run(contract *Contract, input []byte, readOnly bool) (ret []byte, err error) {
    for op = contract.GetOp(pc); ; pc++ {
        operation := in.cfg.JumpTable[op]
        // Static gas check
        if !contract.UseGas(operation.constantGas) {
            return nil, ErrOutOfGas
        }
        // Execute opcode (e.g., ADD)
        res, err := operation.execute(&pc, in, callContext)
    }
}
```

#### **2. Contract Calls & Precompiles**
- **Call types**:
  - `CALL`: External contract invocation.
  - `DELEGATECALL`: Preserves caller’s context.
  - `STATICCALL`: Disallows state modifications.
- **Precompiled contracts**:
  - **Gas calculation**: Fixed or dynamic (e.g., `sha256` costs 60 + 12 per word).
  - **Examples**:
    - `ecrecover`: Validates signatures.
    - `blake2f`: BLAKE2b compression (Istanbul hardfork).

```go
// Example: ecRecover precompile
func (c *ecrecover) Run(input []byte) ([]byte, error) {
    // Validate input length
    if len(input) < 128 { return nil, errInputLength }
    // Extract signature (v, r, s)
    v := input[32:64]; r := input[64:96]; s := input[96:128]
    // Recover public key
    pubkey, err := crypto.Ecrecover(input[:32], append(r, append(s, v...)...)
    return pubkey, err
}
```

#### **3. State Management & Reverts**
- **Snapshots**:
  - Taken before transaction execution.
  - **Revert on failure**: Roll back state changes (e.g., out-of-gas, invalid opcode).
- **Gas refunds**:
  - Issued for storage-clearing operations (`SELFDESTRUCT`, `SSTORE` with zero value).
- **Fee distribution**:
  - Transaction fees paid to the `coinbase` address after execution.

---

### **Post-Execution**
- **Receipt generation**:
  - **Status**: Success/failure flag (post-Byzantium).
  - **Logs**: Events emitted during execution.
  - **Bloom filter**: Compact representation of logs for efficient querying.
- **State finalization**:
  - **Intermediate state roots** skipped post-Byzantium for efficiency.
  - **Block assembly**:
    - Transactions + receipts + header → final block.
    - **Proof-of-Work/Proof-of-Stake**: Consensus-specific finalization (not covered here).