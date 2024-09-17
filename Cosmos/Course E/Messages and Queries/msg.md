Messages
-----------------------------------------------------------------------------------------------------------------------------

`Msg`s are objects whose end-goal is to trigger state-transitions. They are wrapped in [transactions](https://docs.cosmos.network/v0.50/learn/advanced/transactions), which may contain one or more of them.

When a transaction is relayed from the underlying consensus engine to the Cosmos SDK application, it is first decoded by [`BaseApp`](https://docs.cosmos.network/v0.50/learn/advanced/baseapp). Then, each message contained in the transaction is extracted and routed to the appropriate module via `BaseApp`'s `MsgServiceRouter` so that it can be processed by the module's [`Msg` service](https://docs.cosmos.network/v0.50/build/building-modules/msg-services). For a more detailed explanation of the lifecycle of a transaction, click [here](https://docs.cosmos.network/v0.50/learn/beginner/tx-lifecycle).