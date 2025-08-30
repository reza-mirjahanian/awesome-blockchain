---

# 🚀 EIP-2537: Supercharging the EVM

### **Precompiles for BLS12-381 Curve Operations**

---

# 💡 What Are Precompiles?

* **Built-in shortcuts** for executing complex operations at the EVM level.
* They are like smart contracts linked to a specific, permanent address.
* **Key Differences from Smart Contracts:**
    * ❌ **No EVM Bytecode:** The logic isn't stored on-chain; instead, the implementation is hardcoded into the client software.
    * ✅ **Native Implementation:** Because the logic is hardcoded, it benefits from native performance and execution speed.
* **Main Advantages:**
    * 🏎️ **Native Performance:** Executes much faster than interpreted EVM code.
    * ⛽ **Fixed Gas Costs:** They have fixed gas costs, unlike smart contract logic which is charged per opcode.

---

# 🔧 Precompile in Action: `ecrecover`

* A familiar function in Solidity used to verify digital signatures.
* **Functionality:** Recovers the Ethereum address that signed a given hash.
* **Under the Hood:**
    * `ecrecover` compiles to a `call` to the precompile at address `0x01`.
    * This precompile natively implements the **ECDSA** public key recovery function.
    * It operates on the **secp256k1** curve, the standard elliptic curve used for all Ethereum accounts and transactions.

---

# 📈 What is Elliptic Curve Cryptography (ECC)?

* A powerful type of public-key cryptography based on the mathematical structure of elliptic curves.
* **The Curve:**
    * An elliptic curve is a set of points (X, Y) that satisfy a specific equation. 
    * Different `a` and `b` coefficients in the equation define different curves, affecting the shape, security, and performance.
* **Why Different Curves Matter:**
    * 🛡️ **Security:** The choice of curve impacts the strength of the cryptography.
    * ⚡ **Performance:** Different curves have different performance characteristics.
    * ➗ **Mathematical Properties:** Different curves support different mathematical operations, unlocking unique cryptographic schemes.

---

# 🔑 The Core of ECC Security

* ECC relies on mathematical problems that are easy to compute in one direction but extremely hard to reverse.
* **The Concept:**
    * It's **easy** to perform an operation in one direction (e.g., generating a public key).
    * It's **computationally infeasible** to reverse the operation without a secret piece of information.
* That secret information is the **private key**.

---

# 🔗 Introducing the BLS12-381 Curve

* The specific elliptic curve at the heart of **EIP-2537**.
* It's a "pairing-friendly" curve, which is its most important feature.
* **Key Capability: Pairing Operations**
    * A special mathematical operation that enables powerful cryptographic techniques like signature aggregation and some zero-knowledge proofs.
    * This feature is not available on Ethereum's default `secp256k1` curve.

---

# 🔓 What Pairings Unlock

Pairing operations on the BLS12-381 curve are the foundation for:

* ✍️ **Signature Aggregation**
    * Allows multiple distinct BLS signatures to be compressed into a *single, constant-size* signature.
    * This aggregated signature can be verified very efficiently.

* 🤫 **Zero-Knowledge Proofs**
    * Many modern ZK-proof systems and privacy protocols rely on pairing operations.
    * BLS12-381 support on the EVM makes these systems more efficient to use on-chain.

---

# 🏛️ Use Case: Ethereum's Consensus Layer

* The BLS signature scheme is **already in use** by Ethereum's Proof-of-Stake consensus layer.
* **How it Works:**
    * Validators use BLS to sign attestations.
    * These individual signatures can be aggregated into one, making it very efficient to validate that many different entities have signed off on something.

---

# 📄 Smart Contract Applications (Part 1)

### Finality & Advanced Multi-Sigs

* **Verifying Consensus Finality**
    * A smart contract can now efficiently check that enough validators have signed off on a certain epoch.
    * **Crucial for:** High-security applications that are sensitive to reorgs.

* 🚀 **Supercharged Multi-Sigs**
    * For a multi-sig, many signatures can be aggregated into one.
    * The smart contract only needs to verify the single aggregated signature to check if a quorum has been reached, which is very efficient.

---

# 🤫 Smart Contract Applications (Part 2)

### Zero-Knowledge Cryptography

* The introduction of BLS12-381 precompiles makes the EVM "ZK-friendly."
* **Direct Enabler for:**
    * **ZK-Rollups and Privacy Protocols:** These systems can make use of the new precompiles, as the BLS12-381 curve is useful for zero-knowledge cryptography due to its support for pairing operations.

