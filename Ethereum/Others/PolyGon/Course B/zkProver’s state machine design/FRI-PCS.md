### FRI-PCS context

The performance of a STARK is ascribable to the context in which it is deployed, the FRI polynomial commitment scheme (or FRI-PCS). The acronym  [FRI](https://drops.dagstuhl.de/opus/volltexte/2018/9018/pdf/LIPIcs-ICALP-2018-14.pdf)  is short for Fast Reed-Solomon interactive oracle proof of proximity, also abbreviated as “Fast RS-IOPP”.

On a high-level, FRI enables proving whether a given function  f:H→Fp  is “close” to a certain polynomial of low degree. Hence the term  _proof of proximity_.

Loosely put, the FRI protocol allows for a random set of queries (requests for openings of polynomials at randomly selected field elements), used by the verifier to ascertain with high probability, the prover’s knowledge of a committed polynomial.

FRI is in fact a Merkle commitment scheme where commitments are roots of Merkle trees, and therefore needs no trusted setup, as it only uses hash functions.

#### Why is FRI fast?

The FRI protocol is considered fast for the following reasons:

1.  Its resemblance of the ubiquitous Fast Fourier Transforms (FFTs).
2.  The strict linearity of the prover’s arithmetic complexity.
3.  The size of the proof is  O(nlog(n)).
4.  The verififer’s arithmetic complexity is strictly logarithmic.

Our special implementation of a STARK is called  _PIL-STARK_, and its polynomial commitment scheme is also based on the FRI protocol.

We later demonstrate how PIL-STARK is used to prove polynomial identities of the mFibonacci state machine.

Before describing PIL-STARK, let’s take a look at the novel polynomial identity language (PIL), and some of its distinguishing features.

### Polynomial Identity Language (PIL)

PIL is a domain-specific language (DSL) designed for naming polynomials and describing identities that define the computations performed by a state machine.

A typical .pil file for a given state machine specifies the details of the computation the state machine carries out.

-   The size (or degree) of the polynomials. i.e., the number of rows of the execution trace.
-   The namespace of the state machine, which becomes a prefix to names of the state machine's polynomials.
-   Defines the ISLAST polynomial as a constant (preprocessed) polynomial.
-   Defines committed polynomials; a and b.
-   The zero-checks of the transition constraints.
-   A zero-check of the boundary constraint.

In cases where several state machines are proved: - Each polynomial is identified by prefixing its name with the namespace of the state machine it belongs to. - Each state machine may have a polynomial named "ISLAST", yet there would be no name-clashes in the prover and verifier.

For example, mFibonacci.ISLAST is the identifier of ISLAST where the mFibonacci state machine has the namespace mFibonacci.

See the figure below for a description of the mFibonacci state machine in PIL, as an mFibonacci.pil file.


### Compiling PIL into JSON[¶](https://docs.polygon.technology/zkEVM/concepts/mfibonacci/verification-scheme/#compiling-pil-into-json "Link to this section")

Due to the modular design of the zkProver, it is possible to take a  .pil  file describing computations of any state machine and compile it into a parsed version, a  .json  file, that can be interpreted by the other software components.

We demonstrate compiling the  mFibonacci.pil  file into a  .json  file with a novel Polynomial Identities Language Compiler, dubbed  PILCOM. The details of  PILCOM  are documented elsewhere, but its repo can be found  [here](https://github.com/0xPolygonHermez/pilcom). We treat it as a ‘blackbox’ in this demonstration.