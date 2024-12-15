The main endpoint of a Cosmos SDK application is the daemon client, otherwise known as the full-node client. The full-node runs the state-machine, starting from a genesis file. It connects to peers running the same client in order to receive and relay transactions, block proposals and signatures. The full-node is constituted of the application, defined with the Cosmos SDK, and of a consensus engine connected to the application via the ABCI.


`main` function[​](https://docs.cosmos.network/v0.52/learn/advanced/node#main-function "Direct link to main-function")
----------------------------------------------------------------------------------------------------------------------

The full-node client of any Cosmos SDK application is built by running a `main` function. The client is generally named by appending the `-d` suffix to the application name (e.g. `appd` for an application named `app`), and the `main` function is defined in a `./appd/cmd/main.go` file. Running this function creates an executable `appd` that comes with a set of commands. For an app named `app`, the main command is [`appd start`](https://docs.cosmos.network/v0.52/learn/advanced/node#start-command), which starts the full-node.

In general, developers will implement the `main.go` function with the following structure:

-   First, an [`encodingCodec`](https://docs.cosmos.network/main/learn/advanced/encoding) is instantiated for the application.
-   Then, the `config` is retrieved and config parameters are set. This mainly involves setting the Bech32 prefixes for [addresses](https://docs.cosmos.network/v0.52/learn/beginner/accounts#addresses).

types/config.go
```
// Config is the structure that holds the SDK configuration parameters.
// Deprecated: The global SDK config is deprecated and users should prefer using an address codec.
// Users must still set the global config until the Stringer interface on `AccAddress`, `ValAddress`, and `ConsAddress` is removed.
type Config struct{
	bech32AddressPrefix map[string]string
	mtx                 sync.RWMutex
	sealed   bool
	sealedch chanstruct{}
}

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.52.0-beta.2/types/config.go#L20-L29)