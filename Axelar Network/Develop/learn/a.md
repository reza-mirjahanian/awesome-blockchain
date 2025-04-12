

## Interview Questions & Answers based on Axelar Workshop Text

### **I. Introduction & Context**

1.  **Question:** Who is the speaker and what is their role?
    *   **Answer:** The speaker is Idris Olubisi, a Developer Advocate at Interop Labs, working on the Axelar Network.

2.  **Question:** What is the main topic of this workshop session?
    *   **Answer:** The main topic is tracking and troubleshooting cross-chain transactions using AxelarJS (the SDK) and Axelar Scan.

3.  **Question:** What is the stated goal for attendees by the end of the workshop?
    *   **Answer:** By the end of the workshop, attendees should have the tools to send messages across different chains effortlessly, track every step of their transactions, and solve any potential issues (hiccups) encountered along the way.

4.  **Question:** The speaker mentions a personal pain point that motivates the workshop. What was it?
    *   **Answer:** The speaker describes an experience where they tried to send an asset (like USDC) from Ethereum to a friend on another network, but the transaction got stuck. They felt helpless because they didn't know why it was stuck or how to fix it, especially since their friend urgently needed the asset.

5.  **Question:** What two primary Axelar tools are the focus of this session for tracking and troubleshooting?
    *   **Answer:** AxelarJS (specifically the SDK) and Axelar Scan.

### **II. Axelar General Message Passing (GMP) - Concepts**

6.  **Question:** What is Axelar General Message Passing (GMP)?
    *   **Answer:** Axelar General Message Passing (GMP) is described as a feature that allows you to send messages across different chains.

7.  **Question:** What is the core problem GMP aims to solve for users, according to the speaker's introduction?
    *   **Answer:** It aims to prevent users from dealing with stuck transactions when sending assets or messages across chains and provide ways to easily unblock them if they do get stuck.

8.  **Question:** Describe the high-level, end-to-end flow of a GMP transaction from a source chain (e.g., Ethereum) to a destination chain (e.g., Base) as explained in the text.
    *   **Answer:**
        1.  A dApp on the source chain initiates a call (e.g., `callContract`).
        2.  The call goes through the Axelar Gateway deployed on the source network.
        3.  An event is emitted, which is picked up by the Axelar Network validators.
        4.  The Axelar Network validators vote on and validate the call.
        5.  Once successfully validated and approved, the call is sent to the Axelar Gateway on the destination chain.
        6.  An `approvedContractCall` (or similar) is triggered on the destination gateway.
        7.  This triggers the `_execute` (or `_executeWithToken`) function on the receiving dApp/contract on the destination chain automatically.

9.  **Question:** When sending a message from Chain A to Chain B using GMP, what happens on the destination chain (Chain B) automatically?
    *   **Answer:** A corresponding function (`_execute` or `_executeWithToken`) is automatically run (called) on the destination chain's contract.

10. **Question:** What kind of "payload" can be sent using GMP?
    *   **Answer:** The payload can be things like a string or an array, depending on what needs to be encoded and sent from the source chain to the destination chain.

### **III. Axelar General Message Passing (GMP) - Functions (`callContract` / `callContractWithToken`)**

11. **Question:** What are the two main functions highlighted within the Axelar GMP SDK for initiating cross-chain interactions?
    *   **Answer:** `callContract` and `callContractWithToken`.

12. **Question:** What is the primary purpose of the `callContract` function?
    *   **Answer:** To send a message from one chain to another and potentially trigger some custom functionality on the destination chain via the `_execute` function.

13. **Question:** What key parameters must be specified when using `callContract`?
    *   **Answer:** You need to specify the `destinationChain`, the destination `contractAddress`, and the `payload`.

14. **Question:** What function is automatically executed on the destination contract when a message sent via `callContract` arrives?
    *   **Answer:** The `_execute` function.

15. **Question:** What parameters does the `_execute` function on the destination chain receive?
    *   **Answer:** It receives the `sourceChain`, the `sourceAddress` (from where the message originated), and the `payload` that was sent.

16. **Question:** What is the primary difference between `callContract` and `callContractWithToken`?
    *   **Answer:** `callContractWithToken` allows you to send not just a message (payload) but also an asset (token) along with it, while `callContract` only sends a message/payload.

