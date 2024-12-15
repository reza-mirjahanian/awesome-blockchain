**Store in Cosmos SDK**
![
  
](image.png)
### **Core Concepts**

- **Store Definition:**  
  - The **store** in Cosmos SDK is the abstraction for state management.  
  - It represents a persistent data layer where modules read and write state.  
  - Store operations must be deterministic for blockchain consensus.

- **Types of Stores:**  
  1. **KVStore (Key-Value Store):**  
     - The primary store used for most state data.  
     - Allows for efficient key-value lookups.  
  2. **TransientStore:**  
     - Temporary storage used within a block's lifecycle.  
     - Does not persist across blocks.  
  3. **MemoryStore:**  
     - An in-memory store for volatile data, primarily for testing or performance optimizations.  
  4. **GasMeteredStore:**  
     - Wraps stores to measure gas costs for read and write operations.

- **Commit Multi-Store (CMS):**  
  - The **Commit Multi-Store** is the top-level abstraction that manages multiple sub-stores (KV stores, transient stores, etc.).  
  - It ensures atomic commits and consistent state across modules.

---

### **Hierarchy of Stores**

| **Level**        | **Description**                                                   |
|------------------|-------------------------------------------------------------------|
| **CommitMultiStore** | Manages all sub-stores and handles persistence and versioning. |
| **KVStore**         | Stores key-value pairs persistently.                           |
| **TransientStore**  | Stores temporary data (e.g., for intra-block operations).      |
| **GasMeteredStore** | Wraps stores to track gas usage for resource accounting.       |

---

### **Store Keys**

- **Purpose:**  
  Store keys uniquely identify stores in the `CommitMultiStore`.

- **Types of Store Keys:**
  1. **KVStoreKey:** For persistent KV stores.
  2. **TransientStoreKey:** For transient stores.
  3. **MemoryStoreKey:** For in-memory stores.

- **Code Example: Declaring Store Keys**  
  ```go
  var (
      KeyAccount   = sdk.NewKVStoreKey("account")
      KeyStaking   = sdk.NewKVStoreKey("staking")
      TKeyParams   = sdk.NewTransientStoreKey("params")
  )
  ```

---

### **Mounting Stores**

- **Mounting in `BaseApp`:**  
  Before the application starts, stores must be mounted to the `CommitMultiStore`.

- **Code Example: Mounting Stores**  
  ```go
  app.MountStores(
      KeyAccount,   // Persistent KV store for accounts
      KeyStaking,   // Persistent KV store for staking
      TKeyParams,   // Transient store for temporary parameters
  )
  ```

---

### **Accessing Stores in Modules**

- **Store Context:**  
  Modules access stores using `sdk.Context` during transaction execution.

- **Getting a Store from Context:**  
  ```go
  store := ctx.KVStore(KeyAccount)
  ```

- **Reading and Writing Data:**  
  ```go
  // Writing data
  store.Set([]byte("key"), []byte("value"))

  // Reading data
  value := store.Get([]byte("key"))

  // Deleting data
  store.Delete([]byte("key"))
  ```

---

### **Prefixes in KVStore**

- **Purpose:**  
  Prefixes help organize data within a single KV store, enabling efficient queries.

- **Using a Prefix Store:**  
  ```go
  prefixStore := prefix.NewStore(store, []byte("modulePrefix/"))
  prefixStore.Set([]byte("key"), []byte("value"))
  ```

- **Benefits of Prefixes:**
  - Avoids key collisions between modules.
  - Simplifies queries by grouping related data.

---

### **Gas Metering**

- **GasMeteredStore:**  
  - Wraps a KV store to measure gas costs for reads and writes.  
  - Ensures transactions pay for their resource usage.

- **Code Example: Using GasMeteredStore**  
  ```go
  gasMeter := sdk.NewGasMeter(1000000) // 1,000,000 gas limit
  ctx = ctx.WithGasMeter(gasMeter)

  store := ctx.KVStore(KeyAccount)
  store.Set([]byte("key"), []byte("value")) // Gas is consumed here
  ```

---

### **Iterating Over Keys**

- **Iterator API:**  
  - Provides efficient traversal of key-value pairs.  
  - Commonly used for exporting data or running batch operations.

