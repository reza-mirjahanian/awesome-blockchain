## double-spend problem

Let's dive deep into the recent blockhash, to understand the blockhash better let's look at the problem that it tries to solve, the [double-spend](https://solana.com/developers/guides/advanced/introduction-to-durable-nonces#double-spend) problem.

Imagine you're buying an NFT on MagicEden or Tensor. You must sign a transaction that allows the marketplace's program to extract some SOL from your wallet. After signing the transaction the marketplace will submit it to the network. If the marketplace submits it again, without checks, you could be charged twice.

This is known as the double-spend problem and is one of the core issues that blockchains, like Solana, solve. A naive solution could be to crosscheck all transactions made in the past and see if we find a duplicate transaction signature. This is not practically possible, as the size of the Solana ledger is >80 TB. So to solve this, Solana uses recent blockhashes.

A recent blockhash is a 32-byte SHA-256 hash of a valid block's last [entry id](https://solana.com/docs/terminology#blockhash) within the last 150 blocks. Since this recent blockhash was part of the transaction before it was signed, we can guarantee the signer has signed it within the last 150 blocks. Checking 150 blocks is much more reasonable than the entire ledger.

When the transaction is submitted, the Solana validators will do the following:

1.  Check if the signature of the transaction has been submitted within the last 150 blocks - if there is a duplicate signature it'll fail the duplicate transaction.
2.  If the transaction signature has not been found, it will check the recent blockhash to see if it exists within the last 150 blocks - if it does not, it will return a "Blockhash not found" error. If it does, the transaction goes through its execution checks.
