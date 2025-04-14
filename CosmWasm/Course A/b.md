
### **II. Core CosmWasm Concepts & Lifecycle**

4.  **Q: What are the two initial, most crucial steps mentioned in the contract lifecycle for getting code onto the chain?**
    *   **A:** **Storing** and **Instantiating**.

5.  **Q: Describe the "Storing" process in the CosmWasm contract lifecycle.**
    *   **A:** Storing is the process of **uploading the contract's compiled Wasm bytecode** to the blockchain. Upon successful upload, the chain returns a **numeric identifier (Code ID)** for that specific bytecode. (On a fresh local chain, this ID will typically be 1).

6.  **Q: Describe the "Instantiating" process. What does it build upon, and what does it require?**
    *   **A:** Instantiating **builds upon a stored contract's bytecode**. It requires the **Code ID** (to identify *which* bytecode to use) and an **instantiation message** (typically JSON). This message provides initial parameters or configuration, similar to a constructor in OOP, creating a *specific running instance* of the contract on the chain. Each instance gets its own unique contract address.

7.  **Q: What analogy was used to explain the relationship between stored bytecode and instantiated contracts?**
    *   **A:** The bytecode was compared to a **blueprint** (like a class in OOP), and each instantiation is like creating a specific **object** or **implementation** based on that blueprint.8.  **Q: Can the same stored bytecode (Code ID) be instantiated multiple times? Give an example.**
    *   **A:** Yes. The same bytecode can be instantiated many times, creating independent contract instances. A prime example given was **CW20 tokens** on Juno, where many different token contracts point back to the same underlying CW20 bytecode (Code ID 1 on Juno mainnet). Another example was a CW721 (NFT) contract being instantiated multiple times for different collections or mints.

9.  **Q: Besides Storing and Instantiating, what other key lifecycle step was mentioned but not covered in detail in this episode? What is its purpose?**
    *   **A:** **Migration**. This is the process of updating a *running* contract instance to use a *new version* of the bytecode (a different Code ID), potentially fixing bugs or adding features.

10. **Q: What mechanism is required to allow a contract to be migrated?**
    *   **A:** A contract must have an **Admin** address set during instantiation. Only this admin can authorize a migration.

11. **Q: What are "Entry Points" in the context of a CosmWasm contract?**
    *   **A:** Entry points are the defined ways users or other contracts can **interact with** an instantiated contract. They handle incoming messages and trigger the contract's logic.

12. **Q: List the main entry points mentioned in the discussion.**
    *   **A:**
        *   `Instantiate` (Covered via the instantiation process)
        *   `Execute` (For changing state)
        *   `Query` (For reading state)
        *   `Migrate` (For updating code, requires admin)
        *   `Sudo` (For privileged actions, often by governance)
        *   `Reply` (For handling responses from sub-messages, not covered)

13. **Q: Which two entry points are the most common for typical user interactions and were the focus of the practical examples?**
    *   **A:** **Execute** and **Query**.

14. **Q: How was the role of Execute and Query related to contract state?**
    *   **A:**
        *   **Execute:** Corresponds to **writing** or *changing* the contract's state.        *   **Query:** Corresponds to **reading** the contract's *current* state without changing it.

15. **Q: What is "State" in the context of a smart contract? What helper library makes managing state easier in modern CosmWasm?**
    *   **A:** State represents the **data stored** by the contract instance on the blockchain (like data in a database). **`cw-storage-plus`** is a library that abstracts away low-level byte manipulation, allowing developers to store and retrieve complex Rust structs more easily, similar to working with a NoSQL database driver.

16. **Q: Why is determinism crucial in smart contract execution? What example was given of non-determinism causing a problem?**
    *   **A:** Determinism ensures that every validator executing the same transaction on the same state reaches the exact same result, which is essential for **maintaining consensus**. The example given was the **Juno chain halt**, caused by a malicious contract (based on `cw-template`) that read node-specific information (non-deterministic) and saved it to state. When queried, each node returned a different result based on its own saved state, breaking consensus and causing forks.

