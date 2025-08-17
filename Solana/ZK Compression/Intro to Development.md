# 🚀 ZK Compression Dev Crash-Course

> **Prereq:** You already know Solana basics.  
> **Goal:** Ship cheaper, scalable dApps today.

---

## 1️⃣ Two Tracks to Build

| Track | What you do |
|-------|-------------|
| **Client** | Talk to compressed accounts via RPC + SDK |
| **On-chain** | Write programs that CPI into the Light system program |

---

## 2️⃣ Toolbelt 🧰

### 🔌 RPC & Indexer
- **Helius** (devnet/mainnet) – single URL gives you Solana + Photon + Prover  
- **Local** – `light test-validator` spins up everything (validator, Photon, prover)

### 📦 SDKs
| Lang | Package | Purpose |
|------|---------|---------|
| TS | `@lightprotocol/stateless.js` | Generic compression calls |
| TS | `@lightprotocol/compressed-token` | SPL-compatible compressed tokens |
| Rust | `light-sdk` | Native client |

---

## 3️⃣ Quickstart (Node / Web)

### Install
```bash
npm i -g @lightprotocol/zk-compression-cli
npm i @lightprotocol/stateless.js @lightprotocol/compressed-token @solana/web3.js
```

### Connect
```ts
import { createRpc } from '@lightprotocol/stateless.js';

const rpc = createRpc(
  'https://devnet.helius-rpc.com?api-key=<key>', // Solana + Photon + Prover
  undefined,                                     // same URL
  undefined
);
```

### Localnet (optional)
```bash
light test-validator   # starts validator + Photon + prover
```

---

## 4️⃣ Live Code – Mint & Transfer Compressed Tokens
```ts
import { createMint, mintTo, transfer } from '@lightprotocol/compressed-token';
import { Keypair } from '@solana/web3.js';

const payer = Keypair.generate();
const to = Keypair.generate();

// 1) Airdrop
await rpc.requestAirdrop(payer.publicKey, 10e9);

// 2) Create compressed mint
const { mint } = await createMint(rpc, payer, payer.publicKey, 9);

// 3) Mint tokens
await mintTo(rpc, payer, mint, payer.publicKey, payer, 1e9);

// 4) Transfer
await transfer(rpc, payer, mint, 7e8, payer, to.publicKey);
```

---

## 5️⃣ On-Chain Program Dev

> Your program → CPI → `light_system_program`

### Core Programs
- `light-system-program` – enforces compressed state, handles create/write
- `light-compressed-token` – SPL-compatible compressed tokens
- `account-compression` – Merkle trees under the hood

### Local Testing
```bash
light test-validator   # pre-loaded with all programs & Photon
```

---

## 6️⃣ Copy-Paste Examples
- [Web client](https://github.com/Lightprotocol/example-web-client)
- [Node client](https://github.com/Lightprotocol/example-nodejs-client)
- [Anchor escrow](https://github.com/Lightprotocol/light-protocol/tree/light-v0.3.0/examples/token-escrow)

---

## 7️⃣ Networks
- **Localnet** – `light test-validator`  
- **Devnet** – full infra ready  
- **Mainnet-Beta** – production ready

---

## 8️⃣ Need Help?
| Channel | Use for |
|---------|---------|
| [Solana StackExchange](https://solana.stackexchange.com) | Solana-level questions |
| [Light Discord](https://discord.gg/CYvjBgzRFP) | SDK / program issues |
| [Helius Discord](https://discord.gg/Uzzf6a7zKr) | RPC / Photon issues |

> Paste text, not screenshots, for faster answers 🏎️