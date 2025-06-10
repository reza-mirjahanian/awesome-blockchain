# Full Sync and Snap Sync in Ethereum

## Core Synchronization Concepts

**Synchronization** is the process by which Ethereum nodes download and verify blockchain data to maintain consensus with the network. The sync method determines how much historical data is processed and stored locally.

**State** represents the current account balances, contract storage, and nonces at a specific block height. **Chain data** includes all blocks, transactions, and receipts from genesis.

## Full Sync (Archive Sync)

### Mechanism
Full sync downloads and **executes every transaction** from the genesis block to the current head. This recreates the entire state history by processing each state transition.

### Process Flow
1. Download blocks sequentially from genesis
2. Execute every transaction in each block
3. Update state after each transaction
4. Verify state roots match block headers
5. Store all intermediate states

### Storage Requirements
- **Chain data**: ~800GB (mainnet)
- **State data**: ~1.2TB+ (all historical states)
- **Total**: 2TB+ and growing

### Code Example - Full Sync State Processing

```cpp
class FullSyncProcessor {
private:
    StateDB state_db;
    BlockChain blockchain;
    
public:
    bool processBlock(const Block& block) {
        // Verify parent block exists
        if (!blockchain.hasBlock(block.parent_hash)) {
            return false;
        }
        
        // Execute all transactions in block
        StateDB temp_state = state_db;
        for (const auto& tx : block.transactions) {
            if (!executeTransaction(tx, temp_state)) {
                return false;
            }
        }
        
        // Verify state root matches
        if (temp_state.getRoot() != block.state_root) {
            return false;
        }
        
        // Commit state changes
        state_db = temp_state;
        blockchain.addBlock(block);
        return true;
    }
    
private:
    bool executeTransaction(const Transaction& tx, StateDB& state) {
        // Load sender account
        Account sender = state.getAccount(tx.from);
        
        // Verify nonce and balance
        if (sender.nonce != tx.nonce || sender.balance < tx.value + tx.gas_limit * tx.gas_price) {
            return false;
        }
        
        // Execute transaction logic
        if (tx.to.empty()) {
            // Contract creation
            return createContract(tx, state);
        } else {
            // Regular transfer or contract call
            return executeCall(tx, state);
        }
    }
};
```

### Advantages
- **Complete verification**: Every state transition is validated
- **Full historical access**: Can query any historical state
- **Maximum security**: Cryptographically verifies entire chain
- **Archive capabilities**: Supports historical state queries

### Disadvantages
- **Extremely slow**: Takes weeks to months
- **Massive storage**: Requires 2TB+ disk space
- **High computational cost**: Processes millions of transactions
- **Network intensive**: Downloads entire blockchain history

## Snap Sync (Fast Sync)

### Mechanism
Snap sync downloads a **recent state snapshot** and only the most recent blocks. It trusts the network majority for historical data while verifying recent activity.

### Process Flow
1. Identify trusted recent block (pivot point)
2. Download state snapshot at pivot
3. Verify snapshot integrity via state root
4. Download and verify recent blocks from pivot to head
5. Continue normal sync from current head

### Storage Requirements
- **Recent chain data**: ~50GB (last few months)
- **Current state**: ~40GB (single state snapshot)
- **Total**: ~90GB

### Code Example - Snap Sync Implementation