### **III. Development Workflow & Tooling (Episode 2 Practical)**

17. **Q: What template repository was used to create the example contract project?**
    *   **A:** The **InterWasm `cw-template`** (`github.com/InterWasm/cw-template`).

18. **Q: What command is used with `cargo` to generate a new project from this template?**
    *   **A:** `cargo generate --git <repo_url> --name <project_name>` (using the `cargo-generate` subcommand).

19. **Q: What two `cargo install` commands were mentioned as prerequisites for using `cw-template` and its build process?**
    *   **A:**
        1.  `cargo install cargo-generate --features vendored-openssl`
        2.  `cargo install cargo-run-script`

20. **Q: What `cargo` command is useful for generating JSON schema files from the contract's message definitions?**
    *   **A:** `cargo schema`.

21. **Q: What is `cargo clippy`, and why is it relevant to the `cw-template`?**
    *   **A:** `cargo clippy` is a collection of **linters** for Rust code, enforcing best practices and style, often in a very opinionated way. The `cw-template` comes with pre-configured GitHub Actions CI that runs `clippy`. If your code doesn't pass `clippy` (or `cargo schema` isn't up-to-date), the CI build will fail.

22. **Q: What command was suggested to run `clippy` locally, including checking test code?**
    *   **A:** `cargo clippy --all-targets -- -D warnings` (The `-D warnings` turns warnings into errors, mimicking strict CI).

23. **Q: Why is compiling a contract with just `cargo wasm` often insufficient for deploying to a chain?**
    *   **A:** `cargo wasm` typically produces a **large, unoptimized** Wasm binary (potentially several megabytes), which usually exceeds the chain's bytecode size limit (mentioned as roughly 750KB) and/or the gas limit for storing.

24. **Q: What tool or process is used to significantly reduce the size of the compiled Wasm binary?**
    *   **A:** A **Rust optimizer** (specifically mentioning `rust-optimizer` and `workspace-optimizer`). These run in Docker containers, perform optimizations, and strip unnecessary parts of the binary to make it much smaller and suitable for deployment.

25. **Q: What specific flag needs to be added to the optimizer command when compiling on an ARM-based machine (like an M1/M2 Mac)? What is the effect on the output filename?**
    *   **A:** The `--arm64` flag must be added. This results in the output Wasm file being suffixed with `-arm64.wasm` (e.g., `my_contract-arm64.wasm`). Compiling on Intel does not require this flag and produces a filename without the suffix (e.g., `my_contract.wasm`).

26. **Q: Where is the optimized Wasm binary placed after running the optimizer?**
    *   **A:** In an **`artifacts/`** directory created in the root of the contract project.

27. **Q: When running the local chain in Docker, why was it necessary to copy the compiled Wasm artifact *into* the Docker container?**
    *   **A:** The `junod` commands to store the contract were being executed *inside* the Docker container (using `docker exec`). Therefore, the Wasm file needed to be present within the container's filesystem to be accessible by the `junod tx wasm store` command.

28. **Q: What shell environment variable (`TXFLAG`) was set up to hold common flags for transaction commands? What flags did it include and why?**
    *   **A:** `TXFLAG` was used to store flags needed for most transactions against the local chain:
        *   `--gas-prices`: Set a higher gas price (e.g., `0.1ujunox`) because store/instantiate operations are gas-heavy.
        *   `--gas auto`: Automatically estimate gas needed.
        *   `--gas-adjustment 1.3`: Increase estimated gas by 30% as a buffer.
        *   `-b block`: Make the CLI wait for the transaction to be included in a block and return the result synchronously.
        *   `--chain-id testing`: Specify the target chain ID.
        *   `--node tcp://localhost:26657`: Specify the RPC endpoint of the local node.
        *   `-y`: Skip confirmation prompts.
        *   `--output json`: Ensure output is in JSON format for easier parsing.

29. **Q: What tool was used to parse the JSON output from CLI commands to extract specific values like the Code ID or Contract Address?**
    *   **A:** **`jq`** - a command-line JSON processor.

