The `AnteHandler` is a special handler that implements the `AnteHandler` interface. It is used to authenticate a transaction before the transaction's internal messages are processed.

The `AnteHandler` is theoretically optional but still a very important component of public blockchain networks. It serves three primary purposes:

-   It is a first line of defense against spam, and the second line of defense (after the mempool) against transaction replay with fees deduction and sequence checking.
-   It performs preliminary stateful validity checks, like ensuring signatures are valid, or that a sender has enough funds to pay for fees.
-   It plays a role in the incentivization of stakeholders via the collection of transaction fees.

`BaseApp` holds an `AnteHandler` as a parameter that is initialized in the application's constructor. The most widely used `AnteHandler` is the auth module.