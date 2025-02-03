-   Durable transactions have no expiration date unlike regular transactions that have an expiration date of 150 blocks (~80-90 seconds).
-   After signing a durable transaction you can store it in a database or a file or send it to another device to submit it later.
-   A durable transaction is created using a nonce account. A nonce account holds the authority and the nonce value which replaces the recent blockhash to make a durable transaction
-   Durable transactions must start with an `advanceNonce` instruction, and the nonce authority must be a signer of the transaction.
-   If the transaction fails for any reason other than the `advanceNonce` instruction, the nonce will still be advanced, even though all other instructions will be reverted.


-------------------------
Durable Nonces are a way to bypass the expiration date of regular transactions. To understand this, we'll start by looking at the concepts behind regular transactions.

In Solana, transactions have three main parts:

1.  **Instructions**: Instructions are the operations you want to perform on the blockchain, like transferring tokens, creating accounts, or calling a program. These are executed in order.

2.  **Signatures**: Signatures are proof that the transaction was made by someone with the signer's private key - which should usually be the signer themselves. For instance, if you are transferring SOL from your wallet to another, you'll need to sign the transaction so the network can verify that the transaction is valid.

3.  **Recent Blockhash**: The recent blockhash is a unique identifier for each transaction. It is used to prevent replay attacks, where an attacker records a transaction and then tries to submit it again. The recent blockhash ensures that each transaction is unique and can only be submitted once. A recent blockhash is only valid for 150 blocks.

In durable transactions, the first two concepts remain the same. Durable transactions are possible by playing with recent blockhashes.

--------------------
