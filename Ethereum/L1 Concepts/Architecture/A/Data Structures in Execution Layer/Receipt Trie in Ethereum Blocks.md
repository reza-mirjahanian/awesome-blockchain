# Receipt Trie in Ethereum Blocks

## **Foundational Concepts**

### **What is a Receipt Trie?**

The **Receipt Trie** is one of three **Merkle Patricia Tries** in each Ethereum block, containing cryptographic proofs of transaction execution results. It stores transaction receipts that prove what happened during transaction execution without requiring full blockchain replay.

**Key characteristics:**
- **Immutable proof structure** - Cannot be altered after block finalization
- **Efficient verification** - Allows proving transaction outcomes without full execution
- **Gas consumption tracking** - Records exact gas usage for each transaction
- **Event logging** - Stores smart contract event emissions
- **Status verification** - Confirms transaction success/failure

### **Position in Ethereum Architecture**

| **Trie Type** | **Purpose** | **Root Location** |
|---------------|-------------|-------------------|
| **State Trie** | Account states, balances, nonces | `block.stateRoot` |
| **Transaction Trie** | Raw transaction data | `block.transactionsRoot` |
| **Receipt Trie** | Transaction execution results | `block.receiptsRoot` |

## **Transaction Receipt Structure**

### **Core Receipt Fields**

```cpp
struct TransactionReceipt {
    uint64_t status;              // 1 = success, 0 = failure
    uint64_t cumulativeGasUsed;   // Total gas used up to this tx
    bytes32 logsBloom;            // Bloom filter for efficient log searching
    vector<Log> logs;             // Event logs emitted during execution
    
    // EIP-2718: Transaction type support
    uint8_t transactionType;      // 0=legacy, 1=access list, 2=dynamic fee
    
    // EIP-658: Post-byzantium status field
    bytes32 postStateRoot;        // Pre-byzantium only
};
```

### **Log Structure for Events**

```cpp
struct Log {
    address contractAddress;      // Contract that emitted the log
    vector<bytes32> topics;       // Indexed event parameters (max 4)
    bytes data;                   // Non-indexed event parameters
};
```

## **Merkle Patricia Trie Implementation**

### **Trie Node Types**

```rust
#[derive(Clone, Debug)]
enum TrieNode {
    Empty,
    Leaf {
        key_remainder: Vec<u8>,
        value: Vec<u8>,
    },
    Extension {
        shared_prefix: Vec<u8>,
        next_node_hash: H256,
    },
    Branch {
        children: [Option<Box<TrieNode>>; 16],  // 0-F hex digits
        value: Option<Vec<u8>>,                  // Optional value at this node
    },
}
```

### **Receipt Trie Construction**

```go
package main

import (
    "crypto/sha256"
    "encoding/hex"
    "fmt"
)

type ReceiptTrie struct {
    root *TrieNode
    receipts map[uint64]*TransactionReceipt
}

type TransactionReceipt struct {
    Status             uint8
    CumulativeGasUsed  uint64
    LogsBloom          [256]byte
    Logs               []Log
    TransactionIndex   uint64
}

type Log struct {
    Address common.Address
    Topics  []common.Hash
    Data    []byte
}

// Build receipt trie from transaction receipts
func (rt *ReceiptTrie) BuildTrie(receipts []*TransactionReceipt) {
    rt.root = &TrieNode{}
    
    for i, receipt := range receipts {
        // Transaction index as key (RLP encoded)
        key := rlpEncodeUint64(uint64(i))
        
        // Receipt as value (RLP encoded)
        value := rlpEncodeReceipt(receipt)
        
        // Insert into trie
        rt.insertNode(rt.root, key, value, 0)
    }
}

func (rt *ReceiptTrie) insertNode(node *TrieNode, key, value []byte, depth int) {
    if depth == len(key) {
        node.Value = value
        return
    }
    
    nibble := key[depth]
    if node.Children[nibble] == nil {
        node.Children[nibble] = &TrieNode{}
    }
    
    rt.insertNode(node.Children[nibble], key, value, depth+1)
}

// Generate Merkle root hash
func (rt *ReceiptTrie) GetRoot() []byte {
    return rt.hashNode(rt.root)
}

func (rt *ReceiptTrie) hashNode(node *TrieNode) []byte {
    if node == nil {
        return sha256.Sum256([]byte{})[:] // Empty node hash
    }
    
    // Serialize node structure
    serialized := rt.serializeNode(node)
    hash := sha256.Sum256(serialized)
    return hash[:]
}
```

