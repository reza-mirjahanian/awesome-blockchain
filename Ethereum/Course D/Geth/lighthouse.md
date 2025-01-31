Lighthouse is an Ethereum consensus client that connects to other Ethereum consensus clients to form a resilient and decentralized proof-of-stake blockchain

---------------

[Recommended System Requirements](https://lighthouse-book.sigmaprime.io/installation.html#recommended-system-requirements)
--------------------------------------------------------------------------------------------------------------------------

Before [The Merge](https://ethereum.org/en/roadmap/merge/), Lighthouse was able to run on its own with low to mid-range consumer hardware, but would perform best when provided with ample system resources.

After [The Merge](https://ethereum.org/en/roadmap/merge/) on 15^th^ September 2022, it is necessary to run Lighthouse together with an execution client ([Nethermind](https://nethermind.io/), [Besu](https://www.hyperledger.org/use/besu), [Erigon](https://github.com/ledgerwatch/erigon), [Geth](https://geth.ethereum.org/), [Reth](https://github.com/paradigmxyz/reth)). The following system requirements listed are therefore for running a Lighthouse beacon node combined with an execution client , and a validator client with a modest number of validator keys (less than 100):

-   CPU: Quad-core AMD Ryzen, Intel Broadwell, ARMv8 or newer
-   Memory: 32 GB RAM\*
-   Storage: 2 TB solid state drive
-   Network: 100 Mb/s download, 20 Mb/s upload broadband connection

-------
[Step 2: Set up an execution node](https://lighthouse-book.sigmaprime.io/run_a_node.html#step-2-set-up-an-execution-node)
-------------------------------------------------------------------------------------------------------------------------

The Lighthouse beacon node *must* connect to an execution engine in order to validate the transactions present in blocks. The execution engine connection must be *exclusive*, i.e. you must have one execution node per beacon node. The reason for this is that the beacon node *controls* the execution node. Select an execution client from the list below and run it:

-   [Nethermind](https://docs.nethermind.io/nethermind/first-steps-with-nethermind/running-nethermind-post-merge)
-   [Besu](https://besu.hyperledger.org/en/stable/public-networks/get-started/connect/mainnet/)
-   [Erigon](https://github.com/ledgerwatch/erigon#beacon-chain-consensus-layer)
-   [Geth](https://geth.ethereum.org/docs/getting-started/consensus-clients)
-   [Reth](https://reth.rs/run/mainnet.html)


-----------

[Step 3: Set up a beacon node using Lighthouse](https://lighthouse-book.sigmaprime.io/run_a_node.html#step-3-set-up-a-beacon-node-using-lighthouse)
---------------------------------------------------------------------------------------------------------------------------------------------------

In this step, we will set up a beacon node. Use the following command to start a beacon node that connects to the execution node:

### [Staking](https://lighthouse-book.sigmaprime.io/run_a_node.html#staking)

```

`lighthouse bn\
  --network mainnet\
  --execution-endpoint http://localhost:8551\
  --execution-jwt /secrets/jwt.hex\
  --checkpoint-sync-url https://mainnet.checkpoint.sigp.io\
  --http
`
```

