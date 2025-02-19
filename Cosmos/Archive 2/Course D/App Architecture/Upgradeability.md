**Key Points:**

-   **Upgrade Challenges in Traditional Blockchains:**

    -   In most blockchains, upgrading the software on each node is necessary when changes are implemented.
    -   This process is voluntary and can lead to a **hard fork** if participants disagree, resulting in two separate chains: one with the old rules and one with the new rules.
    -   While hard forks can have benefits, they introduce **uncertainty**, which is problematic in contexts requiring **transaction finality** and **authoritative registries**.
-   **Upgrade Mechanism in CometBFT Blockchain:**

    -   **Transaction Finality:** In CometBFT, transactions are finalized irreversibly upon block creation, ensuring certainty.
    -   **Governance of Upgrades:** Upgrades are integrated into the block creation and validation process, meaning nodes must agree simultaneously for an upgrade to occur. If there's no consensus, the upgrade proposal is rejected.
-   **Voting Process:**

    -   **Validators and Delegators:** Upgrades are decided through a voting process where validators and delegators participate.
    -   **Voting Weight:** Votes are weighted based on the stake each party holds.
    -   **Delegator's Role:** If a delegator does not actively vote, their vote defaults to match that of their chosen validator. Hence, delegators should be careful when they act, as their vote has significant influence.