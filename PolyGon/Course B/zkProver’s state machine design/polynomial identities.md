https://docs.polygon.technology/zkEVM/concepts/mfibonacci/mfibonacci-example/#introducing-cyclicity


## Polynomial Identities
Polynomial Identities in Zero-Knowledge (ZK) Proofs: Polynomial identities are **mathematical equalities** involving polynomials that hold true regardless of the values assigned to the variables. In zero-knowledge (ZK) proofs, these identities are used to demonstrate the validity of computations without revealing the underlying data. ZK proofs allow a **prover** to convince a **verifier** that a statement is true, without disclosing any other information. Polynomial identities play a key role in constructing efficient ZK proofs, as they enable the **prover** to **compactly** represent and verify complex computations.


-   First, let's talk about polynomials: Polynomials are mathematical expressions with variables and coefficients. For example, x^2 + 2x + 1 is a polynomial.
-   Now, what are identities? Identities are equations that are always true, no matter what values you plug in. For example, (a + b)^2 = a^2 + 2ab + b^2 is an identity.
-   In ZK proofs, we use polynomial identities to prove things without revealing specific information: Imagine you want to prove you know a secret number, but you don't want to tell anyone what it is.
-   Here's how polynomial identities help:
    -   You can create polynomials using your secret number.
    -   These polynomials will have certain properties or relationships (identities).
    -   You can show these relationships are true without revealing the actual number.