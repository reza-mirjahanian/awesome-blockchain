**Engine API**

The **Engine API** lives in the **Execution Layer** and is used exclusively for internal communication between the **Consensus Layer** and the **Execution Layer (EL)**. The engine exposes two major classes of endpoints to the consensus layer: **fork choice updated** and **new payload** suffixed by the three versions they are exposed as (V1-V3).

1.  **New Payload (V1/V2/V3):**
    Handles payload validation and insertion. When the CL receives a new beacon block, it extracts the execution payload and calls `engine_newPayload` on the EL. The EL validates the payload by:

    -   Checking that the parent block hash in the payload header exists and matches the expected parent in the local chain.
    -   Verifying any additional execution commitments (e.g. post-Cancun data).
    -   Executing transactions and updating the state.

    The response includes a status:

    -   **VALID:** Fully executed and correct.
    -   **INVALID:** Payload or an ancestor fails validation.
    -   **SYNCING:** EL is still catching up (e.g. missing blocks).
    -   **ACCEPTED:** Basic checks pass but full execution is pending (common in shallow-state clients).
2.  **Fork Choice Updated (V1/V2/V3):**
    Manages state synchronization and triggers block building. The CL sends a fork choice update (with head, safe, and finalized block hashes) and may include payload attributes if it is selected to propose a block. The EL will:

    -   Updates its canonical head.
    -   Initiates payload building if payload attributes are provided.
    -   Returns a response with a status and, if building, a `payloadId`. The status indicates the EL's current ability to process the fork choice update and (if applicable) begin building a block.

Possible status returned to the CL:

-   **VALID:** The fork choice update was processed successfully, and the EL's view of the chain is up to date.
-   **INVALID:** The provided fork choice references an invalid block or chain segment.
-   **SYNCING:** The EL is still catching up (e.g., missing blocks or state required to evaluate the fork choice).
-   **ACCEPTED:** The fork choice update is accepted provisionally, but full validation is pending. This may occur when the EL has shallow state or incomplete history for non-canonical forks.