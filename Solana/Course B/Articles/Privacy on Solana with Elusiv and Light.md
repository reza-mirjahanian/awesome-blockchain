-   [A Cypherpunk's Manifesto](https://www.activism.net/cypherpunk/manifesto.html)
-   [Elusiv](https://app.elusiv.io/)
-   [Elusiv Docs](https://docs.elusiv.io/)‍
-   [Light Protocol](https://docs.lightprotocol.com/getting-started/overview)
-   [zk.js](https://github.com/Lightprotocol/light-protocol/tree/main/light-zk.js)
-   [Private Solana Program Example Implementations](https://github.com/Lightprotocol/psp-examples/tree/main)


---

### 🔑 Core Concepts & Insights

#### 1. 🛡️ **Privacy Is Essential**

* **Privacy** is a *fundamental human right*, tied to autonomy, dignity, and freedoms like speech and association.
* Public blockchains expose **transaction details** (public keys, balances, code).
* Even ordinary users face risks of **tracking and exploitation** without privacy layers.
* Transparency without privacy can erode personal sovereignty.

---

#### 2. ⚡ **Elusiv — Privacy Lite**

* Provides private SOL transfers via a **shared pooled fund model**.
* Accessed through a **TypeScript SDK**, simplifying developer integration.
* **Abstraction over privacy**—easy to use, but less customizable.
* Ideal for **simple applications needing private token transfers**.

---

#### 3. 🔒 **Light Protocol — Next-Gen zk Layer**

* Built around **three pillars**:

  * **Private state**: On-chain state encrypted; only the user can decrypt.
  * **UTXO model**: Shielded balances in SOL and SPL tokens.
  * **Client-side zk-SNARK circuits**: Proofs generated locally, verified on-chain.
* Enables **Custom Private Solana Programs (PSPs)** with granular state control.
* Offers **greater flexibility** but requires **more complex setup**.

---

#### 4. ⚙️ **Implementation Differences**

* **Elusiv**

  * SDK-driven, automatic pooled privacy.
  * Seamless top-ups of private balances.
  * Minimal setup, limited control.

* **Light Protocol**

  * Involves **wallet creation, relayer setup, provider & user initialization**.
  * Workflow includes:

    1. Generate wallets.
    2. Fund with test SOL.
    3. Configure relayer for zk proofs.
    4. Initialize provider & user context.
    5. Use `shield()` for private balances and `transfer()` for private sends.
  * Runs locally on a **test validator**, with multiple dependencies.
  * Provides **deep control** for custom private apps.

---

#### 5. 🎯 **Choosing the Right Tool**

* Use **Elusiv** if:

  * You want **fast, easy privacy integration**.
  * You don’t need control over private state logic.
* Use **Light Protocol** if:

  * You require **customizable privacy logic**.
  * You can handle **greater complexity and setup overhead**.

---

### ✨ Quick-Review Cheatsheet

| 🧩 Term / Concept   | 📖 Meaning                                            |
| ------------------- | ----------------------------------------------------- |
| **Privacy**         | *Protection of autonomy and freedom*                  |
| **Elusiv**          | *Simple pooled privacy via SDK*                       |
| **Light Protocol**  | *Encrypted state, UTXO model, zk-SNARKs*              |
| **PSP**             | *Custom Private Solana Programs*                      |
| **zk-SNARKs**       | *Zero-knowledge proofs for validation*                |
| **UTXO Model**      | *Shielded outputs for SOL/SPL*                        |
| **Pooled model**    | *Collective privacy management in Elusiv*             |
| **Local test flow** | *Light Protocol’s dev setup with relayer & validator* |

---

🧠 **Privacy on Solana with Elusiv and Light: In-Depth Line-by-Line Analysis**

---

### 🛠️ **Prerequisites**

- **Node.js installed**  
  ➤ Required to install dependencies, run scripts, and execute private $SOL transactions.  
  ➤ Ensures the development environment supports modern JavaScript/TypeScript tooling.

- **Basic understanding of TypeScript**  
  ➤ The code examples are written in TypeScript, so familiarity is essential for comprehension.  
  ➤ Enables developers to follow type annotations, async/await patterns, and module imports.

- **Basic understanding of `@solana/web3.js`**  
  ➤ This is the standard Solana SDK for interacting with the blockchain.  
  ➤ Necessary for creating connections, keypairs, transactions, and sending requests to the network.

---

### 📄 **What is this article about?**

🔍 **Core Theme**:  
Blockchain transparency vs. user privacy — a foundational tension in decentralized systems.

🌐 **Public Ledger Reality**:  
- All transactions, account balances, and often program source code are publicly visible.  
- While praised for transparency, this openness can compromise user privacy.

🛡️ **Solution Focus**:  
- **Elusiv** and **Light Protocol** are presented as cutting-edge privacy layers for Solana.  
- Both use **zk-SNARKs** (Zero-Knowledge Succinct Non-Interactive Arguments of Knowledge) to enable:
  - ✅ Private transactions
  - ✅ Private program execution
  - ✅ Decentralized compliance

📘 **Article Structure**:  
1. Importance of privacy  
2. Why Solana needs privacy  
3. Step-by-step guide to sending $SOL privately using:
   - Elusiv’s TypeScript SDK
   - Light Protocol’s `zk.js` module

---

### 🔐 **Privacy is Important**

⚖️ **Fundamental Human Right**:  
- Privacy is not just a preference — it's tied to autonomy and human dignity.  
- It enables individuals to think, speak, and act without constant surveillance.

🕊️ **Enables Other Freedoms**:  
- **Freedom of speech** and **freedom of association** depend on the ability to operate without monitoring.  
- Without privacy, dissenting opinions or controversial associations can be suppressed.

🚨 **Real-World Triggers**:  
- **Edward Snowden’s disclosures**: Revealed mass surveillance by governments.  
- **WikiLeaks**: Showed how sensitive information can be exposed.  
- **Facebook-Cambridge Analytica scandal**: Demonstrated how personal data can be weaponized.

💭 **Takeaway**:  
> Privacy creates a safe space away from political and social scrutiny — essential for a free society.

---

### 🧑‍🤝‍🧑 **Everyone Should Care About Privacy**

🤔 **Common Misconception**:  
> “I have nothing to hide, so why care about privacy?”

🧩 **Thought Experiment**:  
Introduces two characters:

- **Alice**: An average user who values privacy for everyday purchases.
- **MA (Malicious Actor)**: Could be:
  - 👮‍♂️ Oppressive government
  - 💼 Controlling employer
  - 🕵️‍♂️ Cybercriminal

🔁 **Sequence of Events**:

1. **Alice makes a purchase** on Solana using her public key.
2. **Transaction is public**: Includes sender, receiver, amount.
3. **MA traces Alice’s public key** across the blockchain.
4. **MA identifies “objectionable” transaction** (e.g., political donation, banned book).
5. **MA takes action**: Imprisonment, job loss, blackmail, or social shaming.

📉 **Implication**:  
Even legal actions can be punished in authoritarian or restrictive environments.

🌐 **Broader Risk**:  
- Rise of **on-chain sleuths** and **data-harvesting bots** makes transaction tracing trivial.
- Without privacy, **individual sovereignty erodes**.
- Every decision becomes monitored and potentially monetized.

🎯 **Conclusion**:  
> Privacy is not just for criminals — it’s for everyone who values freedom from coercion and judgment.

---

### 🔍 **What is Elusiv?**

🏷️ **Self-Description**:  
> “A blockchain-based, zero-knowledge privacy protocol enabling universal encryption.”

🎯 **Mission**:  
- Provide privacy **without sacrificing**:
  - Security
  - Safety
  - Decentralization

🔐 **Core Technology**:  
- Uses **zero-knowledge cryptography**
- Employs **multi-party computation (MPC)**

🧩 **Universal Encryption Goal**:  
- Users control what data is shared on-chain.
- Users decide what remains private.

---

#### ⚙️ **How does Elusiv work?**

🏦 **Shared Pool Model**:

- Users send funds to a **shared pool** governed by the Elusiv program.
- Pool tops up their **private balances**.
- From the pool, users can:
  - Send tokens (SOL, USDC, etc.)
  - Receive tokens
  - Withdraw funds

👀 **Visibility Note**:  
- Deposits to and withdrawals from the pool are **visible on-chain**.
- But **linking deposits to withdrawals** becomes hard with more users and transactions.

👥 **Anonymity Set**:  
> “Anonymity loves company.”  
- With millions of users, it's hard to trace who sent what.
- A single user depositing and withdrawing 10 SOL is easily linked.
- In a large pool, the same action is obscured by volume.

🔢 **Zero-Knowledge Proofs (zk-SNARKs)**:

- Funds in the pool have **no depositor info**.
- But each user has **secret values** tied to their funds.
- These allow **proofs of authority** over unspent funds — without revealing identity or balance.

🔐 **zk-SNARK Analogy**:

> Imagine proving you know a password to a friend —  
> without telling them the password —  
> and they can verify it instantly.

🧠 **Technical Note**:  
- zk-SNARKs allow Elusiv to track private balances without exposing them.
- The network verifies correctness without seeing the data.

📚 **Suggested Learning**:  
- Porter’s class on **Zero-Knowledge Proofs** (mentioned as recommended further study).

---

### 🔦 **What is the Light Protocol?**

🏷️ **Self-Description**:  
> “Next-gen zkLayer on Solana” — enabling **private program execution** directly on Solana.

🔑 **Three Core Concepts**:

1. **Private State**  
   ➤ On-chain state is encrypted.  
   ➤ Only the owner can decrypt it.

2. **UTXO Model**  
   ➤ Unlike account-based models, uses **Unspent Transaction Outputs**.  
   ➤ Each transaction creates a new UTXO → appended to a list.  
   ➤ Avoids in-place updates → prevents state leakage.

3. **Verifying Private State Transitions On-Chain**  
   ➤ Zero-knowledge proofs verify state changes.  
   ➤ Proofs are checked on-chain without revealing the data.

---

#### ⚙️ **How does the Light Protocol work?**

🔒 **Encrypted On-Chain State**:

- Users **own** their encrypted state.
- Only they have **decryption rights**.

🧩 **UTXO-Based Design**:

- Each UTXO is tied to a **shielded keypair**.
- Can hold:
  - SOL (fee token)
  - Another SPL token
- New UTXO created per transaction → added to linear list.
- No in-place updates → no metadata leakage about who updated what.

🧠 **Why UTXO?**  
> Prevents observers from knowing which state was modified and by whom.

🧮 **Program Logic as zk Circuits**:

- Instead of on-chain logic, **program logic is encoded into zk-SNARK circuits**.
- Computation and proof generated **on the client side**.
- Proof submitted on-chain.

✅ **On-Chain Verification**:

1. A **dedicated verifier program** checks the proof.
2. If valid, it calls a **native verifier** for security checks.
3. New commitments are added to **Light’s Merkle tree program**.

🧰 **Custom Private Solana Programs (PSPs)**:

- Developers can build **custom PSPs from scratch**.
- Requires deeper understanding.
- Tutorial available in Light’s documentation.

💡 **Note**:  
Creating custom PSPs is **out of scope** for this article — recommended for advanced users.

---

### 🧪 **Sending SOL Privately using Elusiv**

#### 💻 **The Full Code**

```ts
import * as ed from "@noble/ed25519";
import { sha512 } from "@noble/hashes/sha512";
ed.etc.sha512Sync = (...m) => sha512(ed.etc.concatBytes(...m));
```

🔐 **Why this block?**  
- Uses `@noble/ed25519` for signing (no wallet adapter in this script).
- Solves a common error: `etc.sha512Sync is not set`.
- Manually sets `sha512Sync` to prevent crash during signing.

---

```ts
import {
  Connection,
  PublicKey,
  Keypair,
  ConfirmedSignatureInfo,
  Cluster,
  LAMPORTS_PER_SOL,
  clusterApiUrl,
} from "@solana/web3.js";
import { Elusiv, TokenType, SEED_MESSAGE } from "@elusiv/sdk";
```

📦 **Imports**:
- `@solana/web3.js`: Core Solana interaction.
- `@elusiv/sdk`: Elusiv’s TypeScript SDK.
- `TokenType`: Enum for token types (SOL, USDC, etc.).
- `SEED_MESSAGE`: Required message to generate deterministic seed.

---

```ts
const CLUSTER: Cluster = "devnet";
const RPC = "https://rpc-devnet.helius.xyz";
```

🌐 **Environment Setup**:
- Targets **devnet** for testing.
- Uses **Helius RPC** for faster, reliable access.
- For mainnet: change `CLUSTER` to `"mainnet"` and use a Helius API key.

---

```ts
const main = async () => {
  const connection: Connection = new Connection(RPC);
  const keyPair: Keypair = generateKeypair();
  const recipientPublicKey: PublicKey = new PublicKey(generateKeypair().publicKey);
  await airdropSol(keyPair);
  const seed: Uint8Array = ed.sign(Buffer.from(SEED_MESSAGE, "utf-8"), keyPair.secretKey.slice(0, 32));
  const elusiv: Elusiv = await Elusiv.getElusivInstance(seed, keyPair.publicKey, connection, CLUSTER);
  const balance: bigint = await elusiv.getLatestPrivateBalance("LAMPORTS");
```

🔁 **Main Function Flow**:

1. **Establish connection** to devnet via Helius RPC.
2. **Generate sender keypair** using `generateKeypair()`.
3. **Generate random recipient public key**.
4. **Airdrop 1 SOL** to sender for testing.
5. **Create seed**:
   - Sign `SEED_MESSAGE` with private key (first 32 bytes).
   - Used to derive deterministic private keys for Elusiv.
6. **Initialize Elusiv instance** with:
   - Seed
   - Public key
   - Connection
   - Cluster
7. **Fetch private balance** in Lamports (returned as `BigInt`).

---

```ts
  try {
    if (balance > BigInt(0)) {
      const signature = await send(elusiv, recipientPublicKey, 1 * LAMPORTS_PER_SOL, "LAMPORTS");
      console.log(`Sent with signature ${signature.signature}`);
    } else {
      console.log("Private balance is empty. Topping up...");
      const amount = 1 * LAMPORTS_PER_SOL;
      const tokenType = "LAMPORTS";
      const topUpTxData = await elusiv.buildTopUpTx(amount, tokenType);
      topUpTxData.tx.partialSign(keyPair);
      const topUpSig: ConfirmedSignatureInfo = await elusiv.sendElusivTx(topUpTxData);
      console.log(`Top-up complete with signature ${topUpSig.signature}`);
      const signature: ConfirmedSignatureInfo = await send(elusiv, recipientPublicKey, 1 * LAMPORTS_PER_SOL, "LAMPORTS");
      console.log(`Sent with signature ${signature.signature}`);
    }
  } catch (e) {
    console.error(`Error sending SOL via Elusiv: ${e}`);
  } finally {
    console.log("Exiting program...");
    process.exit(0);
  }
};
```

🚦 **Conditional Logic**:

- **If private balance > 0**:
  - Directly send 1 SOL privately using `send()` function.
- **Else**:
  - Log: “Private balance is empty. Topping up...”
  - Build **top-up transaction**:
    - `buildTopUpTx()` → moves public SOL to private pool.
    - `partialSign()` → signs with sender’s keypair.
    - `sendElusivTx()` → sends and confirms.
  - After top-up, send 1 SOL privately.

🚫 **Error Handling**:
- `try-catch`: Catches any errors during transaction.
- `finally`: Ensures program exits cleanly.

---

```ts
const send = async (
  elusiv: Elusiv,
  recipient: PublicKey,
  amount: number,
  tokenType: TokenType
): Promise => {
  const txt = await elusiv.buildSendTx(amount, recipient, tokenType);
  return elusiv.sendElusivTx(txt);
};
```

📤 **Send Function**:
- Builds private send transaction using `buildSendTx()`.
- Sends via `sendElusivTx()`.
- Returns `ConfirmedSignatureInfo`.

---

```ts
const generateKeypair = (): Keypair => {
  let keyPair = Keypair.generate();
  console.log(`Public key: ${keyPair.publicKey.toBase58()}`);
  console.log(`Private key: ${keyPair.secretKey}`);
  return keyPair;
};
```

🔑 **Keypair Generation**:
- Generates new Solana keypair.
- Logs public and private keys (⚠️ **only for devnet**).
- **Warning**: Never log private keys in production.

---

```ts
const airdropSol = async (wallet: Keypair) => {
  try {
    const connection = new Connection(clusterApiUrl("devnet"));
    const airdropSignature = await connection.requestAirdrop(new PublicKey(wallet.publicKey), 1 * LAMPORTS_PER_SOL);
    const latestBlockHash = await connection.getLatestBlockhash();
    await connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: airdropSignature,
    });
    console.log(`Airdropped 1 SOL to ${wallet.publicKey.toBase58()}`);
  } catch (e) {
    console.error(`Error sending SOL via Elusiv: ${e}`);
  }
};
```

🌧️ **Airdrop Function**:
- Uses Solana’s devnet airdrop.
- Confirms transaction with blockhash and last valid block height.
- Ensures funds are confirmed before proceeding.

---

### 🧪 **Sending SOL Privately using Light Protocol**

#### 💻 **The Full Code**

```ts
import { BN } from "@coral-xyz/anchor";
import { PublicKey, Keypair, Connection } from "@solana/web3.js";
import { User, Provider as LightProvider, TestRelayer, confirmConfig, airdropSol } from "@lightprotocol/zk.js";
```

📦 **Imports**:
- `BN`: Big number handling (from Anchor).
- `@solana/web3.js`: Standard Solana types.
- `@lightprotocol/zk.js`: Light’s privacy SDK.
- Key classes: `User`, `LightProvider`, `TestRelayer`.

---

```ts
const initializeSolanaWallet = async (): Promise => {
  const wallet = Keypair.generate();
  console.log("Wallet initialized");
  return wallet;
};
```

👛 **Wallet Initialization**:
- Generates new Solana wallet.
- Logs success.

---

```ts
const requestAirdrop = async (connection: Connection, publicKey: PublicKey): Promise => {
  await airdropSol({
    connection,
    lamports: 2e9,
    recipientPublicKey: publicKey,
  });
  console.log("Airdrop requested...");
};
```

🌧️ **Airdrop Function**:
- Uses Light’s built-in `airdropSol`.
- Sends **2 billion lamports (2 SOL)**.
- More than Elusiv due to higher fees.

---

```ts
const setupTestRelayer = async (solanaWallet: Keypair): Promise => {
  const testRelayer = new TestRelayer({
    relayerPubkey: solanaWallet.publicKey,
    relayerRecipientSol: solanaWallet.publicKey,
    relayerFee: new BN(100_000),
    payer: solanaWallet,
  });
  console.log("Test relayer initialized");
  return testRelayer;
};
```

📡 **Relayer Setup**:
- **Relayer role**: Submits ZK proofs to chain.
- Pays transaction fees in exchange for a fee (100k lamports).
- `TestRelayer` used for local testing.
- Uses same wallet as payer and recipient.

---

```ts
const initializeLightProvider = async (solanaWallet: Keypair, testRelayer: TestRelayer): Promise => {
  const lightProvider = await LightProvider.init({
    wallet: solanaWallet,
    relayer: testRelayer,
    confirmConfig,
  });
  console.log("Light provider initialized");
  return lightProvider;
};
```

🔌 **Light Provider**:
- Acts as a bridge between wallet and Light network.
- Initialized with:
  - Wallet
  - Relayer
  - `confirmConfig` (transaction confirmation settings)

---

```ts
const initializeLightUser = async (lightProvider: LightProvider): Promise => {
  const user = await User.init({ provider: lightProvider });
  console.log("Light user initialized");
  return user;
};
```

👤 **Light User**:
- Represents a user in the Light ecosystem.
- Initialized with a provider.
- Enables private operations (shield, transfer).

---

```ts
const performShieldOperation = async (user: User) => {
  await user.shield({
    publicAmountSol: "1.1",
    token: "SOL",
  });
  console.log("Performed shield operation");
};
```

🛡️ **Shield Operation**:
- Converts public SOL into **private, shielded SOL**.
- Uses `publicAmountSol: "1.1"` to cover fees.
- `token: "SOL"` specifies asset type.

---

```ts
const executePrivateTransfer = async (user: User, testRecipientPublicKey: string) => {
  const response = await user.transfer({
    amountSol: "1",
    token: "SOL",
    recipient: testRecipientPublicKey,
  });
  console.log(`Executed private transfer! Txt hash: ${response.txHash}`);
};
```

📨 **Private Transfer**:
- Sends **1 shielded SOL** to recipient.
- Recipient must also be a Light user.
- Returns transaction hash.

---

```ts
const main = async () => {
  try {
    const solanaWallet = await initializeSolanaWallet();
    const connection = new Connection("http://127.0.0.1:8899");
    await requestAirdrop(connection, solanaWallet.publicKey);
    const testRelayer = await setupTestRelayer(solanaWallet);
    const lightProvider = await initializeLightProvider(solanaWallet, testRelayer);
    const user = await initializeLightUser(lightProvider);
    await performShieldOperation(user);
    const testRecipientKeypair = Keypair.generate();
    await requestAirdrop(connection, testRecipientKeypair.publicKey);
    const lightProviderRecipient = await initializeLightProvider(testRecipientKeypair, testRelayer);
    const testRecipient = await initializeLightUser(lightProviderRecipient);
    await executePrivateTransfer(user, testRecipient.account.getPublicKey());
    console.log(`Successfully sent 1 $SOL to ${testRecipient.account.getPublicKey()} privately!`);
  } catch (e) {
    console.error(`Error sending SOL via Light: ${e}`);
  } finally {
    console.log("Exiting program...");
    process.exit(0);
  }
};
```

🔁 **Main Function Flow**:

1. Initialize sender wallet.
2. Connect to **localhost validator** (`http://127.0.0.1:8899`).
3. Airdrop SOL to sender.
4. Set up **test relayer**.
5. Initialize **Light provider and user**.
6. **Shield 1.1 SOL** → convert to private.
7. Generate **recipient wallet**.
8. Airdrop to recipient.
9. Initialize **recipient as Light user**.
10. **Privately transfer 1 SOL**.
11. Log success.
12. Error handling and clean exit.

---

### ⚠️ **Important Note: Light Protocol Deployment Status**

🚧 **Not on Devnet/Mainnet**:
- At time of writing, Light Protocol is **not deployed** on devnet or mainnet.
- Helius RPC keys **won’t work**.

🧪 **Local Testing Only**:
- Must run **local test validator**.
- Command: `light test-validator`
- Starts a Solana validator with preloaded Light programs.

🔧 **Dependency Note**:
- Must manually install `chai` (assertion library) due to unit tests in UTXO class.
- Expected to be fixed in next release.

📦 **Required `package.json` Dependencies**:

```json
{
  "devDependencies": {
    "@lightprotocol/cli": "^0.1.1-alpha.21",
    "@types/node": "^20.6.0",
    "chai": "^4.3.8",
    "typescript": "^5.2.2"
  },
  "dependencies": {
    "@coral-xyz/anchor": "^0.28.1-beta.2",
    "@lightprotocol/zk.js": "^0.3.2-alpha.14",
    "@solana/web3.js": "^1.78.4"
  }
}
```