

## Interview Questions & Answers: Multichain Smart Contracts with ITS

### **I. General Concepts & Introduction**

1.  **Q:** What was the main goal of the workshop?
    **A:** The main goal was to demonstrate how to build a **multichain smart contract** featuring a **fee mechanism (a "tax contract")** using Axelar's **Interchain Token Service (ITS)**. It covered conceptual understanding, decision-making for ITS features, contract implementation, and a live demo.

3.  **Q:** What core problem does the Interchain Token Service aim to solve, according to the initial analogy?
    **A:** ITS aims to solve the problem of **token fragmentation and lack of interoperability** across different blockchains. The analogy used was having money (tokens) in different countries (blockchains) that couldn't be easily shared or used elsewhere. ITS acts as a tool to let tokens "travel" between these blockchains while maintaining their properties.

4.  **Q:** What is the basic concept of a "Tax Contract" as explained in the workshop?
    **A:** A "Tax Contract" is a smart contract (specifically, a token contract in this context) that applies a **small fee** when its tokens are moved between different blockchains using a service like ITS. This fee is analogous to a toll or tax for using the cross-chain transfer service facilitated by the contract.

5.  **Q:** What was the airport analogy used to explain the Tax Contract and ITS?
    **A:**    *   **Airport:** Represents the overall multichain ecosystem.
    *   **Terminals:** Represent different blockchain networks (e.g., Ethereum, Avalanche, Base).
    *   **Travelers:** Represent the tokens moving between blockchains.
    *   **Special Inter-Terminal Transport Service:** Represents the Interchain Token Service (ITS) facilitating seamless movement.
    *   **Airport Fees/Transit Fee:** Represents the fee charged by the Tax Contract when tokens travel between chains (terminals).

### **II. Interchain Token Service (ITS)**

6.  **Q:** Define the Interchain Token Service (ITS) in your own words based on the presentation.
    **A:** ITS is described as a **service, a tool, and a suite of smart contracts** designed by Axelar. It empowers developers to **issue and manage tokens** across multiple blockchain ecosystems while crucially **retaining their utility, fungibility, and custom functionality** as they move between chains.

7.  **Q:** What are the four key features of ITS highlighted in the workshop?
    **A:**
    *   **Fungibility:** Allows scaling tokens to 15+ blockchains with *canonical versions* sharing the *same EVM address*.
    *   **No-Code/Permissionless Deployment:** Enables multichain token deployment and management via the **Interchain Token Portal** (interchain.axelar.dev) without needing technical expertise, alongside programmatic options.
    *   **Trustless:** Runs on open-source smart contracts on public blockchains, secured by Axelar Network's dynamic validator set.
    *   **Functional:** Ensures multichain tokens remain *customizable* (e.g., with yield, permissions) and retain these custom features across chains.

8.  **Q:** What is the Interchain Token Portal, and who is it primarily aimed at?
    **A:** The Interchain Token Portal (interchain.axelar.dev) is a **web interface** that allows users to deploy and manage multichain tokens **without writing code**. It's aimed at those who want a quick go-to-market solution or lack deep technical experience but still want to create multichain tokens.

9.  **Q:** How does ITS ensure trustlessness?
    **A:** ITS achieves trustlessness by running on **open-source code** deployed as **smart contracts** on public blockchains. The underlying security is provided by the **Axelar Network's dynamic validator set**.

10. **Q:** What does it mean for ITS to keep customizations "intact" across chains?
    **A:** It means that if a token has special functions, permissions, or logic (like yield generation, specific transfer rules, etc.) implemented, these features will **continue to work** as intended even when the token is moved to and used on a different blockchain via ITS.

11. **Q:** What are the two main scenarios addressed by the ITS Decision Tree?
    **A:**
    *   Scenario 1: **Creating a brand new multichain token** from scratch.
    *   Scenario 2: **Upgrading an existing token** (already deployed on one or more chains) to become a multichain token via ITS.

