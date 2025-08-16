# 🏗️ Solana Dev Crash-Course
![alt text](image-8.png)
![alt text](image-9.png)
---

## 🌐 Clusters – Pick Your Playground  

| Cluster | Purpose | RPC Example |
|---------|---------|-------------|
| **localhost** 🔧 | Dev loop on port `8899` | `http://127.0.0.1:8899` |
| **devnet** 🧪 | Risk-free testing | `https://api.devnet.solana.com` |
| **testnet** 🏁 | Core-team stress tests | `https://api.testnet.solana.com` |
| **mainnet-beta** 🚀 | Real money, real users | `https://api.mainnet-beta.solana.com` |

> *Each cluster is isolated; sending a tx to the wrong one is instantly rejected.*

---

## 📦 Accounts – The Only State

> “Everything is an account.”

### 3️⃣ Flavors
- **Data Account** 🗃️ – plain storage (tokens, user data)
- **Program Account** ⚙️ – executable code (smart contract)
- **Native Program** 🔏 – built-ins (`System`, `BPF Loader`, `Vote`)

### Anatomy
```rust
pub struct AccountInfo {
    key: Pubkey,          // 32-byte address
    lamports: u64,        // balance (1 SOL = 1 B lamports)
    data: Vec<u8>,        // raw bytes
    owner: Pubkey,        // program that controls it
    executable: bool,     // true if it holds code
    rent_epoch: u64,      // next rent due
    is_signer, is_writable // tx flags
}
```

#### 🔑 Rules
- Only **owner** can mutate data or withdraw `lamports`.
- Anyone can **deposit**.
- Transfer ownership → zero-out data first.

---

## 💸 Rent = Storage Fee

> *No rent-paying accounts on mainnet anymore; all must be **rent-exempt**.*

- **Exemption cost** = `getMinimumBalanceForRentExemption(size)`  
- **Close account** → reclaim lamports, data purged.

---

## 🔐 Address Types

- **Ed25519 keypair** – standard 32-byte pubkey *(has private key)*
- **PDA** *(Program Derived Address)* – off-curve, no priv-key  
  Created via `findProgramAddress`.

---

## 🧩 Programs vs Ethereum Contracts

| Feature | Solana | Ethereum |
|---------|--------|----------|
| **Code & State** | Split (program ≠ data) | Coupled |
| **Upgrades** | Program authority can redeploy | Proxy pattern or immutability |
| **State Model** | Stateless programs touch many data accounts | Stateful contract |
| **Cost** | Predictable rent exemption | Gas per opcode/storage |

---

## 📨 Transactions

### Structure
```rust
pub struct Transaction {
    signatures: Vec<Signature>,
    message: Message,   // accounts + instructions + blockhash
}
```

### 🧩 Instruction (smallest unit)
```rust
pub struct Instruction {
    program_id: Pubkey,
    accounts: Vec<AccountMeta>, // read/write flags
    data: Vec<u8>,              // custom payload
}
```

> Multiple instructions execute **atomically & sequentially**.

---

## 🆕 Versioned Transactions (v0)
![alt text](image-10.png)

- **Limit** → 1232 bytes (IPv6 MTU)
- **Fix** → **Address Lookup Tables (ALTs)**  
  Store 32-byte addresses in an on-chain table and reference with 1-byte index.

---

## 🔄 Transaction Flow Cheat-Sheet

1. **Client builds** `Transaction` with `instructions`
2. Signs with required keys
3. Sends to RPC → validator
4. Runtime **parallelizes** read-only accounts, serializes writes
5. Accounts updated, fees burned & distributed

---

## 🧪 Quick CLI Snippets

```bash
# Rent-exempt minimum for 2 kB
solana rent 2000

# Deploy program
solana program deploy ./my_program.so

# Airdrop on devnet
solana airdrop 2
```


-   [Solana Command-line Guide](https://docs.solana.com/cli)
-   [Solana Wiki - Account Model](https://solana.wiki/zh-cn/docs/account-model/)
-   [An Introduction to Account Abstraction](https://squads.so/blog/what-is-account-abstraction-ethereum-vs-solana)
-   [Garbage Collection](https://docs.solana.com/implemented-proposals/persistent-account-storage#garbage-collection)
-   [Address Lookup Tables (ALTs)](https://docs.solana.com/developing/lookup-tables)
-   [Versioned Transactions](https://solanacookbook.com/guides/versioned-transactions.html#facts)
-   [Crate solana\_sdk](https://docs.rs/solana-sdk/latest/solana_sdk/index.html)