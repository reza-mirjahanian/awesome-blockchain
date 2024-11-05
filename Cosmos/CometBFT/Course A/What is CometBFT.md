### In short:
CometBFT performs Byzantine Fault Tolerant (BFT) State Machine Replication (SMR) for arbitrary deterministic, finite state machines

------------
What is CometBFT
================

CometBFT is software for **securely** and **consistently** replicating an application on many machines. 
#### By securely: 
we mean that CometBFT works as long as less than 1/3 of machines fail in arbitrary ways. 
#### By consistently:
 we mean that every non-faulty machine sees the same transaction log and computes the same state. Secure and consistent replication is a fundamental problem in distributed systems; it plays a critical role in the fault tolerance of a broad range of applications, from currencies, to elections, to infrastructure orchestration, and beyond.