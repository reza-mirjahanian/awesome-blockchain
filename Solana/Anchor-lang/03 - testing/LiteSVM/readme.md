**litesvm: Comprehensive Reference for Solana Testing**

---

### 📦 Overview

* **What it is**: `litesvm` is a fast, lightweight, in‑process Solana VM library for testing programs, much faster than `solana-test-validator` or `solana-program-test` ([GitHub][1]).
* **Languages supported**: Native Rust, JavaScript/TypeScript via `npm litesvm`, and Python via solders ([anchor-lang.com][2]).
* **Latest stable**: v0.6.1 (released Mar 31, 2025) ([GitHub][1]).

---

### 🚀 Installation

```bash
# Rust
cargo add --dev litesvm

# JS/TS
npm install litesvm
```

---

### 🧪 Minimal Transfer Test (Rust)

```rust
use litesvm::LiteSVM;
use solana_system_interface::instruction::transfer;
use solana_keypair::Keypair;
use solana_message::Message;
use solana_transaction::Transaction;
use solana_pubkey::Pubkey;
use solana_signer::Signer;

let mut svm = LiteSVM::new();
let from = Keypair::new();
let to = Pubkey::new_unique();
svm.airdrop(&from.pubkey(), 10_000).unwrap();

let ix = transfer(&from.pubkey(), &to, 64);
let tx = Transaction::new(
    &[&from],
    Message::new(&[ix], Some(&from.pubkey())),
    svm.latest_blockhash()
);
svm.send_transaction(tx).unwrap();

assert_eq!(svm.get_account(&to).unwrap().lamports, 64);
```

---

### ⚙️ Advanced Features

#### 1. **Deploying SBF Programs**

Use `svm.add_program(program_id, &bytes)` or `.add_program_from_file(path)` after building with `cargo build-sbf` ([Docs.rs][3]).

#### 2. **Time Travel & Sysvar Manipulation**

```rust
let mut clock = svm.get_sysvar::<Clock>();
clock.unix_timestamp = some_timestamp;
svm.set_sysvar::<Clock>(&clock);
```

Also supports `warp_to_slot` ([Docs.rs][3]).

#### 3. **Custom Accounts / Token Mints**

```rust
svm.set_account(pubkey, solana_account::Account { lamports, data, owner, executable, rent_epoch }).unwrap();
```

e.g., mint unlimited USDC in tests ([Docs.rs][3]).

#### 4. **Compute Budget & Signature Skipping**

Fluent APIs: `.with_compute_budget(...)`, `.with_sigverify(false)` ([Docs.rs][3]).

#### 5. **RPC-less Get Transaction**

`svm.get_transaction(...)` returns details from in‑memory ledger.

---

### 🧩 JS/TS Example

```ts
import { LiteSVM } from "litesvm";
import { Keypair, SystemProgram, Transaction } from "@solana/web3.js";

const svm = new LiteSVM();
const payer = Keypair.generate();
svm.airdrop(payer.publicKey, BigInt(1e9));
const recipient = Keypair.generate().publicKey;

const ix = SystemProgram.transfer({
  fromPubkey: payer.publicKey,
  toPubkey: recipient,
  lamports: BigInt(1000),
});
const tx = new Transaction().add(ix);
tx.recentBlockhash = svm.latestBlockhash();
tx.sign(payer);
svm.sendTransaction(tx);

console.assert(svm.getBalance(recipient) === BigInt(1000));
```

(via npm & docs) ([anchor-lang.com][2])

---

### ⚠️ `litesvm` vs Alternatives

| Feature                 | litesvm     | solana-program-test | solana-test-validator |
| ----------------------- | ----------- | ------------------- | --------------------- |
| Startup speed           | ✅ Very fast | 🚀 Moderate         | 🐢 Slow               |
| In‑process simulation   | ✅ Yes       | ✅ Yes               | ❌ RPC-based           |
| Sysvar override support | ✅ Yes       | ❗ Limited           | ❗ None                |
| Deploy SBF programs     | ✅ Yes       | ✅ Yes               | ✅ Yes                 |
| Real RPC fidelity       | ⚠️ Not full | ⚠️ Partial          | ✅ Full                |
| Suitable for CI         | ✅ Ideal     | ⚠️ Medium           | ⚠️ Heavyweight        |