12. **Q:** According to the decision tree, if you're creating a *new* token and *need custom functionality*, what approach should you take?
    **A:** You should **build a custom ERC20 token** incorporating your specific logic and then make it an interchain/multichain token using ITS (likely by extending the `InterchainTokenStandard` contract).

13. **Q:** If you are creating a *new* token and *do not* need custom functionality (standard ERC20 is fine), what are the options?
    **A:**
    *   Use the **Interchain Token Portal** (no-code).
    *   Deploy **programmatically** using the ITS API/contracts (developer-focused).

14. **Q:** If you want to *upgrade* an existing token that is *already deployed natively on multiple chains*, what is the process with ITS?
    **A:** You need to **link the deployed tokens** on the multiple chains together as a single multichain token using the Interchain Token Service. This interconnects the existing instances.

15. **Q:** If you want to *upgrade* an existing token that is only deployed on *one* chain, what approach does ITS offer?
    **A:** The suggested option is to use a **Canonical Token deployment with a simple wrapper**. This can be done either **programmatically** (interacting with ITS APIs/contracts) or via the **Interchain Token Portal**.

16. **Q:** Where can developers find detailed documentation and step-by-step guides for ITS?
    **A:** Developers can find documentation at **docs.axelar.dev**. This includes integration options, guides, and references to the open-source smart contracts.

### **III. Tax Contract Implementation Details**

17. **Q:** What are the three key components identified for the Tax Contract built in the workshop?
    **A:**
    *   **ERC20 Token Standard Functions:** Basic functionalities like transfer, approve, balance Oof, etc.
    *   **Interchain Token Standard Integration:** Incorporating ITS features, likely by inheriting or extending the `InterchainTokenStandard` contract.
    *   **Fee Calculation & Application Logic:** The custom code to calculate the fee (e.g., a percentage) and apply it during transfers, including deciding what to do with the collected fee.

18. **Q:** What are some potential uses for the fees collected by a Tax Contract, as mentioned in the workshop?
    **A:**
    *   **Sending to a Reward Pool:** For token holders or frequent users.
    *   **Allocating to a Treasury:** For project funding or governance.
    *   **Adding to a Liquidity Pool.**
    *   **Burning:** Removing the tokens from circulation.
    *   **Cross-chain Remittance Services.**
    *   **Platform Maintenance & Expansion.**
    *   **Funding Governance.**

19. **Q:** In the demo's Solidity contract (`Token.sol`), what base contracts were imported and inherited from?
    **A:**
    *   `InterchainTokenStandard` (from `@axelar-network/interchain-token-service/contracts/standards/InterchainTokenStandard.sol`)
    *   `ERC20` (from `@openzeppelin/contracts/token/ERC20/ERC20.sol`)
    *   `Mintable` (from `@axelar-network/interchain-token-service/contracts/Mintable.sol`)

20. **Q:** What parameters were passed into the constructor of the `Token.sol` contract?
    **A:**
    *   `name_`: The token's name (e.g., "My Token").
    *   `symbol_`: The token's symbol (e.g., "MTK").
    *   `decimals_`: The token's decimal places (e.g., 18).
    *   `feePercentage_`: The percentage fee to deduct on transfers.
    *   `interchainTokenService_`: The address of the deployed Interchain Token Service contract on that chain.

21. **Q:** What was the purpose of the `Mintable` contract import?
    **A:** The `Mintable` contract provides functionality for controlling *who* can mint tokens. It includes roles (like a `MINTER_ROLE`) and functions like `addMinter` and `hasRole`, which were used to restrict minting permissions.

22. **Q:** How was the initial minter set up in the contract?
    **A:** Inside the constructor, the `addMinter(msg.sender)` function was called. This automatically granted the deployer of the contract the `MINTER_ROLE`, allowing them to mint tokens initially and grant the role to others later.

