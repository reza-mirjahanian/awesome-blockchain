The ABCI application must be a deterministic result of the CometBFT consensus - any external influence on the application state that didn't come through CometBFT could cause a consensus failure. Thus *nothing* should communicate with the ABCI application except CometBFT via ABCI.

If the ABCI application is written in Go, it can be compiled into the CometBFT binary. Otherwise, it should use a unix socket to communicate with CometBFT. If it's necessary to use TCP, extra care must be taken to encrypt and authenticate the connection.

The Light-Client Daemon is what provides light clients (end users) with nearly all the security of a full node. It formats and broadcasts transactions, and verifies proofs of queries and transaction results. Note that it need not be a daemon - the Light-Client logic could instead be implemented in the same process as the end-user application.

Note for those ABCI applications with weaker security requirements, the functionality of the Light-Client Daemon can be moved into the ABCI application process itself. That said, exposing the ABCI application process to anything besides CometBFT over ABCI requires extreme caution, as all transactions, and possibly all queries, should still pass through CometBFT.
