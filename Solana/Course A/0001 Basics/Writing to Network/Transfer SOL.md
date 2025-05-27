In this example, you'll learn how to transfer SOL between two accounts.

On Solana, each account has a specific program as its owner. Only the program owner can deduct an account's SOL (lamport) balance.

The System Program is the [owner](https://github.com/anza-xyz/agave/blob/v2.1.11/sdk/account/src/lib.rs#L55) for all "wallet" accounts. To transfer SOL, you must invoke the System Program's [transfer](https://github.com/anza-xyz/agave/blob/v2.1.11/programs/system/src/system_processor.rs#L183-L213) instruction.


```ts
import {
  LAMPORTS_PER_SOL,
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction,
  Keypair,
  Connection
} from "@solana/web3.js";

const connection = new Connection("http://localhost:8899", "confirmed");

const sender = new Keypair();
const receiver = new Keypair();

const signature = await connection.requestAirdrop(
  sender.publicKey,
  LAMPORTS_PER_SOL
);
await connection.confirmTransaction(signature, "confirmed");

const transferInstruction = SystemProgram.transfer({
  fromPubkey: sender.publicKey,
  toPubkey: receiver.publicKey,
  lamports: 0.01 * LAMPORTS_PER_SOL
});

const transaction = new Transaction().add(transferInstruction);

const transactionSignature = await sendAndConfirmTransaction(
  connection,
  transaction,
  [sender]
);

console.log("Transaction Signature:", `${transactionSignature}`);
```