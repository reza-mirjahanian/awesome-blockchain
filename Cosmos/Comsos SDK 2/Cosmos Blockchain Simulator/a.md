The Cosmos SDK offers a full fledged simulation framework to fuzz test every message defined by a module.

On the Cosmos SDK, this functionality is provided by [`SimApp`](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/simapp/app_v2.go), which is a `Baseapp` application that is used for running the [`simulation`](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/x/simulation) module. This module defines all the simulation logic as well as the operations for randomized parameters like accounts, balances etc.


Usage[​](https://docs.cosmos.network/v0.50/learn/advanced/simulation#usage "Direct link to Usage")
--------------------------------------------------------------------------------------------------

This is a general example of how simulations are run. For more specific examples check the Cosmos SDK [Makefile](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/Makefile#L282-L318).

```
 $ go test -mod=readonly github.com/cosmos/cosmos-sdk/simapp\
  -run=TestApp<simulation_command>\
...<flags>
  -v -timeout 24h

```