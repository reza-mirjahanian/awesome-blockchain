if you'd like to read about the config.toml please visit [CometBFT docs](https://docs.cometbft.com/v0.37/).


`tools/confix/data/v0.47-app.toml`

https://docs.cosmos.network/v0.50/learn/advanced/config


### minimum-gas-prices = "0stake"
 The minimum gas prices a validator is willing to accept for processing a
 transaction. A transaction's fees must meet the minimum of any denomination
 specified in this config (e.g. 0.25token1,0.0001token2).



### pruning = "default"
 * default: the last 362880 states are kept, pruning at 10 block intervals
 * nothing: all historic states will be saved, nothing will be deleted (i.e. **archiving node**)
 * everything: 2 latest states will be kept; pruning at 10 block intervals.
 * custom: allow pruning options to be manually specified through 'pruning-keep-recent', and 'pruning-interval'




###  halt-height = 0
Note: Commitment of state will be attempted on the corresponding block.
##### **What Problem Does HaltHeight Solve?**

`HaltHeight` is a parameter in the Cosmos SDK that is used to intentionally stop a blockchain at a specific block height. This is particularly useful in scenarios where a network upgrade, maintenance, or a critical bug fix requires all nodes in the network to stop processing blocks at the same point. By setting a `HaltHeight`, developers can ensure that the blockchain will not progress beyond a certain block, allowing for a coordinated update or intervention.



### app-db-backend = ""
AppDBBackend defines the database backend type to use for the application and snapshots DBs.
An empty string indicates that a fallback will be used.
The fallback is the db_backend value set in Tendermint's config.toml.

### state-sync
State sync snapshots allow other nodes to rapidly join the network without replaying historical
blocks, instead downloading and applying a snapshot of the application state at a given height.
