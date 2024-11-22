Multistore[​](https://docs.cosmos.network/v0.47/learn/intro/sdk-design#multistore "Direct link to Multistore")
--------------------------------------------------------------------------------------------------------------

The Cosmos SDK provides a [`multistore`](https://docs.cosmos.network/v0.47/learn/advanced/store#multistore) for persisting state. The multistore allows developers to declare any number of [`KVStores`](https://docs.cosmos.network/v0.47/learn/advanced/store#base-layer-kvstores). These `KVStores` only accept the `[]byte` type as value and therefore any custom structure needs to be marshalled using [a codec](https://docs.cosmos.network/v0.47/learn/advanced/encoding) before being stored.

The multistore abstraction is used to divide the state in distinct compartments, each managed by its own module

------------------

I'll explain the Multistore concept in Cosmos SDK, breaking it down into manageable parts.

1. Basic Concept of Multistore
```go
// Conceptual Structure
type Multistore interface {
    // Store management
    GetStoreType() StoreType
    GetKVStore(StoreKey) KVStore
    GetMemStore(StoreKey) KVStore
    // ... other methods
}
```

2. Key Components:

a) Store Types:
- KVStore (Key-Value Store)
- MemStore (In-memory Store)
- TransientStore (Temporary Storage)
- IAVL Store (Merkle Tree-based)

b) Store Keys:
```go
// Store keys are used to access different stores
type StoreKey interface {
    Name() string
}

// Example of creating store keys
var (
    KeyMain     = sdk.NewKVStoreKey("main")
    KeyAccount  = sdk.NewKVStoreKey("acc")
    KeyStaking  = sdk.NewKVStoreKey("staking")
)
```

3. Practical Implementation:

```go
// In your module
type Keeper struct {
    storeKey sdk.StoreKey
    cdc      codec.BinaryCodec
}

// Using the store in a keeper
func (k Keeper) Set(ctx sdk.Context, key string, value MyType) {
    store := ctx.KVStore(k.storeKey)
    store.Set([]byte(key), k.cdc.MustMarshal(&value))
}

func (k Keeper) Get(ctx sdk.Context, key string) (MyType, bool) {
    store := ctx.KVStore(k.storeKey)
    value := store.Get([]byte(key))
    if value == nil {
        return MyType{}, false
    }
    
    var item MyType
    k.cdc.MustUnmarshal(value, &item)
    return item, true
}
```

4. Setting up Multistore in App:

```go
// In app.go
func NewApp(
    logger log.Logger,
    db dbm.DB,
    // ... other parameters
) *App {
    
    // Create the app
    app := &App{}
    
    // Create the store keys
    keys := sdk.NewKVStoreKeys(
        authtypes.StoreKey,
        banktypes.StoreKey,
        stakingtypes.StoreKey,
        // ... other module store keys
    )
    
    // Create the transaction store keys
    tkeys := sdk.NewTransientStoreKeys(paramstypes.TStoreKey)
    
    // Mount stores to multistore
    app.MountKVStores(keys)
    app.MountTransientStores(tkeys)
    
    // ... rest of app initialization
}
```


6. Best Practices:

```go
// 1. Use meaningful store keys
var (
    StoreKeyMyModule = "my_module"
)

// 2. Proper error handling
func (k Keeper) SafeGet(ctx sdk.Context, key string) (MyType, error) {
    store := ctx.KVStore(k.storeKey)
    value := store.Get([]byte(key))
    if value == nil {
        return MyType{}, errors.New("value not found")
    }
    
    var item MyType
    if err := k.cdc.Unmarshal(value, &item); err != nil {
        return MyType{}, fmt.Errorf("failed to unmarshal: %w", err)
    }
    return item, nil
}

// 3. Efficient key management
func (k Keeper) GetPrefix(prefix string) []byte {
    return []byte(fmt.Sprintf("%s/%s", k.storeKey.Name(), prefix))
}
```

7. Common Operations:

```go
// Iteration
func (k Keeper) IterateAll(ctx sdk.Context, fn func(key []byte, value MyType) bool) {
    store := ctx.KVStore(k.storeKey)
    iterator := store.Iterator(nil, nil)
    defer iterator.Close()
    
    for ; iterator.Valid(); iterator.Next() {
        var value MyType
        k.cdc.MustUnmarshal(iterator.Value(), &value)
        if stop := fn(iterator.Key(), value); stop {
            break
        }
    }
}

// Prefix scanning
func (k Keeper) GetAllWithPrefix(ctx sdk.Context, prefix []byte) []MyType {
    store := ctx.KVStore(k.storeKey)
    iterator := sdk.KVStorePrefixIterator(store, prefix)
    defer iterator.Close()
    
    var items []MyType
    for ; iterator.Valid(); iterator.Next() {
        var item MyType
        k.cdc.MustUnmarshal(iterator.Value(), &item)
        items = append(items, item)
    }
    return items
}
```

