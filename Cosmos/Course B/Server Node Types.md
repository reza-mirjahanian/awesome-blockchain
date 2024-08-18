https://github.com/cosmos/chain-registry

Node Types in the Cosmos Network
================================

The Cosmos network utilizes three main types of nodes, all running on the same daemon:

1.  **Validators**
2.  **Full Nodes**
3.  **Archive Nodes**

1\. Validator Nodes
-------------------

-   **Primary Function**: Sign blocks
-   **Key Responsibilities**:
    -   Submit signatures
    -   Verify transactions
    -   Prevent double-signing
-   **Requirements**:
    -   Runs automatically
    -   Requires sufficient vote power (derived from stakers)
    -   Needs at least 100,000 ATOM to be in the validator set

2\. Full Nodes
--------------

-   **Most Common Node Type**
-   **Features**:
    -   Listed on cosmos.directory and GitHub's chain registry
    -   Provide RPC, REST, and gRPC requests
    -   Often publicly accessible (may be rate-limited)
-   **Uses**:
    -   CLI checks
    -   Website access to the network
    -   Relayers
-   **Data Retention**: Typically stores 30-90 days of data

3\. Archive Nodes
-----------------

-   **Purpose**: Store and provide access to historical data
-   **Characteristics**:
    -   Heavy nodes that index and keep all past network data
    -   Essential for syncing from genesis and accessing old data
-   **Use Cases**:
    -   Historical analysis
    -   Tax-related queries
    -   Accessing data from legacy blocks

Node Information and Access
---------------------------

-   **Cosmos Directory**:
    -   Lists available nodes
    -   Provides information on node status, errors, and current height
-   **RPC Interface**:
    -   Allows querying of basic data (version, last block height, hashes)
-   **Archive Data Access**:
    -   Available at specific URLs (e.g., archive.chain.love for Cosmos Hub)
    -   Large file sizes, even when compressed

Best Practices for Node Usage
-----------------------------

1.  **Round-robin between multiple nodes** to avoid rate limiting
2.  Use **full nodes for recent data** and general queries
3.  Utilize **archive nodes for historical data** beyond the retention period of full nodes
4.  Be aware of the **differences in data availability** between node types
5.  **Check alternative providers** if a node becomes unavailable
6.  