17. **Question:** What additional parameters are required for `callContractWithToken` compared to `callContract`?
    *   **Answer:** You need to specify the token `symbol` and the `amount` of the token being sent.

18. **Question:** What function is automatically executed on the destination contract when a message *and token* sent via `callContractWithToken` arrives?
    *   **Answer:** The `_executeWithToken` function.

19. **Question:** What parameters does the `_executeWithToken` function receive?
    *   **Answer:** It receives the `sourceChain`, `sourceAddress`, `payload`, `tokenSymbol`, and `amount`.

20. **Question:** Where can developers embed custom logic that should run immediately upon receiving a message or message+token on the destination chain?
    *   **Answer:** Developers can embed custom logic directly inside the `_execute` or `_executeWithToken` function on their destination chain contract.

### **IV. Axelar Scan - Purpose & Access**21. **Question:** What is Axelar Scan?
    *   **Answer:** Axelar Scan is described as a portal that allows users to visualize how messages and assets are being sent from one protocol/chain to another within the Axelar ecosystem. It provides visibility into transactions, network statistics, and more.

22. **Question:** What is the URL to access Axelar Scan for the mainnet?
    *   **Answer:** `axelarscan.io`

23. **Question:** What is the URL to access Axelar Scan for the testnet?
    *   **Answer:** `testnet.axelarscan.io`

24. **Question:** Can you switch between mainnet and testnet views directly within the Axelar Scan interface?
    *   **Answer:** Yes, the text mentions you can navigate between mainnet and testnet from within the portal.

### **V. Axelar Scan - Features & Usage**

25. **Question:** Besides tracking individual transactions, what other types of information can be found on Axelar Scan?
    *   **Answer:** You can find network statistics (like transaction volume, average volume), validator information, blocks, transactions, pools, on-chain proposals, and resources like lists of supported chains and assets.

26. **Question:** What specific section on Axelar Scan is highlighted as being particularly interesting for developers tracking cross-chain calls?
    *   **Answer:** The GMP (General Message Passing) section, especially the ability to search for transactions.

27. **Question:** What are the typical stages or statuses of a GMP transaction that can be tracked on Axelar Scan?    *   **Answer:** The stages mentioned and shown are:
        *   `Sent` (Source chain transaction confirmed)
        *   `Gas Paid` (Gas payment status confirmed)
        *   `Confirmed` (Axelar network has confirmed/voted on the message)
        *   `Approved` (Message approved for execution on the destination chain)
        *   `Executed` (Message successfully executed on the destination chain contract)

28. **Question:** How can a user verify the on-chain details for each step of a GMP transaction shown on Axelar Scan?
    *   **Answer:** For each step (like `Sent`, `Gas Paid`, `Executed`), there are links on Axelar Scan that you can click to view the corresponding transaction details directly on the relevant source or destination chain's block explorer (e.g., Snowtrace for Avalanche).

29. **Question:** What feature does Axelar Scan provide to help users resolve transactions stuck due to insufficient gas fees?
    *   **Answer:** It provides a "Recovery" button, specifically an "Add Gas" button, for transactions marked with "insufficient fee". Clicking this allows the user to pay the required additional gas via a wallet pop-up.

30. **Question:** Can you filter transactions on Axelar Scan based on their status?
    *   **Answer:** Yes, the speaker demonstrated filtering transactions by status, specifically filtering for "Executed" transactions.

### **VI. Transaction Tracking, Troubleshooting & Recovery (GMP Recovery API)**

31. **Question:** What are the main capabilities provided by the Axelar GMP Recovery API, according to the presentation?
    *   **Answer:**        *   Querying transaction status.
        *   Manually relaying a transaction to the destination chain (if stuck at `Confirmed`/`Approved`).
        *   Manually executing a transaction (if stuck at the `Execute` step).
        *   Adding native gas to a transaction (if stuck due to insufficient fees).

32. **Question:** Why might a developer want to query the transaction status programmatically using the API?
    *   **Answer:** To track the progress of a transaction within their own application (e.g., a swap interface) and provide real-time feedback to the user without requiring them to leave the app and check Axelar Scan manually. It can also be used in backend systems (e.g., cron jobs) to monitor and potentially automate recovery steps.