30. **Q: What `jq` filter was used to extract the Code ID from the `tx wasm store` output?**
    *   **A:** `'.logs[0].events[1].attributes[1].value'` (This path extracts the `code_id` value from the specific event structure returned by the store transaction). *Note: While the speaker knew it would be 1, this shows how to get it programmatically.*

31. **Q: What `junod` command is used to store the Wasm bytecode onto the chain? What key arguments does it take?**
    *   **A:** `junod tx wasm store <path_to_wasm_file> --from <key_name> [other_flags...]`
        *   `<path_to_wasm_file>`: Location of the Wasm binary.
        *   `--from <key_name>`: Specifies which key/wallet in the `junod` keychain should sign and pay for the transaction.

32. **Q: What `junod` command is used to instantiate a contract from a stored Code ID? What key arguments does it take?**
    *   **A:** `junod tx wasm instantiate <code_id> '<instantiate_message_json>' --from <key_name> --label <label_string> --admin <admin_address_or_leave_out> [other_flags...]`
        *   `<code_id>`: The numeric ID of the stored bytecode.
        *   `<instantiate_message_json>`: The JSON message required by the contract's instantiate entry point.
        *   `--from <key_name>`: The key signing and paying.
        *   `--label <label_string>`: A human-readable label for this specific contract instance.
        *   `--admin <admin_address>`: (Optional) Specifies the address that can migrate the contract.
        *   `--no-admin`: (Alternative to `--admin`) Explicitly sets no admin, making the contract immutable.

33. **Q: What is the significance of the `--no-admin` flag during instantiation? What happens if neither `--admin` nor `--no-admin` is specified?**
    *   **A:** `--no-admin` makes the instantiated contract **immutable**; its code can **never** be migrated or changed. If an admin *is* desired, the `--admin <address>` flag must be used instead. If *neither* flag is provided, the instantiating address (`--from` address) becomes the default admin.

34. **Q: Where can you find the expected structure for the instantiate/execute/query JSON messages for a given contract?**
    *   **A:**
        *   In the generated **JSON schema files** (usually in a `schema/` directory, generated by `cargo schema`).
        *   Directly in the Rust source code, typically in files like `msg.rs` or looking at the `#[derive(JsonSchema)]` structs/enums defining the messages.

35. **Q: How can you programmatically retrieve the address of the most recently instantiated contract based on a Code ID using the CLI and `jq`?**
    *   **A:** By querying the list of contracts for that Code ID and taking the last element:
        `junod query wasm list-contract-by-code <code_id> --output json | jq -r '.contracts[-1]'`
        (The `-r` flag in `jq` outputs the raw string without quotes).

36. **Q: What `junod` command is used to query a smart contract's state? What specific subcommand was used for CosmWasm contracts?**
    *   **A:** `junod query wasm contract-state smart <contract_address> '<query_message_json>' --output json`
        *   `contract-state smart`: Specifies a smart query against a CosmWasm contract.
        *   `<contract_address>`: The address of the specific contract instance.
        *   `<query_message_json>`: The JSON message corresponding to the desired query variant defined in the contract.

37. **Q: Do query operations require gas or signing (`--from` flag)?**
    *   **A:** No. Queries are typically **free** (have a free gas limit) and **do not require signing** as they don't change the blockchain state.

38. **Q: What `junod` command is used to execute a message against a smart contract (i.e., change its state)?**
    *   **A:** `junod tx wasm execute <contract_address> '<execute_message_json>' --from <key_name> [other_flags...]`
        *   Requires the contract address, the execute JSON message, and signing/gas flags (`--from`, `TXFLAG`, etc.).

39. **Q: What troubleshooting step resolved the issue where `docker exec` commands with aliases/variables weren't working initially?**
    *   **A:** Switching the terminal shell from **Zsh to Bash**. It was noted that Zsh sometimes had issues parsing aliases or flags in this specific `docker exec` context, while Bash handled it correctly.

### **IV. Contract Specifics & Examples (`cw-template` Counter)**

40. **Q: What was the required field and its type for the instantiate message in the `cw-template` counter example?**
    *   **A:** A field named `count` of type `integer` (specifically `i32` in Rust). Example: `{"count": 42}`.

