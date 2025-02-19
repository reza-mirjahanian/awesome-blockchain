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


 ================
 CometBFT consists of **two** chief technical components: 
 - a blockchain consensus engine 
 - and a generic application interface.
  
  The consensus engine, which is based on Tendermint consensus algorithm, ensures that the same transactions are recorded on every machine in the same order. The application interface, called the Application BlockChain Interface (**ABCI**), delivers the transactions to applications for processing. Unlike other blockchain and consensus solutions, which come pre-packaged with built in state machines (like a fancy key-value store, or a quirky scripting language), developers can use CometBFT for BFT state machine replication of applications written in whatever programming language and development environment is right for them.