## 🔐 EIP 2537 overview

- **Scope:** Adds new EVM precompiles for fast operations on the BLS12-381 elliptic curve.
- **Motivation:** Enable efficient BLS signatures, aggregation, and modern zero-knowledge primitives that rely on pairings.
- **Impact:** Expands on existing precompiles (e.g., ECRecover, alt_bn128) with a security- and performance-favored curve used widely in consensus and ZK systems.

---

## ⚙️ What precompiles are

- **Definition:** Precompiles are built-in, native implementations of complex operations exposed at fixed EVM addresses, callable like contracts but executed in client code for speed.
- **Behavior:** Fixed and predictable gas schedule; no EVM bytecode; implemented in client (e.g., Geth, Nethermind, Erigon, REVM).
- **Why they exist:** High-performance cryptography and heavy math are prohibitively expensive or impractical in EVM opcodes or Solidity alone.
- **Familiar example:** ECRecover in Solidity compiles to a call to a precompile address that performs ECDSA public key recovery natively.

---

## 🧮 Elliptic curve cryptography basics

- **Concept:** An elliptic curve is the set of points satisfying a specific equation; cryptographic systems use algebra on these points for secure primitives.
- **Asymmetry:** Easy forward operations (e.g., point multiplication) but computationally infeasible to invert without private keys.
- **Ethereum today:** Uses secp256k1 (ECDSA) for transaction signatures; alt_bn128 precompiles support pairings for ZK SNARKs.
- **New curve class:** BLS12-381 supports efficient pairings, enabling signature aggregation and state-of-the-art ZK proof systems.

---

## 🧭 Why BLS12-381 matters

- **Security:** Modern, audited curve with strong security margins, favored by ZK libraries and proof systems.
- **Pairings:** Supports efficient bilinear pairings needed for BLS signature verification and many SNARK/PLONK constructions.
- **Aggregation:** BLS signatures aggregate elegantly into a single signature, drastically reducing on-chain verification costs.
- **Ecosystem fit:** Already used in Ethereum’s consensus layer; aligning execution-layer capabilities with consensus and L2/ZK ecosystems reduces friction.

---

## 🧩 New precompiles introduced

> Typical operations exposed by a BLS12-381 precompile suite. Exact names and inputs vary by client/EIP versions, but the functional set aligns with these.

| Operation               | Inputs (bytes)            | Output (bytes) | Purpose                          | Typical use case                 |
|------------------------|---------------------------|----------------|----------------------------------|----------------------------------|
| G1 point add           | 2 points in G1            | 1 point        | Group addition on G1             | BLS verification helpers         |
| G1 scalar mul          | Point in G1 + scalar      | 1 point        | Scalar multiplication on G1      | Public key ops, hashing gadgets  |
| G1 multi-exp           | Points + scalars          | 1 point        | Batched scalar multiplications   | Batch verifies, MSM in ZK        |
| G2 point add           | 2 points in G2            | 1 point        | Group addition on G2             | BLS verification helpers         |
| G2 scalar mul          | Point in G2 + scalar      | 1 point        | Scalar multiplication on G2      | Aggregation verifiers            |
| G2 multi-exp           | Points + scalars          | 1 point        | Batched scalar multiplications   | Batch verifies, MSM in ZK        |
| Pairing check          | Pairs in G1×G2            | Boolean flag   | Bilinear pairing product equals 1 | BLS signature verification       |
| Map to curve (G1/G2)   | Bytes                     | 1 point        | Deterministic hashing to curve   | Hash-to-curve per spec           |

> Sources: EVM precompile suites typically define canonical encodings, subgroup checks, and consistent endianness rules to prevent invalid-curve or small-subgroup bugs.

---

## 🧠 Cryptographic schemes unlocked

- **BLS signatures:**
  - **Property:** Aggregation-friendly—combine many signatures into one.
  - **Result:** Verify N signatures as cheaply as verifying one aggregate, with O(1) verification size on-chain.
- **Aggregate signatures:**
  - **Property:** Multi-signer proofs compress to a single proof/signature.
  - **Result:** Lightweight quorum attestations, validator approvals, DAO votes.
- **Zero-knowledge proofs:**
  - **Property:** Pairings and MSMs (multi-scalar multiplications) are core to SNARK/PLONK verification.
  - **Result:** Faster on-chain verification for privacy protocols and validity proofs.
- **Hash-to-curve:**
  - **Property:** Map arbitrary bytes to G1/G2 points securely.
  - **Result:** Standards-compliant BLS signatures and verifiable random functions (VRFs).

---