23. **Q:** Explain the logic inside the overridden `transferFrom` function. How was the fee handled?
    **A:**
    *   It **overrides** the standard `ERC20.transferFrom`.
    *   It first **calculates the fee** based on the `amount` being transferred and the stored `feePercentage_`.
    *   It calculates the `amountToSend` (original amount minus the calculated fee).
    *   It calls the **original `transferFrom` function** using `super.transferFrom(from, to, amountToSend)` to transfer the reduced amount to the recipient.
    *   It then **handles the deducted fee**. In the demo, it called `_burn(from, fee)` to burn the fee amount from the sender's balance. *Alternatives mentioned were sending it to other addresses (treasury, reward pool, etc.).*
    *   Finally, it returns `true`.

24. **Q:** Why was the `_spendAllowance` function overridden?
    **A:** It was overridden to ensure correct allowance handling that considers **both the standard ERC20 allowance mechanism and the requirements of the Interchain Token Standard**, particularly when ITS needs to spend tokens on behalf of the user for cross-chain transfers. It calls `_approve` internally after potentially adjusting the allowance based on `UIN_MAX`.

25. **Q:** What was the purpose of the `_addMinter` function created in the `Token.sol` contract?
    **A:** This function allows an account that *already has* the `MINTER_ROLE` to grant the `MINTER_ROLE` to another address. This enables controlled delegation of minting capabilities.

26. **Q:** Besides the core functions, what utility functions were added to the contract?
    **A:** Functions were added to return:
    *   The `interchainTokenService` address.
    *   The `tokenID`.
    *   The `decimals`.

### **IV. Deployment and Scripting (Demo)**

27. **Q:** What tools were primarily used for compiling, deploying, and scripting in the demo?
    **A:**
    *   **Hardhat:** Ethereum development environment for compiling, testing, and deploying contracts.
    *   **Hardhat Ignition:** Used for managing deployments (`npx hardhat ignition deploy`).
    *   **Node.js/npm:** For managing packages and running scripts.
    *   **Solidity:** Language for writing the smart contract.
    *   **JavaScript/TypeScript:** For writing deployment and testing scripts (`deploy.js`, `index.js`).
    *   **`dotenv`:** To manage environment variables (like private keys).
    *   **Axelar JS SDK (`@axelar-network/axelarjs-sdk`):** Used for gas estimation in the testing script.

28. **Q:** What information was stored in the `.env` file?
    **A:** The primary information stored was the `PRIVATE_KEY` required for deploying contracts and sending transactions from that account.

29. **Q:** What networks were targeted for deployment in the demo?
    **A:** The demo deployed the token contract to **Avalanche Fuji testnet** and **Fantom testnet**.

30. **Q:** Describe the purpose of the `deploy.js` script.
    **A:** The `deploy.js` script, used with Hardhat Ignition, contained the logic to **deploy the `Token.sol` smart contract**. It defined the constructor arguments (name, symbol, decimals, fee percentage, ITS address) and executed the deployment transaction.

31. **Q:** After deploying the token contract, what core artifact was needed from the compilation output for the testing script?
    **A:** The **Contract ABI (Application Binary Interface)** for the newly deployed `Token.sol` contract was needed. This was saved into a JSON file (e.g., `interchainTokenABI.json`) to allow the testing script to interact with the contract's functions.

32. **Q:** What is a "Token Manager" in the context of ITS, and why was it deployed?
    **A:** A Token Manager is a contract deployed via the Interchain Token Service for a *specific* interchain token. It's responsible for **handling the logic of locking/unlocking or minting/burning** that token on a particular chain as part of the cross-chain transfer process. It needs to be deployed for each chain the token exists on to enable transfers managed by ITS.

33. **Q:** What is the significance of the `salt` value when deploying a Token Manager?
    **A:** The `salt` is a **unique, developer-provided value** used in conjunction with the deployer's address to deterministically generate the `tokenID`. This `tokenID` uniquely identifies the *specific interchain token representation* across all chains. Using the *same salt* when deploying the token manager for the same logical token on different chains ensures they share the same `tokenID` and are correctly linked by ITS. It's crucial for identifying and managing the token consistently.

