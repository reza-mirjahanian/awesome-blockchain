Database
--------

By default, CometBFT uses the `syndtr/goleveldb` package for its in-process key-value database. If you want maximal performance, it may be best to install the real C-implementation of LevelDB and compile CometBFT to use that using `make build COMETBFT_BUILD_OPTIONS=cleveldb`. See the [install instructions](https://docs.cometbft.com/v0.38/guides/install) for details.



Debugging CometBFT
------------------
`/dump_consensus_state` will give you a detailed overview of the consensus state (proposer, latest validators, peers states). From it, you should be able to figure out why, for example, the network had halted.

```
curl http(s)://{ip}:{rpcPort}/dump_consensus_state

```

There is a reduced version of this endpoint - `/consensus_state`, which returns just the votes seen at the current height.

If, after consulting with the logs and above endpoints, you still have no idea what's happening, consider using `cometbft debug kill` sub-command. This command will scrap all the available info and kill the process. See [Debugging](https://docs.cometbft.com/v0.38/tools/debugging) for the exact format.


Monitoring CometBFT
-------------------

Each CometBFT instance has a standard `/health` RPC endpoint, which responds with 200 (OK) if everything is fine and 500 (or no response) - if something is wrong.

Other useful endpoints include mentioned earlier `/status`, `/net_info` and `/validators`.

CometBFT also can report and serve Prometheus metrics. See [Metrics](https://docs.cometbft.com/v0.38/core/metrics).

`cometbft debug dump` sub-command can be used to periodically dump useful information into an archive. See [Debugging](https://docs.cometbft.com/v0.38/tools/debugging) for more information.