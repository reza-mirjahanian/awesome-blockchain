-   The Issue with Rollups:

    -   Before the Dencun upgrade, rollups had to submit large amounts of call data (compressed transaction batches) to Ethereum.

    -   This data was stored permanently on every node, incurring high gas costs due to storage and computation requirements.

    -   Problem: Ethereum only needs the batch data temporarily to verify its validity, but nodes were forced to store it forever.


    
Blob Transactions and Ethereum Scaling


-   Definition: Blob transactions (Type 03) are a new transaction type introduced in Ethereum's Dencun upgrade on March 13, 2024.

-   Purpose: Allow temporary storage of data on-chain, which is deleted after a short period (20--90 days), unlike regular transactions where data is stored permanently.

-   Analogy: Think of a blob as a sidecar on a motorcycle (the transaction). The sidecar carries temporary data, which is later discarded.

-   Key Benefits:
    -   Reduces storage costs for rollups (Layer 2 solutions).

    -   Addresses Ethereum's high gas fees for data storage.

    -   Enables scalable transaction processing by minimizing permanent on-chain data.

The Problem: Ethereum's Scalability and Cost Issues

-   Blockchain Trilemma: Balancing scalability, security, and decentralization remains a challenge for Ethereum.

-   High Gas Fees:
    -   Sending $1 on Ethereum's main chain can cost $2 or more due to high gas costs.

    -   Gas costs are driven by the need for every Ethereum node to store transaction data permanently.

-   Permanent Data Storage:
    -   Before blob transactions, all transaction data, including compressed batches from rollups, was stored on every Ethereum node forever.

    -   This was inefficient since much of the data (e.g., rollup batch verification data) was only needed temporarily.

-   Example:
    -   If you bought a shitcoin that went to zero, that transaction is visible on-chain forever.

    -   Similarly, rollups had to pay high gas fees to store compressed transaction batches that were only needed for validation.

The Solution: Rollups and Blob Transactions

