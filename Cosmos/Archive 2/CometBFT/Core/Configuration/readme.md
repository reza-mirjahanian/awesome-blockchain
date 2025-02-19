Configuration
=============

CometBFT can be configured via a TOML file in `$CMTHOME/config/config.toml`. Some of these parameters can be overridden by command-line flags. For most users, the options in the `##### main base configuration options #####` are intended to be modified while config options further below are intended for advance power users.



Consensus timeouts explained
----------------------------

There's a variety of information about timeouts in [Running in production](https://docs.cometbft.com/v0.38/core/running-in-production#configuration-parameters). You can also find more detailed explanation in the paper describing the Tendermint consensus algorithm, adopted by CometBFT:





