📘 **How to Migrate from Ethereum to Solana: A Developer’s Guide**

---

### 🔷 Why Migrate?  
Ethereum is powerful but limited:  
- ❌ Single-threaded execution  
- ❌ Volatile gas fees  
- ❌ Scalability bottlenecks  

Solana offers:  
- ✅ High throughput & low latency  
- ✅ Parallel transaction processing  
- ✅ Predictable, ultra-low fees (~$0.001 per tx)  

> 💡 *Solana is built for web-scale dApps.*

---

### ⚙️ Key Differences

#### 🔗 Consensus Mechanisms
- **Ethereum (PoS)**  
  Uses **Gasper**:  
  - Casper-FFG for finality  
  - LMD-GHOST for fork choice  

- **Solana**  
  Combines:  
  - **Proof of History (PoH)** → Cryptographic timekeeping  
  - **Tower BFT** → Fast, secure consensus using PoH as clock  

> ⏱️ PoH lets validators agree on time without communication → faster finality.

---

#### 📦 Transaction Processing
| Ethereum | Solana |
|--------|--------|
| `EVM`: Single-threaded stack machine | `SVM`: Parallel execution via **Sealevel** |
| Sequential tx processing | Non-conflicting txs run in parallel |
| Gas-based, volatile fees | Fixed base fee (5000 lamports), optional priority fee |
| Global congestion → gas wars | **Localized fee markets** → only hotspots pay more |

> 🧩 Solana uses all CPU cores; Ethereum underutilizes hardware.

---

#### 💻 Smart Contract Languages
| Ethereum | Solana |
|--------|--------|
| `Solidity` (main) | `Rust` (primary) |
| `Vyper`, `Yul` | `Seahorse` (Python-like), `C/C++` (LLVM/BPF) |
| Familiar to JS/C++ devs | Memory-safe, performant, but steeper learning curve |

🛠️ **Frameworks**:  
- **Anchor** → Opinionated Rust framework (reduces boilerplate)  
- **Seahorse** → Write Solana programs in Python  

> 🌐 *Solidity isn’t dead on Solana — see Solang & Neon EVM.*

---

### 🧾 Solana’s Account Model

All accounts are 32-byte ed25519 public keys (vs Ethereum’s 20-byte).

#### 🔐 Account Fields (All Accounts Have):
- `lamports` → Balance in SOL  
- `data` → Raw byte array  
- `owner` → Program that can modify it  
- `signer` / `writable` → Tx flags  
- `executable` → Is it a program?  
- `rent_epoch` → When rent is due  

> 💸 Rent: Accounts must be **rent-exempt** (hold 2 years’ rent) to stay alive.

---

#### 🧩 Account Types
- **Executable**  
  - Holds program code (on-chain or native)  
  - e.g., System Program, Token Program  

- **Non-executable**  
  - Stores data only  

> 🔄 *Code and data are separate → modular, reusable programs.*

---

#### 🧭 Program Derived Addresses (PDAs)
- Off-curve addresses (not valid ed25519 points)  
- Generated from:  
  - Program ID  
  - Seeds (e.g., `"user"`, `user_pubkey`)  
  - Bump (1-byte int)  

```rust
let pda = hash(program_id, seeds, bump);
```

🔐 Only the parent program can “sign” for a PDA → enables trustless automation.

---

#### 📥 Transactions & Instructions
A Solana **transaction** includes:
- One or more **signatures**
- Array of **accounts** to read/write
- One or more **instructions**

Each **instruction** = call to one program:
- Specifies program, accounts, and data

✅ **Atomic execution**: All instructions succeed or fail together.

🔁 **Cross-Program Invocations (CPIs)**  
- One program calls another  
- Halts caller until callee finishes  
- Reentrancy: limited to self-recursion (max depth 4)

📦 **Transaction Size Limit**: 1232 bytes (IPv6 MTU)  
- **Versioned Transactions + Address Lookup Tables (ALTs)** → shrink size (1 byte vs 32 per address)

---

### 🧪 Solang: Solidity on Solana

