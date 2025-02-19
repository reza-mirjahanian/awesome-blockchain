You can see the help menu with `cometbft --help`, and the version number with `cometbft version`.


Directory Root
--------------

The default directory for blockchain data is `~/.cometbft`



Initialize
----------

Initialize the root directory by running:

```
cometbft init
```
-------
### Genesis

The `genesis.json` file in `$CMTHOME/config/` defines the initial CometBFT

---------------

### Genesis

The `genesis.json` file in `$CMTHOME/config/` defines the initial CometBFT state upon genesis of the blockchain ([see definition](https://github.com/cometbft/cometbft/blob/v0.38.x/types/genesis.go)).

#### Fields

-   `genesis_time`: Official time of blockchain start.
-   `chain_id`: ID of the blockchain. **This must be unique for every blockchain.** If your testnet blockchains do not have unique chain IDs, you will have a bad time. The ChainID must be less than 50 symbols.
-   `initial_height`: Height at which CometBFT should begin at. If a blockchain is conducting a network upgrade, starting from the stopped height brings uniqueness to previous heights.
-   `consensus_params` ([see spec](https://github.com/cometbft/cometbft/blob/v0.38.x/spec/core/data_structures.md#consensusparams))
    -   `block`
        -   `max_bytes`: Max block size, in bytes.
        -   `max_gas`: Max gas per block.
    -   `evidence`
        -   `max_age_num_blocks`: Max age of evidence, in blocks. The basic formula for calculating this is: MaxAgeDuration / {average block time}.
        -   `max_age_duration`: Max age of evidence, in time. It should correspond with an app's "unbonding period" or other similar mechanism for handling [Nothing-At-Stake attacks](https://vitalik.ca/general/2017/12/31/pos_faq.html#what-is-the-nothing-at-stake-problem-and-how-can-it-be-fixed).
        -   `max_bytes`: This sets the maximum size in bytes of evidence that can be committed in a single block and should fall comfortably under the max block bytes.
    -   `validator`
        -   `pub_key_types`: Public key types validators can use.
    -   `version`
        -   `app_version`: ABCI application version.
-   `validators`: List of initial validators. Note this may be overridden entirely by the application, and may be left empty to make explicit that the application will initialize the validator set upon `InitChain`.
    -   `pub_key`: The first element specifies the key type, using the declared `PubKeyName` for the adopted [key type](https://github.com/cometbft/cometbft/blob/v0.38.x/crypto/ed25519/ed25519.go#L36). The second element are the pubkey bytes.
    -   `power`: The validator's voting power.
    -   `name`: Name of the validator (optional).
-   `app_hash`: The expected application hash (as returned by the `ResponseInfo` ABCI message) upon genesis. If the app's hash does not match, CometBFT will panic.
-   `app_state`: The application state (e.g. initial distribution of tokens)
-   


Run
---

To run a CometBFT node, use:

```
cometbft node

```

By default, CometBFT will try to connect to an ABCI application on `tcp://127.0.0.1:26658`. If you have the `kvstore` ABCI app installed, run it in another window. If you don't, kill CometBFT and run an in-process version of the `kvstore` app:

```
cometbft node --proxy_app=kvstore

```

After a few seconds, you should see blocks start streaming in. Note that blocks are produced regularly, even if there are no transactions. See [No Empty Blocks](https://docs.cometbft.com/v0.38/core/using-cometbft#no-empty-blocks), below, to modify this setting.


---
Transactions
------------

To send a transaction, use `curl` to make requests to the CometBFT RPC server, for example:

```
curl http://localhost:26657/broadcast_tx_commit?tx=\"abcd\"

```

We can see the chain's status at the `/status` end-point:

```
curl http://localhost:26657/status | json_pp

```

and the `latest_app_hash` in particular:

```
curl http://localhost:26657/status | json_pp | grep latest_app_hash

```

Visit `http://localhost:26657` in your browser to see the list of other endpoints. Some take no arguments (like `/status`), while others specify the argument name and use `_` as a placeholder.

> TIP: Find the RPC Documentation [here](https://docs.cometbft.com/v0.38/rpc/)

---

Reset
-----

>  **UNSAFE** Only do this in development and only if you can afford to lose all blockchain data!

To reset a blockchain, stop the node and run:

```
cometbft unsafe_reset_all

```

This command will remove the data directory and reset private validator and address book files.

-------

Configuration
-------------

CometBFT uses a `config.toml` for configuration. For details, see [the config specification](https://docs.cometbft.com/v0.38/core/configuration).

No Empty Blocks
---------------

While the default behavior of `cometbft` is still to create blocks approximately once per second, it is possible to disable empty blocks or set a block creation interval. In the former case, blocks will be created when there are new transactions or when the AppHash changes.

To configure CometBFT to not produce empty blocks unless there are transactions or the app hash changes, run CometBFT with this additional flag:

```
cometbft node --consensus.create_empty_blocks=false

```

or set the configuration via the `config.toml` file:

```
[consensus]
create_empty_blocks = false

```

Remember: because the default is to *create empty blocks*, avoiding empty blocks requires the config option to be set to `false`.

The block interval setting allows for a delay (in time.Duration format [ParseDuration](https://golang.org/pkg/time/#ParseDuration)) between the creation of each new empty block. It can be set with this additional flag:

```
--consensus.create_empty_blocks_interval="5s"

```

or set the configuration via the `config.toml` file:

```
[consensus]
create_empty_blocks_interval = "5s"

```

With this setting, empty blocks will be produced every 5s if no block has been produced otherwise, regardless of the value of `create_empty_blocks`.