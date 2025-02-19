### Configuration file

The information and parameters about the chains, paths, and the name of the relayer private key to sign the messages can generally be found in a configuration file, the *config*. You will look at template configs for the Hermes and Go relayer later, but generally, config files are the place to initialize, add, or edit information required for relaying.

### [#]Chain registry

When you have the template for the config file of the relayer software you are using, where can you find the information that it needs? The [chain-registry Github repository ](https://github.com/cosmos/chain-registry)provides detailed parameters about chains and their assets, and recently a schema was added to submit IBC data. This new addition saves you from having to look up path information or canonical channels on [Mintscan ](https://www.mintscan.io/cosmos/relayers)or [Map of Zones ](https://mapofzones.com/?testnet=false&period=24&tableOrderBy=ibcVolume&tableOrderSort=desc).

The following is an example of the IBC data between Juno and Osmosis:

```
{
  "$schema": "../ibc_data.schema.json",
  "chain-1": {
    "chain-name": "juno",
    "client-id": "07-tendermint-0",
    "connection-id": "connection-0"
  },
  "chain-2": {
    "chain-name": "osmosis",
    "client-id": "07-tendermint-1457",
    "connection-id": "connection-1142"
  },
  "channels": [
    {
      "chain-1": {
        "channel-id": "channel-0",
        "port-id": "transfer"
      },
      "chain-2": {
        "channel-id": "channel-42",
        "port-id": "transfer"
      },
      "ordering": "unordered",
      "version": "ics20-1",
      "tags": {
        "status": "live",
        "preferred": true,
        "dex": "osmosis"
      }
    }
  ]
}

```