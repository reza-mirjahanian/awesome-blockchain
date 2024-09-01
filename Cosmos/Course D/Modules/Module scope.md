Module scope
------------

Modules include **core** functionality that every blockchain node needs:

-   A boilerplate implementation of the Application Blockchain Interface (ABCI) that communicates with CometBFT.
-   A general-purpose data store that persists the module state called `multistore`.
-   A server and interfaces to facilitate interactions with the node.

Modules implement the majority of the application logic while the **core** attends to wiring and infrastructure concerns, and enables modules to be composed into higher-order modules.

A module defines a subset of the overall state, using:

-   One or more keys or value stores, known as `KVStore`.
-   A subset of message types that are needed by the application and do not exist yet.

Modules also define interactions with other modules that already exist.


Most of the work for developers involved in building a Cosmos SDK application consists of building custom modules required by their application that do not exist yet, and integrating them with modules that already exist into one coherent application. Existing modules can come either from the Cosmos SDK itself or from **third-party developers**. You can download these from an online module repository.