### **Advanced Trie Operations**

```rust
use std::collections::HashMap;
use sha3::{Keccak256, Digest};

pub struct ReceiptTrie {
    nodes: HashMap<Vec<u8>, TrieNode>,
    root_hash: Vec<u8>,
}

impl ReceiptTrie {
    // Merkle proof generation for specific receipt
    pub fn generate_proof(&self, transaction_index: u64) -> Vec<Vec<u8>> {
        let mut proof = Vec::new();
        let key = self.encode_transaction_index(transaction_index);
        self.collect_proof_nodes(&self.root_hash, &key, 0, &mut proof);
        proof
    }
    
    fn collect_proof_nodes(
        &self, 
        node_hash: &[u8], 
        key: &[u8], 
        depth: usize, 
        proof: &mut Vec<Vec<u8>>
    ) {
        if let Some(node) = self.nodes.get(node_hash) {
            proof.push(self.serialize_node(node));
            
            match node {
                TrieNode::Branch { children, .. } => {
                    if depth < key.len() {
                        let nibble = key[depth] as usize;
                        if let Some(child_hash) = &children[nibble] {
                            self.collect_proof_nodes(child_hash, key, depth + 1, proof);
                        }
                    }
                },
                TrieNode::Extension { shared_prefix, next_node_hash } => {
                    if key[depth..].starts_with(shared_prefix) {
                        self.collect_proof_nodes(
                            next_node_hash, 
                            key, 
                            depth + shared_prefix.len(), 
                            proof
                        );
                    }
                },
                TrieNode::Leaf { .. } => {
                    // Leaf node - proof collection complete
                }
            }
        }
    }
    
    // Verify receipt existence using Merkle proof
    pub fn verify_receipt_proof(
        &self,
        receipt: &TransactionReceipt,
        transaction_index: u64,
        proof: &[Vec<u8>],
        expected_root: &[u8]
    ) -> bool {
        let key = self.encode_transaction_index(transaction_index);
        let value = self.encode_receipt(receipt);
        
        let calculated_root = self.calculate_root_from_proof(&key, &value, proof);
        calculated_root == expected_root
    }
    
    fn calculate_root_from_proof(
        &self,
        key: &[u8],
        value: &[u8],
        proof: &[Vec<u8>]
    ) -> Vec<u8> {
        // Reconstruct trie path from proof nodes
        let mut current_hash = self.hash_leaf_node(key, value);
        
        for (i, proof_node) in proof.iter().enumerate().rev() {
            current_hash = self.hash_branch_with_child(proof_node, &current_hash, key, i);
        }
        
        current_hash
    }
}
```

## **Gas Tracking and Cumulative Calculations**

### **Gas Usage Accumulation**

