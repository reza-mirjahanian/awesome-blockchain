Core
====

Core (`cosmossdk.io/core`) is package which specifies the interfaces for core components of the Cosmos SDK. Other packages in the SDK implement these interfaces to provide the core functionality. This design provides modularity and flexibility to the SDK, allowing developers to swap out implementations of core components as needed. As such it is often referred to as the Core API.

Environment[​](https://docs.cosmos.network/v0.52/learn/advanced/core#environment "Direct link to Environment")
--------------------------------------------------------------------------------------------------------------

The `Environment` struct is a core component of the Cosmos SDK. It provides access to the core services of the SDK, such as the KVStore, EventManager, and Logger. The `Environment` struct is passed to modules and other components of the SDK to provide access to these services.

v1.0.0-alpha.6/core/appmodule/v2/environment.go
```
type Environment struct{
	Logger log.Logger
	BranchService      branch.Service
	EventService       event.Service
	GasService         gas.Service
	HeaderService      header.Service
	QueryRouterService router.Service
	MsgRouterService   router.Service
	TransactionService transaction.Service
	KVStoreService  store.KVStoreService
	MemStoreService store.MemoryStoreService
}

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/core/v1.0.0-alpha.6/core/appmodule/v2/environment.go#L16-L29)

Historically the SDK has used an [sdk.Context](https://docs.cosmos.network/v0.50/learn/advanced/context) to pass around services and data. `Environment` is a newer construct that is intended to replace an `sdk.Context` in many cases. `sdk.Context` will be deprecated in the future on the same timeline as [Baseapp](https://docs.cosmos.network/v0.52/learn/advanced/baseapp).

Logger[​](https://docs.cosmos.network/v0.52/learn/advanced/core#logger "Direct link to Logger")
-----------------------------------------------------------------------------------------------

The [Logger](https://pkg.go.dev/cosmossdk.io/log) provides a structured logging interface to the SDK. It is used throughout the SDK to log messages at various levels of severity. The Logger service is a thin wrapper around the [zerolog](https://github.com/rs/zerolog) logging library. When used via environment, the logger is scoped to the module that is using it.

runtime/module.go
```
logger.With(log.ModuleKey, fmt.Sprintf("x/%s", key.Name())),

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.52.0-beta.2/runtime/module.go#L274)


Branch Service[​](https://docs.cosmos.network/v0.52/learn/advanced/core#branch-service "Direct link to Branch Service")
-----------------------------------------------------------------------------------------------------------------------

The [BranchService](https://pkg.go.dev/cosmossdk.io/core/branch#Service.Execute) provides an interface to execute arbitrary code in a branched store. This is useful for executing code that needs to make changes to the store, but may need to be rolled back if an error occurs. Below is a contrived example based on the `x/epochs` module's BeginBlocker logic.

```
func(k Keeper)BeginBlocker(ctx context.Context)error{
    err := k.EpochInfo.Walk(
// ...
        ctx,
nil,
func(key string, epochInfo types.EpochInfo)(stop bool, err error){
// ...
if err := k.BranchService.Execute(ctx,func(ctx context.Context)error{
return k.AfterEpochEnd(ctx, epochInfo.Identifier, epochInfo.CurrentEpoch)
}); err !=nil{
returntrue, err
}
})
}

```

Note that calls to `BranchService.Execute` are atomic and cannot share state with each other except when the transaction is successful. If successful, the changes made to the store will be committed. If an error occurs, the changes will be rolled back.


Event Service[​](https://docs.cosmos.network/v0.52/learn/advanced/core#event-service "Direct link to Event Service")
--------------------------------------------------------------------------------------------------------------------

The Event Service returns a handle to an [EventManager](https://pkg.go.dev/cosmossdk.io/core@v1.0.0-alpha.4/event#Manager) which can be used to emit events. For information on how to emit events and their meaning in the SDK see the [Events](https://docs.cosmos.network/v0.52/learn/advanced/events) document.

Note that core's `EventManager` API is a subset of the EventManager API described above; the latter will be deprecated and removed in the future. Roughly speaking legacy `EmitTypeEvent` maps to `Emit` and legacy `EmitEvent` maps to `EmitKV`.

v1.0.0-alpha.4/core/event/service.go
```
type Manager interface{
// Emit emits events represented as a protobuf message (as described in ADR 032).
//
// Callers SHOULD assume that these events will not be included in consensus.
Emit(event transaction.Msg)error
// EmitKV emits an event based on an event and kv-pair attributes.
//
// These events will not be part of consensus and adding, removing or changing these events is
// not a state-machine breaking change.
EmitKV(eventType string, attrs ...Attribute)error
}

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/core/v1.0.0-alpha.4/core/event/service.go#L18-L29)


Gas Service[​](https://docs.cosmos.network/v0.52/learn/advanced/core#gas-service "Direct link to Gas Service")
--------------------------------------------------------------------------------------------------------------

The gas service encapsulates both gas configuration and a gas meter. Gas consumption is largely handled at the framework level for transaction processing and state access but modules can choose to use the gas service directly if needed.

v1.0.0-alpha.4/core/gas/service.go
```
type Service interface{
// GasMeter returns the current transaction-level gas meter. A non-nil meter
// is always returned. When one is unavailable in the context an infinite gas meter
// will be returned.
GasMeter(context.Context) Meter
// GasConfig returns the gas costs.
GasConfig(ctx context.Context) GasConfig
}
// Meter represents a gas meter for modules consumption
type Meter interface{
Consume(amount Gas, descriptor string)error
Refund(amount Gas, descriptor string)error
Remaining() Gas
Consumed() Gas
Limit() Gas
}
// GasConfig defines the gas costs for the application.
type GasConfig struct{
	HasCost          Gas
	DeleteCost       Gas
	ReadCostFlat     Gas
	ReadCostPerByte  Gas
	WriteCostFlat    Gas
	WriteCostPerByte Gas
	IterNextCostFlat Gas
}

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/core/v1.0.0-alpha.4/core/gas/service.go#L26-L54)



Header Service[​](https://docs.cosmos.network/v0.52/learn/advanced/core#header-service "Direct link to Header Service")
-----------------------------------------------------------------------------------------------------------------------

The header service provides access to the current block header. This is useful for modules that need to access the block header fields like `Time` and `Height` during transaction processing.

core/header/service.go
```
// Service defines the interface in which you can get header information
type Service interface{
HeaderInfo(context.Context) Info
}
// Info defines a struct that contains information about the header
type Info struct{
	Height  int64// Height returns the height of the block
	Hash    []byte// Hash returns the hash of the block header
	Time    time.Time // Time returns the time of the block
	AppHash []byte// AppHash used in the current block header
	ChainID string// ChainId returns the chain ID of the block
}

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/a3729c1ad6ba2fb46f879ec7ea67c3afc02e9859/core/header/service.go#L11-L23)

### Custom Header Service[​](https://docs.cosmos.network/v0.52/learn/advanced/core#custom-header-service "Direct link to Custom Header Service")

Core's service oriented architecture (SOA) allows for chain developers to define a custom implementation of the `HeaderService` interface. This would involve creating a new struct that satisfies `HeaderService` but composes additional logic on top. An example of where this would happen (when using depinject is shown below). Note this example is taken from `runtime/v2` but could easily be adapted to `runtime/v1` (the default runtime 0.52). This same pattern can be replicated for any core service.

runtime/v2/module.go
```
// DefaultServiceBindings provides default services for the following service interfaces:
// - store.KVStoreServiceFactory
// - header.Service
// - comet.Service
//
// They are all required.  For most use cases these default services bindings should be sufficient.
// Power users (or tests) may wish to provide their own services bindings, in which case they must
// supply implementations for each of the above interfaces.
funcDefaultServiceBindings() depinject.Config {
var(
		kvServiceFactory store.KVStoreServiceFactory =func(actor []byte) store.KVStoreService {
return services.NewGenesisKVService(
				actor,
				stf.NewKVStoreService(actor),
)
}
		headerService header.Service = services.NewGenesisHeaderService(stf.HeaderService{})
		cometService  comet.Service  =&services.ContextAwareCometInfoService{}
		eventService                 = stf.NewEventService()
)
return depinject.Supply(
		kvServiceFactory,
		headerService,
		cometService,
		eventService,
)
}

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/489aaae40234f1015a7bbcfa9384a89dc8de8153/runtime/v2/module.go#L262-L288)

These bindings are applied to the `depinject` container in simapp/v2 as shown below.

simapp/v2/app\_di.go
```
appConfig = depinject.Configs(
AppConfig(),
	runtime.DefaultServiceBindings(),

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/489aaae40234f1015a7bbcfa9384a89dc8de8153/simapp/v2/app_di.go#L72-L74)


Query and Message Router Service[​](https://docs.cosmos.network/v0.52/learn/advanced/core#query-and-message-router-service "Direct link to Query and Message Router Service")
-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------

Both the query and message router services are implementation of the same interface, `router.Service`.

v1.0.0-alpha.4/core/router/service.go
```
type Service interface{
// CanInvoke returns an error if the given request cannot be invoked.
CanInvoke(ctx context.Context, typeURL string)error
// Invoke execute a message or query. The response should be type casted by the caller to the expected response.
Invoke(ctx context.Context, req transaction.Msg)(res transaction.Msg, err error)
}

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/core/v1.0.0-alpha.4/core/router/service.go#L11-L16)

Both are exposed to modules so that arbitrary messages and queries can be routed to the appropriate handler. This powerful abstraction allows module developers to fully decouple modules from each other by using only the proto message for dispatching. This is particularly useful for modules like `x/accounts` which require a dynamic dispatch mechanism in order to function.

TransactionService[​](https://docs.cosmos.network/v0.52/learn/advanced/core#transactionservice "Direct link to TransactionService")
-----------------------------------------------------------------------------------------------------------------------------------

v1.0.0-alpha.4/core/transaction/service.go
```
// Service creates a transaction service.
type Service interface{
// ExecMode returns the current execution mode.
ExecMode(ctx context.Context) ExecMode
}

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/core/v1.0.0-alpha.4/core/transaction/service.go#L21-L25)

The transaction service provides access to the execution mode a state machine transaction is running in, which may be one of `Check`, `Recheck`, `Simulate` or `Finalize`. The SDK primarily uses these flags in ante handlers to skip certain checks while in `Check` or `Simulate` modes, but module developers may find uses for them as well.


KVStore Service[​](https://docs.cosmos.network/v0.52/learn/advanced/core#kvstore-service "Direct link to KVStore Service")
--------------------------------------------------------------------------------------------------------------------------

v1.0.0-alpha.4/core/store/service.go
```go
// KVStoreService represents a unique, non-forgeable handle to a regular merkle-tree
// backed KVStore. It should be provided as a module-scoped dependency by the runtime
// module being used to build the app.
type KVStoreService interface{
// OpenKVStore retrieves the KVStore from the context.
OpenKVStore(context.Context) KVStore
}

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/core/v1.0.0-alpha.4/core/store/service.go#L5-L11)

The KVStore service abstracts access to, and creation of, key-value stores. Most use cases will be backed by a merkle-tree store, but developers can provide their own implementations if needed. In the case of the `KVStoreService` implementation provided in `Environment`, module developers should understand that calling `OpenKVStore` will return a store already scoped to the module's prefix. The wiring for this scoping is specified in `runtime`.