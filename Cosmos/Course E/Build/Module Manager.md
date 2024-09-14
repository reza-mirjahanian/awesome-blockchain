Cosmos SDK modules need to implement the [`AppModule` interfaces](https://docs.cosmos.network/v0.50/build/building-modules/module-manager#application-module-interfaces), in order to be managed by the application's [module manager](https://docs.cosmos.network/v0.50/build/building-modules/module-manager#module-manager).


It is **recommended** to implement interfaces from the [Core API](https://docs.cosmos.network/main/architecture/adr-063-core-module-api) `appmodule` package. This makes modules less dependent on the SDK. For legacy reason modules can still implement interfaces from the SDK `module` package.


There are 2 main application module interfaces:

-   [`appmodule.AppModule` / `module.AppModule`](https://docs.cosmos.network/v0.50/build/building-modules/module-manager#appmodule) for inter-dependent module functionalities (except genesis-related functionalities).
-   (legacy) [`module.AppModuleBasic`](https://docs.cosmos.network/v0.50/build/building-modules/module-manager#appmodulebasic) for independent module functionalities. New modules can use `module.CoreAppModuleBasicAdaptor` instead.