---

### 🛠 Edge Cases & Caveats

* **Signature verification skipped?** Disable via `.with_sigverify(false)` only when you trust inputs.
* **Compute budget**: Default may be too low/high. Override to mimic on‑chain behavior.
* **Missing RPC behaviors**: No RPC logs, snapshots, voting, snapshots. Use `solana-test-validator` for full-chain test scenarios.
* **Account lifecycles**: If simulating account fails (e.g., owner mismatches), must manually `.set_account` or replicate PDA logic.

---

### ✅ Real‑World Use Cases

* **Anchor testing**: 25× speedup over standard suite via TS wrapper ([anchor-lang.com][2], [GitHub][1], [YouTube][4]).
* **Defi forks**: Combine with Surfpool to fork mainnet + inject token balances ([Solana Stack Exchange][5]).
* **SPL Token testing**: Use `litesvm-token` + arbitrary account injection for token flows ([Lib.rs][6]).
* **E2E program bench/CI**: Complement with `litesvm-testing` for typed assertions ([Crates.io][7]).

---

### 🧩 Related Crates

* **litesvm-token**: Helpers for SPL tokens ([Lib.rs][6])
* **litesvm-loader**, **litesvm-testing**: Loaders & assertion frameworks ([Lib.rs][8])

---

### 🔧 Tricky Parts

* **Bytecode loading**: Must match SBF format exactly; test until exact `.so`.
* **Sysvar consistency**: `Clock`, `Rent`, etc., may need reset after each tx.
* **Transaction deduplication**: Transactions with same signature are rejected; add dummy data to bypass.

---

### 📋 API Reference (Rust)

| Method                         | Description                  |
| ------------------------------ | ---------------------------- |
| `new()`                        | Create VM instance           |
| `airdrop(pubkey, lamports)`    | Fund account                 |
| `latest_blockhash()`           | Get blockhash                |
| `send_transaction(tx)`         | Execute transaction          |
| `simulate_transaction(tx)`     | Simulate only                |
| `get_account(pubkey)`          | Returns `Option<Account>`    |
| `get_balance(pubkey)`          | Gets lamports                |
| `add_program(id, bytes)`       | Load SBF program             |
| `add_program_from_file(path)`  | Load compiled file           |
| `get_sysvar<T: Sysvar>()`      | Retrieve sysvar (e.g. Clock) |
| `set_sysvar<T: Sysvar>(svc)`   | Override sysvar              |
| `set_account(pubkey, Account)` | Set custom account data      |
| `with_compute_budget(budget)`  | Customize compute units      |
| `with_sigverify(bool)`         | Toggle signature checking    |
| `get_transaction(sig)`         | Retrieve tx info             |
| `warp_to_slot(slot)`           | Move to future slot          |

---

### ✅ Pros & Cons

| ✅ Pros                        | ❌ Cons                               |
| ----------------------------- | ------------------------------------ |
| 10‑25× faster tests           | No real RPC stack/logs               |
| Full Rust/TS support          | Lacks validator-specific behavior    |
| Sysvar & account manipulation | Limited snapshot features            |
| CI-friendly & lightweight     | Not reflective of cluster-wide state |

---

### 📁 Real‑Project Example

```rust
#[test]
fn test_my_defi_flow() {
    let mut svm = LiteSVM::new().with_compute_budget(200_000).with_sigverify(true);
    svm.airdrop(&user.pubkey(), 1_000_000_000).unwrap();
    svm.add_program_from_file("target/deploy/my_defi.so").unwrap();

    let ix = mydefi::instruction::swap(...);
    let tx = Transaction::new_signed_with_payer(
        &[ix], Some(&user.pubkey()), &[&user], svm.latest_blockhash());
    let res = svm.send_transaction(tx).unwrap();
    assert_eq!(svm.get_balance(user.pubkey()), expected);
}
```

---

### 🔭 Next Step

To deepen expertise: **Explore `lollipop`: SVM Rollups on Solana**—an advanced protocol for Solana VM rollups, studying formal SVM layering and inter‑VM messaging 