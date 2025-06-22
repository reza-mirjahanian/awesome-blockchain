[A Brief Introduction to ECDSA](https://epf.wiki/#/wiki/Cryptography/ecdsa?id=a-brief-introduction-to-ecdsa)
============================================================================================================

It is impossible to overstate how modern cryptography redefines trust for all our digital interactions - from securing bank account logins with encryption to verifying the authenticity of your favorite apps through digital certificates.

Public key cryptography is a key concept empowering these interactions. It consists of two key pairs:

**Public key**: Widely distributed and used by anyone to verify an entity's identity. **Private key**: Confidential and known only to the owner, used for encryption and signing messages.

**Elliptic curve cryptography (ECC)** is a specific type of public key cryptography that uses mathematics of elliptic curves to create smaller, and more efficient keys. This is especially beneficial in resource-constrained environments like Ethereum. Within Ethereum, the **Elliptic Curve Digital Signature Algorithm (ECDSA)** helps verify the legitimacy of submitted transactions.

Let's consider a real-world scenario to understand how ECDSA works in action.

Alice, a diligent businesswoman, has been abducted and held captive on a remote island. Her captors demand a hefty ransom of $1 million for her release. With limited options for communication, they provide a single postcard for her to instruct her associate, Bob, to transfer the funds.

Alice considers writing the ransom amount and signing the postcard like a check. However, this method poses a significant risk: the kidnappers could easily forge the postcard, inflate the amount, and deceive Bob into sending them more money.

Alice needs a robust approach that allows:

1.  Bob to verify that the transfer has be authorized by her, and
2.  ensure that postcard's message has not been tampered with.

The goal of this exercise is device a method for Alice to create a **secret key 🔑** known only to her. This key will be crucial for her to prove her identity and ensure the message's authenticity to Bob.

Mathematics, as always, comes to the rescue. Through ingenious use of **Elliptic Curves**, let's explore how Alice can generate the **secret key 🔑**.

[Elliptic curves](https://epf.wiki/#/wiki/Cryptography/ecdsa?id=elliptic-curves)
--------------------------------------------------------------------------------

An elliptic curve is a curve **described by the equation**:

y2\=x3+ax+by2\=x3+ax+b

Such that 4a3+27b2≠04a3+27b2\=0 to ensure the curve is non-singular. The equation above is what is called the **Weierstrass normal form** of the long equation:

y2+a1xy+a3y\=x3+a2x2+a4x+a6y2+a1​xy+a3​y\=x3+a2​x2+a4​x+a6​

Examples:

![](https://epf.wiki/images/elliptic-curves/examples.gif)

Observe that elliptic curves are symmetric about the x-axis.

Ethereum uses a standard curve known as [secp256k1](http://www.secg.org/sec2-v2.pdf) with parameters a\=0a\=0, and b\=7b\=7; which is the curve:y2\=x3+7y2\=x3+7

![](https://epf.wiki/images/elliptic-curves/secp256k1.png)

[Groups and Fields](https://epf.wiki/#/wiki/Cryptography/ecdsa?id=groups-and-fields)
------------------------------------------------------------------------------------

### [Group](https://epf.wiki/#/wiki/Cryptography/ecdsa?id=group)

In mathematics, a **GROUP** is a set GG, containing at least two elements, which is closed under a binary operation usually referred to as **addition** (++). A set is closed under an operation when the result of the operation is also a member of the set.

The set of real numbers RR is a familiar example of a group, since arithmetic addition of two real numbers is closed.

3∈R,5∈R3+5\=8∈R3∈R,5∈R3+5\=8∈R

[Field](https://epf.wiki/#/wiki/Cryptography/ecdsa?id=field)
------------------------------------------------------------

Similarly, a **FIELD** is a set FF, containing at least two elements, which is closed under two binary operations usually referred to as **addition** (++), and **multiplication**(××).

In other words, A **FIELD** is a **GROUP** under both addition and multiplication.

Elliptic curves are interesting because the points on the curve form a group, i.e the result of "addition" of two points remains on the curve. This geometric addition, distinct from arithmetic counterparts, involves drawing a line through chosen points (**P** and **Q**) and reflecting the resulting curve intersection(**R'**) across the x-axis to yield their sum (**R**).

![](https://epf.wiki/images/elliptic-curves/addition.gif)

A point (**P**) can also be added to itself (P+PP+P), in which case the straight line becomes a tangent to **P** that reflects the sum (**2P**).

![](https://epf.wiki/images/elliptic-curves/scalar-multiplication.png)

Repeated point-addition is known as **scalar multiplication**:

nP\=P+P+⋯+P⏟n timesnP\=n timesP+P+⋯+P​​

[Discrete logarithm problem](https://epf.wiki/#/wiki/Cryptography/ecdsa?id=discrete-logarithm-problem)
------------------------------------------------------------------------------------------------------

Let's leverage scalar multiplication to generate the **secret key 🔑**. This key, denoted by KK, represents the number of times a base point GG is added to itself, yielding the resulting public point PP:

P\=K∗GP\=K∗G

Given PP and GG it is possible derive the secret key KK by effectively reversing the multiplication, similar to the **logarithm problem**.

We need to ensure that scalar multiplication does not leak our **secret key 🔑**. In other words, scalar multiplication should be "easy" one way and "untraceable" the other way around.

The analogy of a clock helps illustrate the desired one-way nature. Imagine a task starting at 12 noon and ending at 3. Knowing only the final time (3) makes it impossible to determine the exact duration without additional information. This is because **modular arithmetic** introduces a "wrap-around" effect. The task could have taken 3 hours, 15 hours, or even 27 hours, all resulting in the same final time modulo 12.

![](https://epf.wiki/images/elliptic-curves/clock.gif)

Over a **prime modulus**, this is especially hard and is known as **discrete logarithm problem**.

[Elliptic curves over finite field](https://epf.wiki/#/wiki/Cryptography/ecdsa?id=elliptic-curves-over-finite-field)
--------------------------------------------------------------------------------------------------------------------

So far, we have implicitly assumed elliptic curves over the rational field (RR). Ensuring **secret key 🔑** security through the discrete logarithm problem requires a transition to elliptic curves over finite fields defined by a **prime modulus**. This essentially restricts the points on the curve to a finite set by performing modular reduction with a specific prime number.

For the sake of this discussion, we will consider the **secp256k1** curve defined over an **arbitrary finite field** with prime modulus **997**:

y2\=x3+7(mod997)y2\=x3+7(mod997)

![](https://epf.wiki/images/elliptic-curves/finite-field.png)

While the geometric representation of the curve in the finite field may appear abstract compared to a continuous curve, its symmetry remains intact. Additionally, scalar multiplication remains closed, although the "tangent" now "wraps around" given the modulus nature.

![](https://epf.wiki/images/elliptic-curves/finite-scalar-multiplication.gif)

[Generating key pair](https://epf.wiki/#/wiki/Cryptography/ecdsa?id=generating-key-pair)
----------------------------------------------------------------------------------------

Alice can finally generate a key pair using elliptic curve over finite field.

Let's define the elliptic curve over the finite field of prime modulus 997 in [Sage.](https://www.sagemath.org/)

```
sage: E = EllipticCurve(GF(997),[0,7])
Elliptic Curve defined by y^2 = x^3 + 7 over Finite Field of size 997
```

Define the generator point GG by selecting an arbitrary point on the curve.

```
sage: G = E.random_point()
(174 : 487 : 1)
```

Scalar multiplication over an elliptic curve defines a cyclic **subgroup of order nn**. This means that repeatedly adding any point in the subgroup nn times results in the point at infinity (OO), which acts as the identity element.

nP\=OnP\=O

```
sage: n = E.order()
1057
# Illustrating that n*G (or any point) equals O, represented by (0 : 1 : 0).
sage: n*G
(0 : 1 : 0)
```

A key pair consists of:

1.  **Secret key 🔑**(KK): A random integer chosen from the order of the subgroup nn. Ensures only Alice can produce valid signatures.

Alice randomly chooses **42** as the **secret key 🔑**.

```
sage: K = 42
```

2.  **Public key** (PP): A point on the curve, the result of scalar multiplication of **secret key 🔑**(KK) and generator point (GG). Allows anyone to verify Alice's signature.

```
sage: P = K*G
(858 : 832 : 1)
```

We have established that Alice's key pair \=\[P,K\]\=\[(858,832),42\]\=\[P,K\]\=\[(858,832),42\].

[ECDSA in action](https://epf.wiki/#/wiki/Cryptography/ecdsa?id=ecdsa-in-action)
--------------------------------------------------------------------------------

ECDSA is a variant of the Digital Signature Algorithm (DSA). It creates a signature based on a "fingerprint" of the message using a cryptographic hash.

For ECDSA to work, Alice and Bob must establish a common set of domain parameters. Domain parameters for this example are:

| Parameter | Value |
| --- |  --- |
| The elliptic curve equation. | y2\=x3+7y2\=x3+7 |
| --- |  --- |
| The prime modulo of the finite field. | 997 |
| The generator point, GG. | (174, 487) |
| The order of the subgroup, nn. | 1057 |

Importantly, Bob is confident that the public key P\=(858,832)P\=(858,832) actually belongs to Alice.

### [Signing](https://epf.wiki/#/wiki/Cryptography/ecdsa?id=signing)

Alice intends to sign the message **"Send $1 million"**, by following the steps:

1.  Compute the cryptographic hash **mm**.

```
sage: m = hash("Send $1 million")
-7930066429007744594
```

2.  For every signature, a random **ephemeral key pair \[eKeK, ePeP\]** is generated to mitigate an [attack](https://youtu.be/DUGGJpn2_zY?si=4FZ3ZlQZTG9-eah9&t=2117) exposing her **secret key 🔑**.

```
# Randomly selected ephemeral secret key.
sage: eK = 10
# Ephemeral public key.
sage: eP = eK*G
(215 : 295 : 1)
```

Ephemeral key pair \=\[eK,eP\]\=\[10,(215,295)\]\=\[eK,eP\]\=\[10,(215,295)\].

3.  Compute signature component **ss**:

s\=k-1(e+rK)(modn)s\=k-1(e+rK)(modn)

Where rr is the x-coordinate of the ephemeral public key **(eP)**, i.e **215**. Notice the signature uses both Alice's **secret key 🔑 (KK)** and the ephemeral key pair **\[eKeK, ePeP\]**.

```
# x-coordinate of the ephemeral public key.
sage: r = int(eP[0])
215
# Signature component, s.
sage: s = mod(eK**-1 * (m + r*K), n)
160
```

The tuple (r,s)\=(215,160)(r,s)\=(215,160) is the **signature pair**.

Alice then writes the message and signature to the postcard.

![](https://epf.wiki/images/elliptic-curves/postcard.jpg)

### [Verification](https://epf.wiki/#/wiki/Cryptography/ecdsa?id=verification)

Bob verifies the signature by independently calculating the **exact same ephemeral public key** from the signature pair **(r,s)(r,s)**, message, and Alice's public key **PP**:

1.  Compute the cryptographic hash **mm**.

```
sage: m = hash("Send $1 million")
-7930066429007744594
```

2.  Compute the ephemeral public key **RR**, and compare it with **rr**:

R\=(es-1(modn))∗G+(rs-1(modn))∗PR\=(es-1(modn))∗G+(rs-1(modn))∗P

```
sage: R = int(mod(m*s^-1,n)) * G  + int(mod(r*s^-1,n)) * P
(215 : 295 : 1)
# Compare the x-coordinate of the ephemeral public key.
sage: R[0] == r
True # Signature is valid ✅
```

If Alice's captors were to modify the message, it would alter the cryptographic hash, leading to verification failure due to the mismatch with the original signature.

```
sage: m = hash("Send $5 million")
7183426991750327432 # Hash is different!
sage: R = int(mod(m*s^-1,n)) * G  + int(mod(r*s^-1,n)) * P
(892 : 284 : 1)
# Compare the x-coordinate of the ephemeral public key.
sage: R[0] == r
False # Signature is invalid ❌
```

Verification of the signature assures Bob of the message's authenticity, enabling him to transfer the funds and rescue Alice. Elliptic curves saves the day!

[Wrapping up](https://epf.wiki/#/wiki/Cryptography/ecdsa?id=wrapping-up)
------------------------------------------------------------------------

Just like Alice, every account on the [Ethereum uses ECDSA to sign transactions](https://web.archive.org/web/20240229045603/https://lsongnotes.wordpress.com/2018/01/14/signing-an-ethereum-transaction-the-hard-way/). However, ECC in Ethereum involves additional security considerations. While the core principles remain the same, we use secure hash functions like keccak256 and much larger prime field, boasting 78 digits: 2256-232-9772256-232-977.

This discussion is a preliminary treatment of Elliptic Curve Cryptography. For a nuanced understanding, consider the resources below.

And finally: **never roll your own crypto!** Use trusted libraries and protocols to protect your data and transactions.

> ℹ️ Note
> ECDSA faces potential obsolescence from quantum computers -- learn about how [Post-Quantum Cryptography tackles this challenge.](https://epf.wiki/#/wiki/Cryptography/post-quantum-cryptography)

[Further reading](https://epf.wiki/#/wiki/Cryptography/ecdsa?id=further-reading)
--------------------------------------------------------------------------------

**Elliptic curve cryptography**

-   📝 Standards for Efficient Cryptography Group (SECG), ["SEC 1: Elliptic Curve Cryptography."](http://www.secg.org/sec1-v2.pdf)
-   📝 Standards for Efficient Cryptography Group (SECG), ["SEC 2: Recommended Elliptic Curve Domain Parameters."](http://www.secg.org/sec2-v2.pdf)
-   📘 Alfred J. Menezes, Paul C. van Oorschot and Scott A. Vanstone, [Handbook of Applied Cryptography](https://cacr.uwaterloo.ca/hac/)
-   🎥 Fullstack Academy, ["Understanding ECC through the Diffie-Hellman Key Exchange."](https://www.youtube.com/watch?v=gAtBM06xwaw)
-   📝 Andrea Corbellini, ["Elliptic Curve Cryptography: a gentle introduction."](https://andrea.corbellini.name/2015/05/17/elliptic-curve-cryptography-a-gentle-introduction/)
-   📝 William A. Stein, ["Elliptic Curves."](https://wstein.org/simuw06/ch6.pdf)
-   📝 Khan Academy, ["Modular Arithmetic."](https://www.khanacademy.org/computing/computer-science/cryptography/modarithmetic/a/what-is-modular-arithmetic)
-   🎥 Khan Academy, ["The discrete logarithm problem."](https://www.youtube.com/watch?v=SL7J8hPKEWY)

**Mathematics of Elliptic Curves**

-   📘 Joseph H. Silverman, ["The Arithmetic of Elliptic Curves."](https://books.google.co.in/books?id=6y_SmPc9fh4C&redir_esc=y)
-   📝 Joseph H. Silverman, ["An Introduction to the Theory of Elliptic Curves."](https://www.math.brown.edu/johsilve/Presentations/WyomingEllipticCurve.pdf)
-   📘 Neal Koblitz, ["A Course in Number Theory and Cryptography."](https://link.springer.com/book/10.1007/978-1-4419-8592-7)
-   📝 Ben Lynn, ["Stanford Crypto: Elliptic Curves."](https://crypto.stanford.edu/pbc/notes/elliptic/)
-   📝 Rareskills.io, ["Elliptic Curve Point Addition."](https://www.rareskills.io/post/elliptic-curve-addition)
-   📝 John D. Cook, ["Finite fields."](https://www.johndcook.com/blog/finite-fields/)

**Useful tools**

-   🎥 Tommy Occhipinti, ["Elliptic curves in Sage."](https://www.youtube.com/watch?v=-fRWR_QKzuI)
-   🎥 Desmos, ["Introduction to the Desmos Graphing Calculator."](https://www.youtube.com/watch?v=RKbZ3RoA-x4)
-   🧮 Andrea Corbellini, ["Interactive Elliptic Curve addition and multiplication."](https://andrea.corbellini.name/ecc/interactive/reals-add.html)