```go
package snapsync

import (
    "crypto/sha256"
    "fmt"
)

type SnapSyncer struct {
    stateDB     *StateDB
    blockchain  *BlockChain
    pivotBlock  uint64
    snapshots   map[common.Hash]*StateSnapshot
}

type StateSnapshot struct {
    Root     common.Hash
    Accounts map[common.Address]*Account
    Storage  map[common.Hash]common.Hash
    Code     map[common.Hash][]byte
}

func (s *SnapSyncer) SyncFromSnapshot(pivotHash common.Hash) error {
    // Step 1: Download and verify snapshot
    snapshot, err := s.downloadSnapshot(pivotHash)
    if err != nil {
        return fmt.Errorf("failed to download snapshot: %v", err)
    }
    
    // Step 2: Verify snapshot integrity
    if !s.verifySnapshot(snapshot, pivotHash) {
        return fmt.Errorf("snapshot verification failed")
    }
    
    // Step 3: Import snapshot into state DB
    if err := s.importSnapshot(snapshot); err != nil {
        return fmt.Errorf("failed to import snapshot: %v", err)
    }
    
    // Step 4: Sync recent blocks from pivot to head
    return s.syncRecentBlocks()
}

func (s *SnapSyncer) verifySnapshot(snap *StateSnapshot, expectedRoot common.Hash) bool {
    // Reconstruct state trie from snapshot data
    trie := NewStateTrie()
    
    for addr, account := range snap.Accounts {
        trie.Update(addr.Bytes(), account.Encode())
    }
    
    // Verify root matches expected
    return trie.Hash() == expectedRoot
}

func (s *SnapSyncer) downloadSnapshot(pivotHash common.Hash) (*StateSnapshot, error) {
    snapshot := &StateSnapshot{
        Root:     pivotHash,
        Accounts: make(map[common.Address]*Account),
        Storage:  make(map[common.Hash]common.Hash),
        Code:     make(map[common.Hash][]byte),
    }
    
    // Download account data in chunks
    for {
        chunk, hasMore, err := s.requestAccountRange(pivotHash, lastKey)
        if err != nil {
            return nil, err
        }
        
        for addr, account := range chunk.Accounts {
            snapshot.Accounts[addr] = account
        }
        
        if !hasMore {
            break
        }
    }
    
    // Download storage data for contracts
    for addr, account := range snapshot.Accounts {
        if account.IsContract() {
            storage, err := s.downloadStorageRange(addr, pivotHash)
            if err != nil {
                return nil, err
            }
            // Merge storage data
            for key, value := range storage {
                snapshot.Storage[key] = value
            }
        }
    }
    
    return snapshot, nil
}
```

### Snap Sync Phases

**Phase 1: Pivot Selection**
```rust
use std::collections::HashMap;

struct PivotSelector {
    peer_responses: HashMap<PeerId, BlockHash>,
    min_confirmations: usize,
}

impl PivotSelector {
    fn select_pivot(&self) -> Option<BlockHash> {
        let mut vote_count: HashMap<BlockHash, usize> = HashMap::new();
        
        // Count votes from peers
        for (_, block_hash) in &self.peer_responses {
            *vote_count.entry(*block_hash).or_insert(0) += 1;
        }
        
        // Find hash with most votes
        vote_count.into_iter()
            .filter(|(_, count)| *count >= self.min_confirmations)
            .max_by_key(|(_, count)| *count)
            .map(|(hash, _)| hash)
    }
}
```

**Phase 2: Account Range Download**
```cpp
class AccountRangeDownloader {
    struct AccountRange {
        Hash start_hash;
        Hash end_hash;
        std::vector<AccountData> accounts;
        std::vector<Hash> proofs;
    };
    
public:
    bool downloadAccountRange(const Hash& state_root, const Hash& start_key) {
        // Request account range from multiple peers
        std::vector<AccountRange> responses;
        
        for (auto& peer : active_peers) {
            auto response = peer.requestAccountRange(state_root, start_key, RANGE_SIZE);
            if (verifyAccountRange(response, state_root)) {
                responses.push_back(response);
            }
        }
        
        // Use majority response
        auto consensus_range = findConsensusRange(responses);
        if (consensus_range) {
            storeAccountRange(*consensus_range);
            return true;
        }
        
        return false;
    }
    
private:
    bool verifyAccountRange(const AccountRange& range, const Hash& root) {
        // Verify merkle proofs for range boundaries
        return verifyRangeProof(range.accounts, range.proofs, root, 
                               range.start_hash, range.end_hash);
    }
};
```

### Trust Model Comparison

| Aspect | Full Sync | Snap Sync |
|--------|-----------|-----------|
| **Historical Data** | Fully verified | Trusted from network |
| **Current State** | Derived from execution | Downloaded and verified |
| **Security Level** | Maximum | High (assumes honest majority) |
| **Attack Vectors** | Minimal | State snapshot poisoning |

## Advanced Sync Optimizations

### Parallel Processing in Snap Sync

