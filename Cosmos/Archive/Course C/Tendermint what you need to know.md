Tendermint is a consensus algorithm with Byzantine Fault-Tolerance (BFT) and a consensus engine. It enables applications to be replicated in sync on many machines. Blockchain networks require BFT to ensure proper function even with malfunctioning or malicious nodes present. The result is known as a *Replicated State Machine with Byzantine Fault Tolerance*. It guarantees BFT properties for distributed systems and their applications.

It does this:

- **Securely** - Tendermint continues working even if up to 1/3rd of machines fail or misbehave.
- **Consistently** - every machine computes the same state and accesses the same transaction log.

Tendermint is widely used across the industry and is the most mature BFT consensus engine for Proof-of-Stake (PoS) blockchains.