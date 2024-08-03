The zkEVM's basic proof system, for proving correctness of all state machine computations, is a STARK


The zkProver's fundamental configuration:

-   It utilises **STARK proofs** for proving correctness of computations, due to their **speed**.
-   For succinct verification, these STARK proofs are in turn proved with a **single SNARK**.
-   So, it employs STARK proofs **internally**, while the publicised validity proofs are **SNARKs**.