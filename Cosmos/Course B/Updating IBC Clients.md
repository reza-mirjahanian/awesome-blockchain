### Updating IBC Clients

  

-   **Introduction**:
    -   Discussion about maintaining the sync of the light client with the blockchains it interacts with.
    -   Relayers submit 'update clients' to keep the IBC client updated.

  

### Understanding the Update Process

  

-   **Generic Level**:
    -   Update client calls are made with a header, which contains minimal information (client type, height, validation method).
-   **Tendermint-specific Process**:
    -   The header includes a signed header from Tendermint, full validator set, trusted height, and validators.
    -   Important features include the root hash, validator signatures, and consensus-state hash validation.

  

### Key Components Explained

  

-   **Signed Header**: Reflects a combination of the blockchain's header and a commitment signed by validators.
-   **Validator Set**: Essential for verifying the legitimacy of the new block being added to the light client.
-   **Trusted Height & Validators**: Indicates the point of trust used for validation of new headers.

  

### Mechanism of Updating

  

-   Receiving the consensus state from a trusted height and verifying new headers against this state.
-   Ensuring the validators' set given by the relayer hashes correctly with the stored consensus state.
-   Validating that the new header received has been signed with a trust level as defined by the system.

  

### Concerns Addressed

  

-   **Storing Validator Sets**: Only the hash of the validator set is stored on-chain to keep the state size minimal.
-   **Handling Chain Upgrades**: Introduction of revision numbers allows IBC clients to handle blockchain upgrades smoothly, ensuring continuity.

  

### Implications and Queries

  

-   **Relayer Duties**: Need for relayers to construct and submit messages containing necessary validation data.
-   **Client Expiration**: Clients have a 'trusting period' within which updates must occur to prevent expiration.


  