```cpp
class GasTracker {
private:
    uint64_t totalGasUsed;
    std::vector<uint64_t> transactionGasUsage;
    
public:
    // Process transaction and update cumulative gas
    TransactionReceipt processTransaction(
        const Transaction& tx, 
        const WorldState& preState
    ) {
        uint64_t gasUsedBefore = totalGasUsed;
        
        // Execute transaction
        ExecutionResult result = executeTransaction(tx, preState);
        
        // Update cumulative gas
        uint64_t txGasUsed = result.gasUsed;
        totalGasUsed += txGasUsed;
        transactionGasUsage.push_back(txGasUsed);
        
        // Build receipt
        TransactionReceipt receipt;
        receipt.status = result.success ? 1 : 0;
        receipt.cumulativeGasUsed = totalGasUsed;  // Key: cumulative, not individual
        receipt.logs = result.logs;
        receipt.logsBloom = generateLogsBloom(result.logs);
        receipt.transactionIndex = transactionGasUsage.size() - 1;
        
        return receipt;
    }
    
    // Gas calculation verification
    bool verifyGasCalculations(const std::vector<TransactionReceipt>& receipts) {
        uint64_t runningTotal = 0;
        
        for (size_t i = 0; i < receipts.size(); i++) {
            uint64_t expectedGasForTx;
            if (i == 0) {
                expectedGasForTx = receipts[i].cumulativeGasUsed;
            } else {
                expectedGasForTx = receipts[i].cumulativeGasUsed - 
                                 receipts[i-1].cumulativeGasUsed;
            }
            
            runningTotal += expectedGasForTx;
            
            if (runningTotal != receipts[i].cumulativeGasUsed) {
                return false;  // Gas calculation error
            }
        }
        
        return true;
    }
};
```

## **Logs Bloom Filter Implementation**

### **Efficient Log Searching**

```go
package bloom

import (
    "crypto/sha3"
    "math/big"
)

const (
    BloomBitLength = 2048
    BloomHashCount = 3
)

type BloomFilter [256]byte  // 2048 bits = 256 bytes

// Generate bloom filter from logs
func CreateLogsBloom(logs []Log) BloomFilter {
    var bloom BloomFilter
    
    for _, log := range logs {
        // Add contract address to bloom
        addToBloom(&bloom, log.Address[:])
        
        // Add each topic to bloom
        for _, topic := range log.Topics {
            addToBloom(&bloom, topic[:])
        }
    }
    
    return bloom
}

func addToBloom(bloom *BloomFilter, data []byte) {
    hash := sha3.NewLegacyKeccak256()
    hash.Write(data)
    hashBytes := hash.Sum(nil)
    
    // Generate 3 different bit positions from hash
    for i := 0; i < BloomHashCount; i++ {
        // Take 2 bytes from different positions in hash
        idx := i * 2
        if idx+1 >= len(hashBytes) {
            break
        }
        
        // Convert to bit position in bloom filter
        bitPos := (uint16(hashBytes[idx]) | (uint16(hashBytes[idx+1]) << 8)) % BloomBitLength
        
        // Set bit in bloom filter
        byteIndex := bitPos / 8
        bitIndex := bitPos % 8
        bloom[byteIndex] |= (1 << bitIndex)
    }
}

// Check if logs might be present (no false negatives, possible false positives)
func (bloom *BloomFilter) MightContain(address []byte, topics [][]byte) bool {
    // Check address
    if !bloom.containsData(address) {
        return false
    }
    
    // Check all topics
    for _, topic := range topics {
        if !bloom.containsData(topic) {
            return false
        }
    }
    
    return true  // Might contain (or false positive)
}

func (bloom *BloomFilter) containsData(data []byte) bool {
    hash := sha3.NewLegacyKeccak256()
    hash.Write(data)
    hashBytes := hash.Sum(nil)
    
    for i := 0; i < BloomHashCount; i++ {
        idx := i * 2
        if idx+1 >= len(hashBytes) {
            break
        }
        
        bitPos := (uint16(hashBytes[idx]) | (uint16(hashBytes[idx+1]) << 8)) % BloomBitLength
        byteIndex := bitPos / 8
        bitIndex := bitPos % 8
        
        if (bloom[byteIndex] & (1 << bitIndex)) == 0 {
            return false  // Bit not set - definitely not present
        }
    }
    
    return true  // All bits set - might be present
}
```

## **Receipt Trie Verification and Proofs**

### **Light Client Verification**