```go
func (s *SnapSyncer) parallelAccountDownload(stateRoot common.Hash) error {
    const numWorkers = 8
    const chunkSize = 1000
    
    // Create work channels
    ranges := make(chan AccountRange, numWorkers*2)
    results := make(chan *StateChunk, numWorkers*2)
    
    // Start worker goroutines
    for i := 0; i < numWorkers; i++ {
        go s.accountWorker(ranges, results)
    }
    
    // Generate account ranges
    go func() {
        defer close(ranges)
        startKey := common.Hash{}
        
        for {
            endKey := s.calculateRangeEnd(startKey, chunkSize)
            ranges <- AccountRange{
                Start: startKey,
                End:   endKey,
                Root:  stateRoot,
            }
            
            if endKey == (common.Hash{}) {
                break
            }
            startKey = endKey
        }
    }()
    
    // Collect results
    var chunks []*StateChunk
    for i := 0; i < s.calculateTotalChunks(); i++ {
        chunk := <-results
        if chunk == nil {
            return fmt.Errorf("worker failed to download range")
        }
        chunks = append(chunks, chunk)
    }
    
    return s.assembleState(chunks)
}
```

### Healing Process for Snap Sync

```rust
use std::collections::BTreeSet;

struct StateHealer {
    missing_accounts: BTreeSet<Address>,
    missing_storage: BTreeMap<Address, BTreeSet<StorageKey>>,
    missing_code: BTreeSet<CodeHash>,
}

impl StateHealer {
    fn heal_state(&mut self) -> Result<(), SyncError> {
        // Heal missing accounts
        while !self.missing_accounts.is_empty() {
            let batch: Vec<Address> = self.missing_accounts
                .iter()
                .take(MAX_BATCH_SIZE)
                .cloned()
                .collect();
            
            let accounts = self.request_accounts(&batch)?;
            self.verify_and_store_accounts(accounts)?;
            
            for addr in batch {
                self.missing_accounts.remove(&addr);
            }
        }
        
        // Heal missing storage
        for (address, storage_keys) in &mut self.missing_storage {
            while !storage_keys.is_empty() {
                let keys: Vec<StorageKey> = storage_keys
                    .iter()
                    .take(MAX_BATCH_SIZE)
                    .cloned()
                    .collect();
                
                let storage_data = self.request_storage(address, &keys)?;
                self.verify_and_store_storage(address, storage_data)?;
                
                for key in keys {
                    storage_keys.remove(&key);
                }
            }
        }
        
        // Heal missing code
        self.heal_missing_code()?;
        
        Ok(())
    }
    
    fn verify_and_store_accounts(&self, accounts: Vec<AccountProof>) -> Result<(), SyncError> {
        for proof in accounts {
            if !self.verify_account_proof(&proof) {
                return Err(SyncError::InvalidProof);
            }
            self.state_db.insert_account(proof.address, proof.account)?;
        }
        Ok(())
    }
}
```

## Sync Performance Comparison

### Time Complexity Analysis

| Sync Type | Time Complexity | Dominant Factor |
|-----------|----------------|-----------------|
| **Full Sync** | O(n × t) | n=blocks, t=tx per block |
| **Snap Sync** | O(s + b) | s=state size, b=recent blocks |

### Real-world Performance Metrics

```cpp
struct SyncMetrics {
    std::chrono::milliseconds total_time;
    uint64_t blocks_processed;
    uint64_t transactions_executed;
    uint64_t state_entries_downloaded;
    size_t storage_used_gb;
    
    double blocks_per_second() const {
        return blocks_processed / (total_time.count() / 1000.0);
    }
    
    double storage_efficiency() const {
        return static_cast<double>(state_entries_downloaded) / storage_used_gb;
    }
};

// Typical benchmarks
SyncMetrics full_sync_metrics = {
    .total_time = std::chrono::hours(24 * 30), // 30 days
    .blocks_processed = 18'000'000,
    .transactions_executed = 2'000'000'000,
    .state_entries_downloaded = 0,
    .storage_used_gb = 2048
};

SyncMetrics snap_sync_metrics = {
    .total_time = std::chrono::hours(4), // 4 hours
    .blocks_processed = 1000, // recent blocks only
    .transactions_executed = 100'000,
    .state_entries_downloaded = 150'000'000,
    .storage_used_gb = 90
};
```

## Edge Cases and Error Handling

### Pivot Block Reorganization

