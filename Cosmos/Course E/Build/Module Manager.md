Cosmos SDK modules need to implement the [`AppModule` interfaces](https://docs.cosmos.network/v0.50/build/building-modules/module-manager#application-module-interfaces), in order to be managed by the application's [module manager](https://docs.cosmos.network/v0.50/build/building-modules/module-manager#module-manager).


It is **recommended** to implement interfaces from the [Core API](https://docs.cosmos.network/main/architecture/adr-063-core-module-api) `appmodule` package. This makes modules less dependent on the SDK. For legacy reason modules can still implement interfaces from the SDK `module` package.


There are 2 main application module interfaces:

-   [`appmodule.AppModule` / `module.AppModule`](https://docs.cosmos.network/v0.50/build/building-modules/module-manager#appmodule) for inter-dependent module functionalities (except genesis-related functionalities).
-   (legacy) [`module.AppModuleBasic`](https://docs.cosmos.network/v0.50/build/building-modules/module-manager#appmodulebasic) for independent module functionalities. New modules can use `module.CoreAppModuleBasicAdaptor` instead.
-   

The above interfaces are mostly embedding smaller interfaces (extension interfaces), that defines specific functionalities:

-   (legacy) `module.HasName`: Allows the module to provide its own name for legacy purposes.
-   (legacy) [`module.HasGenesisBasics`](https://docs.cosmos.network/v0.50/build/building-modules/module-manager#modulehasgenesisbasics): The legacy interface for stateless genesis methods.
-   [`module.HasGenesis`](https://docs.cosmos.network/v0.50/build/building-modules/module-manager#modulehasgenesis) for inter-dependent genesis-related module functionalities.
-   [`module.HasABCIGenesis`](https://docs.cosmos.network/v0.50/build/building-modules/module-manager#modulehasabcigenesis) for inter-dependent genesis-related module functionalities.
-   [`appmodule.HasGenesis` / `module.HasGenesis`](https://docs.cosmos.network/v0.50/build/building-modules/module-manager#appmodulehasgenesis): The extension interface for stateful genesis methods.
-   [`appmodule.HasPreBlocker`](https://docs.cosmos.network/v0.50/build/building-modules/module-manager#haspreblocker): The extension interface that contains information about the `AppModule` and `PreBlock`.
-   [`appmodule.HasBeginBlocker`](https://docs.cosmos.network/v0.50/build/building-modules/module-manager#hasbeginblocker): The extension interface that contains information about the `AppModule` and `BeginBlock`.
-   [`appmodule.HasEndBlocker`](https://docs.cosmos.network/v0.50/build/building-modules/module-manager#hasendblocker): The extension interface that contains information about the `AppModule` and `EndBlock`.
-   [`appmodule.HasPrecommit`](https://docs.cosmos.network/v0.50/build/building-modules/module-manager#hasprecommit): The extension interface that contains information about the `AppModule` and `Precommit`.
-   [`appmodule.HasPrepareCheckState`](https://docs.cosmos.network/v0.50/build/building-modules/module-manager#haspreparecheckstate): The extension interface that contains information about the `AppModule` and `PrepareCheckState`.
-   [`appmodule.HasService` / `module.HasServices`](https://docs.cosmos.network/v0.50/build/building-modules/module-manager#hasservices): The extension interface for modules to register services.
-   [`module.HasABCIEndBlock`](https://docs.cosmos.network/v0.50/build/building-modules/module-manager#hasabciendblock): The extension interface that contains information about the `AppModule`, `EndBlock` and returns an updated validator set.
-   (legacy) [`module.HasInvariants`](https://docs.cosmos.network/v0.50/build/building-modules/module-manager#hasinvariants): The extension interface for registering invariants.
-   (legacy) [`module.HasConsensusVersion`](https://docs.cosmos.network/v0.50/build/building-modules/module-manager#hasconsensusversion): The extension interface for declaring a module consensus version.
  
  
  ------

The `AppModuleBasic` interface exists to define independent methods of the module, i.e. those that do not depend on other modules in the application. This allows for the construction of the basic application structure early in the application definition, generally in the `init()` function of the [main application file](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#core-application-file). 


The `AppModule` interface exists to define inter-dependent module methods. Many modules need to interact with other modules, typically through [`keeper`s](https://docs.cosmos.network/v0.50/build/building-modules/keeper), which means there is a need for an interface where modules list their `keeper`s and other methods that require a reference to another module's object. `AppModule` interface extension, such as `HasBeginBlocker` and `HasEndBlocker`, also enables the module manager to set the order of execution between module's methods like `BeginBlock` and `EndBlock`, which is important in cases where the order of execution between modules matters in the context of the application.


Use `module.CoreAppModuleBasicAdaptor` instead for creating an `AppModuleBasic` from an `appmodule.AppModule`

--------
All the AppModuleBasic of an application are managed by the BasicManager


```
types/module/module.go
// AppModule is the form for an application module. Most of
// its functionality has been moved to extension interfaces.
type AppModule interface {
	AppModuleBasic
}
```


### Implementing the Application Module Interfaces

Typically, the various application module interfaces are implemented in a file called `module.go`, located in the module's folder (e.g. `./x/module/module.go`).

Almost every module needs to implement the `AppModuleBasic` and `AppModule` interfaces. If the module is only used for genesis, it will implement `AppModuleGenesis` instead of `AppModule`. The concrete type that implements the interface can add parameters that are required for the implementation of the various methods of the interface. For example, the `Route()` function often calls a `NewMsgServerImpl(k keeper)` function defined in `keeper/msg_server.go` and therefore needs to pass the module's [`keeper`](https://docs.cosmos.network/v0.50/build/building-modules/keeper) as a parameter.

```
// example
type AppModule struct{
    AppModuleBasic
    keeper       Keeper
}

```

In the example above, you can see that the `AppModule` concrete type references an `AppModuleBasic`, and not an `AppModuleGenesis`. That is because `AppModuleGenesis` only needs to be implemented in modules that focus on genesis-related functionalities. In most modules, the concrete `AppModule` type will have a reference to an `AppModuleBasic` and implement the two added methods of `AppModuleGenesis` directly in the `AppModule` type.

If no parameter is required (which is often the case for `AppModuleBasic`), just declare an empty concrete type like so:

```
type AppModuleBasic struct{}
```