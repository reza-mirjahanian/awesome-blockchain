## Current Design

In our current design, the central entity responsible for assembling batches for sequencing is a **trusted sequencer**, which is built and managed by Polygon (implemented by us). However, it is prudent to consider the possibility that this sequencer may omit our Layer 2 transactions. In light of this concern, we have implemented additional mechanisms which will be discussed in more detail later on.

Figure 1 shows how the Sequence System works.

!Sequence System Workflow

![alt text](image.png)

- The user initiates L2 transactions via JSON RPCs.
- These are directed to a database known as `the pool`.
- The Sequencer aggregates these transactions.
- Specific criteria are applied to select pending L2 transactions.
- These selected transactions are engaged by the Sequencer.
- A batch is generated to be sequenced and then proved.


## 2 Batch pre-Execution

 The initial step in creating a batch involves verifying that the chosen transactions align
 with the available execution traces and do not surpass the gas limit. This procedure,
 known as batch pre-execution, is carried out by the sequencer through an executor as
 depicted in Figure 2. While no proof is generated during this stage, it ensures that the
 subsequent proof generation process by the prover can be successfully accomplished.

 ![alt text](image-1.png)


 **The objective** is to expedite the sequencing of batches to enhance the user's perception of speed. Therefore, a **fast executor** (single-computer executor) is employed, capable of executing within **the blocktime**. The sequencer communicates with this executor to perform pre-execution batch processing swiftly. Upon pre-execution, the sequencer records rectify till a specific point through successful batch determination transactions that correlate the batch in the node's StateDB as a **closed batch**. Closure may occur when the maximum number of execution trace rows is reached, the maximum gas limit is attained, or if allocated time expires.

During the batch pre-execution, for closing both the sequencer and executor also update **the Merkle tree** of **the zkEVM that is stored in Prover HashDB with L2 state changes**, as shown in Figure 3.


![alt text](image-2.png)


## performance problems 
