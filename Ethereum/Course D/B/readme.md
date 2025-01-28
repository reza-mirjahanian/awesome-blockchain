**Slots and Epochs**
--------------------
https://ethos.dev/beacon-chain

The Beacon Chain provides the heartbeat to Ethereum's consensus. Each slot is 12 seconds and an epoch is 32 slots: 6.4 minutes.

![alt text](image.png)

A slot is a chance for a block to be added to the Beacon Chain. Every 12 seconds, one block is added when the system is running optimally. Validators do need to be roughly [synchronized with time.](https://ethresear.ch/t/network-adjusted-timestamps/4187)

A slot is like the block time, but slots can be empty. The Beacon Chain genesis block is at Slot 0.


**Validators and Attestations**
-------------------------------

While proof-of-work is associated with miners, Ethereum's validators are proof-of-stake "virtual miners". Validators run Ethereum's consensus.

A block **proposer** is a validator that has been pseudorandomly selected to build a block.

Most of the time, validators are **attesters** that vote on blocks.  These votes are recorded in the Beacon Chain and determine the head of the Beacon Chain.

![alt text](image-1.png)
At every epoch, a validator is pseudorandomly assigned to a slot.
