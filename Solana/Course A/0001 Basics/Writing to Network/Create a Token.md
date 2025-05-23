1.  Invoke the System Program to create a new account.
2.  Invoke the Token Extensions Program to initialize that account as a Mint.

By combining both instructions into a single transaction, you ensure that the account creation and initialization happen atomically. Either both instructions succeed, or neither does. This approach is common when building more complex Solana transactions, as it guarantees that all instructions execute together.

```ts
import {
  Connection,
  Keypair,
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction,
  LAMPORTS_PER_SOL
} from "@solana/web3.js";
import {
  MINT_SIZE,
  TOKEN_2022_PROGRAM_ID,
  createInitializeMint2Instruction,
  getMinimumBalanceForRentExemptMint
} from "@solana/spl-token";

const connection = new Connection("http://localhost:8899", "confirmed");

const wallet = new Keypair();
// Fund the wallet with SOL
const signature = await connection.requestAirdrop(
  wallet.publicKey,
  LAMPORTS_PER_SOL
);
await connection.confirmTransaction(signature, "confirmed");

// Generate keypair to use as address of mint account
const mint = new Keypair();

// Calculate lamports required for rent exemption
const rentExemptionLamports =
  await getMinimumBalanceForRentExemptMint(connection);

// Instruction to create new account with space for new mint account
// Create an instruction to create a new account

// Allocate the required space for storing mint data
// Transfer lamports from the wallet to fund the new account
// Assign ownership of the account to the Token Extensions program (TOKEN_2022_PROGRAM_ID)

const createAccountInstruction = SystemProgram.createAccount({
  fromPubkey: wallet.publicKey,
  newAccountPubkey: mint.publicKey,
  space: MINT_SIZE,
  lamports: rentExemptionLamports,
  programId: TOKEN_2022_PROGRAM_ID
});

// Instruction to initialize mint account
const initializeMintInstruction = createInitializeMint2Instruction(
  mint.publicKey,
  2, // decimals
  wallet.publicKey, // mint authority
  wallet.publicKey, // freeze authority
  TOKEN_2022_PROGRAM_ID
);

// Build transaction with instructions to create new account and initialize mint account
const transaction = new Transaction().add(
  createAccountInstruction,
  initializeMintInstruction
);

const transactionSignature = await sendAndConfirmTransaction(
  connection,
  transaction,
  [
    wallet, // payer
    mint // mint address keypair
  ]
);

console.log("Mint Account:", `${mint.publicKey}`);
console.log("Transaction Signature:", `${transactionSignature}`);
```