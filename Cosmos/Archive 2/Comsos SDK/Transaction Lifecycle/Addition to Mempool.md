Each full-node (running CometBFT) that receives a `Tx` sends an [ABCI message](https://docs.cometbft.com/v0.37/spec/p2p/messages/), `CheckTx`, to the application layer to check for validity, and receives an `abci.ResponseCheckTx`. If the `Tx` passes the checks, it is held in the node's [**Mempool**](https://docs.cometbft.com/v0.37/spec/p2p/messages/mempool/), an in-memory pool of transactions unique to each node, pending inclusion in a block - honest nodes discard a `Tx` if it is found to be invalid. Prior to consensus, nodes continuously check incoming transactions and gossip them to their peers.

### Types of Checks[​](https://docs.cosmos.network/v0.50/learn/beginner/tx-lifecycle#types-of-checks "Direct link to Types of Checks")

The full-nodes perform stateless, then stateful checks on `Tx` during `CheckTx`, with the goal to identify and reject an invalid transaction as early on as possible to avoid wasted computation.

***Stateless*** checks do not require nodes to access state - light clients or offline nodes can do them - and are thus less computationally expensive. Stateless checks include making sure addresses are not empty, enforcing nonnegative numbers, and other logic specified in the definitions.

***Stateful*** checks validate transactions and messages based on a committed state. Examples include checking that the relevant values exist and can be transacted with, the address has sufficient funds, and the sender is authorized or has the correct ownership to transact. At any given moment, full-nodes typically have [multiple versions](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#state-updates) of the application's internal state for different purposes. For example, nodes execute state changes while in the process of verifying transactions, but still need a copy of the last committed state in order to answer queries - they should not respond using state with uncommitted changes.

In order to verify a `Tx`, full-nodes call `CheckTx`, which includes both *stateless* and *stateful* checks. Further validation happens later in the [`DeliverTx`](https://docs.cosmos.network/v0.50/learn/beginner/tx-lifecycle#delivertx) stage. `CheckTx` goes through several steps, beginning with decoding `Tx`.

### Decoding[​]

When `Tx` is received by the application from the underlying consensus engine (e.g. CometBFT ), it is still in its [encoded](https://docs.cosmos.network/v0.50/learn/advanced/encoding) `[]byte` form and needs to be unmarshaled in order to be processed. Then, the [`runTx`](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#runtx-antehandler-runmsgs-posthandler) function is called to run in `runTxModeCheck` mode, meaning the function runs all checks but exits before executing messages and writing state changes.

### ValidateBasic (deprecated)

Messages ([`sdk.Msg`](https://docs.cosmos.network/v0.50/learn/advanced/transactions#messages)) are extracted from transactions (`Tx`). The `ValidateBasic` method of the `sdk.Msg` interface implemented by the module developer is run for each transaction. To discard obviously invalid messages, the `BaseApp` type calls the `ValidateBasic` method very early in the processing of the message in the [`CheckTx`](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#checktx) and [`DeliverTx`](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#delivertx) transactions. `ValidateBasic` can include only **stateless** checks (the checks that do not require access to the state).

danger

The `ValidateBasic` method on messages has been deprecated in favor of validating messages directly in their respective [`Msg` services](https://docs.cosmos.network/v0.50/build/building-modules/msg-services#Validation).

Read [RFC 001](https://docs.cosmos.network/main/rfc/rfc-001-tx-validation) for more details.

note

`BaseApp` still calls `ValidateBasic` on messages that implements that method for backwards compatibility.

#### Guideline

`ValidateBasic` should not be used anymore. Message validation should be performed in the `Msg` service when [handling a message](https://docs.cosmos.network/v0.50/build/building-modules/msg-services#Validation) in a module Msg Server.

### AnteHandler

`AnteHandler`s even though optional, are in practice very often used to perform signature verification, gas calculation, fee deduction, and other core operations related to blockchain transactions.

A copy of the cached context is provided to the `AnteHandler`, which performs limited checks specified for the transaction type. Using a copy allows the `AnteHandler` to do stateful checks for `Tx` without modifying the last committed state, and revert back to the original if the execution fails.

For example, the [`auth`](https://github.com/cosmos/cosmos-sdk/tree/main/x/auth/spec) module `AnteHandler` checks and increments sequence numbers, checks signatures and account numbers, and deducts fees from the first signer of the transaction - all state changes are made using the `checkState`.

danger

Ante handlers only run on a** transaction**. If a transaction embed multiple messages (like some x/authz, x/gov transactions for instance), the ante handlers only have awareness of the outer message. Inner messages are mostly directly routed to the [message router](https://docs.cosmos.network/main/learn/advanced/baseapp#msg-service-router) and will skip the chain of ante handlers. Keep that in mind when designing your own ante handler.

### Gas[​](https://docs.cosmos.network/v0.50/learn/beginner/tx-lifecycle#gas "Direct link to Gas")

The [`Context`](https://docs.cosmos.network/v0.50/learn/advanced/context), which keeps a `GasMeter` that tracks how much gas is used during the execution of `Tx`, is initialized. The user-provided amount of gas for `Tx` is known as `GasWanted`. If `GasConsumed`, the amount of gas consumed during execution, ever exceeds `GasWanted`, the execution stops and the changes made to the cached copy of the state are not committed. Otherwise, `CheckTx` sets `GasUsed` equal to `GasConsumed` and returns it in the result. After calculating the gas and fee values, validator-nodes check that the user-specified `gas-prices` is greater than their locally defined `min-gas-prices`.

### Discard or Addition to Mempool[​](https://docs.cosmos.network/v0.50/learn/beginner/tx-lifecycle#discard-or-addition-to-mempool "Direct link to Discard or Addition to Mempool")

If at any point during `CheckTx` the `Tx` fails, it is discarded and the transaction lifecycle ends there. Otherwise, if it passes `CheckTx` successfully, the default protocol is to relay it to peer nodes and add it to the Mempool so that the `Tx` becomes a candidate to be included in the next block.

The **mempool** serves the purpose of keeping track of transactions seen by all full-nodes. Full-nodes keep a **mempool cache** of the last `mempool.cache_size` transactions they have seen, as a first line of defense to prevent replay attacks. Ideally, `mempool.cache_size` is large enough to encompass all of the transactions in the full mempool. If the mempool cache is too small to keep track of all the transactions, `CheckTx` is responsible for identifying and rejecting replayed transactions.

Currently existing preventative measures include fees and a `sequence` (nonce) counter to distinguish replayed transactions from identical but valid ones. If an attacker tries to spam nodes with many copies of a `Tx`, full-nodes keeping a mempool cache reject all identical copies instead of running `CheckTx` on them. Even if the copies have incremented `sequence` numbers, attackers are disincentivized by the need to pay fees.

Validator nodes keep a mempool to prevent replay attacks, just as full-nodes do, but also use it as a pool of unconfirmed transactions in preparation of block inclusion. Note that even if a `Tx` passes all checks at this stage, it is still possible to be found invalid later on, because `CheckTx` does not fully validate the transaction (that is, it does not actually execute the messages).