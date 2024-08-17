## Node Client

The Daemon, or [Full-Node Client](https://docs.cosmos.network/v0.50/learn/advanced/node), is the core process of a Cosmos SDK-based blockchain. Participants in the network run this process to initialize their state-machine, connect with other full-nodes, and update their state-machine as new blocks come in.

```
                ^  +-------------------------------+  ^
                |  |                               |  |
                |  |  State-machine = Application  |  |
                |  |                               |  |   Built with Cosmos SDK
                |  |            ^      +           |  |
                |  +----------- | ABCI | ----------+  v
                |  |            +      v           |  ^
                |  |                               |  |
Blockchain Node |  |           Consensus           |  |
                |  |                               |  |
                |  +-------------------------------+  |   CometBFT
                |  |                               |  |
                |  |           Networking          |  |
                |  |                               |  |
                v  +-------------------------------+  v
```



The blockchain **full-node** presents itself as a binary, generally suffixed by `-d` for "daemon" (e.g. `appd` for `app` or `gaiad` for `gaia`). This binary is built by running a simple [`main.go`](https://docs.cosmos.network/v0.50/learn/advanced/node#main-function) function placed in `./cmd/appd/`. This operation usually happens through the [Makefile](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#dependencies-and-makefile).

Once the main binary is built, the node can be started by running the [`start` command](https://docs.cosmos.network/v0.50/learn/advanced/node#start-command). This command function primarily does three things:

1.  Create an instance of the state-machine defined in [`app.go`](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#core-application-file).
2.  Initialize the state-machine with the latest known state, extracted from the `db` stored in the `~/.app/data` folder. At this point, the state-machine is at height `appBlockHeight`.
3.  Create and start a new CometBFT instance. Among other things, the node performs a handshake with its peers. It gets the latest `blockHeight` from them and replays blocks to sync to this height if it is greater than the local `appBlockHeight`. The node starts from genesis and CometBFT sends an `InitChain` message via the ABCI to the `app`, which triggers the [`InitChainer`](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#initchainer)
   

   When starting a CometBFT instance, the genesis file is the `0` height and the state within the genesis file is committed at block height `1`. When querying the state of the node, querying block height 0 will return an error.


Core Application File
---------------------------------------------------------------------------------------------------------------------------------------------------

In general, the core of the state-machine is defined in a file called `app.go`. This file mainly contains the **type definition of the application** and functions to **create and initialize it**.

### Type Definition of the Application[​](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#type-definition-of-the-application "Direct link to Type Definition of the Application")

The first thing defined in `app.go` is the `type` of the application. It is generally comprised of the following parts:

-   **A reference to [`baseapp`](https://docs.cosmos.network/v0.50/learn/advanced/baseapp).** The custom application defined in `app.go` is an extension of `baseapp`. When a transaction is relayed by CometBFT to the application, `app` uses `baseapp`'s methods to route them to the appropriate module. `baseapp` implements most of the core logic for the application, including all the [ABCI methods](https://docs.cometbft.com/v0.37/spec/abci/) and the [routing logic](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#routing).
-   **A list of store keys**. The [store](https://docs.cosmos.network/v0.50/learn/advanced/store), which contains the entire state, is implemented as a [`multistore`](https://docs.cosmos.network/v0.50/learn/advanced/store#multistore) (i.e. a store of stores) in the Cosmos SDK. Each module uses one or multiple stores in the multistore to persist their part of the state. These stores can be accessed with specific keys that are declared in the `app` type. These keys, along with the `keepers`, are at the heart of the [object-capabilities model](https://docs.cosmos.network/v0.50/learn/advanced/ocap) of the Cosmos SDK.
-   **A list of module's `keeper`s.** Each module defines an abstraction called [`keeper`](https://docs.cosmos.network/v0.50/build/building-modules/keeper), which handles reads and writes for this module's store(s). The `keeper`'s methods of one module can be called from other modules (if authorized), which is why they are declared in the application's type and exported as interfaces to other modules so that the latter can only access the authorized functions.
-   **A reference to an [`appCodec`](https://docs.cosmos.network/v0.50/learn/advanced/encoding).** The application's `appCodec` is used to serialize and deserialize data structures in order to store them, as stores can only persist `[]bytes`. The default codec is [Protocol Buffers](https://docs.cosmos.network/v0.50/learn/advanced/encoding).
-   **A reference to a [`legacyAmino`](https://docs.cosmos.network/v0.50/learn/advanced/encoding) codec.** Some parts of the Cosmos SDK have not been migrated to use the `appCodec` above, and are still hardcoded to use Amino. Other parts explicitly use Amino for backwards compatibility. For these reasons, the application still holds a reference to the legacy Amino codec. Please note that the Amino codec will be removed from the SDK in the upcoming releases.
-   **A reference to a [module manager](https://docs.cosmos.network/v0.50/build/building-modules/module-manager#manager)** and a [basic module manager](https://docs.cosmos.network/v0.50/build/building-modules/module-manager#basicmanager). The module manager is an object that contains a list of the application's modules. It facilitates operations related to these modules, like registering their [`Msg` service](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#msg-services) and [gRPC `Query` service](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#grpc-query-services), or setting the order of execution between modules for various functions like [`InitChainer`](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#initchainer), [`PreBlocker`](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#preblocker) and [`BeginBlocker` and `EndBlocker`](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#beginblocker-and-endblocker).

See an example of application type definition from `simapp`, the Cosmos SDK's own app used for demo and testing purposes:

https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/simapp/app.go#L173-L212

### Constructor Function

Also defined in `app.go` is the constructor function, which constructs a new application of the type defined in the preceding section. The function must fulfill the `AppCreator` signature in order to be used in the [`start` command](https://docs.cosmos.network/v0.50/learn/advanced/node#start-command) of the application's daemon command.

```
server/types/app.go
// AppCreator is a function that allows us to lazily initialize an
// application using various configurations.
AppCreator func(log.Logger, dbm.DB, io.Writer, AppOptions) Application
```
https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/server/types/app.go#L66-L68


Here are the main actions performed by this function:

-   Instantiate a new [`codec`](https://docs.cosmos.network/v0.50/learn/advanced/encoding) and initialize the `codec` of each of the application's modules using the [basic manager](https://docs.cosmos.network/v0.50/build/building-modules/module-manager#basicmanager).
-   Instantiate a new application with a reference to a `baseapp` instance, a codec, and all the appropriate store keys.
-   Instantiate all the [`keeper`](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#keeper) objects defined in the application's `type` using the `NewKeeper` function of each of the application's modules. Note that keepers must be instantiated in the **correct order**, as the `NewKeeper` of one module might require a reference to another module's `keeper`.
-   Instantiate the application's [module manager](https://docs.cosmos.network/v0.50/build/building-modules/module-manager#manager) with the [`AppModule`](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#application-module-interface) object of each of the application's modules.
-   With the module manager, initialize the application's [`Msg` services](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#msg-services), [gRPC `Query` services](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#grpc-query-services), [legacy `Msg` routes](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#routing), and [legacy query routes](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#query-routing). When a transaction is relayed to the application by CometBFT via the ABCI, it is routed to the appropriate module's [`Msg` service](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#msg-services) using the routes defined here. Likewise, when a gRPC query request is received by the application, it is routed to the appropriate module's [`gRPC query service`](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#grpc-query-services) using the gRPC routes defined here. The Cosmos SDK still supports legacy `Msg`s and legacy CometBFT queries, which are routed using the legacy `Msg` routes and the legacy query routes, respectively.
-   With the module manager, register the [application's modules' invariants](https://docs.cosmos.network/v0.50/build/building-modules/invariants). Invariants are variables (e.g. total supply of a token) that are evaluated at the end of each block. The process of checking invariants is done via a special module called the [`InvariantsRegistry`](https://docs.cosmos.network/v0.50/build/building-modules/invariants#invariant-registry). The value of the invariant should be equal to a predicted value defined in the module. Should the value be different than the predicted one, special logic defined in the invariant registry is triggered (usually the chain is halted). This is useful to make sure that no critical bug goes unnoticed, producing long-lasting effects that are hard to fix.
-   With the module manager, set the order of execution between the `InitGenesis`, `PreBlocker`, `BeginBlocker`, and `EndBlocker` functions of each of the [application's modules](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#application-module-interface). Note that not all modules implement these functions.
-   Set the remaining application parameters:
    -   [`InitChainer`](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#initchainer): used to initialize the application when it is first started.
    -   [`PreBlocker`](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#preblocker): called before BeginBlock.
    -   [`BeginBlocker`, `EndBlocker`](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#beginblocker-and-endblocker): called at the beginning and at the end of every block.
    -   [`anteHandler`](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#antehandler): used to handle fees and signature verification.
-   Mount the stores.
-   Return the application.
-   

---------------------
------------
Note that the constructor function only creates an instance of the app, while the actual state is either carried over from the `~/.app/data` folder if the node is restarted, or generated from the genesis file if the node is started for the first time.

See an example of application constructor from `simapp`



https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/simapp/app.go#L223-L575


simapp/app.go
```
// NewSimApp returns a reference to an initialized SimApp.
funcNewSimApp(
	logger log.Logger,
	db dbm.DB,
	traceStore io.Writer,
	loadLatest bool,
	appOpts servertypes.AppOptions,
	baseAppOptions ...func(*baseapp.BaseApp),
)*SimApp {
```