`prefix.Store` is a `KVStore` wrapper which provides automatic key-prefixing functionalities over the underlying `KVStore`:

-   When `Store.{Get, Set}()` is called, the store forwards the call to its parent with the key prefixed with the `Store.prefix`.
-   When `Store.Iterator()` is called, it does not simply prefix the `Store.prefix` since it does not work as intended. Some of the elements are traversed even when they are not starting with the prefix in this case.