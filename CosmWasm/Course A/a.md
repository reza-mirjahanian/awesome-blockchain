

13. **Q: What architectural model do CosmWasm smart contracts follow, and how does this influence their design?**
    *   **A:** They follow an **actor model**. This encourages splitting functionality into much **smaller, more granular chunks** compared to traditional monolithic applications.

14. **Q: How was CosmWasm contract design compared to microservice or serverless architectures?**
    *   **A:** It was compared favorably to **modern microservice architectures**, stating the concepts will feel familiar. However, smart contracts were described as even *smaller* or more granular than typical serverless functions ("it's not 50 lines of code it's like a whole 500").

15. **Q: What core principle was suggested for designing the scope of a single smart contract?**
    *   **A:** A smart contract should aim to **do one thing well**. While not always strictly possible, you should be able to clearly state "this smart contract does *this* sort of thing and it does this well." It's often better to have two contracts interact than one contract handling both responsibilities.

16. **Q: How are incoming messages typically handled within a CosmWasm contract's code structure?**
    *   **A:** Through **entry points** (`instantiate`, `execute`, `query`). These act like the `main` function. Inside the entry point, there's typically a `match` statement (similar to pattern matching or routing in web frameworks) that directs the execution flow to other internal functions based on the incoming message type.

17. **Q: What is the fundamental flow of execution within a smart contract, ?**
    *   **A:** It's essentially a **chain of function calls**. An entry point function is called, which then calls other functions, which might call further functions, ultimately producing the behavior and result returned to the user.

18. **Q: The question arose about using Rust features like traits and inheritance for code reuse across contracts. What was the speaker's stated preference?**
    *   **A:** Frey, despite maintaining a library (CW NFTs) that *does* use traits heavily, expressed a personal preference for writing **libraries of functions** and importing/using only the ones needed, rather than encapsulating behavior around objects or traits.

19. **Q: What potential complexity was mentioned regarding the use of traits in Rust, especially for new programmers?**
    *   **A:**
        *   Multiple traits can be applied to the same base struct, making it easy to get lost in implementations.
        *   Rust's opinionated type system, while good for security/code quality, adds complexity and can be a lot to keep in one's head when combined with traits.

### **III. Decentralized Applications (DApps) & Components**

20. **Q: What are the primary components of a CosmWasm-based decentralized application (DApp) as outlined in the discussion?**
    *   **A:**
        *   **The Blockchain (Chain):** The underlying network where the contract runs (e.g., Juno).
        *   **Smart Contracts:** The CosmWasm code uploaded to the chain (e.g., the name service contract).
        *   **Frontend:** The user interface, often built with frameworks like **Next.js**.
        *   **Interaction Library:** A JavaScript/TypeScript library like **CosmJS** used by the frontend to communicate with the smart contracts via their defined endpoints.

21. **Q: Where do smart contracts actually run in relation to the underlying blockchain?**
    *   **A:** Smart contracts run in a **virtual machine (Wasm VM)** that sits *above* the chain level logic.

22. **Q: Do developers need deep knowledge of the specific chain (e.g., Juno) they are deploying to?**
    *   **A:** You need to know a little bit, but not necessarily deep details. The examples shown were primarily focused on Juno but would likely run on WasmD (the reference implementation) or other permissionless CosmWasm chains.

23. **Q: Can CosmWasm be used on permissioned chains or application-specific blockchains (appchains)?**
    *   **A:** Yes. CosmWasm is configurable. It's possible to build an appchain that uses CosmWasm but only allows contracts to be added via governance (permissioned), rather than allowing anyone to upload contracts (permissionless). Stargaze was mentioned as an example.

24. **Q: What is the potential advantage mentioned for building appchain logic using permissioned CosmWasm contracts instead of native Cosmos SDK modules?**
    *   **A:** It allows developers more comfortable with **Rust** (the language of CosmWasm contracts) than **Go** (the language of the Cosmos SDK) to implement chain logic without learning the SDK. However, there's a performance hit compared to native modules.

### **IV. Developer Tooling & Environment**

25. **Q: What command-line tool associated with the chain itself can be used to interact with smart contracts, especially on a local chain?**
    *   **A:** The chain's own **command-line interface (CLI)** (e.g., `junod` for Juno).

26. **Q: What web-based tool was mentioned for providing a simple GUI to interact (query, execute) with contracts using JSON inputs/outputs? Who developed it?**
    *   **A:** **cosmos.tools**, developed by `@dez_weaver`. It's handy for debugging.

