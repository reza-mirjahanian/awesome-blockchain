Configuring State Commitment [#](https://solana.com/docs/rpc#configuring-state-commitment)
------------------------------------------------------------------------------------------

For preflight checks and transaction processing, Solana nodes choose which **bank state** to query based on a commitment requirement set by the client. The commitment describes how finalized a block is at that point in time. When **querying** the ledger state, it's recommended to use lower levels of commitment to report progress and higher levels to ensure the state will not be rolled back.

