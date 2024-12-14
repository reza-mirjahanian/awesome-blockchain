The `ProcessProposal` function is called by the BaseApp as part of the ABCI message flow, and is executed during the `FinalizeBlock` phase of the consensus process. The purpose of this function is to give more control to the application for block validation, allowing it to check all transactions in a proposed block before the validator sends the **prevote** for the block. It allows a validator to perform application-dependent work in a proposed block, enabling features such as immediate block execution, and allows the Application to reject invalid blocks.

The `ProcessProposal` function performs several key tasks, including:

1.  Validating the proposed block by checking all transactions in it.
2.  Checking the proposed block against the current state of the application, to ensure that it is valid and that it can be executed.
3.  Updating the application's state based on the proposal, if it is valid and passes all checks.
4.  Returning a response to CometBFT indicating the result of the proposal processing.

The `ProcessProposal` is an important part of the application's overall governance system. It is used to manage the network's parameters and other key aspects of its operation. It also ensures that the coherence property is adhered to i.e. all honest validators must accept a proposal by an honest proposer.

It's important to note that `ProcessProposal` complements the `PrepareProposal` method which enables the application to have more fine-grained transaction control by allowing it to reorder, drop, delay, modify, and even add transactions as they see necessary. The combination of these two methods means that it is possible to guarantee that no invalid transactions are ever committed. Furthermore, such a setup can give rise to other interesting use cases such as Oracles, threshold decryption and more.

CometBFT calls it when it receives a proposal and the CometBFT algorithm has not locked on a value. The Application cannot modify the proposal at this point but can reject it if it is invalid. If that is the case, CometBFT will prevote `nil` on the proposal, which has strong liveness implications for CometBFT. As a general rule, the Application SHOULD accept a prepared proposal passed via `ProcessProposal`, even if a part of the proposal is invalid (e.g., an invalid transaction); the Application can ignore the invalid part of the prepared proposal at block execution time.

However, developers must exercise greater caution when using these methods. Incorrectly coding these methods could affect liveness as CometBFT is unable to receive 2/3 valid precommits to finalize a block.

`ProcessProposal` returns a response to the underlying consensus engine of type [`abci.ProcessProposalResponse`](https://docs.cometbft.com/v1.0/spec/abci/abci++_methods#processproposal). The response contains:

-   `Status (ProposalStatus)`: Status of the proposal processing

where `ProposalStatus` can be one of the [following status value](https://docs.cometbft.com/v1.0/spec/abci/abci++_methods#proposalstatus):

```
enum ProposalStatus {
UNKNOWN = 0; // Unknown status. Returning this from the application is always an error.
ACCEPT  = 1; // Status that signals that the application finds the proposal valid.
REJECT  = 2; // Status that signals that the application finds the proposal invalid.
}
```