```go
func (s *SnapSyncer) handlePivotReorg(newHead common.Hash) error {
    currentPivot := s.getCurrentPivot()
    
    // Check if pivot is still valid
    if !s.blockchain.IsAncestor(currentPivot, newHead) {
        log.Warn("Pivot block not in canonical chain, restarting sync")
        
        // Clear downloaded state
        s.stateDB.Reset()
        
        // Select new pivot
        newPivot, err := s.selectNewPivot(newHead)
        if err != nil {
            return fmt.Errorf("failed to select new pivot: %v", err)
        }
        
        // Restart snap sync from new pivot
        return s.SyncFromSnapshot(newPivot)
    }
    
    return nil
}
```

### State Snapshot Corruption Detection

```rust
impl StateValidator {
    fn validate_snapshot_integrity(&self, snapshot: &StateSnapshot) -> Result<(), ValidationError> {
        // Check for duplicate accounts
        let mut seen_accounts = HashSet::new();
        for addr in snapshot.accounts.keys() {
            if !seen_accounts.insert(addr) {
                return Err(ValidationError::DuplicateAccount(*addr));
            }
        }
        
        // Validate account encoding
        for (addr, account) in &snapshot.accounts {
            if !self.is_valid_account_encoding(account) {
                return Err(ValidationError::InvalidAccountEncoding(*addr));
            }
            
            // Check storage consistency
            if account.is_contract() {
                self.validate_storage_consistency(addr, &snapshot.storage)?;
            }
        }
        
        // Verify state root reconstruction
        let reconstructed_root = self.calculate_state_root(snapshot)?;
        if reconstructed_root != snapshot.root {
            return Err(ValidationError::StateMismatch {
                expected: snapshot.root,
                calculated: reconstructed_root,
            });
        }
        
        Ok(())
    }
}
```

## Security Considerations

### Attack Vectors and Mitigations

**State Poisoning Attack**
- **Attack**: Malicious peers provide invalid state data
- **Mitigation**: Cryptographic verification using merkle proofs

```cpp
class SnapSecurityManager {
public:
    bool verifyStateMerkleProof(const StateData& data, 
                               const std::vector<Hash>& proof,
                               const Hash& root) {
        Hash current = hashStateData(data);
        
        for (size_t i = 0; i < proof.size(); ++i) {
            if (isLeftNode(data.path, i)) {
                current = hash(current, proof[i]);
            } else {
                current = hash(proof[i], current);
            }
        }
        
        return current == root;
    }
    
    bool validatePeerConsensus(const std::vector<StateResponse>& responses) {
        std::map<Hash, int> vote_count;
        
        for (const auto& response : responses) {
            vote_count[response.state_root]++;
        }
        
        // Require 2/3 majority for acceptance
        const int required_votes = responses.size() * 2 / 3;
        
        for (const auto& [root, votes] : vote_count) {
            if (votes >= required_votes) {
                return true;
            }
        }
        
        return false;
    }
};
```

## Modern Sync Strategy Selection

### Decision Matrix

```go
type SyncDecision struct {
    RecommendedSync SyncType
    Reasons        []string
    Requirements   SyncRequirements
}

type SyncRequirements struct {
    DiskSpace     uint64 // in GB
    SyncTime      time.Duration
    BandwidthNeed uint64 // in GB
}

func (s *SyncSelector) RecommendSync(criteria SyncCriteria) SyncDecision {
    switch {
    case criteria.ArchiveAccess && criteria.HistoricalQueries:
        return SyncDecision{
            RecommendedSync: FullSync,
            Reasons: []string{
                "Archive access required",
                "Historical state queries needed",
                "Maximum security desired",
            },
            Requirements: SyncRequirements{
                DiskSpace:     2048,
                SyncTime:      time.Hour * 24 * 30,
                BandwidthNeed: 800,
            },
        }
    
    case criteria.FastStart && !criteria.HistoricalQueries:
        return SyncDecision{
            RecommendedSync: SnapSync,
            Reasons: []string{
                "Fast synchronization needed",
                "Limited storage available",
                "Current state access sufficient",
            },
            Requirements: SyncRequirements{
                DiskSpace:     90,
                SyncTime:      time.Hour * 4,
                BandwidthNeed: 50,
            },
        }
    
    default:
        return s.hybridSyncRecommendation(criteria)
    }
}
```

Both sync methods serve different use cases in the Ethereum ecosystem. **Full sync provides maximum security and complete historical access** but requires significant resources. **Snap sync offers rapid synchronization with current state access** while trusting network consensus for historical data. The choice depends on specific requirements for security, speed, storage, and historical data access.