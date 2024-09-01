CLI
----

For the command-line interface (CLI), module developers create subcommands to add as children to the module-level message commands. These commands describe how to craft a message for inclusion in a transaction.

![](https://ida.interchain.io/hi-tip-icon.svg)

With v0.50, the Cosmos SDK introduces the [`autocli` facility (opens new window)](https://docs.cosmos.network/v0.50/learn/advanced/autocli#module-wiring--customization). This takes care of a lot of the boilerplate and lets you define the available CLI commands in a descriptive manner.

To summarize, this section has explored:

-   Messages, one of two primary objects handled by a module in the Cosmos SDK, which inform the state and have the potential to alter it.
-   How one or more messages form a transaction in the Cosmos SDK, and messages are only processed after a transaction is signed by a validator and included in a block by the consensus layer.
-   An example of more complex message handling capabilities related to the checkers game blockchain.