34. **Q:** How was the `tokenID` generated or retrieved in the script?
    **A:** The `tokenID` was retrieved by calling the `interchainTokenId` function on the Interchain Token Service contract instance, passing the **deployer's address** and the **`salt`** value used during the token manager deployment. `tokenID = await itsContract.interchainTokenId(signer.address, salt);`

35. **Q:** How was the address of the deployed Token Manager determined *before* it might have been emitted in an event?
    **A:** By calling the `tokenManagerAddress` function on the Interchain Token Service contract instance, passing the unique `tokenID`. `tokenManagerAddress = await itsContract.tokenManagerAddress(tokenID);`

36. **Q:** What two distinct types of Token Managers were deployed in the demo, and on which chains?
    **A:**
    *   A **Lock/Unlock (with fee)** type (type 3) was deployed on the source chain (**Avalanche**).
    *   A **Mint/Burn (with fee)** type (type 4) was deployed on the destination chain (**Fantom**).

37. **Q:** Why was it necessary to call `addMinter` targeting the Token Manager address after deploying it?
    **A:** The Token Manager needs permission to interact with the underlying token contract (e.g., to lock tokens for the Lock/Unlock type, or mint/burn tokens for the Mint/Burn type). Calling `addMinter` (or a similar permission-granting function defined by the token standard being used, like `approve` in some setups) on the *Token Contract* grants the deployed *Token Manager Contract* the necessary permissions to perform these actions on behalf of ITS.

38. **Q:** Explain the purpose of the `getGasEstimate` function in the testing script (`index.js`).
    **A:** This function used the **Axelar JS SDK (`estimateGasFee`)** to calculate the estimated gas cost required to execute the *cross-chain* portion of an operation (like deploying a token manager remotely or performing an interchain transfer) on the Axelar network and the destination chain. This estimated value is then paid on the source chain to fund the subsequent cross-chain actions.

39. **Q:** How was the Token Manager deployed on the *remote* chain (Fantom) from the script running on the *source* chain (Avalanche)?
    **A:** By calling the `deployTokenManager` function on the **Interchain Token Service contract on Avalanche**. Crucially, this call included:
    *   The *same* `salt` used for the Avalanche deployment.
    *   The `destinationChain` parameter set to "Fantom".
    *   The `tokenManagerType` set to Mint/Burn (type 4).
    *   The `gasValue` (the estimated gas cost obtained from `getGasEstimate`).40. **Q:** What function on the ITS contract was called to initiate the cross-chain token transfer?
    **A:** The `interchainTransfer` function was called on the Interchain Token Service contract instance on the source chain (Avalanche).

41. **Q:** What were the key parameters passed to the `interchainTransfer` function?
    **A:**
    *   `tokenId`: The unique ID of the interchain token being transferred.
    *   `destinationChain`: The name of the target chain (e.g., "Fantom").
    *   `destinationAddress`: The recipient's address on the destination chain.
    *   `amount`: The quantity of tokens to transfer.
    *   `metadata`: Optional byte data for arbitrary messages (passed as `0x` in the demo).
    *   `gasValue`: The estimated gas cost for the cross-chain call.

42. **Q:** Before initiating the `interchainTransfer`, what two actions needed to be performed regarding the token contract itself?
    **A:**
    *   **Minting:** Tokens needed to be minted to the deployer's address (`mintAndApproveIts` function called `tokenContract.mint`).
    *   **Approving:** The Interchain Token Service contract needed to be approved to spend the tokens on behalf of the deployer (`mintAndApproveIts` function called `tokenContract.approve` targeting the ITS address with `UIN_MAX`).

43. **Q:** How was the successful fee deduction verified after the `interchainTransfer`?
    **A:** By checking the transaction details on the **Avalanche block explorer (Snowtrace)**. It showed:
    *   The initial `transferFrom` call within the ITS interaction, transferring the *full* amount (500) *temporarily* (likely to the token manager or ITS).
    *   A subsequent `Transfer` event showing the *fee amount* (25) being transferred *to the zero address* (indicating a burn, as per the contract logic).
    *   The logs indicating the final amount bridged to the destination chain was the original amount minus the fee (475).

