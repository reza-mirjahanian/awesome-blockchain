The Cosmos SDK is a framework that facilitates the development of secure state-machines on top of CometBFT. At its core, the Cosmos SDK is a **boilerplate implementation** of the [ABCI](https://docs.cosmos.network/v0.50/learn/intro/sdk-app-architecture#abci) in Golang. It comes with a [`multistore`](https://docs.cosmos.network/v0.50/learn/advanced/store#multistore) to persist data and a [`router`](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#routing) to handle transactions.

Here is a simplified view of how transactions are handled by an application built on top of the Cosmos SDK when transferred from CometBFT via `DeliverTx`:

1.  Decode `transactions` received from the CometBFT consensus engine (remember that CometBFT only deals with `[]bytes`).
2.  Extract `messages` from `transactions` and do basic sanity checks.
3.  Route each message to the appropriate module so that it can be processed.
4.  Commit state changes.


## baseapp
`aseapp` is the boilerplate implementation of a Cosmos SDK application. It comes with an implementation of the ABCI to handle the connection with the underlying consensus engine. Typically, a Cosmos SDK application extends `baseapp` by embedding it in [`app.go`](https://docs.cosmos.network/v0.50/learn/beginner/app-anatomy#core-application-file).

Here is an example of this from `simapp`, the Cosmos SDK demonstration app:

The goal of `baseapp` is to provide a secure interface between the store and the extensible state machine while defining as little about the state machine as possible (staying true to the ABCI).




For more on `baseapp`, please click [here](https://docs.cosmos.network/v0.50/learn/advanced/baseapp).



Multistore[​](https://docs.cosmos.network/v0.50/learn/intro/sdk-design#multistore "Direct link to Multistore")
--------------------------------------------------------------------------------------------------------------

The Cosmos SDK provides a [`multistore`](https://docs.cosmos.network/v0.50/learn/advanced/store#multistore) for persisting state. The multistore allows developers to declare any number of [`KVStores`](https://docs.cosmos.network/v0.50/learn/advanced/store#base-layer-kvstores). These `KVStores` only accept the `[]byte` type as value and therefore any custom structure needs to be marshalled using [a codec](https://docs.cosmos.network/v0.50/learn/advanced/encoding) before being stored.

The multistore abstraction is used to divide the state in distinct compartments, each managed by its own module. For more on the multistore, click [here](https://docs.cosmos.network/v0.50/learn/advanced/store#multistore)


Modules[​](https://docs.cosmos.network/v0.50/learn/intro/sdk-design#modules "Direct link to Modules")
-----------------------------------------------------------------------------------------------------

The power of the Cosmos SDK lies in its modularity. Cosmos SDK applications are built by aggregating a collection of interoperable modules. Each module defines a subset of the state and contains its own message/transaction processor, while the Cosmos SDK is responsible for routing each message to its respective module.

Here is a simplified view of how a transaction is processed by the application of each full-node when it is received in a valid block:

```
                                      +
                                      |
                                      |  Transaction relayed from the full-node's
                                      |  CometBFT engine to the node's application
                                      |  via DeliverTx
                                      |
                                      |
                +---------------------v--------------------------+
                |                 APPLICATION                    |
                |                                                |
                |     Using baseapp's methods: Decode the Tx,    |
                |     extract and route the message(s)           |
                |                                                |
                +---------------------+--------------------------+
                                      |
                                      |
                                      |
                                      +---------------------------+
                                                                  |
                                                                  |
                                                                  |  Message routed to
                                                                  |  the correct module
                                                                  |  to be processed
                                                                  |
                                                                  |
+----------------+  +---------------+  +----------------+  +------v----------+
|                |  |               |  |                |  |                 |
|  AUTH MODULE   |  |  BANK MODULE  |  | STAKING MODULE |  |   GOV MODULE    |
|                |  |               |  |                |  |                 |
|                |  |               |  |                |  | Handles message,|
|                |  |               |  |                |  | Updates state   |
|                |  |               |  |                |  |                 |
+----------------+  +---------------+  +----------------+  +------+----------+
                                                                  |
                                                                  |
                                                                  |
                                                                  |
                                       +--------------------------+
                                       |
                                       | Return result to CometBFT
                                       | (0=Ok, 1=Err)
                                       v
```


Each module can be seen as a little **state-machine**. Developers need to define the subset of the state handled by the module, as well as custom message types that modify the state (Note: messages are extracted from transactions by baseapp). In general, each module declares its own KVStore in the multistore to persist the subset of the state it defines. Most developers will need to access other 3rd party modules when building their own modules. Given that the Cosmos SDK is an open framework, some of the modules may be malicious, which means there is a need for security principles to reason about inter-module interactions. These principles are based on object-capabilities. In practice, this means that instead of having each module keep an access control list for other modules, each module implements special objects called keepers that can be passed to other modules to grant a pre-defined set of capabilities.

Cosmos SDK modules are defined in the x/ folder of the Cosmos SDK. Some core modules include:

x/auth: Used to manage accounts and signatures.
x/bank: Used to enable tokens and token transfers.
x/staking + x/slashing: Used to build Proof-Of-Stake blockchains.
In addition to the already existing modules in x/, that anyone can use in their app, the Cosmos SDK lets you build your own custom modules. You can check an example of that in the tutorial.


#### **Module Structure and State Management**

-   **State-Machine Concept:**

    -   Each module functions as a small state-machine.
    -   Developers must define:
        -   The subset of the state managed by the module.
        -   Custom message types that modify the state.
-   **KVStore Usage:**

    -   Each module declares its own `KVStore` in the `multistore` to persist the subset of the state it handles.

* * * *

#### **Interacting with Other Modules**

-   **3rd Party Module Access:**

    -   Developers often need to access other 3rd party modules when building their own.
-   **Security Considerations:**

    -   **Open Framework Caution:**
        -   Be aware that some modules might be malicious due to the open nature of the Cosmos SDK.
    -   **Object-Capability Principles:**
        -   Modules should not keep access control lists for other modules.
        -   Implement "keepers" to grant a pre-defined set of capabilities securely.

* * * *

#### **Core Modules in the Cosmos SDK**

-   **Location:**

    -   Cosmos SDK modules are located in the `x/` folder of the Cosmos SDK.
-   **Examples of Core Modules:**

    -   **x/auth:** Manages accounts and signatures.
    -   **x/bank:** Enables tokens and token transfers.
    -   **x/staking + x/slashing:** Used for building Proof-Of-Stake blockchains.

* * * *

#### **Custom Module Development**

-   **Building Custom Modules:**
    -   In addition to existing modules in the `x/` folder, the Cosmos SDK allows developers to build their own custom modules for specific applications.