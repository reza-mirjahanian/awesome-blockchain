The [Application-Blockchain Interface](https://github.com/cometbft/cometbft/blob/v0.37.x/spec/abci/abci++_basic_concepts.md) (ABCI) is a generic interface that connects a state-machine with a consensus engine to form a functional full-node. It can be wrapped in any language, and needs to be implemented by each application-specific blockchain built on top of an ABCI-compatible consensus engine like CometBFT.

The consensus engine handles two main tasks:

-   The networking logic, which mainly consists in gossiping block parts, transactions and consensus votes.
-   The consensus logic, which results in the deterministic ordering of transactions in the form of blocks.

It is **not** the role of the consensus engine to define the state or the validity of transactions. Generally, transactions are handled by the consensus engine in the form of `[]bytes`, and relayed to the application via the ABCI to be decoded and processed. At keys moments in the networking and consensus processes (e.g. beginning of a block, commit of a block, reception of an unconfirmed transaction, ...), the consensus engine emits ABCI messages for the state-machine to act on.

Developers building on top of the Cosmos SDK need not implement the ABCI themselves, as `BaseApp` comes with a built-in implementation of the interface. Let 
us go through the main ABCI messages that BaseApp implements:

-   [`Prepare Proposal`](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#prepare-proposal)
-   [`Process Proposal`](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#process-proposal)
-   [`CheckTx`](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#checktx)
-   [`FinalizeBlock`](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#finalizeblock)
-   [`ExtendVote`](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#extendvote)
-   [`VerifyVoteExtension`](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#verifyvoteextension)

### Prepare Proposal[​](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#prepare-proposal "Direct link to Prepare Proposal")

The `PrepareProposal` function is part of the new methods introduced in Application Blockchain Interface (ABCI++) in CometBFT and is an important part of the application's overall governance system. In the Cosmos SDK, it allows the application to have more fine-grained control over the transactions that are processed, and ensures that only valid transactions are committed to the blockchain.

Here is how the `PrepareProposal` function can be implemented:

1.  Extract the `sdk.Msg`s from the transaction.
2.  Perform *stateful* checks by calling `Validate()` on each of the `sdk.Msg`'s. This is done after *stateless* checks as *stateful* checks are more computationally expensive. If `Validate()` fails, `PrepareProposal` returns before running further checks, which saves resources.
3.  Perform any additional checks that are specific to the application, such as checking account balances, or ensuring that certain conditions are met before a transaction is proposed.hey are processed by the consensus engine, if necessary.
4.  Return the updated transactions to be processed by the consensus engine

Note that, unlike `CheckTx()`, `PrepareProposal` process `sdk.Msg`s, so it can directly update the state. However, unlike `FinalizeBlock()`, it does not commit the state updates. It's important to exercise caution when using `PrepareProposal` as incorrect coding could affect the overall liveness of the network.

It's important to note that `PrepareProposal` complements the `ProcessProposal` method which is executed after this method. The combination of these two methods means that it is possible to guarantee that no invalid transactions are ever committed. Furthermore, such a setup can give rise to other interesting use cases such as Oracles, threshold decryption and more.

`PrepareProposal` returns a response to the underlying consensus engine of type [`abci.ResponseCheckTx`](https://github.com/cometbft/cometbft/blob/v0.37.x/spec/abci/abci++_methods.md#processproposal). The response contains:

-   `Code (uint32)`: Response Code. `0` if successful.
-   `Data ([]byte)`: Result bytes, if any.
-   `Log (string):` The output of the application's logger. May be non-deterministic.
-   `Info (string):` Additional information. May be non-deterministic.

### Process Proposal[​](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#process-proposal "Direct link to Process Proposal")

The `ProcessProposal` function is called by the BaseApp as part of the ABCI message flow, and is executed during the `FinalizeBlock` phase of the consensus process. The purpose of this function is to give more control to the application for block validation, allowing it to check all transactions in a proposed block before the validator sends the prevote for the block. It allows a validator to perform application-dependent work in a proposed block, enabling features such as immediate block execution, and allows the Application to reject invalid blocks.

The `ProcessProposal` function performs several key tasks, including:

1.  Validating the proposed block by checking all transactions in it.
2.  Checking the proposed block against the current state of the application, to ensure that it is valid and that it can be executed.
3.  Updating the application's state based on the proposal, if it is valid and passes all checks.
4.  Returning a response to CometBFT indicating the result of the proposal processing.

The `ProcessProposal` is an important part of the application's overall governance system. It is used to manage the network's parameters and other key aspects of its operation. It also ensures that the coherence property is adhered to i.e. all honest validators must accept a proposal by an honest proposer.

It's important to note that `ProcessProposal` complements the `PrepareProposal` method which enables the application to have more fine-grained transaction control by allowing it to reorder, drop, delay, modify, and even add transactions as they see necessary. The combination of these two methods means that it is possible to guarantee that no invalid transactions are ever committed. Furthermore, such a setup can give rise to other interesting use cases such as Oracles, threshold decryption and more.

CometBFT calls it when it receives a proposal and the CometBFT algorithm has not locked on a value. The Application cannot modify the proposal at this point but can reject it if it is invalid. If that is the case, CometBFT will prevote `nil` on the proposal, which has strong liveness implications for CometBFT. As a general rule, the Application SHOULD accept a prepared proposal passed via `ProcessProposal`, even if a part of the proposal is invalid (e.g., an invalid transaction); the Application can ignore the invalid part of the prepared proposal at block execution time.

However, developers must exercise greater caution when using these methods. Incorrectly coding these methods could affect liveness as CometBFT is unable to receive 2/3 valid precommits to finalize a block.

`ProcessProposal` returns a response to the underlying consensus engine of type [`abci.ResponseCheckTx`](https://github.com/cometbft/cometbft/blob/v0.37.x/spec/abci/abci++_methods.md#processproposal). The response contains:

-   `Code (uint32)`: Response Code. `0` if successful.
-   `Data ([]byte)`: Result bytes, if any.
-   `Log (string):` The output of the application's logger. May be non-deterministic.
-   `Info (string):` Additional information. May be non-deterministic.

### CheckTx[​](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#checktx "Direct link to CheckTx")

`CheckTx` is sent by the underlying consensus engine when a new unconfirmed (i.e. not yet included in a valid block) transaction is received by a full-node. The role of `CheckTx` is to guard the full-node's mempool (where unconfirmed transactions are stored until they are included in a block) from spam transactions. Unconfirmed transactions are relayed to peers only if they pass `CheckTx`.

`CheckTx()` can perform both *stateful* and *stateless* checks, but developers should strive to make the checks **lightweight** because gas fees are not charged for the resources (CPU, data load...) used during the `CheckTx`.

In the Cosmos SDK, after [decoding transactions](https://docs.cosmos.network/v0.50/learn/advanced/encoding), `CheckTx()` is implemented to do the following checks:

1.  Extract the `sdk.Msg`s from the transaction.
2.  **Optionally** perform *stateless* checks by calling `ValidateBasic()` on each of the `sdk.Msg`s. This is done first, as *stateless* checks are less computationally expensive than *stateful* checks. If `ValidateBasic()` fail, `CheckTx` returns before running *stateful* checks, which saves resources. This check is still performed for messages that have not yet migrated to the new message validation mechanism defined in [RFC 001](https://docs.cosmos.network/main/rfc/rfc-001-tx-validation) and still have a `ValidateBasic()` method.
3.  Perform non-module related *stateful* checks on the [account](https://docs.cosmos.network/v0.50/learn/beginner/accounts). This step is mainly about checking that the `sdk.Msg` signatures are valid, that enough fees are provided and that the sending account has enough funds to pay for said fees. Note that no precise [`gas`](https://docs.cosmos.network/v0.50/learn/beginner/gas-fees) counting occurs here, as `sdk.Msg`s are not processed. Usually, the [`AnteHandler`](https://docs.cosmos.network/v0.50/learn/beginner/gas-fees#antehandler) will check that the `gas` provided with the transaction is superior to a minimum reference gas amount based on the raw transaction size, in order to avoid spam with transactions that provide 0 gas.

`CheckTx` does **not** process `sdk.Msg`s - they only need to be processed when the canonical state needs to be updated, which happens during `FinalizeBlock`.

Steps 2. and 3. are performed by the [`AnteHandler`](https://docs.cosmos.network/v0.50/learn/beginner/gas-fees#antehandler) in the [`RunTx()`](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#runtx-antehandler-and-runmsgs) function, which `CheckTx()` calls with the `runTxModeCheck` mode. During each step of `CheckTx()`, a special [volatile state](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#state-updates) called `checkState` is updated. This state is used to keep track of the temporary changes triggered by the `CheckTx()` calls of each transaction without modifying the [main canonical state](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#main-state). For example, when a transaction goes through `CheckTx()`, the transaction's fees are deducted from the sender's account in `checkState`. If a second transaction is received from the same account before the first is processed, and the account has consumed all its funds in `checkState` during the first transaction, the second transaction will fail `CheckTx`() and be rejected. In any case, the sender's account will not actually pay the fees until the transaction is actually included in a block, because `checkState` never gets committed to the main state. The `checkState` is reset to the latest state of the main state each time a blocks gets [committed](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#commit).

`CheckTx` returns a response to the underlying consensus engine of type [`abci.ResponseCheckTx`](https://github.com/cometbft/cometbft/blob/v0.37.x/spec/abci/abci++_methods.md#checktx). The response contains:

-   `Code (uint32)`: Response Code. `0` if successful.
-   `Data ([]byte)`: Result bytes, if any.
-   `Log (string):` The output of the application's logger. May be non-deterministic.
-   `Info (string):` Additional information. May be non-deterministic.
-   `GasWanted (int64)`: Amount of gas requested for transaction. It is provided by users when they generate the transaction.
-   `GasUsed (int64)`: Amount of gas consumed by transaction. During `CheckTx`, this value is computed by multiplying the standard cost of a transaction byte by the size of the raw transaction. Next is an example:

x/auth/ante/basic.go
```
ctx.GasMeter().ConsumeGas(params.TxSizeCostPerByte*storetypes.Gas(len(ctx.TxBytes())),"txSize")

```

[See full example on GitHub](https://github.com/cosmos/cosmos-sdk/blob/v0.50.0-alpha.0/x/auth/ante/basic.go#L102)

-   `Events ([]cmn.KVPair)`: Key-Value tags for filtering and indexing transactions (eg. by account). See [`event`s](https://docs.cosmos.network/v0.50/learn/advanced/events) for more.
-   `Codespace (string)`: Namespace for the Code.

#### RecheckTx[​](https://docs.cosmos.network/v0.50/learn/advanced/baseapp#rechecktx "Direct link to RecheckTx")

After `Commit`, `CheckTx` is run again on all transactions that remain in the node's local mempool excluding the transactions that are included in the block. To prevent the mempool from rechecking all transactions every time a block is committed, the configuration option `mempool.recheck=false` can be set. As of Tendermint v0.32.1, an additional `Type` parameter is made available to the `CheckTx` function that indicates whether an incoming transaction is new (`CheckTxType_New`), or a recheck (`CheckTxType_Recheck`). This allows certain checks like signature verification can be skipped during `CheckTxType_Recheck`.