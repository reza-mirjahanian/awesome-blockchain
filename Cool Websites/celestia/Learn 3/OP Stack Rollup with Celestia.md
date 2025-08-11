Introduction
------------

-   **Objective**: Learn to run an OP Stack rollup using Celestia.
-   **Versions**:
    -   **Devnet**: Posts to Celestia's Mocha testnet.
    -   **Testnet**: Uses Sepolia testnet for Ethereum.

Tools and Integrations
----------------------

-   **Roll-op**: Fork by 0xFableOrg on celestiaorg repo.
-   **op-plasma-celestia**: Alt-DA integration for OP Stack.

Setup and Dependencies
----------------------

-   **Dependencies**:
    -   `celestia-node` (light node must be fully synced and funded).
    -   **Functionality**: Submit and retrieve PayForBlobs to/from Mocha testnet.
-   **Default Configuration**: Uses node store in default directory, account named `my_celes_key`.

Initial Steps
-------------

1.  **Check README**:

    -   **Environment**: Python virtual environment on Mac OS.
    -   **Overview Section**: Recommended to read before starting.
2.  **Clone and Set Up**:

    -   Clone `roll-op`.
    -   Set up virtual environment.

Running the Devnet
------------------

1.  **Run Rollop Setup**:

    -   Install Foundry (takes ~5 minutes on M1 Mac with 16GB RAM).
2.  **Start Devnet**:

    -   Uses mock L1 for EVM, runs L2 rollup, Alt DA mode with Celestia node.
    -   **Commands**: Start devnet, overwrite existing data.
    -   **Monitoring**: Check logs for L1 at port 8545, Celestia light node activity.
3.  **Check Activity**:

    -   Use Celenium to view account activity.
    -   **Outcome**: Devnet running with OP Stack and Celestia.

Transition to Testnet
---------------------

1.  **Close Processes**:

    -   Stop running devnet processes.
2.  **Deploy to Testnet**:

    -   **Configuration**: Update config file for Sepolia Ethereum.
    -   **Private Key**: Required with Sepolia ETH.
3.  **Deploy Slowly**:

    -   Set `deploy_slowly` to true to avoid RPC issues.
    -   **Start Testnet**: Overwrite existing versions, start light node.
4.  **Monitor Deployment**:

    -   **Duration**: ~30 minutes for full deployment.
    -   **Logs**: Check Plasma DA server logs.
5.  **Verify Contracts**:

    -   Use Etherscan to verify contract deployments.

Final Steps and Troubleshooting
-------------------------------

1.  **Check L2 Components**:

    -   Ensure all L2 components are running.
    -   **Activity**: Monitor Celestia for blob submissions.
2.  **Resources**:

    -   **Optimism Docs**: For Alt DA mode chain configuration.
    -   **Discord and GitHub**: Reach out for questions, report bugs, or make PRs.
3.  **Conclusion**:

    -   Successful setup indicated by blob submissions on Celestia.
    -   Encouragement to engage with community resources for support.