The zkEVM's basic proof system, for proving correctness of all state machine computations, is a STARK


The zkProver's fundamental configuration:

-   It utilises **STARK proofs** for proving correctness of computations, due to their **speed**.
-   For succinct verification, these STARK proofs are in turn proved with a **single SNARK**.
-   So, it employs STARK proofs **internally**, while the publicised validity proofs are **SNARKs**.


What is a STARK?
------------------------------------------------------------------------------------------------------------------------------------------

A [STARK](https://eprint.iacr.org/2018/046.pdf) is a Scalable Transparent ARgument of Knowledge based on the [Interactive Oracle Proof](https://www.iacr.org/archive/tcc2016b/99850156/99850156.pdf) (IOP) model. Although a STARK is not adequately succinct as a proof system, it is generally categorised as a special [SNARK](https://eprint.iacr.org/2011/443.pdf) (which is short for Succinct Non-interactive ARgument of Knowledge).

*Succinctness* refers to producing short proofs that are independent of the size of the witness, and thus enabling NP computations to be proved with substantially lower complexity than it is classically required, [AN2019](https://www.di.ens.fr/~nitulesc/files/Survey-SNARKs.pdf). In other words, an argument system for NP statements is succinct, if its communication complexity is polylogarithmic in the size of the statement or the witness.

A STARK falls short of succinctness because, although verifier arithmetic complexity is strictly logarithmic with respect to statement or witness size, prover arithmetic complexity is strictly linear, [BBHR18](https://eprint.iacr.org/2018/046.pdf). Yet, a STARK is *scalable* because it has at most a polylogarithmic prover overhead, and it is transparent as it requires no trusted setup.

See the table below, taken from the presentation [here](https://docs.google.com/presentation/d/1gfB6WZMvM9mmDKofFibIgsyYShdf0RV_Y8TLz3k1Ls0/edit#slide=id.g443ebc39b4_0_110), for a quick comparison of proofs sizes, prover and verification times, between STARKs, SNARKs and [Bulletproofs](https://eprint.iacr.org/2017/1066.pdf)

.

![alt text](image-1.png)

