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

