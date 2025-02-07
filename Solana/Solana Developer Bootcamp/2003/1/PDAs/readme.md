https://chainstack.com/tag/solana/

There are two main reasons for PDAs;

1.  Store program state
2.  Sign for CPIs


> What problem or use case does it solve?

PDA's allow you to create a "hashmap-like" interface for indexing accounts. The seeds used to create the PDAs, function as a way to lookup addresses for a particular piece of data. The seeds can be anything. A pubkey, a string, an array of numbers etc.


1. PDAs are mainly used to store program data (or "state") on Solana. Think of state as important information your program needs - like how much money users have, their game scores, or agreement details. PDAs are perfect for programs that need to keep track of lots of information.

1. PDAs can also act like automatic signers for your program when it needs to work with other programs on Solana. This means your program can safely interact with other programs without needing actual private keys. This makes it much easier to build apps that need to work with multiple programs.