- **Code Example: Iterating Over a Range of Keys**  
  ```go
  iterator := store.Iterator([]byte("startKey"), []byte("endKey"))
  defer iterator.Close()

  for ; iterator.Valid(); iterator.Next() {
      key := iterator.Key()
      value := iterator.Value()
      // Process key-value pairs
  }
  ```

- **Reverse Iteration:**  
  ```go
  iterator := store.ReverseIterator([]byte("startKey"), []byte("endKey"))
  defer iterator.Close()

  for ; iterator.Valid(); iterator.Next() {
      key := iterator.Key()
      value := iterator.Value()
      // Process key-value pairs
  }
  ```

---

### **Best Practices for Store Usage**

1. **Use Prefix Stores:**  
   Avoid key collisions and improve modularity.
   
2. **Efficient Iterations:**  
   Limit iteration ranges to avoid performance bottlenecks.

3. **Avoid Frequent Writes:**  
   Write operations are costly in terms of gas. Use batching where possible.

4. **Leverage Transient Stores:**  
   Use transient stores for temporary data that doesn't need to persist across blocks.

5. **Test for Determinism:**  
   Ensure all store operations are deterministic across nodes to prevent consensus failures.

6. **Optimize Key Design:**  
   Use short, compact keys for better storage efficiency and faster queries.

---

### **Store Lifecycle**

| **Phase**      | **Description**                                                        |
|----------------|------------------------------------------------------------------------|
| **Initialization** | Stores are mounted to the `CommitMultiStore`.                       |
| **BeginBlock**  | Transient stores are reset.                                           |
| **DeliverTx**   | Transactions read/write data to stores.                               |
| **EndBlock**    | State is finalized for the block.                                     |
| **Commit**      | `CommitMultiStore` writes the updated state to disk.                  |

---

### **Store Pruning**

- **Purpose:**  
  Pruning reduces storage by deleting old state data.  

- **Pruning Strategies:**
  1. **PruneNothing:** Retains all historical state.
  2. **PruneEverything:** Retains only the most recent state.
  3. **PruneDefault:** Keeps a configurable number of recent states.

- **Configuring Pruning:**
  ```toml
  [app]
  pruning = "default"
  ```

---

### **Testing Store Operations**

- **Using In-Memory Stores for Testing:**  
  ```go
  memStore := store.NewMemoryStoreKey("testStore")
  ctx := sdk.NewContext(memStore, header, false, logger)
  ```

- **Mock Store for Unit Tests:**  
  Replace real stores with mock implementations to test module logic.

---

### **Example: Store Usage in a Module**

```go
// Keeper definition
type Keeper struct {
    storeKey sdk.StoreKey
}

// Setting a value in the store
func (k Keeper) SetValue(ctx sdk.Context, key, value string) {
    store := ctx.KVStore(k.storeKey)
    store.Set([]byte(key), []byte(value))
}

// Getting a value from the store
func (k Keeper) GetValue(ctx sdk.Context, key string) string {
    store := ctx.KVStore(k.storeKey)
    value := store.Get([]byte(key))
    if value == nil {
        return ""
    }
    return string(value)
}
```

---

### **Store Debugging Tips**

1. **Enable State Export:**  
   Use `app export` to examine the full state of the blockchain.

2. **Logging Store Operations:**  
   Temporarily log all store reads and writes during debugging.

3. **Use Iterators for Inspection:**  
   Write custom CLI commands to iterate and print store contents.

---

### **Summary Table**

| **Feature**            | **Description**                                             |
|------------------------|-------------------------------------------------------------|
| **KVStore**            | Persistent key-value store for blockchain state.            |
| **TransientStore**     | Temporary store for intra-block operations.                 |
| **GasMeteredStore**    | Measures gas usage for store operations.                    |
| **Prefix Store**       | Scoped access to specific subsets of the KVStore.           |
| **CommitMultiStore**   | Aggregates all sub-stores and handles persistence.           |
| **Pruning**            | Configurable strategy for managing historical state data.   |
| **Iteration**          | Provides efficient traversal of key-value pairs.            |

By following these tips and patterns, you can effectively use the Cosmos SDK store system to manage blockchain state efficiently and deterministically.