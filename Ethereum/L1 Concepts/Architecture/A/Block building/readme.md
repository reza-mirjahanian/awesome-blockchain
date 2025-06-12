When a validator is selected to propose a block during a slot, it looks for the block produced by CL. Importantly, a validator isn't limited to broadcasting a block solely from its own EL. It can also broadcast a block produced by external builders; for details, refer to [PBS](https://ethereum.org/en/roadmap/pbs/). This article specifically explores how a block is produced by EL and the elements contributing to its successful production and transaction execution.


Payload building routine
---------------------------------------------------------------------------------------------------

A block is created when the consensus layer instructs the execution layer client to do so through the engine API's fork choice updated endpoint, which then initiates the process of constructing the block via the payload building routine.

Note: The fee recipient of the built payload may deviate from the suggested fee recipient of the payload attributes:


![alt text](image.png)

Nodes broadcast transactions through a peer-to-peer network using the gossip protocol. These transactions are validated against specific criteria (e.g. , checking nonce correctness, sufficient balance, and proper signatures) and stored in the mempool awaiting inclusion in a block.