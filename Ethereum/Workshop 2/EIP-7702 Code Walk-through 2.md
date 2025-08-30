https://github.com/bluealloy/revm

### **EIP-7702: A Deep Dive into the EVM Implementation (revm)** 💻

* **Goal:** To trace a transaction through an EVM implementation (revm - Rust EVM) and see exactly where and how EIP-7702 changes the execution flow.
* **Structure of revm:**
    * `evm` (struct): The main data structure.
        * `context`: Manages state and environment.
        * `handler`: Manages logic and execution.
    * `transact` (method): The entry point for processing an incoming transaction.

---

### **Phase 1: Transaction Pre-Verification** 🛡️

* ***`pre_verify_transaction_inner`***: The first major step, responsible for initial validation checks *before* execution.
* **Step 1.1: Validate Environment (`validate_env`)**
    * This is the first place we see a specific check for the new transaction type.
    * **Conditional Logic:** `if transaction_type == EIP7702`
        * ✅ **Pectra Fork Check:** It first verifies that the EVM is running on a post-Pectra hard fork. If not, this transaction type is invalid and an error is thrown.
        * gas & chain ID checks (similar to other transaction types).
        * ✅ **Authorization List Check:** It specifically verifies that the `authorization_list` included in the transaction is *not empty* (i.e., it contains at least one authorization), as required by the EIP specification.

---

### **Phase 1: Pre-Verification (Continued)** 🛡️

* **Step 1.2: Validate Against State (`validate_transaction_against_state`)**
    * **The EIP-3607 Problem:** An older EIP (3607) prevents smart contracts from initiating transactions to prevent certain security risks. It does this by checking if the sender's account has empty bytecode.
    * **EIP-7702's Exception:** EIP-7702 *allows* an EOA to have non-empty bytecode (the Delegation Designator).
    * **The New Logic:**
        * The check is now updated to: *Reject the transaction if the sender's code is NOT empty **UNLESS** that code is a valid EIP-7702 Delegation Designator*.
        * This ensures that only standard EOAs or EIP-7702 upgraded EOAs can start a transaction.

---

### **Anatomy of the Delegation Designator Bytecode** 🔬

* ***`struct EIP7702Bytecode`***: A data structure representing the new bytecode format.
* **Composition:**
    * `delegated_address`: The address of the smart contract wallet being pointed to.
    * `version`: The version of the designator format.
    * `raw_bytes`: The complete bytecode string.
* **Construction (`new` method):**
    * Starts with the "magic bytes" prefix: `0xEF01`.
    * Appends the version number: `0x00`.
    * Appends the 20-byte `delegated_address`.
* **Validation:** The total length of the `raw_bytes` must be exactly **23 bytes** (2 for prefix + 1 for version + 20 for address).

---

### **Phase 2: Applying the Authorizations** ✨

* ***`apply_eip7702_auth_list`***: This is the core state-changing logic that happens *after* pre-verification but *before* the main execution.
* **The Process:** The EVM loops through every single authorization in the transaction's `authorization_list`.
* **For each authorization, it performs a series of checks:**
    1.  **Chain ID:** Must match the current chain's ID (or be zero).
    2.  **Nonce:** Cannot be `u64::MAX` to prevent overflow issues when incremented.
    3.  **Signature:** Verifies the signature is valid and recovers the signer's address (the `authority`). This actually happens at the node level before the EVM, but is checked here.
    4.  **Existing Code:** Loads the `authority`'s account and checks that its code is either empty or already an EIP-7702 designator. It will *not* overwrite a standard smart contract.
    5.  **Account Nonce:** The nonce in the authorization must match the current nonce of the `authority`'s account.

---

### **Phase 2: Gas Costing & The State Change** 💰

* **Interesting Gas Logic:** EIP-7702 has different gas costs for a first-time upgrade vs. a re-upgrade.
    * **First-time upgrade:** `25,000 gas`.
    * **Re-upgrade** (changing the delegated address): `12,500 gas`.
    * The code keeps a `refunded_accounts_counter` to calculate gas refunds if an account in the list was already upgraded.
* **The Upgrade:** This is the most important part.
    1.  A new **Delegation Designator** bytecode is constructed using the `delegated_address` from the authorization.
    2.  The hash of this new bytecode is calculated.
    3.  The `code` and `code_hash` of the `authority`'s account state are **set to these new values**.
    4.  The `authority`'s account nonce is incremented.
* **Crucial Rule:** These state changes are **permanent for the block**, even if the main transaction execution reverts later.

---

### **Phase 3: Main Transaction Execution** ⚙️

* **The Call Stack:** The EVM begins its normal execution loop, creating frames for each call in the call stack.
* **The `Frame`:** Each frame contains an `Interpreter` which is responsible for executing the bytecode of the contract being called.
* **The EIP-7702 Interception:** The key change happens during frame creation.
    * **Standard Flow:** When calling a contract, the EVM loads the account, takes its `bytecode`, and passes it to the interpreter.
    * **EIP-7702 Flow:**
        1.  The EVM loads the account being called (the `to` address in the transaction).
        2.  It checks if the account's `bytecode` is an EIP-7702 Delegation Designator.
        3.  **If it is:**
            * It extracts the `delegated_address` from the designator.
            * It **loads the account of the `delegated_address`** from the database.
            * It takes the `bytecode` from *that* account.
            * It passes this *delegated bytecode* to the new interpreter for execution.
* **Result:** The EVM executes the logic of the smart contract wallet, but within the context of the user's EOA.

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