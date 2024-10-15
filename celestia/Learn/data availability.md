data availability isn’t just a resource that rollups consume. DA allows anyone to directly verify that a blockchain is running correctly. Until now, rollups have had to trust small committees to relieve the DA constraint. That means anyone who wants to interact with the rollup must rely on trusted third parties to access and verify the network. So, fixing the DA constraint with proofs instead of committees allows rollups to regain verifiability.

#### What is data availability?

Data availability is about proving data was published to the network. So, when a chain produces new blocks, nodes verify DA by downloading all the data. Although there is a more efficient way to verify DA (more on this later).

Really, data availability is like streaming a sports game. DA lets anyone download transactions to see what happened, just like streaming lets anyone watch a game if they aren't at the stadium.

The one thing that data availability doesn't cover is the long-term storage of transaction data. DA is just about publishing data and temporary storage.

![alt text](image-2.png)

The difference might not seem important, but DA and long-term data storage actually have varying security properties.


#### Data availability layers

Now there are specialized providers known as data availability layers (DA layers) that supply DA to other chains. Let's look at two distinct types of DA layers:

-   Data availability committees: a small, permissioned committee that is trusted to provide DA.
-   DA layers with [data availability sampling (DAS)](https://celestia.org/what-is-celestia/#what-is-data-availability-sampling): a decentralized network that provides DA and allows anyone to efficiently verify via DAS.

And a DA layer with DAS is what **Celestia** is...