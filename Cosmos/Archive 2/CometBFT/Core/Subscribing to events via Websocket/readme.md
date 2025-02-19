CometBFT emits different events, which you can subscribe to via [Websocket](https://en.wikipedia.org/wiki/WebSocket). This can be useful for third-party applications (for analysis) or for inspecting state.

[List of events](https://godoc.org/github.com/cometbft/cometbft/types#pkg-constants)

To connect to a node via websocket from the CLI, you can use a tool such as [wscat](https://github.com/websockets/wscat) and run:

```
wscat -c ws://127.0.0.1:26657/websocket

```

NOTE: If your node's RPC endpoint is TLS-enabled, utilize the scheme `wss` instead of `ws`.

You can subscribe to any of the events above by calling the `subscribe` RPC method via Websocket along with a valid query.

```
{"jsonrpc":"2.0","method":"subscribe","id":0,"params":{"query":"tm.event='NewBlock'"}}
```

Check out [API docs](https://docs.cometbft.com/v0.38/rpc/) for more information on query syntax and other options.

You can also use tags, given you had included them into FinalizeBlock response, to query transaction results. See [Indexing transactions](https://docs.cometbft.com/v0.38/app-dev/indexing-transactions) for details.