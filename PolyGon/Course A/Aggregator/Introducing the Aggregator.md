## Introducing the Aggregator
The aggregator is the component within the zkEVM architecture that will be in charge of
performing the proof aggregation schema. The aggregator invokes the verifyBatches()
function (See Figure 9) on the smart contract, passing parameters such as the initial batch
number **initNumBatch**, the final batch number (**finalNewBatch**), the **newStateRoot**, and
the **aggregated proof π**. The previous root is stored in the smart contract, eliminating
the need to transmit it. Recall that the smart contract contains a summary of the batch
information in the accumulated input hash.

![alt text](image-8.png)

Figure 9: The role of the Aggregator is to aggregate several proofs in one and send it to the L1
Smart Contract through the verifyBatches() function.



The aggregator operates as a network server, establishing connections with provers
that function as network clients. Provers link up with the aggregator to send their proofs.
The aggregator, acting as a server, is responsible for deciding how to horizontally scale
provers in order to achieve an optimal batch consolidation rate. Scaling is essential to
avoid an accumulation of extra batches awaiting consolidation. The aggregator keeps a
record of authorized provers. Both the aggregator and the provers operate in a cloudbased environment (See Figure 10), with the provers being configured as high-resource
instances. This configuration enables effective and scalable control of evidence processing,
guaranteeing the system can handle different workloads and maintain an efficient batch
**consolidation rate**


Inputs and Outputs of the Proof

The proof generation process requires several inputs, as shown in **Figure 11**, to ensure its soundness:

- The **aggregator address**, serving as a safeguard against malleability in the `verifyBatches()` function, ensuring that no one can use another aggregator's proof.
- The contract state root (**oldStateRoot**), which is already included in the smart contract and does not require explicit sending.
- The previous accumulated input hash (**oldAccInputHash**).
- Initial batch number (**initNumBatch**)
- The **chainID** and the **forkID** ensure that the proof is valid only within the intended chain and version of the zkEVM.

![alt text](image-9.png)

