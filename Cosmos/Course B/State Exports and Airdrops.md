https://github.com/Reecepbcups/airdrop-tools


Token Management and Airdrops
-----------------------------

### **Distributing Tokens**

-   **Objective**: As a developer, you may want to distribute tokens to community members based on their activity, token holdings, or stake with validators.
-   **Challenges**: Obtaining the necessary data can be difficult, requiring specialized scripts and tools.

### **CW20 Token Data Extraction**

-   **Purpose**: Extract and manipulate data from CW20 tokens, which are similar to ERC-20 tokens but within the Cosmos ecosystem.
-   **Process**:
    -   **Pagination**: Due to gas limits, large data queries (e.g., over 300,000 gas) must be paginated.
    -   **Script Workflow**:
        -   Query smart contract state.
        -   Iterate through and save data to files.
        -   Merge data using a Python script for further processing.

### **Native Token Conversion and Airdrops**

-   **Use Case**: Convert and distribute tokens from one format to another, such as airdropping tokens to CW20 holders or migrating token data when launching a new chain.
-   **Tools**:
    -   **Scripts**: Bash scripts to handle the data collection and conversion.
    -   **Python**: Used to manage balances and automate transaction creation.

* * * *

NFT Management
--------------

### **CW721 NFT Data Extraction**

-   **Equivalent**: CW721 tokens are equivalent to ERC-721 tokens (NFTs) in the Cosmos ecosystem.
-   **Use Cases**:
    -   **Airdrops**: Distribute new NFTs to holders of existing collections.
    -   **Whitelist Creation**: Generate a list of eligible addresses for future NFT drops or other rewards.
-   **Process**:
    -   **Snapshot**: Capture a snapshot of the NFT collection at a specific time, detailing holder addresses and owned NFT IDs.
    -   **Script Execution**: Use REST APIs to query and save NFT data in a structured format.

* * * *

Exporting Blockchain Data
-------------------------

### **Public Node Data Export**

-   **Purpose**: Export data from a blockchain node at a specific block height to analyze or distribute tokens.
-   **Challenges**:
    -   **Validator Downtime**: Exporting data requires the node to stop signing blocks, which may cause disruptions.
    -   **Infrastructure Costs**: Setting up the required infrastructure can be costly (~$100/month).
    -   **Process**:
        -   Sync a node and perform a data export at a specific height.
        -   Use a reverse proxy to manage requests and backups.

### **Automated Export Tools**

-   **Daily Exports**:
    -   **Functionality**: Automates data exports at a specific time each day, such as 3-4 AM.
    -   **Data Retention**: Maintains archived data from previous block heights, useful for long-term analysis and airdrop planning.

### **Data Types Exported**

-   **Examples**:
    -   **Bank Data**: Token supply, balances.
    -   **Governance Data**: Proposals, votes, delegation details.
    -   **Validator Data**: Delegations, slashing events, validator shares.

### **API Access**

-   **Export API**: Open-source API that allows developers to access exported data easily.
-   **Data Queries**:
    -   **Validators**: Check all delegations to a specific validator.
    -   **Supply**: Retrieve the total supply of a token at a specific block height.

* * * *

Data Retrieval and Utility Scripts
----------------------------------

### **Handling JSON Data**

-   **Output**: Data is often exported in JSON format for easy integration with other tools or custom scripts.
-   **Examples**:
    -   **Validator Delegations**: Iterate through JSON data to analyze delegations and their corresponding validators.
    -   **Notification Systems**: Use JSON data to build systems that notify users about changes or important events (e.g., NFT drops, validator updates).

### **Custom Tools and Scripts**

-   **Use Cases**: Developers may need to run their own nodes and scripts to access data not publicly available.
-   **Importance of Archiving**: Archive data regularly to ensure it is available when needed, particularly for long-term projects.