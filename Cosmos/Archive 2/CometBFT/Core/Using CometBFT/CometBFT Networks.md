When `cometbft init` is run, both a `genesis.json` and `priv_validator_key.json` are created in `~/.cometbft/config`. The `genesis.json` might look like:

```
{"validators":[{"pub_key":{"value":"h3hk+QE8c6QLTySp8TcfzclJw/BG79ziGB/pIA+DfPE=","type":"tendermint/PubKeyEd25519"},"power":10,"name":""}],"app_hash":"","chain_id":"test-chain-rDlYSN","genesis_time":"0001-01-01T00:00:00Z"}
```

And the `priv_validator_key.json`:

```
{"last_step":0,"last_round":"0","address":"B788DEDE4F50AD8BC9462DE76741CCAFF87D51E2","pub_key":{"value":"h3hk+QE8c6QLTySp8TcfzclJw/BG79ziGB/pIA+DfPE=","type":"tendermint/PubKeyEd25519"},"last_height":"0","priv_key":{"value":"JPivl82x+LfVkp8i3ztoTjY6c6GJ4pBxQexErOCyhwqHeGT5ATxzpAtPJKnxNx/NyUnD8Ebv3OIYH+kgD4N88Q==","type":"tendermint/PrivKeyEd25519"}}
```

The `priv_validator_key.json` actually contains a private key, and should thus be kept absolutely secret; for now we work with the plain text. Note the `last_` fields, which are used to prevent us from signing conflicting messages.

Note also that the `pub_key` (the public key) in the `priv_validator_key.json` is also present in the `genesis.json`.