-   What Are Rollups?
    -   Rollups are Layer 2 (L2) solutions designed to scale Ethereum by processing transactions off-chain and submitting compressed batches to the Ethereum main chain (Layer 1).

    -   Examples: ZKSync, Arbitrum, Optimism.

    -   How They Work:
        1.  Execute transactions on their own chain.

        2.  Compress transactions into a batch.

        3.  Submit the batch to Ethereum for verification.

    -   Benefit: Significantly reduces transaction costs (e.g., sending $1 on an L2 is much cheaper than on Ethereum's main chain).

-   The Issue with Rollups:
    -   Before the Dencun upgrade, rollups had to submit large amounts of call data (compressed transaction batches) to Ethereum.

    -   This data was stored permanently on every node, incurring high gas costs due to storage and computation requirements.

    -   Problem: Ethereum only needs the batch data temporarily to verify its validity, but nodes were forced to store it forever.

-   Blob Transactions to the Rescue:
    -   Introduced via EIP-4844 (Proto-Danksharding), named after the researchers who proposed it.

    -   Allow rollups to submit temporary data in a blob (Binary Large Object) that is deleted after verification.

    -   Outcome: Reduces gas costs for rollups, making L2 transactions cheaper and more efficient.

How Blob Transactions Work

-   Blob Characteristics:
    -   Blobs are a temporary storage mechanism for large data chunks (e.g., rollup transaction batches).

    -   Stored on-chain for a short period (20--90 days) before being deleted.

    -   Not accessible directly by Ethereum smart contracts, but their hash can be used for verification.

-   Technical Implementation:
    -   EIP-4844 (Proto-Danksharding):
        -   Introduces Type 03 transactions for blobs.

        -   Adds a new blob hash opcode and a point evaluation precompile for cryptographic verification.

    -   Blob Hash Opcode:
        -   Generates a cryptographic hash of the blob data without requiring the full blob to be stored on-chain.

    -   Point Evaluation Precompile:
        -   Performs mathematical cryptography to verify the blob's validity using the blob hash and off-chain cryptographic proofs.

-   Example Workflow (e.g., ZKSync submitting a batch):
    1.  ZKSync processes transactions on its L2 chain.

    2.  Compresses transactions into a batch and submits it as a blob to Ethereum's L1.

    3.  Ethereum smart contracts use the blob hash opcode to access the blob's hash.

    4.  The point evaluation precompile verifies the blob's validity using cryptographic proofs (e.g., Pub data commitments).

    5.  Once verified, the blob is marked for deletion, reducing storage costs.

-   Code Example: Verifying a Blob Transaction
    solidity

    ```
    // Example function to verify blob informationfunctionverifyBlobInformation(bytescalldata pubDataCommitments,bytes32[]calldata blobHashes
    )internalreturns(bool){// Call the point evaluation precompile to verify the blobreturnpointPrecompile(pubDataCommitments, blobHashes);}
    // Pseudo-code for ZKSync's commitBatches functionfunctioncommitBatches(bytescalldata batchData,bytes32[]calldata blobHashes,bytescalldata proofs
    )external{// Verify blob data using blob hash opcode and point evaluation precompilerequire(verifyBlobInformation(proofs, blobHashes),"Invalid blob data");// Proceed with batch processing}
    ```

-   Etherscan Integration:
    -   Etherscan displays blob data under a new section called Total Block Blobs.

    -   Shows blob gas used vs. call data gas, highlighting significant cost savings (e.g., blobs are much cheaper than storing call data permanently).

Blob Gas Market and Implications

-   Blob Gas Market:
    -   Blob transactions introduce a new type of gas market specifically for blobs.

    -   Blob gas fees are separate from regular gas fees, creating a dynamic pricing model for temporary data storage.

    -   Impact: Encourages efficient use of blob storage and reduces costs for rollups.

-   Downstream Effects:
    -   Lower costs for L2 solutions make Ethereum more accessible to users.

    -   Supports the long-term vision of danksharding, a future Ethereum upgrade for even greater scalability.

    -   Potential for new use cases, such as temporary data storage for decentralized applications (dApps).

Sending Your Own Blob Transaction

-   Prerequisites:
    -   Set up a connection to an Ethereum node (e.g., using Anvil for local testing).

    -   Use a programming language like Python with libraries such as Web3.py or Ry.

-   Steps to Send a Blob Transaction:
    1.  Prepare Blob Data:

        -   Blobs must be at least 4,096 words (32-byte chunks).

        -   Pad smaller data with zeros to meet the size requirement.

        python

        ```
        # Example: Preparing blob dataencoded_text ="Sample blob data".encode()blob_data = encoded_text +b"\x00"*(4096*32-len(encoded_text))# Pad with zeros
        ```

    2.  Create a Type 03 Transaction:

        -   Specify transaction type as 0x03 (blob transaction).

        -   Include blob gas fee parameters (e.g., maxFeePerBlobGas).

        python

        ```
        transaction ={"type":0x03,# Blob transaction"to":"0xRecipientAddress","value":0,"data":"0x","maxFeePerGas":1000000000,"maxPriorityFeePerGas":1000000000,"maxFeePerBlobGas":100000000,# Blob-specific gas fee"chainId":1}
        ```

    3.  Attach Blob Data:

        -   Add the blob data to a dedicated blob compartment in the transaction.

        python

        ```
        transaction["blobs"]=[blob_data]
        ```

    4.  Estimate Gas and Sign:

        -   Estimate gas for the transaction.

        -   Sign the transaction with your private key.

        python

        ```
        gas_estimate = web3.eth.estimate_gas(transaction)transaction["gas"]= gas_estimate
        signed_tx = web3.eth.account.sign_transaction(transaction, private_key)
        ```

    5.  Send the Transaction:

        -   Send the signed transaction to the Ethereum network.

        python

        ```
        tx_hash = web3.eth.send_raw_transaction(signed_tx.rawTransaction)tx_receipt = web3.eth.wait_for_transaction_receipt(tx_hash)print(f"Transaction receipt: {tx_receipt}")
        ```

-   Running Locally:
    -   Use Anvil to set up a local Ethereum blockchain.

    -   Run the script with a command like ry run send\_blob.

    -   Check the transaction receipt and blob data in the output.

Proto-Danksharding and the Future

-   What is Proto-Danksharding?:
    -   EIP-4844 (Proto-Danksharding) is an intermediate step in Ethereum's scaling roadmap.

    -   Lays the groundwork for danksharding, a future upgrade that will further enhance scalability with features like:
        -   Increased blob capacity.

        -   More efficient data availability sampling.

        -   Enhanced cryptographic proofs for faster verification.

-   Why Proto-Danksharding Now?:
    -   Rollups are critical to Ethereum's scaling strategy today.

    -   High gas costs for call data were a bottleneck for rollups.

    -   Proto-Danksharding reduces costs immediately, making rollups more viable.

-   Ethereum Docs:
    -   The Ethereum documentation provides detailed explanations of danksharding and its future implications.

    -   Key resource for understanding Ethereum's long-term scaling vision.

Practical Use Case: ZKSync Example

-   Scenario:
    -   ZKSync submits a compressed batch of transactions to Ethereum's L1 as a blob.

    -   The blob contains a massive chunk of data (e.g., thousands of transactions).

    -   Ethereum verifies the batch using the blob hash opcode and point evaluation precompile.

-   Etherscan Visualization:
    -   Navigate to a ZKSync transaction on Etherscan.

    -   Under Total Block Blobs, view the blob data and gas savings.

    -   Example: A blob transaction might use 10x less gas than equivalent call data.

-   Technical Details:
    -   ZKSync's smart contract calls the commitBatches function, passing:
        -   Batch data: Compressed transaction data.

        -   Blob hashes: Hashes of the blob data.

        -   Proofs: Cryptographic proofs for verification.

    -   The point evaluation precompile ensures the blob is valid without storing it permanently.

Key Takeaways

-   Blob Transactions:
    -   Enable temporary data storage for rollups, reducing gas costs.

    -   Introduced in the Dencun upgrade (March 13, 2024) via EIP-4844.

    -   Use new opcodes and precompiles for efficient verification.

-   Impact on Rollups:
    -   Make L2 solutions like ZKSync, Arbitrum, and Optimism more cost-effective.

    -   Support Ethereum's scaling strategy for the next several years.

-   Future Outlook:
    -   Proto-Danksharding is a stepping stone to full danksharding.

    -   Expect further improvements in scalability, data availability, and cost efficiency.

-   Try It Yourself:
    -   Check the GitHub repository for a sample blob transaction script (link in description).

    -   Experiment with sending blob transactions on a local blockchain using Anvil and Ry.

Additional Resources

-   Ethereum Documentation: Detailed guides on danksharding and blob transactions.

-   Etherscan: Explore blob transactions and gas savings in real-time.

-   GitHub Repo: Example code for sending blob transactions (link in description).