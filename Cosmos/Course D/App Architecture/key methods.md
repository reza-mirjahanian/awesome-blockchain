Any application that uses CometBFT for consensus must implement ABCI. You do not have to do this manually, because the Cosmos SDK provides a boilerplate known as BaseApp to get you started

### `CheckTx`

Many transactions that could be broadcast should not be broadcast. Examples include malformed transactions and spam-like artifacts. However, CometBFT cannot determine the transaction interpretation because it is agnostic to it. To address this, the Application Blockchain Interface includes a `CheckTx` method. **CometBFT** uses this method to ask the application layer if a transaction is valid. Applications implement this function.

###`DeliverTx`

CometBFT calls the `DeliverTx` method to pass block information to the application layer for interpretation and possible state machine transition.

### `BeginBlock` and `EndBlock`

`BeginBlock` and `EndBlock` messages are sent through the ABCI even if blocks contain **no transactions**. This provides positive confirmation of basic connectivity and helps identify time periods with no operations. These methods facilitate the execution of scheduled processes that should always run because they call methods at the application level, where developers define processes.

It is wise to be cautious about adding too much computational weight at the start or completion of each block, as blocks arrive at approximately seven-second intervals. Too much work could slow down your blockchain.