33. **Question:** Under what circumstance might you use the "Manual Relay to Destination Chain" API function?
    *   **Answer:** If a transaction appears stuck at the `Confirmed` or `Approved` step on Axelar Scan, a developer might choose to manually trigger the relay process themselves using this API call, perhaps for expediency or testing.

34. **Question:** Similarly, when might the "Execute Transaction" API function be used?
    *   **Answer:** If a transaction has been successfully relayed and approved but gets stuck just before the final execution step on the destination chain, this API call can be used to attempt manual execution. (Though the speaker notes the system is usually stable and executes automatically).

35. **Question:** How can a developer programmatically add gas to a transaction stuck due to insufficient fees, similar to the "Add Gas" button on Axelar Scan?
    *   **Answer:** By using the `addNativeGas` function available through the GMP Recovery API/SDK.

36. **Question:** What information is typically needed to call the `addNativeGas` function programmatically?
    *   **Answer:** The `chain` name (where gas is needed), the transaction `hash` of the stuck GMP transaction, and gas-related options (like estimated gas limit). A wallet/private key with funds on the source chain is also needed to sign and pay for the gas top-up transaction.

37. **Question:** In the demo, what happened immediately on Axelar Scan after the `addNativeGas` script was successfully run for a transaction with an "insufficient fee"?
    *   **Answer:** The status on Axelar Scan automatically updated from "insufficient fee" to showing progress through the subsequent steps (`Gas Paid`, `Confirmed`, etc.).

### **VII. Axelar Query API**

38. **Question:** Beyond recovery, what other API category does AxelarJS provide for developers?
    *   **Answer:** The Axelar Query API.

39. **Question:** What are the main functions available under the Axelar Query API mentioned in the text?
    *   **Answer:**
        *   `estimateGasFee`: Estimate the cost to send a cross-chain transaction.
        *   `getTransferFee`: Get the fee for transferring assets (distinct from GMP calls).
        *   `getDenomFromSymbol`: Get the on-chain asset identifier (denom) given its common symbol (e.g., "aUSDC").
        *   `getSymbolFromDenom`: Get the common symbol given the on-chain asset identifier (denom).

40. **Question:** What is the primary benefit of using the `estimateGasFee` function?
    *   **Answer:** It helps developers determine the appropriate amount of gas to pay for a GMP transaction, reducing the chances of overpaying or underpaying (which would cause the transaction to get stuck). It provides a more accurate cost estimation than hardcoding a random value.

41. **Question:** What parameters are typically required for the `estimateGasFee` function?
    *   **Answer:** `sourceChain`, `destinationChain`, `sourceChainTokenSymbol`, `gasLimit`, `gasMultiplier` (can be 'auto'), `minGasPrice`, and potentially others like `data` or GMP-specific parameters.

42. **Question:** What does the `getDenomFromSymbol` function do? Give an example from the text.
    *   **Answer:** It takes a common token symbol (like "aUSDC") and a chain name (like "Moonbeam") and returns the specific identifier (denom) used for that asset *on that particular chain*. The example given was passing "aUSDC" and "Moonbase" (testnet) returned "uausdc".

43. **Question:** What does the `getSymbolFromDenom` function do?
    *   **Answer:** It does the reverse of `getDenomFromSymbol`. It takes the on-chain identifier (denom) and the chain name and returns the common symbol associated with it.

### **VIII. Gas Management**

44. **Question:** How does Axelar simplify gas payments for cross-chain transactions, according to the speaker?
    *   **Answer:** Axelar has a "Gas Service" that allows users to pay gas only *once* on the source chain. This single payment covers the costs across the Axelar network and the execution on the destination chain, eliminating the need for the user to hold gas tokens on Axelar or the destination chain.

45. **Question:** What happens if a user overpays the gas fee for a GMP transaction (either intentionally or because the estimation was high)?
    *   **Answer:** Axelar has a refund mechanism. The excess gas fee (the amount paid minus the actual cost) is automatically refunded to an address specified by the user or sender. The `addNativeGas` demo also showed a `refundAddress` parameter.

46. **Question:** In the `estimateGasFee` demo, what was the format of the returned gas fee?
    *   **Answer:** The fee was returned as a large number, which the speaker identified as being denominated in 'wei' (the smallest unit of Ether), requiring conversion if needed for display in other units.