```rust
pub struct ReceiptVerifier {
    known_block_headers: HashMap<H256, BlockHeader>,
}

impl ReceiptVerifier {
    // Verify receipt authenticity using only block header
    pub fn verify_receipt_in_block(
        &self,
        receipt: &TransactionReceipt,
        transaction_index: u64,
        merkle_proof: &[Vec<u8>],
        block_hash: &H256
    ) -> Result<bool, VerificationError> {
        
        // Get block header (contains receipt trie root)
        let header = self.known_block_headers.get(block_hash)
            .ok_or(VerificationError::UnknownBlock)?;
        
        // Verify receipt against trie root
        let receipt_key = self.encode_transaction_index(transaction_index);
        let receipt_value = self.encode_receipt(receipt);
        
        let is_valid = self.verify_merkle_proof(
            &receipt_key,
            &receipt_value,
            merkle_proof,
            &header.receipts_root
        );
        
        if !is_valid {
            return Err(VerificationError::InvalidProof);
        }
        
        // Additional verification: gas usage consistency
        self.verify_gas_consistency(receipt, transaction_index, block_hash)?;
        
        // Verify logs bloom filter consistency
        self.verify_logs_bloom_consistency(receipt, &header.logs_bloom)?;
        
        Ok(true)
    }
    
    fn verify_gas_consistency(
        &self,
        receipt: &TransactionReceipt,
        transaction_index: u64,
        block_hash: &H256
    ) -> Result<(), VerificationError> {
        
        let header = self.known_block_headers.get(block_hash).unwrap();
        
        // Cumulative gas must not exceed block gas limit
        if receipt.cumulative_gas_used > header.gas_limit {
            return Err(VerificationError::ExcessiveGasUsage);
        }
        
        // For last transaction, cumulative gas should equal block gas used
        if transaction_index == header.transaction_count - 1 {
            if receipt.cumulative_gas_used != header.gas_used {
                return Err(VerificationError::GasMismatch);
            }
        }
        
        Ok(())
    }
    
    fn verify_logs_bloom_consistency(
        &self,
        receipt: &TransactionReceipt,
        block_logs_bloom: &BloomFilter
    ) -> Result<(), VerificationError> {
        
        // Generate bloom from receipt logs
        let receipt_bloom = self.create_logs_bloom(&receipt.logs);
        
        // Check if receipt bloom is subset of block bloom
        if !self.is_bloom_subset(&receipt_bloom, block_logs_bloom) {
            return Err(VerificationError::BloomMismatch);
        }
        
        Ok(())
    }
    
    fn is_bloom_subset(subset: &BloomFilter, superset: &BloomFilter) -> bool {
        for i in 0..256 {
            if (subset.0[i] & superset.0[i]) != subset.0[i] {
                return false;  // Subset has bits not in superset
            }
        }
        true
    }
}

#[derive(Debug)]
pub enum VerificationError {
    UnknownBlock,
    InvalidProof,
    ExcessiveGasUsage,
    GasMismatch,
    BloomMismatch,
}
```

## **Advanced Use Cases and Optimizations**

### **Batch Receipt Processing**

```cpp
class BatchReceiptProcessor {
private:
    std::unordered_map<uint256, ReceiptTrie> blockReceipts;
    BloomFilter aggregatedBloom;
    
public:
    // Process multiple blocks efficiently
    void processBatchBlocks(const std::vector<Block>& blocks) {
        #pragma omp parallel for
        for (size_t i = 0; i < blocks.size(); i++) {
            const Block& block = blocks[i];
            
            // Build receipt trie for this block
            ReceiptTrie trie;
            std::vector<TransactionReceipt> receipts;
            
            for (size_t txIdx = 0; txIdx < block.transactions.size(); txIdx++) {
                TransactionReceipt receipt = processTransaction(
                    block.transactions[txIdx], 
                    block.preState
                );
                receipts.push_back(receipt);
            }
            
            trie.buildTrie(receipts);
            
            // Verify trie root matches block header
            if (trie.getRoot() != block.header.receiptsRoot) {
                throw std::runtime_error("Receipt trie root mismatch");
            }
            
            #pragma omp critical
            {
                blockReceipts[block.hash] = std::move(trie);
                
                // Aggregate bloom filters
                for (const auto& receipt : receipts) {
                    aggregateBloom(receipt.logsBloom);
                }
            }
        }
    }
    
    // Optimized multi-block log search
    std::vector<Log> searchLogsAcrossBlocks(
        const std::vector<uint256>& blockHashes,
        const LogFilter& filter
    ) {
        std::vector<Log> results;
        
        for (const auto& blockHash : blockHashes) {
            // Quick bloom filter check first
            if (!aggregatedBloom.mightContain(filter.addresses, filter.topics)) {
                continue;  // Skip this block entirely
            }
            
            const ReceiptTrie& trie = blockReceipts[blockHash];
            auto blockLogs = trie.searchLogs(filter);
            results.insert(results.end(), blockLogs.begin(), blockLogs.end());
        }
        
        return results;
    }
};
```

