# 🕵️ Zero-Knowledge Proofs — Fundamentals Crash-Course

> Prove you know a secret **without revealing the secret**.

---

## 1️⃣ The Big Idea

> **Zero-Knowledge (ZK)** =  
> *Convince someone a statement is true while revealing **zero extra info**.*

### 🧩 Classic Motivation — Graph 3-Coloring
- **Problem**: Color a graph with 3 colors so **no adjacent vertices share a color**.
- **ZK twist**: University proves a valid course schedule to auditors **without disclosing** which student is in which class.

---

## 2️⃣ Three Pillars

| Property | Emoji | Meaning |
|----------|-------|---------|
| **Completeness** | ✅ | Honest prover always convinces honest verifier. |
| **Soundness** | 🔒 | Cheating prover **never** convinces verifier of a lie (except tiny prob.). |
| **Zero-Knowledge** | 🙈 | Verifier learns **nothing** beyond “statement is true”.

---

## 3️⃣ Interactive vs Non-Interactive

- **Interactive**  
  - Prover ↔ Verifier, multiple rounds.  
  - *Downside*: Needs live back-and-forth.

- **Non-Interactive (NIZK)**  
  - **Fiat-Shamir heuristic**: hash transcript → single message.  
  - *Result*: one-time proof anyone can verify offline.

---

## 4️⃣ Math Primer — Just Enough

### 📐 Set Theory
```text
apple ∈ Fruit        potato ∉ Fruit
Citrus ⊆ AllFruits
```

### 🔢 Number Theory
- **ℤ** = {…, -2, -1, 0, 1, 2, …}
- **ℚ** = {p/q | p, q ∈ ℤ, q ≠ 0}
- **ℝ** = rationals + irrationals (π, √2)

### 🌀 Modular Arithmetic
```
11 + 2 ≡ 1 (mod 12)
```
→ *wraps around*; essential for **finite fields**.

### 🎓 Group Theory (4 Axioms)
1. **Closure**  
2. **Associativity**  
3. **Identity** (e.g., 0 for addition)  
4. **Inverse** (e.g., -a for a)

> **Example**: ℤ under addition.

### 🌾 Finite Fields 𝔽ₚ
- **Order** = prime p  
- **Generator** g:  
  g¹, g², …, g^(p−1) produce all non-zero elements.

---

## 5️⃣ Polynomial Superpowers

### 🧬 Key Facts
- **Degree d** polynomial intersects another at **≤ d** points.
- **Schwartz-Zippel**: random eval detects identity w/ high prob.
- **Lagrange Interpolation**: rebuild polynomial from **d + 1** points.

### 🏗️ Commit-and-Prove Workflow
1. Encode data → `P(x)`  
2. Commit Merkle root of evaluations.  
3. Verifier samples random **r**; prover responds `P(r)`, proof.  
4. Check on-the-fly → **succinct**.

---

## 6️⃣ Cryptography Toolbelt

### 🔐 Symmetric Encryption
- **AES-128/192/256**  
- **ChaCha20** (stream cipher)

> Same key encrypts & decrypts — **fast**, but **key distribution** pain.

### 🔑 Asymmetric Encryption
- **RSA** (prime factorization)  
- **ECC** (elliptic curves)  
  - Solana uses **Ed25519** (Edwards25519).

> Public key encrypts, private key decrypts.

### ✍️ Digital Signatures
- **DSA / ECDSA**  
- **EdDSA** (Edwards25519).

### 🧮 Discrete Log Problem
Given `g^k ≡ h (mod p)` find **k**.  
Hard ⇒ secure **Diffie-Hellman**, **ElGamal**, **zk-SNARKs**.

---

## 7️⃣ Elliptic Curves in One Minute

- **Curve**: `y² = x³ + ax + b` over 𝔽ₚ  
- **Point addition** is group law.  
- **Scalar mult** `k·P` is one-way (trapdoor).  
- **Key sizes**: 256-bit ECC ≈ 3072-bit RSA.

### 🛡️ Curve Types
| Type | Equation | Pros |
|------|----------|------|
| **Montgomery** | `By² = x³ + Ax² + x` | Constant-time ladder, side-channel resistant |
| **Edwards** | `x² + y² = 1 + d x² y²` | Fast, unified addition formula (Ed25519) |

---

## 8️⃣ Randomness & Trust

### 🎲 Verifiable Random Function (VRF)
- Output = random value + proof.  
- Used in **Algorand**, **Cardano**, **Solana leader schedule**.

### 🎭 Trusted Setups
- **MPC ceremonies** combine many secrets → public parameters.  
- One honest party ⇒ security.  
- **zk-STARKs**: **no trusted setup** needed.

---

## 9️⃣ Quick Symbols Reference

| Symbol | Meaning |
|--------|---------|
| ∈ | “is an element of” |
| ℤ | Integers |
| ℚ | Rationals |
| 𝔽ₚ | Finite field order p |
| g^a | Generator exponentiation |
| mod p | Modulo prime |
| P(x) | Polynomial |

---

> Ready? The next stop is **ZK Compression on Solana**.


# 🔐 Zero-Knowledge Proofs: An Introduction to the Fundamentals

## 🔍 The Theory Behind Zero-Knowledge Proofs