### **IX. SDK & Development Environment (Demo)**

47. **Question:** What npm packages were installed during the demo setup?
    *   **Answer:** `@axelar-network/axelar-gmp-sdk` (specifically v5 was mentioned), `ethers`, and `dotenv`.

48. **Question:** Which two main Axelar SDK objects were initialized in the demo script?
    *   **Answer:** An object for the `AxelarGMPRecoveryAPI` (named `recoverSdk`) and an object for the `AxelarQueryAPI` (named `axelarQueryApi`). Both were initialized for the 'testnet' environment.

49. **Question:** Why was the `dotenv` package used in the demo?
    *   **Answer:** To load environment variables, specifically the user's private key (`PRIVATE_KEY`), from a `.env` file securely, rather than hardcoding it in the script.

50. **Question:** What was a key learning point regarding chain names when using the SDK, as demonstrated by the `estimateGasFee` error?
    *   **Answer:** Chain names are case-sensitive and must match the specific identifiers used by Axelar (e.g., "ethereum-sepolia" instead of "ethereum"). The SDK provided helpful error messages suggesting the correct name. The speaker also pointed to the Axelar documentation as a place to find the correct chain names.

### **X. Support & Resources**

51. **Question:** Where can developers get support or ask questions if they run into issues while building with Axelar?
    *   **Answer:** The speaker mentioned two primary resources:
        *   The Axelar Support repo on GitHub (`axelar-core/axelar-support-rollups` likely, though path wasn't fully shown) where they can open issues for prompt responses.
        *   The Axelar Discord server.

52. **Question:** What resource was mentioned that provides code examples for various scenarios (EVM, Cosmos, Sui) using Axelar?
    *   **Answer:** The Axelar Examples repository (`axelar-network/axelar-examples` likely). It includes examples that can be tested locally.

***



---

## **Section 1: Axelar Ecosystem Basics**

### **1. What is the Axelar Network?**
- **Answer:** Axelar is a decentralized interoperability network that enables cross-chain communication and asset transfers. It provides tools like **General Message Passing (GMP)** and **Axelar Scan** to simplify cross-chain development and troubleshooting.

### **2. What is the role of a Developer Advocate at Axelar?**
- **Answer:** A Developer Advocate (e.g., Idris Oli) educates developers on Axelar’s tools (like GMP and Axelar Scan), creates tutorials/demos, and helps troubleshoot cross-chain transactions.

---

## **Section 2: Axelar General Message Passing (GMP)**

### **3. What is Axelar GMP?**
- **Answer:** GMP allows developers to send messages and assets across blockchains. Key functions include:
  - **`callContract`**: Send messages with custom logic.
  - **`callContractWithToken`**: Send messages + tokens (e.g., USDC).

### **4. Explain the difference between `callContract` and `callContractWithToken`.**
- **Answer:**
  - **`callContract`**: Sends a message (payload) to a destination chain contract. Example: Triggering a smart contract function on another chain.
  - **`callContractWithToken`**: Sends a message **and** tokens (e.g., transferring USDC from Ethereum to Avalanche).

### **5. Describe the end-to-end flow of a GMP transaction.**
- **Answer:**
  1. User initiates a transaction on the source chain (e.g., Ethereum).
  2. Axelar Gateway emits an event picked up by Axelar validators.
  3. Validators vote on the transaction’s validity.
  4. Approved transaction is relayed to the destination chain (e.g., Avalanche).
  5. Destination Gateway triggers the `_execute` or `_executeWithToken` function.

---

## **Section 3: Axelar Scan**

### **6. What is Axelar Scan?**
- **Answer:** A portal for tracking cross-chain transactions. It visualizes steps like **sent**, **gas paid**, **confirmed**, and **executed**, and offers recovery options for stuck transactions.

### **7. How do you recover a transaction stuck due to insufficient gas?**
- **Answer:**
  1. Navigate to the transaction on [Axelar Scan](https://scan.axelar.dev/).
  2. Click **Recovery** > **Add Gas**.
  3. Approve the gas payment via your wallet.

### **8. What information can you find on Axelar Scan’s transaction details page?**
- **Answer:**  
  - Source/destination chains.
  - Transaction status (e.g., "waiting for finality").
  - Gas fees paid.
  - Links to on-chain explorers (e.g., SnowTrace for Avalanche).

---

## **Section 4: GMP Recovery API & Troubleshooting**

### **9. How do you query a transaction’s status using the SDK?**
- **Answer:**
  ```javascript
  const { status } = await sdk.queryTransactionStatus("0x...txHash");
  console.log(status); // Returns steps like "sent" or "executed"
  ```

### **10. What is the purpose of the `manualRelayToDestinationChain` function?**
- **Answer:** It manually relays a transaction stuck at the **confirm** or **approve** step. Example use case: Validator delays on the destination chain.

### **11. How do you add native gas programmatically?**
- **Answer:**
  ```javascript
  await recoverySDK.addNativeGas({
    chain: "avalanche",
    txHash: "0x...",
    gasLimit: "100000",
    refundAddress: "0x...",
  });
  ```

### **12. What is the refund mechanism in Axelar?**
- **Answer:** If users overpay gas, excess funds are refunded to a specified address after transaction completion.

---

## **Section 5: Axelar Query API**

### **13. How do you estimate gas fees for a cross-chain transaction?**
- **Answer:**
  ```javascript
  const fee = await querySDK.estimateGasFee({
    sourceChain: "ethereum",
    destinationChain: "avalanche",
    gasLimit: "500000",
  });
  ```

### **14. How do you convert a token symbol (e.g., USDC) to its denom?**
- **Answer:**
  ```javascript
  const denom = await querySDK.getDenomFromSymbol("USDC", "moonbeam");
  // Returns "uusdc" (Axelar-denominated USDC).
  ```

---

## **Section 6: Demo & Code Implementation**

### **15. What packages are required to interact with Axelar’s SDK?**
- **Answer:** Install via npm:
  ```bash
  npm install @axelar-network/axelarjs-sdk ethers dotenv
  ```

### **16. How do you initialize the Axelar SDK?**
- **Answer:**
  ```javascript
  const { AxelarGMPRecoveryAPI, AxelarQueryAPI } = require("@axelar-network/axelarjs-sdk");
  const recoverySDK = new AxelarGMPRecoveryAPI({ environment: "testnet" });
  const querySDK = new AxelarQueryAPI({ environment: "testnet" });
  ```

### **17. Write a script to track a transaction’s status.**
- **Answer:**
  ```javascript
  const txHash = "0x...";
  const { status, logs } = await recoverySDK.queryTransactionStatus(txHash);
  console.log(`Status: ${status}`);
  ```

---

## **Section 7: Advanced Scenarios**

### **18. A transaction is stuck at “gas paid” but not confirmed. How do you resolve this?**
- **Answer:** Use `manualRelayToDestinationChain` to force the transaction to proceed.

### **19. How does Axelar handle gas across multiple chains?**
- **Answer:** Users pay gas once on the source chain. Axelar’s **Gas Service** covers destination chain fees, avoiding multi-chain gas management.

### **20. What are common reasons for transaction failures?**
- **Answer:**
  - Insufficient gas.
  - Invalid destination address.
  - Validator consensus failures.

---

## **Section 8: Best Practices**

### **21. How do you ensure smooth cross-chain UX for users?**
- **Answer:**
  - Use `estimateGasFee` to avoid overpayment.
  - Integrate Axelar Scan for real-time tracking.
  - Implement auto-recovery for stuck transactions.

### **22. Why is testing on Axelar’s testnet critical?**
- **Answer:** Testnet (e.g., `testnet.axelarscan.io`) allows developers to simulate cross-chain flows without risking real assets.

---

## **Section 9: Use Cases**

### **23. Give an example of a GMP use case.**
- **Answer:** Distributing rewards to users on 10 chains from a single source chain transaction.

### **24. How would you build a cross-chain NFT bridge with Axelar?**
- **Answer:**
  1. Lock NFT on source chain.
  2. Use `callContract` to trigger minting on destination chain.
  3. Validate via Axelar validators.

---

## **Section 10: Miscellaneous**

### **25. Where can developers find Axelar documentation?**
- **Answer:** [Axelar Docs](https://docs.axelar.dev/) and [GitHub Examples](https://github.com/axelarnetwork/axelar-examples).

---

**Total Questions:** 25 (Expand further by diving deeper into SDK methods, security practices, and validator roles.)