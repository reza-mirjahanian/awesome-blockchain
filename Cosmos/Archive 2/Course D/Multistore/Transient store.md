### Transient store

As the name suggests, `Transient.Store` is a `KVStore` that is discarded automatically at the end of each block. `Transient.Store` is a `dbadapter.Store` with a `dbm.NewMemDB()`. All `KVStore` methods are reused. A new `dbadapter.Store` is assigned when `Store.Commit()` is called, discarding the previous reference. Garbage collection is attended to automatically.


Take a closer look at the [IAVL spec ](https://github.com/cosmos/iavl/blob/v0.15.0-rc5/docs/overview.md)for when working with the IAVL store.



The default implementation of `KVStore` and `CommitKVStore` is the `IAVL.Store`. The `IAVL.Store` is a self-balancing binary search tree that ensures get and set operations are `O(log n)`, where `n` is the number of elements in the tree.