### **Cross-Chain Receipt Verification**

```go
// Cross-chain bridge receipt verification
type CrossChainReceiptVerifier struct {
    sourceChainHeaders map[common.Hash]*types.Header
    destChainBridge    *BridgeContract
}

func (v *CrossChainReceiptVerifier) VerifyBridgeEvent(
    sourceBlockHash common.Hash,
    txIndex uint64,
    bridgeEventLog *types.Log,
    proof [][]byte,
) error {
    
    // Verify receipt exists in source chain
    sourceHeader := v.sourceChainHeaders[sourceBlockHash]
    if sourceHeader == nil {
        return errors.New("unknown source block")
    }
    
    // Build receipt from bridge event
    receipt := &types.Receipt{
        Status:            types.ReceiptStatusSuccessful,
        CumulativeGasUsed: bridgeEventLog.TxGasUsed, // Simplified
        Logs:              []*types.Log{bridgeEventLog},
        LogsBloom:         types.CreateBloom(types.Receipts{receipt}),
    }
    
    // Verify receipt proof against source chain
    if !v.verifyReceiptProof(receipt, txIndex, proof, sourceHeader.ReceiptHash) {
        return errors.New("invalid receipt proof")
    }
    
    // Verify bridge event parameters
    if err := v.validateBridgeEvent(bridgeEventLog); err != nil {
        return fmt.Errorf("invalid bridge event: %v", err)
    }
    
    // Submit to destination chain bridge
    return v.destChainBridge.ProcessBridgeEvent(
        sourceBlockHash,
        txIndex,
        bridgeEventLog,
        proof,
    )
}

func (v *CrossChainReceiptVerifier) validateBridgeEvent(log *types.Log) error {
    // Verify event signature
    expectedSig := crypto.Keccak256Hash([]byte("BridgeTransfer(address,address,uint256)"))
    if len(log.Topics) == 0 || log.Topics[0] != expectedSig {
        return errors.New("invalid event signature")
    }
    
    // Verify event parameters
    if len(log.Topics) != 3 { // signature + 2 indexed params
        return errors.New("invalid topic count")
    }
    
    // Decode and validate transfer amount
    amount := new(big.Int).SetBytes(log.Data[:32])
    if amount.Cmp(big.NewInt(0)) <= 0 {
        return errors.New("invalid transfer amount")
    }
    
    return nil
}
```

## **Performance Optimization Techniques**

### **Trie Compression and Caching**

