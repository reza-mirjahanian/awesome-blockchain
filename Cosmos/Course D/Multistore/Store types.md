The Cosmos SDK offers different store types to work with. It is important to gain a good overview of the different store types available for development.

### [#](https://ida.interchain.io/academy/2-cosmos-concepts/7-multistore-keepers.html#kvstore-and-multistore-in-the-interchain)`KVStore` and `Multistore` in the interchain

Each Cosmos SDK application contains a state at its root, the `Multistore`. It is subdivided into separate compartments managed by each module in the application. The `Multistore` is a store of `KVStore`s that follows the [`Multistore interface` ](https://github.com/cosmos/cosmos-sdk/blob/v0.40.0-rc6/store/types/store.go#L104-L133).

The base `KVStore` and `Multistore` implementations are wrapped in extensions that offer specialized behavior. A [`CommitMultistore` ](https://github.com/cosmos/cosmos-sdk/blob/v0.40.0-rc6/store/types/store.go#L141-L184)is a `Multistore` with a committer. This is the main type of multistore used in the Cosmos SDK. The underlying `KVStore` is used primarily to restrict access to the committer.

The [`rootMulti.Store` ](https://github.com/cosmos/cosmos-sdk/blob/v0.40.0-rc6/store/rootmulti/store.go#L43-L61)is the go-to implementation of the `CommitMultiStore` interface. It is a base-layer multistore built around a database on top of which multiple `KVStore`s can be mounted. It is the default multistore store used in `BaseApp`.