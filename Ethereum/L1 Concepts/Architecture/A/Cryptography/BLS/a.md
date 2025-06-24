
---

## 🔹 **1. Foundational Concepts**

### 🔸 What is BLS Signature?

BLS is a **digital signature scheme** that provides:

* **Short signatures**
* **Signature aggregation**
* **Efficient verification**
* **Security based on pairing-based cryptography (Bilinear maps)**

It operates over:

* **Elliptic Curve Groups** `G1`, `G2`, and a **Target Group** `GT`
* A **bilinear map**: `e: G1 × G2 → GT`

---

## 🔹 **2. Mathematical Foundations**

### 🔸 Groups

* `G1`, `G2`: Cyclic groups of prime order `q`
* `GT`: Another group where pairings land

### 🔸 Bilinear Pairing Properties

Let `g1 ∈ G1`, `g2 ∈ G2`. For all `a, b ∈ ℤ_q`:

* **Bilinearity**: `e(g1^a, g2^b) = e(g1, g2)^{ab}`
* **Non-degeneracy**: `e(g1, g2) ≠ 1`
* **Computability**: Efficient algorithm exists to compute `e(g1, g2)`

### 🔸 Hard Problems

* **Computational Diffie-Hellman (CDH)** in `G1`, `G2`
* **Bilinear Diffie-Hellman (BDH)** assumption: given `(g, g^a, g^b, g^c)`, compute `e(g,g)^{abc}`

---

## 🔹 **3. BLS Signature Scheme Overview**

### 🔸 **Key Generation**

* Choose random `sk ∈ ℤ_q`
* Compute `pk = g2^sk ∈ G2`

  * (`g2` is the generator of `G2`)

### 🔸 **Signing**

* Hash message `m` to a point in `G1`: `H(m) ∈ G1`
* Signature: `σ = H(m)^sk ∈ G1`

### 🔸 **Verification**

* Check:
  `e(σ, g2) == e(H(m), pk)`

### 🔸 **Aggregation**

* Given signatures `σ1, ..., σn` for messages `m1, ..., mn`:

  * `σ_agg = σ1 * σ2 * ... * σn ∈ G1`
* Verify aggregated signature:

  * For same message `m`:
    `e(σ_agg, g2) == e(H(m), pk1 * pk2 * ... * pkn)`

---

## 🔹 **4. Rust Example using `blst`**

> `blst` is a performant BLS12-381 library written in C, with Rust bindings.

```toml
# Cargo.toml
[dependencies]
blst = "0.3"
rand = "0.8"
```

```rust
use blst::*;
use rand::rngs::OsRng;

fn hash_to_g1(msg: &[u8]) -> blst_p1 {
    let dst = b"BLS_SIG_BLS12381G1_XMD:SHA-256_SSWU_RO_NUL_";
    let mut p1 = blst_p1::default();
    unsafe {
        blst_hash_to_g1(&mut p1, msg.as_ptr(), msg.len(), dst.as_ptr(), dst.len(), std::ptr::null(), 0);
    }
    p1
}

fn main() {
    let mut rng = OsRng;

    // Secret Key
    let ikm = b"my-random-seed-32-bytes................";
    let sk = SecretKey::key_gen(ikm, &[]).unwrap();

    // Public Key
    let pk = sk.sk_to_pk();

    // Message
    let msg = b"hello BLS";

    // Hash to curve
    let p1 = hash_to_g1(msg);

    // Sign
    let sig = sk.sign(msg, b"", &[]);

    // Verify
    assert!(sig.verify(true, msg, b"", &[], &pk, true).is_ok());
}
```

---

## 🔹 **5. Go Example using `github.com/herumi/bls-eth-go-binary`**

```bash
go get github.com/herumi/bls-eth-go-binary/bls
```

