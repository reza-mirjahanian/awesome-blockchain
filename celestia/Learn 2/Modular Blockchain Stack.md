#### The Modular Blockchain Stack

The four functions that modular blockchains can consist of are execution, settlement, consensus, and data availability.

-   Execution: The environment where applications live and state changes are executed.
-   Settlement: An optional hub for execution layers to verify proofs, resolve fraud disputes, and bridge between other execution layers.
-   Consensus: Agreement on the order of transactions.
-   Data availability: Verification that transaction data is available to download.
-   


#### Layer 1 and 2
![alt text](image.png)
The most prominent type of layer 2 is a rollup, which provides an environment for applications to be deployed to, and for transactions to be processed that interact with those applications. Layer 1 supports the rollup by allowing it to publish its blocks, which at minimum ensures that the transaction data in the block is ordered and available. Since layer 1 also has execution capabilities, it can ensure the validity of transactions if the layer 2 requires. Additionally, the layer 1 can also act as a hub to connect layer 2s, allowing them to bridge tokens and liquidity between them.

Essentially, the layer 1 is a monolithic chain that receives additional scale from layer 2. In most cases, the capacity of layer 2 is also dependent on layer 1s capacity. As a result, this implementation of a layer 1 & layer 2 stack is suboptimal for scalability.


#### Execution, settlement, and data availability