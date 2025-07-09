

---

## **Creating a New Program**

- **Purpose:**  
  - To start fresh and avoid issues with previous deployments.
- **Steps:**
  - **Delete the Target Folder:**  
    - This folder contains the key pair and other program artifacts.
  - **Run `anchor keys sync`:**  
    - Generates a new key pair for the program.
  - **Run Tests:**
    - Command: `anchor test --skip-local-validator`
    - Verifies that the new program builds, deploys to the test validator, and passes tests (e.g., incrementing and initializing).

---

## **Deploying to Devnet**

- **Switch Network:**
  - Use the command:  
    - `solana config set --url devnet`
  - Changes network from local to devnet.
- **Obtain Devnet SOL:**
  - Use the command:  
    - `solana airdrop 2`
  - **Note:**  
    - Can only run once per day.
- **Verify Balance:**
  - Command:  
    - `solana balance`
  - Confirm that you have the required SOL (e.g., 2 SOL).

---

## **Updating the Anchor Configuration**

- **Modify the `Anchor.toml` File:**
  - Change the configuration to target **devnet** instead of localnet.
  - Update the program address declaration to use the devnet address.

---

## **Deploying the Program to Devnet**

- **Run `anchor test`:**
  - Without the `--skip-local-validator` flag, as testing is now on devnet.
- **Deployment Outcome:**
  - Upon successful deployment, a program ID is printed.
  - **Action:**  
    - Note down the program address for future reference.
- **Public Access:**
  - The program is now publicly accessible.
  - Anyone can interact with it (e.g., increment the counter).

---

## **Managing Deployed Programs**

- **Viewing Programs:**
  - Use the command:  
    - `solana program show`
  - Displays all programs owned by the current wallet.
- **Handling Deployment Issues:**
  - **Devnet Downtime or Partial Uploads:**
    - If a program fails during upload (especially for large programs), there is a buffer.
  - **Reclaiming Devnet SOL:**
    - Use commands like:
      - `solana program close buffers`
      - `solana program close --all`
    - **Warning:**  
      - Closing all programs is risky and destroys them permanently.

---

## **Transition to Front End Deployment**

- **Tooling:**
  - Use **Vite** as a lightweight front-end tool.
  - The front end will use **React** and **TypeScript**.
- **Dependency Installation:**
  - Navigate to the front-end folder and run:
    - `yarn` (to install all dependencies)
- **Windows-Specific Setup:**
  - For Windows users on WSL, update the Vite config file:
    - Add `"server": { "hmr": { "usePolling": true } }`
  - **Purpose:**  
    - Enables hot refresh/reload in environments where direct browser connections are not possible.

---

## **Installing Solana and Wallet Adapter Libraries**

- **Libraries to Install:**
  - **Solana Web3.js**
  - **Solana Wallet Adapter Packages:**
    - `@solana/wallet-adapter-base`
    - `@solana/wallet-adapter-react`
    - `@solana/wallet-adapter-react-ui`
    - `@solana/wallet-adapter-wallets`
- **Source of Installation Instructions:**
  - Refer to the official Solana Wallet Adapter documentation for guidance.

---

## **Front End Code Integration**

- **Setting Up the Anchor Interface (IDL):**
  - **IDL File:**
    - Locate the generated IDL (e.g., `counter.ts`) from the `target` folder.
    - Copy and paste the contents into a new file named `idl.json` within a new folder called `anchor` in the front-end `src` folder.
  - **Purpose:**
    - Defines the functions, instructions, and data structures of the program.
- **Creating the Setup File:**
  - **File:** `setup.ts` in the `anchor` folder.
  - **Contents:**
    - Import necessary libraries.
    - Declare the public key of the program.
    - Connect to devnet (or update later for mainnet).
    - Derive the counter PDA and define the counter data type.
  - **Role:**
    - Acts as the connecting layer between the program and the front end.

---

## **Displaying Data from the Blockchain**

- **Creating a Component:**
  - **File:** `CounterState.tsx` in a new `components` folder.
  - **Functionality:**
    - Imports React and necessary hooks.
    - Fetches and displays the counter PDA data.
    - **Subscription:**  
      - Uses the connection object's subscription feature to listen for changes on the counter account.
      - On change, fetches and updates the displayed data.

- **Integration in the Main App:**
  - Import and render the `CounterState` component (e.g., in `App.tsx`).

---

## **Implementing the Increment Button**

- **Creating the Increment Button Component:**
  - **File:** `IncrementButton.tsx` in the `components` folder.
  - **Key Functionalities:**
    - Use the `useConnection` hook to send transactions.
    - Use the `useSendTransaction` hook from the wallet adapter.
    - Create a transaction using `program.methods.increment`.
    - The transaction requires the connected user to sign.
    - Once signed, the transaction signature is logged and the counter is incremented.
- **Placing the Component:**
  - Import and place the `IncrementButton` component next to the `CounterState` component in `App.tsx`.

---

## **Testing and Verifying Front End Integration**

- **Running the Front End:**
  - Command:  
    - `yarn dev`
  - **Expected Outcome:**
    - The main page opens at `localhost:5173`.
    - Displays a "Connect Wallet" button and initial UI text.
- **Wallet Connection:**
  - **Test with Available Wallets:**
    - E.g., Phantom, Backpack.
  - **Behavior:**
    - When a wallet is connected, the UI updates.
    - The counter value is displayed (e.g., current count is shown).
- **Increment Functionality:**
  - Click the increment button:
    - The transaction is sent.
    - The blockchain updates the counter.
    - The new counter value is logged (and visible in the browser console).
    - Example log:  
      - "Previous counter: 3" and "Current counter: 4".

---

## **Final Notes on Deployment and App Completion**

- **Full-Stack Solana Application:**
  - The guide covers building a complete full-stack application on Solana.
  - The front end interacts with the deployed program on devnet.
- **Optional Deployment to Vercel:**
  - Use the Vercel CLI with the command:  
    - `vercel`  
  - **Requirement:**  
    - Ensure the Vercel CLI is installed.
- **Documentation and Troubleshooting:**
  - If issues arise, refer to the provided guide link in the description.
  - The guide is maintained to help resolve potential errors during deployment.

