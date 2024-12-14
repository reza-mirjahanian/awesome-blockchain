# BaseApp in Cosmos SDK

## Core Components

**BaseApp Structure**
- Primary application structure in Cosmos SDK
- Implements basic ABCI interface
- Handles routing of messages and queries
- Manages state transitions
- Controls application lifecycle

## Key Methods

**SetupKeeper**
```go
func (app *BaseApp) SetupKeeper() {
    app.MountStores()
    app.SetInitChainer()
    app.SetBeginBlocker()
    app.SetEndBlocker()
}
```

**Mount Points**
```go
// Store mounting
app.MountStores(
    keys.StoreKey,
    keys.TransientStoreKey,
    keys.MemStoreKey,
)
```

## Middleware Chain

**Order of Operations**
1. Authentication
2. Gas metering
3. Fee collection
4. Signature verification
5. Message routing

## State Management

**Store Types**
- `KVStore`: Persistent key-value storage
- `TransientStore`: Temporary storage per block
- `MemStore`: In-memory storage
- `CommitMultiStore`: Multiple KV stores

**State Access**
```go
ctx.KVStore(storeKey)
ctx.TransientStore(storeKey)
ctx.MemStore(storeKey)
```

## Message Routing

**Router Configuration**
```go
app.Router().
    AddRoute("bank", bank.NewHandler()).
    AddRoute("staking", staking.NewHandler())
```

**Query Routing**
```go
app.QueryRouter().
    AddRoute("custom", CustomQueryHandler)
```

## ABCI Implementation

**Key Interfaces**
- `InitChain`: Chain initialization
- `BeginBlock`: Block start processing
- `DeliverTx`: Transaction processing
- `EndBlock`: Block end processing
- `Commit`: State commitment

## Configuration Options

**SetMinGasPrices**
```go
app.SetMinGasPrices(sdk.NewDecCoins(
    sdk.NewDecCoin("stake", sdk.NewInt(10)),
))
```

**SetInterfaceRegistry**
```go
app.SetInterfaceRegistry(interfaceRegistry)
```

## Error Handling

**Common Patterns**
```go
if err != nil {
    return nil, sdkerrors.Wrap(err, "failed to process tx")
}
```

## Performance Optimization

**Caching**
- Use `CacheContext()` for speculative execution
- Implement store prefixes for better organization
- Use `PrecommitStore()` for temporary state changes

## Security Features

**Panic Recovery**
```go
defer func() {
    if r := recover(); r != nil {
        app.Logger().Error("recovered panic", "error", r)
    }
}()
```

## Event System

**Event Emission**
```go
ctx.EventManager().EmitEvent(
    sdk.NewEvent(
        eventType,
        sdk.NewAttribute(key, value),
    ),
)
```

## Upgrade Handling

**Upgrade Management**
```go
app.SetUpgradeHandler(
    "v2",
    func(ctx sdk.Context, plan upgradetypes.Plan) {
        // upgrade logic
    },
)
```

## Testing Utilities

**Test Helpers**
```go
func SetupTestApp() *BaseApp {
    db := dbm.NewMemDB()
    return NewBaseApp(
        appName,
        logger,
        db,
        nil,
        baseapp.SetPruning(storetypes.NewPruningOptionsFromString(pruningStr)),
    )
}
```

## Best Practices

**Performance**
- Use appropriate store types
- Implement efficient queries
- Optimize gas consumption
- Cache frequently accessed data

**Security**
- Implement proper access controls
- Validate all inputs
- Handle panics gracefully
- Use secure cryptographic primitives

**Maintenance**
- Keep modules independent
- Document state changes
- Implement clear upgrade paths
- Maintain backward compatibility

## Common Pitfalls

**Avoid**
- Storing large objects in state
- Unnecessary state reads/writes
- Unbounded iterations
- Unhandled edge cases
- Missing gas metering