27. **Q: What is the recommended template repository for quickly spinning up a boilerplate CosmWasm smart contract project? Where is it hosted?**
    *   **A:** **cw-template**. It's hosted under the `InterWasm` GitHub organization (previously `CosmWasm`).

28. **Q: What features does the `cw-template` provide besides basic contract structure?**
    *   **A:** It includes basic **workflows** (likely GitHub Actions) to sanity-check contracts on push, run tests, check for schema changes, and run linters.

29. **Q: What equivalent template or starter kit was mentioned for bootstrapping the *frontend* of a DApp, specifically within the Juno ecosystem?**
    *   **A:** The **Juno Starter Kit** (likely found under the `CosmosContracts` GitHub org).

30. **Q: What tooling component simplifies running a local blockchain environment for development and testing?**
    *   **A:** Pre-defined **Docker containers** (like the ones provided by Juno) that spin up a local chain.

31. **Q: What other development environment tools were mentioned, specifically IDE integration?**
    *   **A:**
        *   Someone is building an **IDE** with CosmWasm-specific hooks/snippets.
        *   A user named **Sporty** created a **VS Code extension** for interacting with a chain directly from the editor.



### **V. Optimization, Gas, and Complexity**

33. **Q: How important is logic optimization (space and time complexity) when writing CosmWasm contracts? Why?**
    *   **A:** It is **very important**. The main reason is that **users pay gas** for execution. Inefficient code leads to high gas costs for users.

34. **Q: What data structure was casually mentioned as frequently used due to efficiency?**
    *   **A:** **Maps**. ("Everything becomes a map because maps are quite efficient").

35. **Q: Unlike Ethereum where interaction often happens via indexers, what is becoming more common in Cosmos according to the speakers, and what implication does this have for optimization?**
    *   **A:** Storing more data **on-chain** (like NFT metadata in the Name Service or data in Howl Social) is becoming more common. This directly increases the need to be concerned about optimizations to manage storage growth and interaction costs.

36. **Q: What specific Rust programming technique was recommended over traditional `for each` loops for potentially better performance and gas calculation?**
    *   **A:** Using Rust's **iterator system** (functions like `map`, `fold`, `filter`, etc.). These are often evaluated lazily (only running when `collect` is called) and can allow the VM to determine gas usage more easily.

37. **Q: What is the concept of lazy evaluation in iterators, and why is it performant?**
    *   **A:** Lazy evaluation means that intermediate steps in a chain of iterator operations (like multiple maps) aren't computed immediately. The computations are deferred until a final "collection" step (like `collect` or `sum`) is needed. This allows the compiler/runtime to optimize the entire chain, potentially removing redundant steps and performing the work more efficiently in one go. It's a technique common in functional programming and big data systems.

38. **Q: Besides user costs, what other limit related to gas must developers be aware of? What can exceeding it cause?**
    *   **A:** The **block gas limit**. If a transaction (or combination of transactions in a block) consumes too much gas, it can exceed the block's capacity, making the block invalid. While hard for a single *well-behaved* contract to hit, a poorly optimized contract or a deliberate attack *could* potentially halt a chain by consistently producing invalid blocks.

39. **Q: How lenient are the gas limits typically on chains like Juno, according to the speakers?**
    *   **A:** They are described as "quite lenient." An example given was adding 500 addresses to a whitelist on Stargaze in a single transaction without hitting the gas limit. However, gas limits *are* configurable per chain.

40. **Q: For simple "toy projects," is deep optimization usually a major concern?**
    *   **A:** Likely not. Most small projects won't perform enough iterations or use enough storage to hit performance bottlenecks or gas limits, given that modern chains are quite performant.

### **VI. Local Development Environment (Juno Example)**

41. **Q: What are the two main methods discussed for running a local Juno chain for development?**
    *   **A:**
        *   Using **Docker Compose** with the source code (`docker-compose up`): Builds the chain from scratch inside the container. Easier (one command) but slower, especially the first time. Usually only needed if testing changes *to Juno itself*.
        *   Using a **pre-built Docker image** (`docker run ...`): Pulls an existing, published image of a specific Juno version. Much faster if the image is already downloaded or needs downloading.

