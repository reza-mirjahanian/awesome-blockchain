

##  Transaction Lifecycle:
![alt text](image.png)

* Users send transactions to the current leader, who compiles, executes, and records them in a block.

* The block is then broadcast to other validators for execution and confirmation.
![alt text](image-1.png)

--

## Wallets

* **Public keys** are 32-byte values, represented as Base58-encoded strings.
* **Private keys** (secret keys) are also 32 bytes and function as the password granting full access to an account.
* Signing with a private key is how blockchains handle authorization—anyone with the private key has complete control over the account.
* **Keypairs** are 64-byte structures, consisting of the public key (first 32 bytes) and the private key (last 32 bytes).

---
Here’s the cleaned and organized version:

---

**Public Key (Base58)**

```
FDKJvWcJNae6wecbgDYDFPCfgs14aJnVsUfWQRYWLn4Tn
```

**Keypair (Base58)**

```
3j15jr41S9KmdfughusutvvqBjAeEDbU5sDQp8EbwQ3Hify2pfM1hiEsuFFAVq8bwGywnZpswrbDzPENbBZbd5nj
```

**Keypair (Integer Array)**

```
[63,107,47,255,141,135,58,142,191,2,45,78,18,90,162,107,197,8,33,211,15,228,235,250,30,
185,122,105,23,147,115,115,86,8,155,67,155,110,51,117,0,19,150,143,217,132,205,122,91,
167,61,6,246,107,39,51,110,185,81,13,81,16,182,30,71]
```

![alt text](image-3.png)


**Key Derivation**

* Private keys can be generated from mnemonic seed phrases (12 or 24 words), commonly used for backup and recovery.
* Multiple keys can be deterministically derived from a single seed phrase.

**Cryptography**

* Solana uses **Ed25519** elliptic curve cryptography for public/private keys.
* Ed25519 offers small key/signature sizes, fast computation, and resistance to common attacks.
* Each Solana wallet address corresponds to a point on the Ed25519 curve.

**Transactions & Signing**

* Transactions are signed with the private key; signatures are verified using the public key.
* The signature ensures authenticity, prevents tampering, and uniquely identifies the transaction.
* Transactions are the only way to change state on Solana and are **atomic**—they either execute fully or fail entirely.



---

## A Solana Transaction

![alt text](image-2.png)

Here’s a cleaned and compact version while keeping it technically precise:

---

**Solana Transaction Structure**
A transaction (or **transaction message**) has four parts:

1. **Header** – References the account address list and marks which accounts must sign the transaction.
2. **Account Addresses** – Lists all accounts read or written during execution. This explicit list is unique to Solana, enabling performance optimizations but requiring developers to determine all involved accounts beforehand.
3. **Recent Blockhash** – Prevents duplicate or stale transactions. Expires after 151 blocks (\~1 minute). RPCs retry every \~2 seconds until the transaction is finalized or the blockhash expires.
4. **Instructions** – Each specifies the program to run, accounts involved, and necessary data (e.g., transfer, mint, burn, create account, close account).

**Constraints**

* **Size limit:** 1,232 bytes.
* **Account limit:** Restricted number of referenced accounts.
* **Compute limit:** Measured in **compute units (CUs)**, representing processing cost.

---

If you’d like, I can also make a **developer cheat sheet diagram** showing how these four parts fit together visually. That would make it very easy to reference.
