`BaseApp` is a base type that implements the core of a Cosmos SDK application, namely:

-   The [Application Blockchain Interface](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#main-abci-messages), for the state-machine to communicate with the underlying consensus engine (e.g. CometBFT).
-   [Service Routers](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#service-routers), to route messages and queries to the appropriate module.
-   Different [states](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#state-updates), as the state-machine can have different volatile states updated based on the ABCI message received.

-------
The goal of `BaseApp` is to provide the fundamental layer of a Cosmos SDK application that developers can easily extend to build their own custom application. Usually, developers will create a custom type for their application, like so:

```
type App struct{
// reference to a BaseApp
*baseapp.BaseApp
// list of application store keys
// list of application keepers
// module manager
}
```

Extending the application with BaseApp gives the former access to all of BaseApp's methods. This allows developers to compose their custom application with the modules they want, while not having to concern themselves with the hard work of implementing the ABCI, the service routers and state management logic.