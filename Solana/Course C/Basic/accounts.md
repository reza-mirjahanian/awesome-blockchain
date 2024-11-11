all data is contained in what we call "accounts". You can think of data on Solana as a public database with a single "Accounts" table, where each entry in this table is an individual account.

Accounts on Solana can store "state" or "executable" programs, all of which can be thought of as entries in the same "Accounts" table. Each account has an "address" (public key) that serves as its unique ID used to locate its corresponding on-chain data.

Solana accounts contain either:

-   State: This is data that's meant to be read from and persisted. It could be information about tokens, user data, or any other type of data defined within a program.
-   Executable Programs: These are accounts that contain the actual code of Solana programs. They contain the instructions that can be executed on the network.

This separation of program code and program state is a key feature of Solana's Account Model. For more details, refer to the [Solana Account Model](https://solana.com/docs/core/accounts) page.