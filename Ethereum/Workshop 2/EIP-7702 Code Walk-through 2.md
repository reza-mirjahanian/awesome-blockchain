### 🔧 **Core Components of revm**
*   **`EVM` Struct**: The central data structure.
    *   **`context`**: Manages all state-related and environment-related data.
    *   **`handler`**: Handles all logic execution and operational instructions.

### 📋 **Transaction Lifecycle: The `transact` Method**
*   The primary method for handling and executing incoming transactions.
*   We follow its flow to identify changes introduced by **EIP 7702**.

---

### 1. **Pre-Validation: `pre_verify_transaction_inner`**
*   **Purpose**: Performs initial checks and validations on the transaction before execution.
*   **Key Steps**:
    *   **`validate_env`**: Validates the transaction environment.
        *   For **EIP 7702 transactions** (a new type), specific validation occurs.
        *   **Check 1**: ✅ Ensures the network is *post-Prague* upgrade (the upgrade that introduces EIP 7702). *Pre-Prague, this transaction type is rejected.*
        *   **Check 2**: ✅ Validates the `authorization_list` is present and contains *at least one authorization*.
    *   **`validate_initial_transaction_gas`**: Calculates upfront gas cost. *(No EIP 7702-specific logic)*.
    *   **`validate_transaction_against_state`**: Validates the transaction sender (caller) against the current state.
        *   **Critical Change**: Historically (EIP 3607), a transaction sender *must* be an Externally Owned Account (EOA), identified by *empty* `code_hash`.
        *   **EIP 7702 Update**: An EOA can now have non-empty code, but *only* if it's the specific **Delegation Designator** code format.
        *   This method now checks that the caller's code is either:
            *   Empty (traditional EOA), *or*
            *   A valid **EIP 7702 Delegation Designator**.

---

### 🧬 **The EIP 7702 Delegation Designator**
*   **Purpose**: A special code stored in an EOA's `code` field to identify it as a "smart account" and specify its delegate.
*   **Structure (`Eip7702Code` struct)**:
    *   `delegated_address`: The address of the smart contract wallet the EOA delegates to.
    *   `version`: A version number.
    *   `raw_bytes`: The full bytecode constructed as:
        *   **Magic Prefix**: `0xef01`
        *   **Version**: `0x00` (initial version)
        *   **Address**: The 20-byte `delegated_address`
        *   **Total Length**: *Exactly 23 bytes*.
*   **Validation**: The `is_eip_7702_code` check ensures any non-empty EOA code conforms to this precise structure.

---

### 2. **Applying Authorizations: `apply_eip_7702_auth_list`**
*   **When**: After pre-validation, before main execution.
*   **What**: Processes the `authorization_list` from the EIP 7702 transaction.
    *   Each `Authorization` contains:
        *   `authority`: The EOA address owner who signed the upgrade permission.
        *   `delegate`: The smart contract address to delegate to.
        *   `nonce`: The EOA's nonce at the time of signing.
        *   `chain_id`: The chain the authorization is intended for.
*   **Process**: Loop through each authorization in the list:
    *   **Validation Checks**:
        *   ✅ `chain_id` must be 0 or match the current chain ID.
        *   ✅ `nonce` must not be `u64::MAX` (to prevent overflow on increment).
        *   ✅ `authority` must be `Some(address)` (proving signature was valid and verified *before* EVM execution).
        *   ✅ Load the EOA's account state from the DB.
        *   ✅ Verify the EOA's current code is *either empty or an existing EIP 7702 code*.
        *   ✅ The authorization `nonce` must match the EOA's current `nonce`.
    *   **Gas Cost Logic**:
        *   If the EOA's code is *empty* (first-time upgrade): Charges **25,000 gas**.
        *   If the EOA's code is *already* an EIP 7702 code (re-upgrade): Charges **12,500 gas** and increments a `refunded_accounts` counter for a potential gas refund.
    *   **State Change**:
        *   Constructs the new **Delegation Designator** bytecode for the EOA.
        *   **Sets this new code to the EOA's account state**.
        *   **Increments the EOA's nonce**.
        *   ⚡ **Crucially, this state change is *NOT* atomic with the rest of the transaction execution**. It persists even if the transaction later reverts.

---

### 3. **Main Execution & Call Handling**
*   **Standard Execution**: Proceeds normally, creating call frames and a call stack.
*   **The Key EIP 7702 Adjustment**: In frame creation, when preparing to execute the code of the called address:
    *   **Standard Logic**: Load the `code` from the account at the `to` address.
    *   **EIP 7702 Logic**:
        *   ✅ Check if the loaded `code` is of type `Eip7702Code`.
        *   If yes, this is a call to a "smart account".
        *   **Instead of executing the short Delegation Designator code**, the EVM:
            *   Retrieves the `delegated_address` from the `Eip7702Code`.
            *   Loads the *actual runtime bytecode* from the account at the `delegated_address`.
            *   Executes that smart contract code instead.
*   **Result**: A transaction sent to an EOA's address transparently executes the logic of its delegated smart contract, enabling smart account functionality without changing the outer transaction format.