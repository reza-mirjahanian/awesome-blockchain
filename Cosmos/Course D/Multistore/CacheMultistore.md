A [`cachemulti.Store` ](https://github.com/cosmos/cosmos-sdk/blob/v0.40.0-rc6/store/cachemulti/store.go#L17-L28)is used whenever the `rootMulti.Store` needs to be branched. `cachemulti.Store` branches all substores, creates a virtual store for each substore in its constructor, and holds them in `Store.stores`. This is used primarily to create an isolated store, typically when it is necessary to mutate the state but it might be reverted later.

`CacheMultistore` caches all read queries. `Store.GetKVStore()` returns the store from `Store.stores`, and `Store.Write()` recursively calls `CacheWrap.Write()` on all substores.