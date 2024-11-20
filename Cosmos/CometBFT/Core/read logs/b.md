List of modules
---------------

Here is the list of modules you may encounter in CometBFT's log and a little overview what they do.

-   `abci-client` As mentioned in [Application Development Guide](https://docs.cometbft.com/v0.38/app-dev/abci-cli), CometBFT acts as an ABCI client with respect to the application and maintains 3 connections: mempool, consensus and query. The code used by CometBFT can be found [here](https://github.com/cometbft/cometbft/blob/v0.38.x/abci/client).
-   `blockchain` Provides storage, pool (a group of peers), and reactor for both storing and exchanging blocks between peers.
-   `consensus` The heart of CometBFT, which is the implementation of the consensus algorithm. Includes two "submodules": `wal` (write-ahead logging) for ensuring data integrity and `replay` to replay blocks and messages on recovery from a crash.
-   `events` Simple event notification system. The list of events can be found [here](https://github.com/cometbft/cometbft/blob/v0.38.x/types/events.go). You can subscribe to them by calling `subscribe` RPC method. Refer to [RPC docs](https://docs.cometbft.com/v0.38/core/rpc) for additional information.
-   `mempool` Mempool module handles all incoming transactions, whenever they are coming from peers or the application.
-   `p2p` Provides an abstraction around peer-to-peer communication. For more details, please check out the [README](https://github.com/cometbft/cometbft/blob/v0.38.x/p2p/README.md).
-   `rpc` [CometBFT's RPC](https://docs.cometbft.com/v0.38/core/rpc).
-   `rpc-server` RPC server. For implementation details, please read the [doc.go](https://github.com/cometbft/cometbft/blob/v0.38.x/rpc/jsonrpc/doc.go).
-   `state` Represents the latest state and execution submodule, which executes blocks against the application.
-   `types` A collection of the publicly exposed types and methods to work with them.