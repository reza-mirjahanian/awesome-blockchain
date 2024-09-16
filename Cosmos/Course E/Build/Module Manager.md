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