41. **Q: What query message was used to read the current count? What was the initial incorrect guess?**
    *   **A:** The correct query message was `{"get_count": {}}`. The initial incorrect guess was `{"count": {}}`, which resulted in an "unknown variant count" error message from the contract.

42. **Q: What execute message was used to increment the counter? Did it require any arguments?**
    *   **A:** `{"increment": {}}`. It required no additional arguments within its object.

43. **Q: What execute message was used to reset the counter? What argument did it require?**
    *   **A:** `{"reset": {"count": <new_value>}}`. It required a `count` field specifying the integer value to reset to. Example: `{"reset": {"count": 5}}`.

### **V. Miscellaneous & Audience Questions**

44. **Q: Can anyone instantiate a contract from any uploaded bytecode (Code ID) on a permissionless chain?**
    *   **A:** Yes. If the bytecode is stored on the chain, anyone can call the `instantiate` command with that Code ID (provided they format the instantiate message correctly and pay the gas), creating their own independent instance of the contract. The JunoSwap/Marble DEX fork was cited as an example.

45. **Q: If someone re-instantiates a contract like the Unity contract (which holds funds), would it affect the original?**
    *   **A:** No. It would create a **new, separate, empty instance**. The state (including funds) of the original contract instance remains completely independent and unaffected.

46. **Q: How can you identify or differentiate between multiple instances created from the same bytecode?**
    *   **A:**
        *   **Contract Address:** Each instance gets a unique address. This is the definitive identifier.
        *   **Label:** The human-readable label provided during instantiation (though labels don't have to be unique and rely on the instantiator choosing a helpful one). Version numbers can be put here.
        *   **(Indirectly) State:** While not an identifier *for* the instance, the unique state of each instance differentiates their purpose/contents.
        *   **(For Code) Checksum:** You can download the bytecode associated with an instance and compare its checksum to a known checksum for a specific version to verify the underlying code.

47. **Q: What are "attributes" in the transaction logs (like the `raw_log`)? What is their purpose?**
    *   **A:** Attributes are **key-value pairs** emitted by the contract (or the module processing the transaction) during execution. They are included in the transaction logs. Developers can add custom attributes in their contract code (e.g., `("action", "increment")`, `("sender", sender_address)`). They provide **metadata** about the transaction, useful for off-chain indexing, frontends confirming actions, or debugging. Standard attributes often include the contract address and sender. Custom ones might indicate the specific method called or other relevant event details.

48. **Q: How does the interaction model using the CLI (passing JSON to `junod`) relate to how a frontend application using CosmJS would interact with the contract?**
    *   **A:** It's fundamentally the same principle. The CLI acts as a wrapper that sends the JSON payload to the chain's **RPC endpoint**. A frontend using **CosmJS** also constructs and sends similar JSON message payloads to the same RPC endpoint (via HTTP requests), just using JavaScript/TypeScript instead of the command line.

49. **Q: What underlying communication protocol is used when interacting with contracts via the methods shown (CLI, CosmJS)?**
    *   **A:** **RPC (Remote Procedure Call)** over HTTP(S), typically interacting with the Tendermint RPC endpoint (default port 26657).

50. **Q: What is the main difference in value proposition between an original popular contract (like JunoSwap) and a fork (like Marble DEX)?**
    *   **A:** **State**. While the code (functionality) might be identical initially, the original contract has established state representing real value – liquidity pools, user balances, transaction history (implied). The fork starts with empty state and needs to attract its own liquidity and users. The value lies primarily in the established, active state of the original protocol.

51. **Q: What topics were mentioned for potential future episodes?**
    *   **A:**
        *   Writing a contract from a sparse template (re-implementing a "Tier List" contract).
        *   Using VS Code collaborative coding.
        *   Diving deeper into CosmJS for frontend interaction.
        *   Breaking down the Unity contract.
        *   Exploring other entry points in detail: `Migrate`, `Sudo`, `Reply`.
        *   Migration strategies.

---