```go
package main

import (
    "fmt"
    "github.com/herumi/bls-eth-go-binary/bls"
)

func main() {
    bls.Init(bls.BLS12_381)

    var sk bls.SecretKey
    sk.SetByCSPRNG()
    pk := sk.GetPublicKey()

    msg := []byte("hello BLS")
    sig := sk.Sign(string(msg))

    if sig.VerifyByte(pk, msg) {
        fmt.Println("✅ Signature verified!")
    } else {
        fmt.Println("❌ Verification failed!")
    }
}
```

---

## 🔹 **6. C++ Example using `herumi/bls`**

Install: [https://github.com/herumi/bls](https://github.com/herumi/bls)

```cpp
#include <bls/bls.hpp>
#include <iostream>

int main() {
    bls::init(bls::BLS12_381);

    bls::SecretKey sk;
    sk.setByCSPRNG();

    bls::PublicKey pk = sk.getPublicKey();
    bls::Signature sig = sk.sign("message");

    if (sig.verify(pk, "message")) {
        std::cout << "Signature verified" << std::endl;
    } else {
        std::cout << "Verification failed" << std::endl;
    }
}
```

---

## 🔹 **7. Advanced Features**

### 🔸 Signature Aggregation (Same Message)

* Combine multiple sigs for same message:

  * `σ_agg = σ1 + σ2 + ...`
  * Verification: `e(σ_agg, g2) == e(H(m), pk1 + pk2 + ...)`

### 🔸 Signature Aggregation (Different Messages)

* Each signer signs different `m_i`
* Verify:

  ```
  e(σ1, g2) * e(σ2, g2) * ... == e(H(m1), pk1) * e(H(m2), pk2) * ...
  ```

### 🔸 Batch Verification

Verify `e(σi, g2) == e(H(mi), pki)` for all `i`.

Use product of pairings for verification optimization:

```text
∏ e(σi, g2) == ∏ e(H(mi), pki)
```

---

## 🔹 **8. Edge Cases**

| Scenario                             | Handling Strategy                                       |
| ------------------------------------ | ------------------------------------------------------- |
| Invalid curve point hash             | Use safe hash-to-curve functions (e.g., SSWU)           |
| Signature forgery                    | Ensure hash is in `G1`, and use domain separation       |
| Aggregation with mismatched messages | Use aggregate-with-message separation                   |
| Rogue public key attack              | Require proof-of-possession or random oracle assumption |

---

## 🔹 **9. Comparison Table**

| Feature                  | BLS                  | ECDSA              | Schnorr             |
| ------------------------ | -------------------- | ------------------ | ------------------- |
| Signature size           | **\~48 bytes (G1)**  | \~64 bytes         | \~64 bytes          |
| Aggregation              | ✅ Native & efficient | ❌                  | ✅ (with extra work) |
| Verification speed       | Slower (pairings)    | Faster             | Fast                |
| Security assumptions     | Pairing-based, BDH   | ECDLP              | ECDLP               |
| Batch verification       | ✅ Very efficient     | ❌                  | ✅                   |
| Deterministic signatures | ✅                    | ❌ (unless RFC6979) | ✅                   |

---

## 🔹 **10. Real-World Usage**

* **Ethereum 2.0 (Beacon Chain)** uses BLS signatures for validator attestations
* **Filecoin**
* **Zcash Orchard Protocol**
* **Chia Network**

---

## 🔹 **11. Expert-Level Topics**

### 🔸 Proof of Possession (PoP)

* Prevents rogue public key attacks
* Signer proves they know the secret key for the public key

### 🔸 Threshold BLS

* `t-of-n` scheme using Shamir's Secret Sharing over BLS signatures

### 🔸 BLS12-381 Curve Internals

* `G1`: subgroup on curve over F\_p
* `G2`: subgroup on twisted curve over F\_p^2
* `GT`: F\_p^12, result of pairing

### 🔸 Hash to Curve

Use [IETF hash-to-curve](https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-hash-to-curve) recommendations:

* SSWU (Simplified Shallue–van de Woestijne–Ulas)
* Domain separation tags (DSTs)

---