42. **Q: When running the local Juno chain via Docker Compose, what was the purpose of setting environment variables like `STAKE_TOKEN=ujunox` and `UNSAFE_CORS=true`?**
    *   **A:**
        *   `STAKE_TOKEN=ujunox`: Sets the denomination for the staking token on the local chain to `ujunox` instead of `ujuno`. This helps prevent confusion with real Juno when connecting wallets like Keplr.
        *   `UNSAFE_CORS=true`: Enables Cross-Origin Resource Sharing. This allows frontend applications running on a different port (e.g., `localhost:3000`) to communicate with the chain running on its ports (e.g., `localhost:1317`, `localhost:26657`) on the same machine. Without it, browser security rules might block requests.

43. **Q: What is the difference between the REST endpoint (port 1317) and the RPC endpoint (port 26657)? Which one is generally preferred for CosmWasm contract interaction?**
    *   **A:**
        *   **REST (1317):** A standard HTTP API following REST principles (GET, POST, etc.). Familiar to web developers but some functionality (like querying transactions) has been deprecated in newer SDK versions.
        *   **RPC (26657):** Tendermint RPC endpoint. Allows for more arbitrary data structures in requests/responses. Generally **preferred** for interacting with CosmWasm smart contracts because it offers more flexibility fitting the custom nature of contract interactions. CosmJS primarily uses the RPC endpoint.

44. **Q: How can you interact with the Juno chain running inside a Docker container using the `junod` CLI installed on your host machine?**
    *   **A:** If the Docker container correctly **exposes the necessary ports** (like 26657 for RPC) to the host machine, the `junod` CLI on the host can connect to `localhost:<port>` and interact with the chain inside the container as if it were running directly on the host.

45. **Q: What steps are needed to build and install the `junod` binary locally from the source code?**
    *   **A:**
        1.  Clone the Juno repository (`git clone ...`).
        2.  Navigate into the repository directory (`cd juno`).
        3.  Run `make build` (compiles the binary, often placing it in a `build` or `bin` subdirectory).
        4.  Run `make install` (copies the compiled binary to your system's executable path, like `/usr/local/bin`).

46. **Q: When using the Juno Starter Kit frontend, what configuration file needs to be set up, and what key details does it contain?**
    *   **A:** You need to copy `.env.example` to **`.env.local`**. This file contains:
        *   RPC endpoint URL (`NEXT_PUBLIC_RPC_ENDPOINT`)        *   REST endpoint URL (`NEXT_PUBLIC_REST_ENDPOINT`)
        *   Chain ID (`NEXT_PUBLIC_CHAIN_ID`)
        *   Staking Denomination (`NEXT_PUBLIC_STAKING_DENOM`)
        *   Coin Decimals (`NEXT_PUBLIC_COIN_DECIMALS`)
        *   Gas Price (`NEXT_PUBLIC_GAS_PRICE`)
        *   Kepler Chain Suggestion Info (Name, Bech32 prefix, etc.)

47. **Q: What feature allows the frontend application to prompt the user to add the local development chain to their Keplr wallet?**
    *   **A:** Keplr's **`suggestChain`** feature. The configuration in `.env.local` provides the necessary details (Chain ID, RPC/REST endpoints, currency info, etc.) for the frontend to make this request to Keplr.

48. **Q: Does the state of the local Docker-based chain persist by default when you stop and restart the container? How *can* persistence be achieved?**
    *   **A:** **No**, by default the state is **ephemeral** and gets reset each time you start the container (starting from block 1). Persistence *can* be achieved by attaching a **Docker data volume** to the container run command, which saves the chain state outside the container. However, the speakers noted many projects prefer the repeatability of starting fresh each time using setup scripts.

49. **Q: What is the alternative to running a local chain if you need persistent state, especially for testing with others?**
    *   **A:** Use a public **testnet** (like Juno's Uni testnet). You would configure your `.env.local` file to point to the testnet's RPC/REST endpoints instead of `localhost`.

50. **Q: What is the significance of the pre-funded "test user" mnemonic provided in the Juno documentation for local development?**
    *   **A:** This mnemonic corresponds to an account that is automatically created and funded with test tokens (`ujunox`) when the local chain starts using the provided scripts/Docker configurations. It allows developers to immediately have funds to pay for transactions (like deploying contracts or sending tokens) in the local environment without needing a faucet. **Crucially, this mnemonic should *never* be used on a mainnet or even a public testnet as it's publicly known.**

51. **Q: What continuous integration (CI) checks were mentioned as being run on the Juno repository itself?**
    *   **A:** The Juno repo runs GitHub Actions on pull requests that:
        *   Test basic smart contract interactions (instantiation, basic behavior).
        *   Check if migrations work.
        *   Specifically instantiate the "Unity" contract (from Prop 20) and submit a governance proposal against it to ensure that critical functionality hasn't regressed.

---