```rust
pub struct OptimizedReceiptTrie {
    compressed_nodes: HashMap<H256, CompressedNode>,
    node_cache: LruCache<H256, TrieNode>,
    compression_stats: CompressionStats,
}

#[derive(Clone)]
struct CompressedNode {
    data: Vec<u8>,           // Compressed node data
    original_size: usize,    // Original size before compression
    access_count: u64,       // Access frequency for caching decisions
}

impl OptimizedReceiptTrie {
    // Compress frequently accessed nodes
    pub fn compress_hot_nodes(&mut self, access_threshold: u64) {
        let mut nodes_to_compress = Vec::new();
        
        for (hash, node) in &self.compressed_nodes {
            if node.access_count >= access_threshold {
                nodes_to_compress.push(*hash);
            }
        }
        
        for node_hash in nodes_to_compress {
            self.compress_node(&node_hash);
        }
    }
    
    fn compress_node(&mut self, node_hash: &H256) {
        if let Some(compressed) = self.compressed_nodes.get_mut(node_hash) {
            // Use LZ4 compression for speed
            let original_data = &compressed.data;
            let compressed_data = lz4_flex::compress_prepend_size(original_data);
            
            if compressed_data.len() < original_data.len() {
                let savings = original_data.len() - compressed_data.len();
                
                compressed.data = compressed_data;
                self.compression_stats.total_savings += savings;
                self.compression_stats.compressed_nodes += 1;
            }
        }
    }
    
    // Optimized proof generation with caching
    pub fn generate_cached_proof(&mut self, tx_index: u64) -> Vec<Vec<u8>> {
        let cache_key = H256::from_low_u64_be(tx_index);
        
        // Check proof cache first
        if let Some(cached_proof) = self.proof_cache.get(&cache_key) {
            self.compression_stats.cache_hits += 1;
            return cached_proof.clone();
        }
        
        // Generate proof and cache it
        let proof = self.generate_proof_internal(tx_index);
        self.proof_cache.put(cache_key, proof.clone());
        self.compression_stats.cache_misses += 1;
        
        proof
    }
}

struct CompressionStats {
    total_savings: usize,
    compressed_nodes: usize,
    cache_hits: u64,
    cache_misses: u64,
}
```

## **Comparison with Related Structures**

### **Receipt Trie vs Transaction Trie vs State Trie**

| **Aspect** | **Receipt Trie** | **Transaction Trie** | **State Trie** |
|------------|------------------|---------------------|----------------|
| **Purpose** | Execution results | Raw transaction data | Account states |
| **Mutability** | Read-only after block | Read-only after block | Updated per block |
| **Size Growth** | Linear with transactions | Linear with transactions | Grows with accounts |
| **Verification Use** | Prove execution results | Prove transaction inclusion | Prove account states |
| **Light Client Priority** | Medium | Low | High |
| **Gas Tracking** | ✅ Cumulative usage | ❌ None | ❌ None |
| **Event Logs** | ✅ Full log storage | ❌ None | ❌ None |
| **Bloom Filtering** | ✅ For log searches | ❌ None | ❌ None |

### **Receipt Trie vs Traditional Event Logs**

| **Feature** | **Receipt Trie** | **Traditional DB Logs** |
|-------------|------------------|-------------------------|
| **Cryptographic Proof** | ✅ Merkle proofs | ❌ Trust-based |
| **Immutability** | ✅ Blockchain guaranteed | ❌ Database dependent |
| **Decentralized Verification** | ✅ Any node can verify | ❌ Centralized |
| **Storage Efficiency** | ⚖️ Trie overhead | ✅ Direct storage |
| **Query Performance** | ⚖️ Requires traversal | ✅ Indexed queries |
| **Historical Integrity** | ✅ Cannot be altered | ❌ Can be modified |

## **Edge Cases and Error Handling**

### **Receipt Trie Anomalies**