## 🛠️ Smart contract use cases

- **Finality proofs and safety checks:**
  - **Need:** Verify a quorum of validator attestations to gate high-value actions.
  - **Benefit:** On-chain safeguards for reorg-sensitive apps (bridges, settlement, rollups).
- **Multisig and DAOs:**
  - **Need:** Compress M-of-N approvals into a single signature.
  - **Benefit:** Lower gas usage, simpler data structures, clearer audit trails.
- **Bridges and light clients:**
  - **Need:** Check signatures across chains or rollups efficiently.
  - **Benefit:** Succinct trust-minimized interoperability.
- **ZK-native applications:**
  - **Need:** Verify proofs for privacy, identity, and compression.
  - **Benefit:** Practical costs for frequent verification on L1.
- **Rollup infrastructure:**
  - **Need:** Validate sequencer sets and state confirmations.
  - **Benefit:** Reduced calldata, faster verification during upgrades or challenges.

---

## 💡 Developer notes and calling patterns

- **Encoding discipline:**
  - **Rule:** Use the exact byte encodings, subgroup checks, and domain separation tags defined by the spec.
  - **Tip:** Always perform input validation (finite field check, not infinity, subgroup membership).
- **Hash-to-curve:**
  - **Rule:** Use standardized suites (e.g., hash_to_curve with expand_message_xmd) matching BLS signature spec.
  - **Tip:** Ensure consistent domain separation between networks/components.
- **Gas modeling:**
  - **Rule:** Precompiles use fixed cost schedules; expect orders-of-magnitude cheaper than Solidity implementations.
  - **Tip:** Batch work with multi-exp and aggregate verification where possible.
- **Example call (Yul-style):**
  - **Pattern:** Prepare input bytes in memory → call precompile address → check success → parse output.
  - **Note:** Addresses are fixed by the EIP/client; consult your client docs for exact values.

```solidity
// Pseudocode: pairing precompile call
function pairing(bytes memory input) internal view returns (bool ok) {
    bool success;
    uint256 out;
    assembly {
        // precompile address placeholder (replace with actual)
        let addr := 0xPRECOMPILE
        let inPtr := add(input, 0x20)
        let inLen := mload(input)
        let outPtr := mload(0x40)
        success := staticcall(gas(), addr, inPtr, inLen, outPtr, 0x20)
        out := mload(outPtr)
    }
    return success && (out == 1);
}
```

---

## ⚡ Performance and gas considerations

- **Aggregation leverage:** Verifying one aggregate often beats verifying many signatures individually by a large margin.
- **Batch-friendly ops:** Multi-exponentiation precompiles exploit algorithmic speedups over naive loops in Solidity.
- **Data minimization:** Aggregates shrink calldata, improving L1 fees and block propagation behavior.
- **Throughput benefits:** Cheaper cryptography encourages in-protocol verification vs. trusting off-chain oracles.

---

## 🛡️ Security and correctness checklist

- **Subgroup checks:** Ensure points lie in correct subgroups; reject small-subgroup points to prevent signature forgeries.
- **Canonical encodings:** Enforce standardized byte encodings to avoid malleability and cross-client discrepancies.
- **Domain separation:** Distinct tags per application/chain to prevent replay across contexts.
- **Determinism:** Precompiles must be deterministic across clients; avoid any non-deterministic sources.
- **Edge cases:** Validate infinity points, zero scalars, and malformed inputs consistently; return errors rather than undefined behavior.

---

## 🌉 Interoperability and ecosystem impact

- **Consensus alignment:** Matches the signature scheme used by Ethereum validators, bridging execution and consensus worlds.
- **ZK library support:** Popular proof systems and libraries prefer BLS12-381, reducing custom crypto glue code on-chain.
- **Rollup synergy:** Pairs naturally with data-availability and validity-proof designs, improving security-per-gas.
- **Standards momentum:** Harmonizes with IETF BLS specs and hash-to-curve standards, improving cross-chain portability.

---

## ❓ Quick Q&A

- **What’s the difference from ECRecover?** ECRecover verifies ECDSA on secp256k1; BLS precompiles verify pairing-based signatures and enable aggregation.
- **Is it only for signatures?** No—pairings, multi-exponentiation, and hash-to-curve underpin many ZK and cryptographic protocols.
- **Do I need custom Solidity libs?** Yes—use audited BLS libraries that wrap the precompiles, enforce encodings, and manage domains.
- **Will gas be predictable?** Yes—precompiles have fixed gas schedules per operation, more stable than hand-rolled EVM math.

---