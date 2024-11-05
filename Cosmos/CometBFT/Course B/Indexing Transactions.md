
## Configuration

Operators can configure indexing via the  `[tx_index]`  section. The  `indexer`  field takes a series of supported indexers. If  `null`  is included, indexing will be turned off regardless of other values provided.

```
[tx-index]

# The backend database to back the indexer.
# If indexer is "null", no indexer service will be used.
#
# The application will set which txs to index. In some cases a node operator will be able
# to decide which txs to index based on configuration set in the application.
#
# Options:
#   1) "null"
#   2) "kv" (default) - the simplest possible indexer, backed by key-value storage (defaults to levelDB; see DBBackend).
#     - When "kv" is chosen "tx.height" and "tx.hash" will always be indexed.
#   3) "psql" - the indexer services backed by PostgreSQL.
# indexer = "kv"
```

KV
The kv indexer type is an embedded key-value store supported by the main underlying CometBFT database. Using the kv indexer type allows you to query for block and transaction events directly against CometBFT’s RPC. However, the query syntax is limited and so this indexer type might be deprecated or removed entirely in the future.