Ethereum state is stored in four different modified Merkle Patricia Tries (MMPTs):

-   Transaction Trie
-   Receipt Trie
-   World State Trie
-   Account State Trie

![alt text](image-3.png)

At each block there is one transaction, receipt, and state trie which are referenced by their root hashes in the block Header. For every contract deployed on Ethereum there is a storage trie used to hold that contract's persistent variables, each storage trie is referenced by their root hash in the state account object stored in the state trie leaf node corresponding to that contract's address.