44. **Q:** What was the purpose of the `crypto` package import in the testing script?
    **A:** It was used to generate a **random `salt` value** (`crypto.randomBytes(32)`). This ensures that each deployment test run uses a potentially unique salt, preventing collisions if the script were run multiple times without cleaning up previous deployments.

### **V. Axelar Ecosystem & Support**

45. **Q:** What role does the Axelar Network itself play in this process?
    **A:** The Axelar Network acts as the underlying **secure communication layer** that connects the different blockchains. Its **dynamic validator set** secures the cross-chain messages and ensures the integrity of operations initiated via ITS, such as remote token manager deployments and interchain transfers.

46. **Q:** What resources were mentioned for developers needing help or wanting to explore further examples?
    **A:**
    *   **Axelar Support:** A dedicated channel or repository (found via Google search for "Axelar Support") for asking questions, reporting bugs, or giving feedback.
    *   **Axelar Examples Repository:** Contains code snippets and example implementations for various Axelar features, including ITS.
    *   **Axelar Documentation:** docs.axelar.dev47. **Q:** What is the "paradigm shift" mentioned in one of the comments during the Q&A regarding how developers approach bridging?
    **A:** The comment suggested that the current paradigm is often: build a dApp, *then* think about bridging/multichain later as an add-on. The shift enabled by tools like ITS is towards **integrating bridging and multichain capabilities directly *into* the dApp's core logic** from the start, making it a more seamless and fundamental part of the application.

### **VI. Conceptual & Scenario-Based**

48. **Q:** If you wanted the collected fee to be sent to a project treasury address instead of being burned, which specific line(s) in the `Token.sol` contract's `transferFrom` function would you modify, and how?
    **A:** You would modify the line `_burn(from, fee);`. Instead of burning, you would implement a transfer of the `fee` amount to the treasury address. This would likely look something like: `_transfer(from, TREASURY_ADDRESS, fee);` (assuming `TREASURY_ADDRESS` is a defined state variable or constant holding the treasury's address, and acknowledging potential implications for token supply/sender balance depending on the exact implementation of `_transfer` vs `_burn`). *Alternatively, the fee could potentially be transferred directly from the contract's perspective if the fee mechanism involved temporarily holding the fee.*

49. **Q:** Why is using a programmatic approach (writing scripts) potentially preferable to using the Interchain Token Portal for complex or automated setups?
    **A:** A programmatic approach offers:
    *   **Greater Control:** Fine-grained control over every step of the deployment and management process.
    *   **Automation:** Easier integration into CI/CD pipelines and automated workflows.
    *   **Customization:** Ability to integrate complex logic or interact with other contracts as part of the deployment/setup process.
    *   **Repeatability:** Scripts ensure consistency across multiple deployments or environments.

50. **Q:** Imagine you deployed your Tax Token using ITS. Later, you decide to change the fee percentage from 0.05% to 0.1%. How would you achieve this based on the contract implemented in the demo?
    **A:** The demo contract included a function `setFeePercentage(uint256 feePercentage_) onlyMinter`. An account with the `MINTER_ROLE` could call this function, passing the new fee percentage (correctly formatted, likely considering the 18 decimals used in the calculation) to update the `feePercentage_` state variable within the contract. This avoids needing to redeploy the entire token contract.

51. **Q:** What are the potential trade-offs of implementing a fee on cross-chain transfers for users of your token?
    **A:**
    *   **Pros (for the project):** Generates revenue (for treasury, LPs, rewards), can fund development/maintenance, potentially creates deflationary pressure (if burned), enables specific tokenomics.
    *   **Cons (for the user):** Increases the cost of moving tokens between chains, potentially reduces user experience compared to fee-less transfers, might make the token less attractive compared to competitors without fees if the value proposition isn't clear.

---