```cpp
class ReceiptTrieValidator {
public:
    enum ValidationError {
        EMPTY_TRIE_WITH_TRANSACTIONS,
        MISSING_RECEIPT_FOR_TRANSACTION,
        INVALID_CUMULATIVE_GAS,
        BLOOM_FILTER_MISMATCH,
        RECEIPT_COUNT_MISMATCH,
        INVALID_TRANSACTION_STATUS,
        GAS_LIMIT_EXCEEDED
    };
    
    ValidationResult validateReceiptTrie(
        const ReceiptTrie& trie,
        const Block& block
    ) {
        ValidationResult result;
        
        // Edge case: Empty block should have empty receipt trie
        if (block.transactions.empty()) {
            if (!trie.isEmpty()) {
                result.addError(EMPTY_TRIE_WITH_TRANSACTIONS);
            }
            return result;
        }
        
        // Validate receipt count matches transaction count
        if (trie.getReceiptCount() != block.transactions.size()) {
            result.addError(RECEIPT_COUNT_MISMATCH);
        }
        
        uint64_t expectedCumulativeGas = 0;
        BloomFilter aggregatedBloom;
        
        for (size_t i = 0; i < block.transactions.size(); i++) {
            auto receipt = trie.getReceiptByIndex(i);
            
            if (!receipt) {
                result.addError(MISSING_RECEIPT_FOR_TRANSACTION, i);
                continue;
            }
            
            // Validate cumulative gas progression
            uint64_t txGasUsed = (i == 0) ? 
                receipt->cumulativeGasUsed : 
                receipt->cumulativeGasUsed - trie.getReceiptByIndex(i-1)->cumulativeGasUsed;
            
            expectedCumulativeGas += txGasUsed;
            
            if (receipt->cumulativeGasUsed != expectedCumulativeGas) {
                result.addError(INVALID_CUMULATIVE_GAS, i);
            }
            
            // Validate gas doesn't exceed block limit
            if (receipt->cumulativeGasUsed > block.header.gasLimit) {
                result.addError(GAS_LIMIT_EXCEEDED, i);
            }
            
            // Validate transaction status consistency
            if (!isValidTransactionStatus(receipt->status)) {
                result.addError(INVALID_TRANSACTION_STATUS, i);
            }
            
            // Aggregate bloom filters
            aggregatedBloom = BloomFilter::combine(aggregatedBloom, receipt->logsBloom);
        }
        
        // Validate final gas usage matches block header
        if (expectedCumulativeGas != block.header.gasUsed) {
            result.addError(INVALID_CUMULATIVE_GAS, block.transactions.size() - 1);
        }
        
        // Validate aggregated bloom matches block bloom
        if (aggregatedBloom != block.header.logsBloom) {
            result.addError(BLOOM_FILTER_MISMATCH);
        }
        
        return result;
    }
    
private:
    bool isValidTransactionStatus(uint8_t status) {
        return status == 0 || status == 1;  // Only 0 (failure) or 1 (success)
    }
};
```

### **Recovery from Corrupted Receipt Data**

```go
type ReceiptRecoverySystem struct {
    stateDB       *state.StateDB
    txPool        *core.TxPool
    blockchain    *core.BlockChain
    eventRecorder *EventRecorder
}

// Recover receipt from transaction re-execution
func (r *ReceiptRecoverySystem) RecoverReceipt(
    blockHash common.Hash,
    txIndex int,
) (*types.Receipt, error) {
    
    block := r.blockchain.GetBlockByHash(blockHash)
    if block == nil {
        return nil, errors.New("block not found")
    }
    
    if txIndex >= len(block.Transactions()) {
        return nil, errors.New("transaction index out of range")
    }
    
    // Get parent state
    parentBlock := r.blockchain.GetBlockByHash(block.ParentHash())
    if parentBlock == nil {
        return nil, errors.New("parent block not found")
    }
    
    stateDB, err := r.blockchain.StateAt(parentBlock.Root())
    if err != nil {
        return nil, fmt.Errorf("failed to get parent state: %v", err)
    }
    
    // Re-execute transactions up to target transaction
    var cumulativeGasUsed uint64
    var allLogs []*types.Log
    
    for i := 0; i <= txIndex; i++ {
        tx := block.Transactions()[i]
        
        // Execute transaction
        receipt, err := r.executeTransaction(tx, stateDB, cumulativeGasUsed, block.Header())
        if err != nil {
            return nil, fmt.Errorf("failed to re-execute tx %d: %v", i, err)
        }
        
        cumulativeGasUsed = receipt.CumulativeGasUsed
        allLogs = append(allLogs, receipt.Logs...)
        
        // Return receipt if this is target transaction
        if i == txIndex {
            return receipt, nil
        }
    }
    
    return nil, errors.New("unexpected execution path")
}

func (r *ReceiptRecoverySystem) executeTransaction(
    tx *types.Transaction,
    stateDB *state.StateDB,
    prevCumulativeGas uint64,
    header *types.Header,
) (*types.Receipt, error) {
    
    // Create transaction context