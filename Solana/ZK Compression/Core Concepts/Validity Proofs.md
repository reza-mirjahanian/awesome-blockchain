# 🔐 Validity Proofs – ZK Compression

> **Always 128 bytes. Always zero-knowledge.**  
> One proof → one leaf verified against a root.

---

## 1️⃣ What Exactly Is a Validity Proof?

- **Type** – `Groth16` zk-SNARK  
- **Size** – **128 bytes** (constant)  
- **Content** – *“This leaf exists in the tree and the root is correct.”*  
- **Location** – generated **off-chain**, verified **on-chain**

---

## 2️⃣ Under the Hood

### 🧮 Ingredients
- **Merkle path** (purple siblings)  
- **Leaf data** (compressed account)  
- **Public inputs**  
  - `stateRoot` (from on-chain account)  
  - `leafIndex`  
  - `accountHash`

### 🪄 Groth16 Magic
- **Prover** → runs circuit → outputs **128-byte proof**  
- **Verifier (Solana program)** → `verify(proof, publicInputs)`  
- **Result** ✅ / ❌ in ~50k CU

---

## 3️⃣ Developer Reality Check

> You **never** touch the circuit code.  
> SDK abstracts: `createProof()` → `submitTx(proof, publicInputs)`

---

## 4️⃣ Mental Snapshot

```text
Off-Chain                   On-Chain
┌────────────────────────┐  ┌────────────────────┐
│  Build Merkle path     │  │  Store 32-B root   │
│  + leaf data           │  │                    │
│  ↓                     │  │  Verify 128-B ZKP  │
│  Groth16 proof (128 B) │→ │  ✅ / ❌            │
└────────────────────────┘  └────────────────────┘
```