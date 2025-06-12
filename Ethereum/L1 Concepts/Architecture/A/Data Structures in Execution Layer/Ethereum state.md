Ethereum state is stored in four different modified Merkle Patricia Tries (MMPTs):

-   Transaction Trie
-   Receipt Trie
-   World State Trie
-   Account State Trie

![alt text](image-3.png)

At each block there is one transaction, receipt, and state trie which are referenced by their root hashes in the block Header. For every contract deployed on Ethereum there is a storage trie used to hold that contract's persistent variables, each storage trie is referenced by their root hash in the state account object stored in the state trie leaf node corresponding to that contract's address.

Transaction Trie
----------------------------------------------------------------------------------

The Transaction Trie is a data structure responsible for storing all the transactions within a specific block. Every block has its own Transaction Trie, corresponding to the respective transactions that are included in that block. Ethereum is a transaction based state machine. This means every action or change in Ethereum is due to a transaction. Every block is made up of a block header and a transaction list(among other things). Thus, once a transaction is executed and a block is finalized the transaction trie for that block can never be changed.(in contrast to the World State trie).


A transaction is mapped in the trie so that the key is a transaction index and the value is the transaction T . Both the transaction index and the transaction itself are RLP encoded. It compose a key-value pair, stored in the trie: `𝑅𝐿𝑃 (𝑖𝑛𝑑𝑒𝑥) → 𝑅𝐿𝑃 (𝑇)`

The structure `T` consists of the following:

-   **Nonce**: For every new transaction submitted by the same sender, the nonce is increased. This value allows for tracking order of transactions and prevents replay attacks.
-   **maxPriorityFeePerGas** \- The maximum price of the consumed gas to be included as a tip to the validator.
-   **gasLimit**: The maximum amount of gas units that can be consumed by the transaction.
-   **maxFeePerGas** \- the maximum fee per unit of gas willing to be paid for the transaction (including baseFeePerGas and maxPriorityFeePerGas).
-   **from** -- The address of the sender, that will be signing the transaction. This must be an externally-owned account as contract accounts cannot send transactions.
-   **to**: Address of an account to receive funds, or zero for contract creation.
-   **value**: amount of ETH to transfer from sender to recipient.
-   **input data**: optional field to include arbitrary data.
-   **data**: Input data for a message call together with the message signature.
-   **(v, r, s)**: Values encoding signature of a sender. Serves as identifier of the sender.