- **Historical Context**  
  In 1989, Goldwasser, Micali, and Rackoff asked: *“How do you convince someone you know a secret without revealing it?”*  

- **Core Idea**  
  A *prover* convinces a *verifier* that a statement is true, revealing nothing beyond its truth.

---

### 🎓 Classic Example: Graph Three-Coloring

1. **Problem Definition**  
   Color each vertex of a graph with one of three colors so that no two adjacent vertices share the same color.

2. **Real-World Analogy: University Timetabling**  
   - Vertices → classes  
   - Edges → shared students  
   - Colors → time slots  

3. **Zero-Knowledge Protocol Steps**  
   - **Commit**: University creates hashes of assigned time slots.  
   - **Challenge**: Auditor picks a random edge (pair of adjacent classes).  
   - **Reveal**: University opens the two hashes, proving they differ.  
   - **Repeat**: Iterate until auditor gains high confidence.

---

## 🧩 NP-Complete Problems

- **Definition**  
  1. Outputs are “yes” or “no.”  
  2. If “yes,” there’s a short *witness* (solution).  
  3. Witness verified in polynomial time.  
  4. Brute-force search is computationally hard.

- **Notable Examples**  
  1. Traveling Salesman Problem  
  2. Knapsack Problem  
  3. Job Scheduling  

- **Cook–Levin Theorem**  
  Any NP problem reduces to Boolean satisfiability (*SAT*), meaning ZK proofs can cover all NP.

---

## 🔐 Properties of Zero-Knowledge Proofs

1. **Completeness**  
   Honest prover convinces honest verifier every time.

2. **Soundness**  
   Dishonest prover cannot convince verifier of a false statement (except with negligible probability).

3. **Zero-Knowledge**  
   Verifier learns *only* that the statement is true—no additional information leaks.

---

## 🔄 Interactive vs. Non-Interactive Proofs

1. **Interactive Structure**  
   - Prover → *commitment* (e.g., a hash)  
   - Verifier → *random challenge*  
   - Prover → *response/proof*

2. **Practical Challenges**  
   - Risk of prover/verifier collusion  
   - Verifier must securely store secrets  
   - Inefficient for asynchronous or broadcast scenarios

3. **Fiat–Shamir Heuristic**  
   - Replace the verifier’s random challenge with a *hash* of the commitment.  
   - Yields a *non-interactive* proof that anyone can verify.

4. **Spot-Checking with Polynomials**  
   - Prover computes a Merkle root of a big computation.  
   - Use the root to pseudo-randomly pick branches for verification.  
   - Polynomials let the verifier be confident every piece is correct without inspecting each bit.

---

## 🧮 The Math Behind Zero-Knowledge Proofs

> *“To not know math is a severe limitation to understanding the world.”*  
> — Richard P. Feynman

### 1. 📚 Set Theory

- **Definition**  
  A *set* is a collection of *distinct* elements, written as `{a, b, c}`.

- **Notation**  
  - `x ∈ S` means *x is in set S*.  
  - `y ∉ S` means *y is not in set S*.  
  - `A ⊆ B` means *A is a subset of B*.

- *Why it matters*: Defines ranges, constraints, and membership proofs without revealing the element.

---

### 2. 🔢 Number Theory

- **Integers (ℤ)**  
  `{…,-2, -1, 0, 1, 2,…}`

- **Rational Numbers (ℚ)**  
  `{p/q | p, q ∈ ℤ, q ≠ 0}`

- **Real Numbers (ℝ)**  
  All rationals and irrationals (like π, √2).

- *Why it matters*: Underpins key spaces and arithmetic constraints in proofs.

---

### 3. 🔄 Modular Arithmetic

- **Clock Analogy**  
  An analog clock wraps after 12: `11 + 2 ≡ 1 (mod 12)`

- **Modulo Operation**  
  `n mod k` = remainder of `n ÷ k`  
  - Example: `25 mod 3 = 1`

- *Why it matters*: Keeps computations within finite fields, preventing overflow and enabling secure arithmetic.

---

### 4. 🔢 Group Theory

- **Group (G, ⊕)**  
  A set G with an operation ⊕ satisfying:  
  1. **Closure**: `a ⊕ b ∈ G`  
  2. **Associativity**: `(a ⊕ b) ⊕ c = a ⊕ (b ⊕ c)`  
  3. **Identity**: ∃ e ∈ G such that `e ⊕ a = a`  
  4. **Inverse**: ∀ a ∈ G, ∃ b ∈ G such that `a ⊕ b = e`

- **Examples**  
  - `(ℤ, +)` with `0` as identity  
  - `(ℚ*, ×)` with `1` as identity

- **Subgroups**  
  A subset H ⊆ G that itself satisfies the group axioms.

- *Why it matters*: Foundations for RSA, elliptic-curve operations, and algebraic circuits in ZKPs.

---

### 5. 🔗 Fields

- **Field**  
  A set where addition and multiplication form abelian groups, and multiplication distributes over addition.

- **Finite Field (𝔽ₚ)**  
  Elements are `{0,1,…,p–1}` with operations `mod p`, where `p` is prime.

- **Generator**  
  An element g such that `{g^0, g^1, …, g^(p–2)}` covers all nonzero elements.

- *Why it matters*: Enables arithmetic circuits and compact, efficient proofs over finite domains.