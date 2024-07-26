
- A layer 2 is EVM compatible or equivalent if it
can run EVM byte code without modifying the
underlying smart contract logic.
- EVM compatibility allow L2's to use existing
Ethereum smart contracts, patterns,
standards, and tooling.
- Being EVM compatible is important for the
widespread adoption of these L2 since this
allows using existing tools can be used.


### In practice, there are several types of
compatibility.
- Type 1: Fully Ethereum equivalent, i.e. they do
not change any part of the Ethereum system
but generating proofs can take several hours.

- Type 2: Fully EVM-equivalent, but changes some
different internal representations like how they
store the state of the chain, for the purpose of
improving ZK proof generation times.
- Type 2.5: Fully EVM-equivalent, except they use
different gas costs for some operations to
"significantly improve worst-case prover times".
- Type 3: Almost EVM-equivalent zkEVMs make
sacrifices in exact equivalence to further enhance
prover times and simplify EVM development.
- Type 4: High-level language equivalent zkEVMs
compile smart contract source code written in a
high-level language to a friendly language for zk,
resulting in faster prover times but potentially
introducing incompatibilities and limitations.