Compile Solidity → Solana BPF bytecode.

#### 🛠️ Setup (via Anchor)
```bash
anchor init my_project --solidity
```
Creates a project using Solang compiler.

#### 📌 Key Annotations
```solidity
@program_id("...")
contract MyContract {
    @payer(payer)
    @seed("user_data")
    @bump(bump)
    @space(200)
    constructor() { ... }
}
```
- `@payer` → Who pays for account creation  
- `@seed`, `@bump` → PDA derivation  
- `@space` → Data size (in bytes)

#### 🧩 Accessing Accounts
```solidity
@account(ichigo)
function my_func() {
    tx.accounts.ichigo.lamports += 1000;
}
```
`tx.accounts` gives access to all passed accounts.

#### ⚠️ Limitations
- `msg.sender` ❌ → Use account annotations  
- `ecrecover()` ❌ → Use `signatureVerify()` for ed25519  
- No try-catch, no error messages  
- No `transfer()`/`send()` with value  
- No ERC-20 → Use **SPL Token** via `spl_token.sol`  
- Prefer `uint64` over `uint256` → faster, cheaper

> 💡 64-bit integers are native to SVM → better performance.

#### ✅ Advantages
- Interact with **Anchor programs** via IDL → like ABI  
  ```bash
  solang idl my_program.json
  ```
- Built-in **SPL Token** and **System Program** libraries  
- Auto-generate `AccountMeta` for CPIs  
- PDA creation & rent calculation built-in

---

### ⚡ Neon EVM: Full EVM on Solana

Run Ethereum dApps **unchanged** on Solana.

#### 🏗️ Architecture
- **Neon EVM Program** → Runs EVM logic on Solana  
- **Neon Proxy** → Converts Ethereum txs → Solana txs  
- **Neon DAO** → Governance by NEON token holders

#### 🔄 EVM Compatibility
✅ Supports:  
- Solidity & Vyper  
- Metamask, Foundry, Remix, Hardhat  
- Most opcodes  
- Legacy (type 0) Ethereum txs  

⚠️ Differences:
- No `bigModExp`, `bn256Add`, etc. (precompiles)  
- `block.timestamp`, `block.number` behave differently → avoid  
- `transfer()`/`send()` not reentrancy-safe  
- Max 64 accounts per tx (ALT limit)  
- 256 KB heap memory (BPF limit)

---

#### 🌐 Connecting to Neon RPC
Use **Chainlist** to connect wallet to:
- Neon Devnet or Mainnet

💸 Get test tokens via **Neon Faucet**:
```bash
curl -X POST http://localhost:3333/request_neon \
  -d '{"wallet": "0x...", "amount": 1}'
```

---

#### 🚀 Deployment (via Foundry)
1. Set up `.env`:
   ```env
   RPC_URL_DEVNET=https://devnet.neonevm.org
   PRIVATE_KEY=your_key_here
   ```
2. Deploy:
   ```bash
   forge create --rpc-url $RPC_URL_DEVNET \
     --private-key $PRIVATE_KEY \
     src/MyContract.sol:MyContract --legacy
   ```
3. Verify:
   ```bash
   forge verify-contract --chain-id 245022926 \
     MyContract --verifier blockscout
   ```

---

#### 🪄 NeonPass: Solana ↔ Neon EVM Bridge
- Move SPL tokens ↔ ERC-20 tokens **directly**  
- No mint/burn → true asset transfer  
- Enables dApps to access Solana’s liquidity

---

### 🏁 Past Migrations

#### 🌐 **Helium**
- Moved from own L1 → Solana (HIP 70)  
- April 2023 → “boring” success (no issues)  
- Gained uptime, speed, composability

#### 🖼️ **Render Network**
- Upgraded from Ethereum → Solana (RNP-002)  
- Nov 2023 → faster, cheaper GPU rendering  
- Users migrate via **Upgrade Assistant**

#### 💰 **MakerDAO**
- Exploring **Solana-based appchain**  
- Forking Solana’s codebase for DeFi scalability

> 🚀 *Migrations are feasible — and happening.*